use crate::constants::RANDOM_JSON_URL;
use crate::types::{Gender, Gender::*};

pub fn random_with_params<'a>(
    gender: Gender,
    usage: &'a str,
    number: u8,
    random_surname: bool,
) -> impl Fn(&str) -> String + 'a {
    let surname_key = if random_surname { "yes" } else { "no" }.to_string();

    let usage_segment = if usage.is_empty() {
        "".to_string()
    } else {
        "&usage=".to_owned() + usage
    };

    let gender_segment = if format!("{}", gender).is_empty() {
        "".to_string()
    } else {
        format!("&gender={}", gender)
    };

    move |key| {
        format!(
            "{}?key={}{}{}&number={}&randomsurname={}",
            RANDOM_JSON_URL, key, usage_segment, gender_segment, number, surname_key
        )
    }
}

pub fn random() -> impl Fn(&str) -> String {
    random_with_params(Any, "", 1, false)
}

pub fn random_with_surname() -> impl Fn(&str) -> String {
    random_with_params(Any, "", 1, true)
}

pub fn random_with_gender(gender: Gender) -> impl Fn(&str) -> String {
    random_with_params(gender, "", 1, false)
}

pub fn random_with_usage<'a>(usage: &'a str) -> impl Fn(&str) -> String + 'a {
    random_with_params(Any, usage, 1, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random() {
        let req = random();
        assert_eq!(
            req("asdf"),
            "https://www.behindthename.com/api/random.json?key=asdf&number=1&randomsurname=no"
        );
    }

    #[test]
    fn test_random_with_surname() {
        let req = random_with_surname();
        assert_eq!(
            req("asdf"),
            "https://www.behindthename.com/api/random.json?key=asdf&number=1&randomsurname=yes"
        );
    }

    #[test]
    fn test_random_with_gender() {
        let req_male = random_with_gender(Male);
        assert_eq!(req_male("asdf"), "https://www.behindthename.com/api/random.json?key=asdf&gender=m&number=1&randomsurname=no");
        let req_female = random_with_gender(Female);
        assert_eq!(req_female("asdf"), "https://www.behindthename.com/api/random.json?key=asdf&gender=f&number=1&randomsurname=no");
        let req_neutral = random_with_gender(Neutral);
        assert_eq!(req_neutral("asdf"), "https://www.behindthename.com/api/random.json?key=asdf&gender=mf&number=1&randomsurname=no");
        let req_any = random_with_gender(Any);
        assert_eq!(
            req_any("asdf"),
            "https://www.behindthename.com/api/random.json?key=asdf&number=1&randomsurname=no"
        );
    }

    #[test]
    fn test_random_with_usage() {
        let req = random_with_usage("eng");
        assert_eq!(req("asdf"), "https://www.behindthename.com/api/random.json?key=asdf&usage=eng&number=1&randomsurname=no");
    }

    #[test]
    fn test_random_with_params() {
        let req = random_with_params(Gender::Female, "ita", 5, true);
        assert_eq!(req("asdf"), "https://www.behindthename.com/api/random.json?key=asdf&usage=ita&gender=f&number=5&randomsurname=yes");
    }
}
