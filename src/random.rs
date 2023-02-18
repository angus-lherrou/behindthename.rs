use crate::constants::RANDOM_JSON_URL;
use crate::types::{Gender, Gender::*};
use url::Url;

pub fn random_with_params(
    gender: Gender,
    usage: Option<&str>,
    number: Option<u8>,
    random_surname: bool,
) -> impl FnOnce(&str) -> String + '_ {
    move |key| {
        let mut params: Vec<(&str, &str)> = vec![("key", key)];

        if let Some(u) = usage {
            params.push(("usage", u))
        }

        let gstring: String;
        match gender {
            Any => (),
            Ambiguous => {
                // This is needed because behindthename's "random" API only recognizes 'u' as the ambiguous/neutral key
                gstring = Neutral.to_string();
                params.push(("gender", &gstring))
            }
            g => {
                gstring = g.to_string();
                params.push(("gender", &gstring))
            }
        }

        let nstring: String;
        if let Some(n) = number {
            nstring = n.to_string();
            params.push(("number", &nstring))
        }

        if random_surname {
            params.push(("randomsurname", "yes"))
        }

        Url::parse_with_params(RANDOM_JSON_URL, params)
            .unwrap()
            .to_string()
    }
}

pub fn random() -> impl FnOnce(&str) -> String {
    random_with_params(Any, None, None, false)
}

pub fn random_with_surname() -> impl FnOnce(&str) -> String {
    random_with_params(Any, None, None, true)
}

pub fn random_with_gender(gender: Gender) -> impl FnOnce(&str) -> String {
    random_with_params(gender, None, None, false)
}

pub fn random_with_usage(usage: &str) -> impl FnOnce(&str) -> String + '_ {
    random_with_params(Any, Some(usage), None, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random() {
        let req = random();
        assert_eq!(
            req("asdf"),
            "https://www.behindthename.com/api/random.json?key=asdf"
        );
    }

    #[test]
    fn test_random_with_surname() {
        let req = random_with_surname();
        assert_eq!(
            req("asdf"),
            "https://www.behindthename.com/api/random.json?key=asdf&randomsurname=yes"
        );
    }

    #[test]
    fn test_random_with_gender() {
        let req_male = random_with_gender(Male);
        assert_eq!(
            req_male("asdf"),
            "https://www.behindthename.com/api/random.json?key=asdf&gender=m"
        );
        let req_female = random_with_gender(Female);
        assert_eq!(
            req_female("asdf"),
            "https://www.behindthename.com/api/random.json?key=asdf&gender=f"
        );
        let req_neutral = random_with_gender(Neutral);
        assert_eq!(
            req_neutral("asdf"),
            "https://www.behindthename.com/api/random.json?key=asdf&gender=u"
        );
        let req_ambiguous = random_with_gender(Ambiguous);
        assert_eq!(
            req_ambiguous("asdf"),
            "https://www.behindthename.com/api/random.json?key=asdf&gender=u"
        );
        let req_any = random_with_gender(Any);
        assert_eq!(
            req_any("asdf"),
            "https://www.behindthename.com/api/random.json?key=asdf"
        );
    }

    #[test]
    fn test_random_with_usage() {
        let req = random_with_usage("eng");
        assert_eq!(
            req("asdf"),
            "https://www.behindthename.com/api/random.json?key=asdf&usage=eng"
        );
    }

    #[test]
    fn test_random_with_params() {
        let req = random_with_params(Gender::Female, Some("ita"), Some(5), true);
        assert_eq!(req("asdf"), "https://www.behindthename.com/api/random.json?key=asdf&usage=ita&gender=f&number=5&randomsurname=yes");
    }
}
