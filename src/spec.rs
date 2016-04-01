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

#[test]
fn complex_examples() {
    let r = Regex::from("..+.").unwrap();
    assert_eq!(Some(3), r.match_offset("abc"));
    assert_eq!(Some(5), r.match_offset("ababc"));
    assert_eq!(None, r.match_offset("ac"));

    let r = Regex::from("((abc|acc)b)+").unwrap();
    assert_eq!(Some(4), r.match_offset("abcb"));
    assert_eq!(Some(4), r.match_offset("accb"));
    assert_eq!(None, r.match_offset("zzzb"));
    assert_eq!(Some(8), r.match_offset("abcbaccb"));
}

