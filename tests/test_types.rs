use behindthename::types::*;
use Gender::*;

#[test]
fn test_serde_deserialize_gender() {
    match serde_json::from_str::<Gender>(r#""m""#) {
        Ok(Male) => (),
        Ok(g) => panic!("{}", g),
        Err(e) => panic!("{}", e)
    }
}

#[test]
fn test_from_str_gender() {
    match str::parse::<Gender>("m") {
        Ok(Male) => (),
        Ok(g) => panic!("{}", g),
        Err(e) => panic!("{}", e)
    }
    match str::parse::<Gender>("f") {
        Ok(Female) => (),
        Ok(g) => panic!("{}", g),
        Err(e) => panic!("{}", e)
    }
    match str::parse::<Gender>("u") {
        Ok(Neutral) => (),
        Ok(g) => panic!("{}", g),
        Err(e) => panic!("{}", e)
    }
    match str::parse::<Gender>("mf") {
        Ok(Ambiguous) => (),
        Ok(g) => panic!("{}", g),
        Err(e) => panic!("{}", e)
    }
    match str::parse::<Gender>("fm") {
        Ok(Ambiguous) => (),
        Ok(g) => panic!("{}", g),
        Err(e) => panic!("{}", e)
    }
    match str::parse::<Gender>("ynA") {
        Ok(Any) => panic!("shouldn't be able to create Any from \"ynA\""),
        Ok(g) => panic!("{}", g),
        Err(e) => ()
    }
    match str::parse::<Gender>("Any") {
        Ok(Any) => (),
        Ok(g) => panic!("{}", g),
        Err(e) => panic!("{}", e)
    }
}

#[test]
fn test_display_gender() {
    assert_eq!(
        "m",
        format!("{}", Male)
    );
    assert_eq!(
        "f",
        format!("{}", Female)
    );
    assert_eq!(
        "u",
        format!("{}", Neutral)
    );
    assert_eq!(
        "mf",
        format!("{}", Ambiguous)
    );
    assert_eq!(
        "",
        format!("{}", Any)
    );
}