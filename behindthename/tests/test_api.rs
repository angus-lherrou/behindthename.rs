use std::env;
use std::thread::sleep;
use std::time::Duration;

use behindthename::{lookup, random, session, types::*};
use Gender::*;
use JsonResponse::*;
use RateLimited::*;

#[test]
fn test_env_session_key() {
    let key_string = env::var("BTN_API_KEY").unwrap_or_else(|_| "none".to_string());
    let key = key_string.as_str();
    let sesh = session::Session::new_default(key);
    assert_eq!(
        env::var("BTN_API_KEY").unwrap_or_else(|_| "none".to_string()),
        sesh.key
    )
}

#[test]
fn test_rate_limit() {
    let key_string = env::var("BTN_API_KEY").unwrap_or_else(|_| "none".to_string());
    let key = key_string.as_str();
    let sesh = session::Session::new_default(key);
    let req_1 = lookup::lookup("Jordan");
    let req_2 = random::random_with_params(Male, Some("ita"), Some(2), true);
    let req_3 = lookup::lookup("Emily");
    let req_4 = lookup::lookup("Joanne");

    match sesh.request(req_1) {
        Allowed(r) => println!("first request: {:?}", r.text().unwrap()),
        Limited(i, n) => panic!("first request failed: limiter {}, {:?}", i, n),
        Error(e) => panic!("first request failed: {:?}", e),
    };
    match sesh.request(req_2) {
        Allowed(r) => println!("second request: {:?}", r.text().unwrap()),
        Limited(i, n) => panic!("second request failed: limiter {}, {:?}", i, n),
        Error(e) => panic!("second request failed: {:?}", e),
    };
    // whether third or fourth request is the one that fails is up to chance
    match sesh.request(req_3) {
        Allowed(r) => match sesh.request(req_4) {
            Allowed(s) => panic!(
                "third and fourth requests succeeded: {:?}, {:?}",
                r.text().unwrap(),
                s.text().unwrap()
            ),
            Limited(i, n) => {
                println! {"third request: {:?}; fourth request: limiter {}, {:?}", r.text().unwrap(), i, n}
            }
            Error(e) => panic!("fourth request failed: {:?}", e),
        },
        Limited(i, n) => println! {"third request: limiter {}, {:?}", i, n},
        Error(e) => panic!("third request failed: {:?}", e),
    };
}

#[test]
fn test_json_lookup() {
    let key_string = env::var("BTN_API_KEY").unwrap_or_else(|_| "none".to_string());
    let key = key_string.as_str();
    let sesh = session::Session::new_default(key);
    let req = lookup::lookup("Jordan");

    match sesh.request_json(req) {
        Allowed(Okay(JsonResponseBody::NameDetails(e))) => println!("{:?}", e),
        Allowed(Okay(JsonResponseBody::NameList(e))) => {
            panic!("request parsed as name list: {:?}", e)
        }
        Allowed(NotAvailable(_)) => panic!("request service unavailable"),
        Limited(i, n) => panic!("request failed: limiter {}, {:?}", i, n),
        Error(e) => panic!("request failed: {:?}", e),
    };

    sleep(Duration::from_secs(2));
}

#[test]
fn test_json_random() {
    let key_string = env::var("BTN_API_KEY").unwrap_or_else(|_| "none".to_string());
    let key = key_string.as_str();
    let sesh = session::Session::new_default(key);
    let req = random::random_with_params(Male, Some("ita"), Some(2), true);

    match sesh.request_json(req) {
        Allowed(Okay(JsonResponseBody::NameList(e))) => println!("{:?}", e),
        Allowed(Okay(JsonResponseBody::NameDetails(e))) => {
            panic!("request parsed as name details: {:?}", e)
        }
        Allowed(NotAvailable(_)) => panic!("request service unavailable"),
        Limited(i, n) => panic!("request failed: limiter {}, {:?}", i, n),
        Error(e) => panic!("request failed: {:?}", e),
    };

    sleep(Duration::from_secs(2));
}

#[test]
fn test_json_service_unavailable() {
    let key_string = env::var("BTN_API_KEY").unwrap_or_else(|_| "none".to_string());
    let key = key_string.as_str();
    let sesh = session::Session::new_default(key);
    let req_1 = lookup::lookup("Jordan");
    let req_2 = random::random_with_params(Male, Some("ita"), Some(2), true);
    let req_3 = lookup::lookup("Emily");
    let req_4 = lookup::lookup("Joanne");

    match sesh.request(req_1) {
        Allowed(r) => println!("first request: {:?}", r.text().unwrap()),
        Limited(i, n) => panic!("first request failed: limiter {}, {:?}", i, n),
        Error(e) => panic!("first request failed: {:?}", e),
    };
    match sesh.request(req_2) {
        Allowed(r) => println!("second request: {:?}", r.text().unwrap()),
        Limited(i, n) => panic!("second request failed: limiter {}, {:?}", i, n),
        Error(e) => panic!("second request failed: {:?}", e),
    };
    // whether third or fourth request is the one that fails is up to chance
    match sesh.request_json(req_3) {
        Allowed(NotAvailable(e)) => assert_eq!(
            (e.error_code, e.error),
            (2usize, "service not available".to_string())
        ),
        Allowed(Okay(r)) => match sesh.request_json(req_4) {
            Allowed(NotAvailable(e)) => assert_eq!(
                (e.error_code, e.error),
                (2usize, "service not available".to_string())
            ),
            Allowed(Okay(s)) => panic!("third and fourth requests succeeded: {:?}, {:?}", r, s),
            Limited(i, n) => {
                println! {"third request: {:?}; fourth request: limiter {}, {:?}", r, i, n}
            }
            Error(e) => panic!("fourth request failed: {:?}", e),
        },
        Limited(i, n) => println! {"third request: limiter {}, {:?}", i, n},
        Error(e) => panic!("third request failed: {:?}", e),
    };
}
