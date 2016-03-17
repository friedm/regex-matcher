
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

#[derive(PartialEq, Debug)]
pub struct Multiplicity {
    minimum: usize,
    maximum: usize
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

    for c in text.chars() {
        match c {
            '?' => {
                match result.pop() {
                    Some(value) => {
                        match value {
                            Expression::Token(token, multiplicity) => {
                                result.push(Expression::Token(
                                        token,
                                        Multiplicity {
                                            minimum: 0,
                                            maximum: 1
                                        }));
                            },
                            _ => { return Err("invalid token before the `?` metacharacter"); }
                        }},
                    None => { return Err("no token before `?` metacharacter"); }

                }
            },
            c => { // Not a meta-character, treat as a literal
                result.push(Expression::Token(
                    Token::Literal(c),
                    Multiplicity {
                        minimum: 1,
                        maximum: 1
                    }
                ));
            }
            
        }
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
                           minimum: 1,
                           maximum: 1
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
                       minimum: 0,
                       maximum: 1
                   })
        ], parse_expressions("a?").unwrap());

        assert!(parse_expressions("?").is_err());
        assert!(parse_expressions("??").is_err());
    }
}


