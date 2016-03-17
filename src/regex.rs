use std::str::FromStr;
use ::token::parse_expressions;
use ::token::{Expression, Token};

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

    pub fn is_match(&self, text: &str) -> bool {
        let expressions = parse_expressions(&self.pattern).unwrap();

        let mut regex_i = 0;
        let mut text_i = 0;

        while regex_i < expressions.len() {
            let mut options = Self::ways_to_grab_text(&text[text_i..], &expressions[regex_i]);

            if options.len() == 0 {
                text_i += 1;

                if text_i >= text.len() {
                    return false;
                }

                continue;
            }

            let chosen_option = options.pop().unwrap();

            text_i += chosen_option;
            regex_i += 1;
        }

        true
    }

    fn ways_to_grab_text(text: &str, expr: &Expression) -> Vec<usize> {
        let mut valid_offsets = Vec::<usize>::new();

        match expr {
            &Expression::Token(ref token, ref multiplicity) => {
                if multiplicity.minimum == None ||
                    multiplicity.minimum == Some(0) {
                    valid_offsets.push(0); //it is an option to do nothing with this expr
                }

                match token {
                    &Token::Literal(value) => {
                        let mut offset = 0;

                        for c in text.chars() {
                            if value != c {
                                break;
                            }

                            offset += 1;

                            match multiplicity.minimum {
                                Some(min) => {
                                    if min < offset { continue; }
                                },
                                None => ()
                            }

                            match multiplicity.maximum {
                                Some(max) => {
                                    if max > offset { continue; }
                                },
                                None => ()
                            }

                            valid_offsets.push(offset);
                        }
                    },
                    _ => ()
                }
            },
            _ => ()
        }

        valid_offsets
    }

    pub fn first(&self, text: &str) -> Option<(usize, usize)> {
        if text.contains(&self.pattern) {
            Some((1, 2))
        } else {
            None
        }
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
    }

    #[test]
    fn does_not_match() {
        assert!(!Regex::from("ab?c").is_match("z"));
        assert!(!Regex::from("a+").is_match(""));
    }

    #[test]
    fn backtracks_to_find_match() {
        assert!(Regex::from("[bc]?c").is_match("cb"));
    }
}
