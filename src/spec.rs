use super::{Regex};

#[test]
fn creates_new() {
    assert!(Regex::from("a").is_ok());
}

#[test]
fn creates_empty() {
    assert!(Regex::from("").is_err());
}

#[test]
#[ignore]
fn returns_err_on_invalid_regex() {
    assert!(Regex::from("[").is_err());
}

#[test]
fn matches_optionally() {
    assert!(Regex::from("a?").unwrap().is_match("b"));
}

