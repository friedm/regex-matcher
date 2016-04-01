use super::Regex;

#[test]
fn only_matches_at_start_of_text() {
    assert!(!Regex::from("ab?c").unwrap().is_match("zac"));
    assert!(!Regex::from("ab?c").unwrap().is_match("abbbc"));
}

#[test]
fn matches_simple_examples() {
    assert!(Regex::from("a?").unwrap().is_match(""));
    assert!(Regex::from("a+").unwrap().is_match("a"));
    assert!(Regex::from("ab+").unwrap().is_match("abbbb"));
}

#[test]
fn does_not_match() {
    assert!(!Regex::from("ab?c").unwrap().is_match("z"));
    assert!(!Regex::from("a+").unwrap().is_match(""));
    assert!(!Regex::from("ab+").unwrap().is_match("bbbb"));
}

#[test]
fn matches_with_kleene_star() {
    let regex = Regex::from("ab*c*d").unwrap();
    assert!(regex.is_match("abbbbcd"));
    assert!(regex.is_match("acd"));
    assert!(regex.is_match("ad"));
    assert!(regex.is_match("acccd"));

    assert!(!regex.is_match("bbbcccd"));
    assert!(!regex.is_match("abbbccc"));
}

#[test]
#[ignore]
fn matches_character_class() {
    let regex = Regex::from("[abc]").unwrap();
    assert!(regex.is_match("a"));
    assert!(regex.is_match("b"));
    assert!(regex.is_match("c"));
    assert!(!regex.is_match("["));
    assert!(!regex.is_match("]"));
    assert!(!regex.is_match("z"));
}

#[test]
#[ignore]
fn backtracks_to_find_match() {
    let regex = Regex::from("[bc]?c").unwrap();
    assert!(regex.is_match("cb"));
    assert!(regex.is_match("cc"));
    assert!(regex.is_match("c"));
    assert!(!regex.is_match("b"));

    //assert_eq!(Some((0,3)), Regex::from("a.b").unwrap().first("aaab"));
}

#[test]
#[ignore]
fn matches_zero_or_more() {
    let regex = Regex::from("ab*").unwrap();
    assert!(regex.is_match("abbbb"));
    assert!(regex.is_match("ab"));
    assert!(regex.is_match("a"));
    assert!(!regex.is_match("bb"));
}

#[test]
#[ignore]
fn optional_metachar_is_greedy() {
    assert!(false);
}

#[test]
#[ignore]
fn one_or_more_metachar_is_greedy() {
    assert!(false);
}

#[test]
#[ignore]
fn zero_or_more_metachar_is_greedy() {
    assert!(false);
}

#[test]
#[ignore]
fn dot_matches_any_one_character() {
    assert!(false);
}

#[test]
#[ignore]
fn dot_does_not_match_newline() {
    assert!(false);
}

