
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
    pub minimum: Option<usize>,
    pub maximum: Option<usize>
}

// A regex "expression" represents a token with multiplicity, or a position
// Eg. `[abc]*`, `[abc]{1,3}`, `^`
#[derive(PartialEq, Debug)]
pub enum Expression {
    BeginPosition,
    EndPosition,
    Token(Token, Multiplicity)
}

pub fn parse_expressions(text: &str) -> Result<Vec<Expression>,&str> {
    let mut result = Vec::<Expression>::new();
    let mut in_char_class = false;
    let mut chars_in_class = Vec::new();

    for c in text.chars() {
        if in_char_class && c != ']' {
            chars_in_class.push(c);
            continue
        }

        match c {
            '?' => {
                match result.pop() {
                    Some(value) => {
                        match value {
                            Expression::Token(token, multiplicity) => {
                                result.push(Expression::Token(
                                        token,
                                        Multiplicity {
                                            minimum: Some(0),
                                            maximum: Some(1)
                                        }));
                            },
                            _ => { return Err("invalid token before `?` metacharacter"); }
                        }
                    },
                    None => { return Err("no token before `?` metacharacter"); }
                }
            },
            '+' => {
                match result.pop() {
                    Some(value) => {
                        match value {
                            Expression::Token(token, multiplicity) => {
                                result.push(Expression::Token(
                                        token,
                                        Multiplicity {
                                            minimum: Some(1),
                                            maximum: None
                                        }));
                            },
                            _ => { return Err("invalid token before `+` metacharacter"); }
                        }
                    },
                    None => { return Err("no token before `+` metacharacter"); }
                }
            },
            '[' => {
                in_char_class = true;
                chars_in_class.clear();
            },
            ']' => {
                if in_char_class == false {
                    return Err("missing char class start `[`");
                }

                in_char_class = false;
                result.push(Expression::Token(
                        Token::Class(chars_in_class.clone()),
                        Multiplicity {
                            minimum: Some(1),
                            maximum: Some(1)
                        }));
            },
            c => { // Not a meta-character, treat as a literal
                result.push(Expression::Token(
                    Token::Literal(c),
                    Multiplicity {
                        minimum: Some(1),
                        maximum: Some(1)
                    }
                ));
            }
            
        }
    }

    if in_char_class == true {
        return Err("incomplete char class, expected `]`");
    }

    Ok(result)
}

#[cfg(test)]
mod expression_spec {
    use super::{Token, Multiplicity, Expression, parse_expressions};

    #[test]
    fn parses_single_literal() {
        assert_eq!(vec![
                   Expression::Token(
                       Token::Literal('a'),
                       Multiplicity {
                           minimum: Some(1),
                           maximum: Some(1)
                       }
                   )
        ], parse_expressions("a").unwrap())
    }

    #[test]
    fn parses_many_literals() {
        assert_eq!(6, parse_expressions("abcxyz").unwrap().len());
    }

    #[test]
    fn handles_optional_multiplicity() {
        assert_eq!(vec![
                   Expression::Token(Token::Literal('a'),
                   Multiplicity {
                       minimum: Some(0),
                       maximum: Some(1)
                   })
        ], parse_expressions("a?").unwrap());

        assert!(parse_expressions("?").is_err());
        assert!(parse_expressions("??").is_err());
    }

    #[test]
    fn handles_one_or_more_multiplicity() {
        assert_eq!(vec![
                   Expression::Token(Token::Literal('a'),
                   Multiplicity {
                       minimum: Some(1),
                       maximum: None
                   })
        ], parse_expressions("a+").unwrap());

        assert!(parse_expressions("+").is_err());
        assert!(parse_expressions("++").is_err());
    }

    #[test]
    fn handles_character_class() {
        assert_eq!(vec![
                   Expression::Token(Token::Class(vec!['x', 'y', 'z']),
                                     Multiplicity {
                                         minimum: Some(1),
                                         maximum: Some(1)
                                     })
        ], parse_expressions("[xyz]").unwrap());

        assert!(parse_expressions("]").is_err());
        assert!(parse_expressions("[").is_err());
        assert!(parse_expressions("[]").is_ok());
    }
}


