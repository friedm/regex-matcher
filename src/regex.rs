use std::str::FromStr;
use ::token::parse_expressions;
use ::token::{Expression, Token, Multiplicity};

#[derive(PartialEq,Debug)]
pub struct Regex {
    pub pattern: String
}

impl Regex {
    pub fn from(pattern: &str) -> Regex {
        Regex {
            pattern: pattern.to_owned()
        }
    }

    pub fn first(&self, text: &str) -> Option<(usize, usize)> {
        let expressions = parse_expressions(&self.pattern).unwrap();

        let mut regex_i = 0;
        let mut text_i = 0;
        let mut match_start = 0;

        while regex_i < expressions.len() {
            let mut options = Self::ways_to_grab_text(&text[text_i..], &expressions[regex_i]);

            if options.len() == 0 {
                text_i += 1;
                match_start = text_i;

                if text_i >= text.len() {
                    return None;
                }

                continue;
            }

            let chosen_option = options.pop().unwrap();

            text_i += chosen_option;
            regex_i += 1;
        }

        Some((match_start, text_i))
    }

    fn ways_to_grab_text(text: &str, expr: &Expression) -> Vec<usize> {
        let mut valid_offsets = Vec::<usize>::new();
        let mut valid_chars = Vec::new();
        let mut valid_multiplicity = Multiplicity {
            minimum: Some(1), maximum: Some(1)
        };

        match expr {
            &Expression::Token(ref token, ref multiplicity) => {
                valid_multiplicity = multiplicity.clone();
                match token {
                    &Token::Literal(ref value) => {
                        valid_chars.push(value.clone());
                    },
                    &Token::Class(ref values) => {
                        valid_chars.append(&mut values.clone());
                    }
                    _ => ()
                }
            },
            _ => ()
        }

        if valid_multiplicity.minimum == None || valid_multiplicity.minimum == Some(0) {
            valid_offsets.push(0); //it is an option to do nothing with this expr
        }

        let mut offset = 0;

        for c in text.chars() {
            if !valid_chars.contains(&c) {
                break;
            }

            offset += 1;

            match valid_multiplicity.minimum {
                Some(min) => {
                    if min < offset { continue; }
                },
                None => ()
            }

            match valid_multiplicity.maximum {
                Some(max) => {
                    if max > offset { continue; }
                },
                None => ()
            }

            valid_offsets.push(offset);
        }


        valid_offsets
    }

    pub fn is_match(&self, text: &str) -> bool {
        return self.first(text).is_some();
    }
}

#[cfg(test)]
mod regex_spec {
    use super::Regex;

    #[test]
    fn matches_simple_examples() {
        assert!(Regex::from("ab?c").is_match("zac"));
        assert!(Regex::from("ab?c").is_match("abbbc"));
        assert!(Regex::from("a?").is_match(""));
        assert!(Regex::from("a+").is_match("a"));
        assert!(Regex::from("ab+").is_match("abbbb"));
    }

    #[test]
    fn does_not_match() {
        assert!(!Regex::from("ab?c").is_match("z"));
        assert!(!Regex::from("a+").is_match(""));
        assert!(!Regex::from("ab+").is_match("bbbb"));
    }

    #[test]
    fn matches_character_class() {
        assert!(Regex::from("[abc]").is_match("a"));
        assert!(Regex::from("[abc]").is_match("b"));
        assert!(Regex::from("[abc]").is_match("c"));
        assert!(!Regex::from("[abc]").is_match("["));
        assert!(!Regex::from("[abc]").is_match("]"));
        assert!(!Regex::from("[abc]").is_match("z"));
    }

    #[test]
    fn backtracks_to_find_match() {
        assert!(Regex::from("[bc]?c").is_match("cb"));
        assert!(Regex::from("[bc]?c").is_match("cc"));
        assert!(Regex::from("[bc]?c").is_match("c"));
        assert!(!Regex::from("[bc]?c").is_match("b"));
    }

    #[test]
    fn matches_zero_or_more() {
        let regex = Regex::from("ab*");
        assert!(regex.is_match("abbbb"));
        assert!(regex.is_match("ab"));
        assert!(regex.is_match("a"));
        assert!(!regex.is_match("bb"));
    }
}
