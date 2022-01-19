use crate::constants::RELATED_JSON_URL;
use crate::types::{Gender, Gender::*};
use url::Url;

pub fn related_with_params<'a>(
    name: &'a str,
    usage: Option<&'a str>,
    gender: Gender,
) -> impl FnOnce(&str) -> String + 'a {
    move |key| {
        let mut params: Vec<(&str, &str)> = vec![("key", key), ("name", name)];

        if let Some(u) = usage {
            params.push(("usage", u))
        }

        let gstring: String;
        match gender {
            Any => (),
            Neutral => {
                // This is needed because behindthename's "related" API only recognizes 'mf' as the ambiguous/neutral key
                gstring = Ambiguous.to_string();
                params.push(("gender", &gstring))
            },
            g => {
                gstring = g.to_string();
                params.push(("gender", &gstring))
            }
        }

        Url::parse_with_params(RELATED_JSON_URL, params)
            .unwrap()
            .to_string()
    }
}

pub fn related(name: &str) -> impl FnOnce(&str) -> String + '_ {
    related_with_params(name, None, Any)
}

pub fn related_with_usage<'a>(name: &'a str, usage: &'a str) -> impl FnOnce(&str) -> String + 'a {
    related_with_params(name, Some(usage), Any)
}

pub fn related_with_gender(name: &str, gender: Gender) -> impl FnOnce(&str) -> String + '_ {
    related_with_params(name, None, gender)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_related() {
        let req = related("Richard");
        assert_eq!(
            req("asdf"),
            "https://www.behindthename.com/api/related.json?key=asdf&name=Richard"
        );
    }

    #[test]
    fn test_related_with_usage() {
        let req = related_with_usage("Rebecca", "eng");
        assert_eq!(
            req("asdf"),
            "https://www.behindthename.com/api/related.json?key=asdf&name=Rebecca&usage=eng"
        );
    }

    #[test]
    fn test_related_with_gender() {
        let req_male = related_with_gender("Jordan", Male);
        assert_eq!(
            req_male("asdf"),
            "https://www.behindthename.com/api/related.json?key=asdf&name=Jordan&gender=m"
        );
        let req_female = related_with_gender("Jordan", Female);
        assert_eq!(
            req_female("asdf"),
            "https://www.behindthename.com/api/related.json?key=asdf&name=Jordan&gender=f"
        );
        let req_neutral = related_with_gender("Jordan", Neutral);
        assert_eq!(
            req_neutral("asdf"),
            "https://www.behindthename.com/api/related.json?key=asdf&name=Jordan&gender=mf"
        );
        let req_ambiguous = related_with_gender("Jordan", Ambiguous);
        assert_eq!(
            req_ambiguous("asdf"),
            "https://www.behindthename.com/api/related.json?key=asdf&name=Jordan&gender=mf"
        );
        let req_any = related_with_gender("Jordan", Any);
        assert_eq!(
            req_any("asdf"),
            "https://www.behindthename.com/api/related.json?key=asdf&name=Jordan"
        );
    }

    #[test]
    fn test_related_with_params() {
        let req = related_with_params("Sasha", Some("rus"), Male);
        assert_eq!(
            req("asdf"),
            "https://www.behindthename.com/api/related.json?key=asdf&name=Sasha&usage=rus&gender=m"
        );
    }
}
