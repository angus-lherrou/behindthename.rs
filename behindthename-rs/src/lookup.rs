use crate::constants::LOOKUP_JSON_URL;


fn _lookup<'a>(name: &'a str, exact: bool) -> impl Fn(&str) -> String + 'a {
    let exact_key = if exact { "yes" } else { "no" }.to_string();
    move |key| format!("{}?key={}&name={}&exact={}", LOOKUP_JSON_URL, key, name, exact_key)
}

pub fn lookup<'a>(name: &'a str) -> impl Fn(&str) -> String + 'a {
    _lookup(name, false)
}

pub fn lookup_exact<'a>(name: &'a str) -> impl Fn(&str) -> String + 'a {
    _lookup(name, true)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup_exact() {
        let req = lookup_exact("Angus");
        assert_eq!(req("asdf"), "https://www.behindthename.com/api/lookup.json?key=asdf&name=Angus&exact=yes");
    }

    #[test]
    fn test_lookup() {
        let req = lookup("Angus");
        assert_eq!(req("asdf"), "https://www.behindthename.com/api/lookup.json?key=asdf&name=Angus&exact=no");
    }
}