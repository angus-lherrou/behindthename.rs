use crate::constants::LOOKUP_JSON_URL;
use url::Url;

fn _lookup(name: &str, exact: bool) -> impl FnOnce(&str) -> String + '_ {
    move |key| {
        let mut params: Vec<(&str, &str)> = vec![("key", key), ("name", name)];

        if exact {
            params.push(("exact", "yes"))
        }

        Url::parse_with_params(LOOKUP_JSON_URL, params)
            .unwrap()
            .to_string()
    }
}

pub fn lookup(name: &str) -> impl FnOnce(&str) -> String + '_ {
    _lookup(name, false)
}

pub fn lookup_exact(name: &str) -> impl FnOnce(&str) -> String + '_ {
    _lookup(name, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup_exact() {
        let req = lookup_exact("Angus");
        assert_eq!(
            req("asdf"),
            "https://www.behindthename.com/api/lookup.json?key=asdf&name=Angus&exact=yes"
        );
    }

    #[test]
    fn test_lookup() {
        let req = lookup("Angus");
        assert_eq!(
            req("asdf"),
            "https://www.behindthename.com/api/lookup.json?key=asdf&name=Angus"
        );
    }
}
