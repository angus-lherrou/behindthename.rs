use governor::clock::{Clock, DefaultClock};
use governor::NotUntil;
use serde::Deserialize;
use std::fmt;
use Gender::*;
use serde_json;
use std::str::FromStr;

#[derive(Clone, Copy, Deserialize, Debug)]
pub enum Gender {
    #[serde(rename = "m")]
    Male,
    #[serde(rename = "f")]
    Female,
    #[serde(rename = "u")]
    Neutral,
    #[serde(rename = "mf")]
    Ambiguous,
    Any
}

impl FromStr for Gender {
    type Err = serde_json::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str::<Gender>(format!("\"{}\"", s).as_str())
            // todo: hotfix to catch `fm` for ambiguous
            .or_else(|err| {
                if s == "fm" {
                    Ok(Ambiguous)
                } else { Err(err) }
            })
    }
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Male => "m",
                Female => "f",
                Neutral => "u",
                Ambiguous => "mf",
                Any => "",
            }
        )
    }
}

#[derive(Deserialize)]
pub struct NotAvailable {
    pub error_code: usize,
    pub error: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Usage {
    pub usage_code: String,
    pub usage_full: String,
    pub usage_gender: Gender,
}

#[derive(Deserialize, Debug)]
pub struct JsonNameDetailItem {
    pub name: String,
    pub gender: String,
    pub usages: Vec<Usage>,
}

#[derive(Deserialize, Debug)]
pub struct JsonNameDetails(pub Vec<JsonNameDetailItem>);

#[derive(Deserialize, Debug)]
pub struct JsonNameList {
    pub names: Vec<String>,
}

#[derive(Debug)]
pub enum JsonResponse {
    NameDetails(JsonNameDetails),
    NameList(JsonNameList),
}

pub(crate) type DefaultInstant = <DefaultClock as Clock>::Instant;

pub enum RateLimited<'a, S, T, E> {
    Allowed(S),
    Governed(&'static str, NotUntil<'a, DefaultInstant>),
    Limited(T),
    Error(E),
}
