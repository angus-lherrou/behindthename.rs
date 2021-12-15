use crate::types::*;
use governor::clock::DefaultClock;
use governor::state::{InMemoryState, NotKeyed};
use governor::{NotUntil, Quota, RateLimiter};
use nonzero_ext::nonzero;
use reqwest::blocking::{Client, Response};
use serde_json::from_str;
use std::cmp::max_by;
use std::cmp::Ordering;
use std::fmt::Formatter;
use std::num::{NonZeroU32, NonZeroU64};
use std::time::Duration;
use RateLimited::*;

type DirectRateLimiter = RateLimiter<NotKeyed, InMemoryState, DefaultClock>;

static DEFAULT_USAGE_LIMIT: UsageLimit = UsageLimit {
    per_second: nonzero!(2u32),
    per_hour: nonzero!(400u32),
    per_day: nonzero!(4_000u64),
    per_year: nonzero!(400_000u64),
};

static LIMIT_INTERVALS: [&str; 4] = ["Second", "Hour", "Day", "Year"];

pub struct Session<'a> {
    pub key: &'a str,
    limiters: RateLimiters<'a>,
    client: Client,
}

impl std::fmt::Display for Session<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Session(key: {}, limiters: {})", self.key, self.limiters)
    }
}

impl Session<'_> {
    pub fn new<'a>(key: &'a str, usage_limit: &'a UsageLimit) -> Session<'a> {
        Session {
            key,
            limiters: usage_limit.create_limiters(),
            client: Client::new(),
        }
    }

    pub fn new_default(key: &str) -> Session {
        Session::new(key, &DEFAULT_USAGE_LIMIT)
    }

    fn check(&self) -> Result<(), (&'static str, NotUntil<'_, DefaultInstant>)> {
        match self.limiters.check() {
            Ok(_) => Ok(()),
            Err(earliest) => Err(earliest),
        }
    }

    fn request_internal(
        &self,
        req: impl FnOnce(&str) -> String,
    ) -> RateLimited<'_, Response, (), reqwest::Error> {
        match self.check() {
            Err((i, earliest)) => Governed(i, earliest),
            Ok(_) => match self.client.get(req(self.key)).send() {
                Err(e) => Error(e),
                Ok(resp) => Allowed(resp),
            },
        }
    }

    pub fn request(
        &self,
        req: impl FnOnce(&str) -> String,
    ) -> RateLimited<'_, JsonResponse, NotAvailable, reqwest::Error> {
        match self.request_internal(req) {
            Allowed(resp) => {
                let text = resp.text().unwrap();
                let text_str = text.as_str();
                match from_str::<JsonNameDetails>(text_str) {
                    Ok(jnd) => Allowed(JsonResponse::NameDetails(jnd)),
                    Err(_) => match from_str::<JsonNameList>(text_str) {
                        Ok(jnl) => Allowed(JsonResponse::NameList(jnl)),
                        Err(_) => match from_str::<NotAvailable>(text_str) {
                            Ok(e) => Limited(e),
                            Err(_) => panic!("Failed to parse {:?} with any branch", text_str),
                        },
                    },
                }
            },
            Limited(_) => unreachable!(), // we should never generate Limited from the internal request
            Governed(i, n) => Governed(i, n),
            Error(e) => Error(e),
        }
    }
}

struct RateLimiters<'a> {
    limits: &'a UsageLimit,
    limiters: Vec<DirectRateLimiter>,
}

impl std::fmt::Display for RateLimiters<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let &UsageLimit {
            per_second,
            per_hour,
            per_day,
            per_year,
        } = self.limits;
        write!(
            f,
            "RateLimiters({} per second, {} per hour, {} per day, {} per year)",
            per_second, per_hour, per_day, per_year
        )
    }
}

impl RateLimiters<'_> {
    fn check(&self) -> Result<(), (&'static str, NotUntil<'_, DefaultInstant>)> {
        let (_, not_untils): (_, Vec<_>) = self
            .limiters
            .iter()
            .enumerate()
            .map(|(i, item)| (i, item.check()))
            .partition(|(_, item)| item.is_ok());
        if not_untils.is_empty() {
            return Ok(());
        }
        let (i, earliest): (usize, Option<NotUntil<'_, DefaultInstant>>) =
            not_untils.into_iter().fold(
                (usize::MAX, None),
                |(i, acc): (usize, Option<NotUntil<'_, DefaultInstant>>),
                 (j, res): (usize, Result<(), NotUntil<'_, DefaultInstant>>)| {
                    max_by(
                        (i, acc),
                        (j, Some(res.unwrap_err())),
                        |(_, v1), (_, v2)| match (v1, v2) {
                            (None, None) => Ordering::Equal,
                            (Some(_), None) => Ordering::Greater,
                            (None, Some(_)) => Ordering::Less,
                            (Some(nu_1), Some(nu_2)) => {
                                nu_1.earliest_possible().cmp(&nu_2.earliest_possible())
                            }
                        },
                    )
                },
            );
        Err((LIMIT_INTERVALS[i], earliest.unwrap()))
    }
}

pub struct UsageLimit {
    per_second: NonZeroU32,
    per_hour: NonZeroU32,
    per_day: NonZeroU64,
    per_year: NonZeroU64,
}

impl UsageLimit {
    fn create_limiters(&self) -> RateLimiters {
        RateLimiters {
            limits: self,
            limiters: vec![
                RateLimiter::direct(
                    Quota::per_second(self.per_second).allow_burst(self.per_second),
                ),
                RateLimiter::direct(Quota::per_hour(self.per_hour).allow_burst(self.per_hour)),
                RateLimiter::direct(
                    Quota::with_period(Duration::from_secs(60 * 60 * 24 / self.per_day))
                        .unwrap()
                        .allow_burst(NonZeroU32::try_from(self.per_day).unwrap()),
                ),
                RateLimiter::direct(
                    Quota::with_period(Duration::from_secs(60 * 60 * 24 * 365 / self.per_year))
                        .unwrap()
                        .allow_burst(NonZeroU32::try_from(self.per_year).unwrap()),
                ),
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_default_session() {
        let _ = Session::new_default("some_key");
    }

    #[test]
    fn test_construct_custom_session() {
        let usage_limit = UsageLimit {
            per_second: nonzero!(4u32),
            per_hour: nonzero!(24u32),
            per_day: nonzero!(90u64),
            per_year: nonzero!(1000u64),
        };
        let _ = Session::new("some_key", &usage_limit);
    }
}
