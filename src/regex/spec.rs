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
fn backtracks_to_find_match() {
    let regex = Regex::from(".?c").unwrap();
    assert!(regex.is_match("cb"));
    assert!(regex.is_match("cc"));
    assert!(regex.is_match("c"));
    assert!(!regex.is_match("b"));
}

#[test]
fn finds_match_position() {
    assert_eq!(None, Regex::from("a.b").unwrap().match_offset("aaab"));
    assert_eq!(Some(3), Regex::from("a.a").unwrap().match_offset("aaab"));
}

#[test]
fn matches_zero_or_more() {
    let regex = Regex::from("ab*").unwrap();
    assert!(regex.is_match("abbbb"));
    assert!(regex.is_match("ab"));
    assert!(regex.is_match("a"));
    assert!(!regex.is_match("bb"));

    let regex = Regex::from("ab*c").unwrap();
    assert!(!regex.is_match("ababc"));
    assert!(!regex.is_match("aabc"));
}

#[test]
fn matches_zero_or_more_subexpr() {
    let r = Regex::from("(ab)*").unwrap();
    assert_eq!(Some(0), r.match_offset(""));
    assert_eq!(Some(2), r.match_offset("ab"));
    assert_eq!(Some(4), r.match_offset("abab"));
    assert_eq!(Some(8), r.match_offset("abababab"));

    let r = Regex::from("(ab|bc)*").unwrap();
    assert_eq!(Some(0), r.match_offset(""));
    assert_eq!(Some(2), r.match_offset("ab"));
    assert_eq!(Some(2), r.match_offset("bc"));
    assert_eq!(Some(4), r.match_offset("abab"));
    assert_eq!(Some(6), r.match_offset("abbcab"));
}

#[test]
fn matches_or() {
    let r = Regex::from("(a|bc)").unwrap();
    assert_eq!(Some(2), r.match_offset("bc"));
    assert_eq!(Some(1), r.match_offset("a"));
    assert_eq!(None, r.match_offset("c"));
    assert_eq!(None, r.match_offset(""));
}

#[test]
fn optional_metachar_is_greedy() {
    assert_eq!(Some(1), Regex::from(".?").unwrap().match_offset("a"));
}

#[test]
fn one_or_more_metachar_is_greedy() {
    assert_eq!(Some(6), Regex::from(".+").unwrap().match_offset("aaaaaa"));
}

#[test]
fn zero_or_more_metachar_is_greedy() {
    assert_eq!(Some(6), Regex::from(".*").unwrap().match_offset("aaaaaa"));
}

#[test]
fn dot_matches_any_one_character() {
    let regex = Regex::from(".").unwrap();
    assert!(regex.is_match("a"));
    assert!(regex.is_match("b"));
    assert!(regex.is_match("5"));
    assert!(regex.is_match("*"));
    assert!(!regex.is_match(""));

    assert!(Regex::from(".*").unwrap().is_match("ab5*"));
}

#[test]
fn dot_does_not_match_newline() {
    let regex = Regex::from(".").unwrap();
    assert!(!regex.is_match("\n"));
}

#[test]
fn char_class_matches() {
    let regex = Regex::from("[abc]").unwrap();
    assert!(regex.is_match("a"));
    assert!(regex.is_match("b"));
    assert!(regex.is_match("c"));
    assert!(!regex.is_match("d"));

    let regex = Regex::from("([ab]d)+").unwrap();
    assert!(!regex.is_match(""));
    assert!(!regex.is_match("aad"));
    assert!(regex.is_match("adad"));
    assert!(regex.is_match("adbd"));
}

