use governor::clock::{Clock, DefaultClock};
use governor::NotUntil;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::json;
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy, Deserialize, Serialize, Debug, PartialEq, Eq, Hash)]
pub enum Gender {
    #[serde(rename = "m")]
    Male,
    #[serde(rename = "f")]
    Female,
    #[serde(rename = "u")]
    Neutral,
    #[serde(rename = "mf")]
    #[serde(rename(deserialize = "fm"))]
    Ambiguous,
    #[serde(rename(serialize = ""))]
    Any,
}

impl FromStr for Gender {
    type Err = serde_json::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_value::<Gender>(json!(s))
    }
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = serde_json::to_value(self).unwrap();
        write!(f, "{}", val.as_str().unwrap())
    }
}

#[derive(Deserialize, Debug)]
pub struct RemoteError {
    pub error_code: usize,
    pub error: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
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

pub enum RateLimited<'a, S, E> {
    Allowed(S),
    Governed(&'static str, NotUntil<'a, DefaultInstant>),
    Failed(E),
    ReqwestError(reqwest::Error),
}
