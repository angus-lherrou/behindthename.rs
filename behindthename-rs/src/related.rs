use crate::constants::RELATED_JSON_URL;
use crate::types::{Gender, Gender::*};


pub fn related_with_params<'a>(
    name: &'a str,
    usage: &'a str,
    gender: Gender,
) -> impl Fn(&str) -> String + 'a {
    let usage_segment = if usage.is_empty() { "".to_string() } else { "&usage=".to_owned() + usage };
    let gender_segment = if format!("{}", gender).is_empty() { "".to_string() } else { format!("&gender={}", gender) };
    move |key| format!(
        "{}?key={}&name={}{}{}",
        RELATED_JSON_URL, key, name, usage_segment, gender_segment,
    )
}

pub fn related<'a>(
    name: &'a str,
) -> impl Fn(&str) -> String + 'a {
    related_with_params(name, "", Any)
}

pub fn related_with_usage<'a>(
    name: &'a str,
    usage: &'a str,
) -> impl Fn(&str) -> String + 'a {
    related_with_params(name, usage, Any)
}

pub fn related_with_gender<'a>(
    name: &'a str,
    gender: Gender,
) -> impl Fn(&str) -> String + 'a {
    related_with_params(name, "", gender)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_related() {
        let req = related("Richard");
        assert_eq!(req("asdf"), "https://www.behindthename.com/api/related.json?key=asdf&name=Richard");
    }

    #[test]
    fn test_related_with_usage() {
        let req = related_with_usage("Rebecca", "eng");
        assert_eq!(req("asdf"), "https://www.behindthename.com/api/related.json?key=asdf&name=Rebecca&usage=eng");
    }

    #[test]
    fn test_related_with_gender() {
        let req_male = related_with_gender("Jordan", Male);
        assert_eq!(req_male("asdf"), "https://www.behindthename.com/api/related.json?key=asdf&name=Jordan&gender=m");
        let req_female = related_with_gender("Jordan", Female);
        assert_eq!(req_female("asdf"), "https://www.behindthename.com/api/related.json?key=asdf&name=Jordan&gender=f");
        let req_neutral = related_with_gender("Jordan", Neutral);
        assert_eq!(req_neutral("asdf"), "https://www.behindthename.com/api/related.json?key=asdf&name=Jordan&gender=mf");
        let req_any = related_with_gender("Jordan", Any);
        assert_eq!(req_any("asdf"), "https://www.behindthename.com/api/related.json?key=asdf&name=Jordan");
    }

    #[test]
    fn test_related_with_params() {
        let req = related_with_params("Sasha", "rus", Male);
        assert_eq!(req("asdf"), "https://www.behindthename.com/api/related.json?key=asdf&name=Sasha&usage=rus&gender=m");
    }
}