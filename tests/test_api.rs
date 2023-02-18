use serial_test::serial;
use std::env;
use std::thread::sleep;
use std::time::Duration;

use behindthename::{lookup, random, session, types::*};
use Gender::*;
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

// #[test]
// #[serial]
// fn test_rate_limit() {
//     let key_string = env::var("BTN_API_KEY").unwrap_or_else(|_| "none".to_string());
//     let key = key_string.as_str();
//     let sesh = session::Session::new_default(key);
//     let req_1 = lookup::lookup("Jordan");
//     let req_2 = random::random_with_params(Male, Some("ita"), Some(2), true);
//     let req_3 = lookup::lookup("Emily");
//     let req_4 = lookup::lookup("Joanne");
//
//     match sesh.request_internal(req_1) {
//         Allowed(r) => println!("first request: {:?}", r.text().unwrap()),
//         Governed(i, n) => panic!("first request failed: limiter {}, {:?}", i, n),
//         Error(e) => panic!("first request failed: {:?}", e),
//     };
//     match sesh.request_internal(req_2) {
//         Allowed(r) => println!("second request: {:?}", r.text().unwrap()),
//         Governed(i, n) => panic!("second request failed: limiter {}, {:?}", i, n),
//         Error(e) => panic!("second request failed: {:?}", e),
//     };
//     // whether third or fourth request is the one that fails is up to chance
//     // TODO: this test is not quite right as it does not catch ``Allowed`` as service not available
//     match sesh.request_internal(req_3) {
//         Allowed(r) => match sesh.request_internal(req_4) {
//             Allowed(s) => panic!(
//                 "third and fourth requests succeeded: {:?}, {:?}",
//                 r.text().unwrap(),
//                 s.text().unwrap()
//             ),
//             Governed(i, n) => {
//                 let status = r.status();
//                 println! {"third request: {}, {}; fourth request: limiter {}, {:?}", r.text().unwrap(), status, i, n}
//             }
//             Error(e) => panic!("fourth request failed: {:?}", e),
//         },
//         Governed(i, n) => println! {"third request: limiter {}, {:?}", i, n},
//         Error(e) => panic!("third request failed: {:?}", e),
//     };
//     sleep(Duration::from_secs(2));
// }

#[test]
#[serial]
fn test_json_lookup() {
    let key_string = env::var("BTN_API_KEY").unwrap_or_else(|_| "none".to_string());
    let key = key_string.as_str();
    let sesh = session::Session::new_default(key);
    let req = lookup::lookup("Jordan");

    match sesh.request(req) {
        Allowed(JsonResponse::NameDetails(e)) => println!("{:?}", e),
        Allowed(JsonResponse::NameList(e)) => {
            panic!("request parsed as name list: {:?}", e)
        }
        Failed(_) => panic!("first request failed: limited by API"),
        Governed(i, n) => panic!("request failed: limiter {}, {:?}", i, n),
        ReqwestError(e) => panic!("request failed: {:?}", e),
    };

    sleep(Duration::from_secs(2));
}

#[test]
#[serial]
fn test_json_random() {
    let key_string = env::var("BTN_API_KEY").unwrap_or_else(|_| "none".to_string());
    let key = key_string.as_str();
    let sesh = session::Session::new_default(key);
    let req = random::random_with_params(Male, Some("ita"), Some(2), true);

    match sesh.request(req) {
        Allowed(JsonResponse::NameList(e)) => println!("{:?}", e),
        Allowed(JsonResponse::NameDetails(e)) => {
            panic!("request parsed as name details: {:?}", e)
        }
        Failed(_) => panic!("first request failed: limited by API"),
        Governed(i, n) => panic!("request failed: limiter {}, {:?}", i, n),
        ReqwestError(e) => panic!("request failed: {:?}", e),
    };

    sleep(Duration::from_secs(2));
}

#[test]
#[serial]
fn test_json_service_unavailable() {
    let key_string = env::var("BTN_API_KEY").unwrap_or_else(|_| "none".to_string());
    let key = key_string.as_str();
    let sesh = session::Session::new_default(key);
    let req_1 = lookup::lookup("Jordan");
    let req_2 = random::random_with_params(Male, Some("ita"), Some(2), true);
    let req_3 = lookup::lookup("Emily");
    let req_4 = lookup::lookup("Joanne");

    match sesh.request(req_1) {
        Allowed(r) => println!("first request: {:?}", r),
        Governed(i, n) => panic!("first request failed: limiter {}, {:?}", i, n),
        Failed(_) => panic!("first request failed: limited by API"),
        ReqwestError(e) => panic!("first request failed: {:?}", e),
    };
    match sesh.request(req_2) {
        Allowed(r) => println!("second request: {:?}", r),
        Governed(i, n) => panic!("second request failed: limiter {}, {:?}", i, n),
        Failed(_) => panic!("second request failed: limited by API"),
        ReqwestError(e) => panic!("second request failed: {:?}", e),
    };
    // whether third or fourth request is the one that fails is up to chance
    match sesh.request(req_3) {
        Failed(e) => {
            assert_eq!(
                (e.error_code, e.error),
                (2usize, "service not available".to_string())
            );
            println!("got NotAvailable on third request")
        }
        Allowed(r) => match sesh.request(req_4) {
            Failed(e) => {
                assert_eq!(
                    (e.error_code, e.error),
                    (2usize, "service not available".to_string())
                );
                println!("got NotAvailable on fourth request")
            }
            Allowed(s) => panic!("third and fourth requests succeeded: {:?}, {:?}", r, s),
            Governed(i, n) => {
                println! {"third request: {:?}; fourth request: limiter {}, {:?}", r, i, n}
            }
            ReqwestError(e) => panic!("fourth request failed: {:?}", e),
        },
        Governed(i, n) => println! {"third request: limiter {}, {:?}", i, n},
        ReqwestError(e) => panic!("third request failed: {:?}", e),
    };

    sleep(Duration::from_secs(2));
}
