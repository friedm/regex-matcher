use std::str::FromStr;
use ::tokenize::tokenize_regex;
use ::expression::{Expression, Token, Multiplicity};

#[cfg(test)] mod spec;

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

