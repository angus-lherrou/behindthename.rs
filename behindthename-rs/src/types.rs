use std::fmt;
use governor::clock::{Clock, DefaultClock};
use governor::NotUntil;
use serde::Deserialize;
use Gender::*;

#[derive(Clone, Copy, Deserialize, Debug)]
pub enum Gender {
    #[serde(rename = "m")]
    Male,
    #[serde(rename = "f")]
    Female,
    #[serde(rename = "mf")]
    Neutral,
    Any,
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match *self {
            Male => "m",
            Female => "f",
            Neutral => "mf",
            Any => ""
        })
    }
}

#[derive(Deserialize)]
pub struct JsonNotAvailable {
    pub error_code: usize,
    pub error: String
}

#[derive(Deserialize, Debug)]
pub struct Usage {
    usage_code: String,
    usage_full: String,
    usage_gender: Gender,
}

#[derive(Deserialize, Debug)]
pub struct JsonNameDetailItem {
    name: String,
    gender: String,
    usages: Vec<Usage>
}

#[derive(Deserialize, Debug)]
pub struct JsonNameDetails(Vec<JsonNameDetailItem>);

#[derive(Deserialize, Debug)]
pub struct JsonNameList {
    names: Vec<String>
}

#[derive(Debug)]
pub enum JsonResponseBody {
    NameDetails(JsonNameDetails),
    NameList(JsonNameList)
}

pub enum JsonResponse {
    Okay(JsonResponseBody),
    NotAvailable(JsonNotAvailable),
}

pub(crate) type DefaultInstant = <DefaultClock as Clock>::Instant;

pub enum RateLimited<'a, T, E> {
    Allowed(T),
    Limited(&'static str, NotUntil<'a, DefaultInstant>),
    Error(E)
}