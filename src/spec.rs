use super::{Regex};

#[test]
#[ignore]
fn creates_new() {
    Regex::from("").unwrap();
}

#[test]
#[ignore]
fn returns_err_on_invalid_regex() {
    assert!(Regex::from("[").is_err());
}

#[test]
#[ignore]
fn matches() {
    let regex = Regex::from("a").unwrap();
    assert_eq!(true, regex.is_match("baaa"));
}

#[test]
#[ignore]
fn matches_optionally() {
    assert!(Regex::from("a?").unwrap().is_match("b"));
}

