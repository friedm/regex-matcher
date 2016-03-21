use std::str::FromStr;
use ::tokenize::tokenize_regex;
use ::expression::{Expression, Token, Multiplicity};

#[derive(PartialEq,Debug)]
pub struct Regex {
    pub expressions: Vec<Expression>
}

impl Regex {
    pub fn from(pattern: &str) -> Result<Regex, &str> {
        tokenize_regex(pattern).map(|exprs| Regex {
            expressions: exprs
        })
    }

    pub fn first(&self, text: &str) -> Option<(usize, usize)> {
        let mut regex_i = 0; // the current position in the list of regex "expressions"
        let mut text_i = 0; // the current position in the text
        let mut match_start = 0; // the start position of the current match

        // a stack of valid offsets, in the order we encountered them
        let mut backtrack_stack = Vec::<(usize, usize, usize)>::new();

        while regex_i < self.expressions.len() {
            let mut options = self.expressions[regex_i].valid_offsets(&text[text_i..]);

            backtrack_stack.extend(options.iter()
                                     .map(|&option| (regex_i, text_i, option))
                                     .collect::<Vec<_>>());

            if backtrack_stack.len() == 0 {
                regex_i = 0;
                text_i += 1;
                match_start = text_i;

                if text_i >= text.len() {
                    return None;
                }

                continue;
            }

            let (reg_i,tex_i,chosen_option) = backtrack_stack.pop().unwrap();

            text_i = tex_i + chosen_option;
            regex_i = reg_i + 1;
        }

        Some((match_start, text_i))
    }

    pub fn is_match(&self, text: &str) -> bool {
        return self.first(text).is_some();
    }
}

#[cfg(test)]
mod match_spec {
    use super::Regex;

    #[test]
    #[ignore]
    fn matches_simple_examples() {
        assert!(Regex::from("ab?c").unwrap().is_match("zac"));
        assert!(Regex::from("ab?c").unwrap().is_match("abbbc"));
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

        assert_eq!(Some((0,3)), Regex::from("a.b").unwrap().first("aaab"));
    }

    #[test]
    fn finds_correct_match() {
        let regex = Regex::from("[bc]?c").unwrap();
        assert_eq!(Some((0,1)), 
                   regex.first("cb"));
        assert_eq!(Some((0,2)),
                   regex.first("cc"));
        assert_eq!(Some((0,1)),
                   regex.first("c"));
    }

    #[test]
    fn matches_zero_or_more() {
        let regex = Regex::from("ab*").unwrap();
        assert!(regex.is_match("abbbb"));
        assert!(regex.is_match("ab"));
        assert!(regex.is_match("a"));
        assert!(!regex.is_match("bb"));
    }

    #[test]
    fn finds_correct_match_position() {
        let regex = Regex::from("a").unwrap();
        assert_eq!(Some((0,1)), regex.first("a"));
        assert_eq!(Some((3,4)), regex.first("bbba"));
        assert_eq!(Some((0,4)), Regex::from("aaaa").unwrap().first("aaaa"));
    }

    #[test]
    fn finds_first_occurrence() {
        assert_eq!(Some((1,2)), Regex::from("z").unwrap().first("azzzzzzz"));
        assert_eq!(Some((2,5)), Regex::from("abc").unwrap().first("zxabcabc"));
        assert_eq!(Some((0,5)), Regex::from("[zx]+abc").unwrap().first("zxabczxabc"));
    }

    #[test]
    fn optional_metachar_is_greedy() {
        assert_eq!(Some((0,1)), Regex::from("a?").unwrap().first("aa"));
    }

    #[test]
    fn one_or_more_metachar_is_greedy() {
        assert_eq!(Some((0,2)), Regex::from("a+").unwrap().first("aa"));
    }

    #[test]
    fn zero_or_more_metachar_is_greedy() {
        assert_eq!(Some((0,2)), Regex::from("a*").unwrap().first("aa"));
    }

    #[test]
    fn dot_matches_any_one_character() {
        assert_eq!(Some((0,1)), Regex::from(".").unwrap().first("abc"));
    }

    #[test]
    fn dot_does_not_match_newline() {
        assert_eq!(None, Regex::from(".").unwrap().first("\n"));
        let regex = Regex::from(".*").unwrap();
        assert_eq!(Some((0,0)), regex.first("\n\n"));

        assert_eq!(None, Regex::from(".+").unwrap().first("\n\n"));
    }

    #[test]
    fn dot_matches_with_multiplicity() {
        assert_eq!(Some((0,10)), 
                   Regex::from(".*").unwrap().first("some stuff\nmore stuff"));
        assert_eq!(Some((3, 8)),
                   Regex::from(".+").unwrap().first("\n\n\nstuff"));
    }
}

