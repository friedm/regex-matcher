// A regex "token" represents a single character in the text
// It can be one of:
//     a character literal
//     a character class
//     a any character (.)
#[derive(PartialEq, Debug)]
pub enum Token {
    Literal(char),
    Class(Vec<char>),
    Any
}

// Represents an upper and lower bound on the multiplicity of a regex token
// Eg. `(Some(0), Some(1))` corresponds to `?` and `(Some(0), None)` corresponds
// to `*`
#[derive(PartialEq, Debug, Clone)]
pub struct Multiplicity {
    pub minimum: usize,
    pub maximum: Option<usize>
}

impl Multiplicity {
    pub fn one() -> Multiplicity {
        Self::new(1, Some(1))
    }

    pub fn optional() -> Multiplicity {
        Self::new(0, Some(1))
    }

    pub fn one_or_more() -> Multiplicity {
        Self::new(1, None)
    }

    pub fn zero_or_more() -> Multiplicity {
        Self::new(0, None)
    }

    pub fn new(min: usize, max: Option<usize>) -> Multiplicity {
        Multiplicity {
            minimum: min,
            maximum: max
        }
    }
}

// A regex "expression" represents a token with multiplicity, or a position
// Eg. `[abc]*`, `[abc]{1,3}`, `^`
#[derive(PartialEq, Debug)]
pub enum Expression {
    BeginPosition,
    EndPosition,
    Token(Token, Multiplicity)
}

impl Expression {
    // returns a stack of valid offsets, given a string and a certain
    // Regex "expression" (token and multiplicity)
    // Multiplicity operator greediness is handled by ensuring that
    // the offsets on the top of the stack are the largest valid offsets
    pub fn valid_offsets(&self, text: &str) -> Vec<usize> {
        let mut valid_offsets = Vec::<usize>::new();
        let mut valid_chars = Vec::new();
        let mut expr_is_dot = false;
        let mut valid_multiplicity = Multiplicity::one();

        match self {
            &Expression::Token(ref token, ref multiplicity) => {
                valid_multiplicity = multiplicity.clone();
                match token {
                    &Token::Literal(ref value) => {
                        valid_chars.push(value.clone());
                    },
                    &Token::Class(ref values) => {
                        valid_chars.append(&mut values.clone());
                    },
                    &Token::Any => {
                        expr_is_dot = true;
                    }
                }
            },
            _ => ()
        }

        if valid_multiplicity.minimum == 0 {
            valid_offsets.push(0); //it is an option to do nothing with this expr
        }

        let mut offset = 0;

        for c in text.chars() {
            if !expr_is_dot && !valid_chars.contains(&c) {
                break;
            }

            if expr_is_dot && c == '\n' { // the 'dot' metachar should not match newline
                break;
            }

            offset += 1;

            if offset < valid_multiplicity.minimum {
                continue;
            }

            match valid_multiplicity.maximum {
                Some(max) => {
                    if offset > max { continue; }
                },
                None => ()
            }

            valid_offsets.push(offset);
        }

        valid_offsets
    }
}

