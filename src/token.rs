
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
                match update_last_with_multiplicity(&mut result, Multiplicity::optional()) {
                    Ok(_) => (),
                    Err(e) => {
                        e.to_owned().push_str(" `?`");
                        return Err(e);
                    }
                }
            },
            '+' => {
                match update_last_with_multiplicity(&mut result, Multiplicity::one_or_more()) {
                    Ok(_) => (),
                    Err(e) => {
                        e.to_owned().push_str(" `+`");
                        return Err(e);
                    }
                }
            },
            '*' => {
                match update_last_with_multiplicity(&mut result, Multiplicity::zero_or_more()) {
                    Ok(_) => (),
                    Err(e) => {
                        e.to_owned().push_str(" `*`");
                        return Err(e);
                    }
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
                        Multiplicity::one()));
            },
            c => { // Not a meta-character, treat as a literal
                result.push(Expression::Token(
                    Token::Literal(c),
                    Multiplicity::one()
                ));
            }
            
        }
    }

    if in_char_class == true {
        return Err("incomplete char class, expected `]`");
    }

    Ok(result)
}

fn update_last_with_multiplicity<'a>(expressions: &mut Vec<Expression>, multiplicity: Multiplicity) 
    -> Result<(), &'a str> {

    match expressions.pop() {
        Some(value) => {
            match value {
                Expression::Token(token, _) => {
                    expressions.push(Expression::Token(
                            token,
                            multiplicity));
                },
                _ => { return Err("invalid token before metacharacter"); }
            }
        },
        None => { return Err("no token before metacharacter"); }
    }

    Ok(())
}

#[cfg(test)]
mod expression_spec {
    use super::{Token, Multiplicity, Expression, parse_expressions};

    #[test]
    fn parses_single_literal() {
        assert_eq!(vec![
                   Expression::Token(
                       Token::Literal('a'),
                       Multiplicity::one()
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
                   Multiplicity::optional())
        ], parse_expressions("a?").unwrap());

        assert!(parse_expressions("?").is_err());
        assert!(parse_expressions("??").is_err());
    }

    #[test]
    fn handles_one_or_more_multiplicity() {
        assert_eq!(vec![
                   Expression::Token(Token::Literal('a'),
                   Multiplicity::one_or_more())
        ], parse_expressions("a+").unwrap());

        assert!(parse_expressions("+").is_err());
        assert!(parse_expressions("++").is_err());
    }

    #[test]
    fn handles_zero_or_more_multiplicity() {
        assert_eq!(vec![
                   Expression::Token(Token::Literal('a'),
                   Multiplicity::zero_or_more())
        ], parse_expressions("a*").unwrap());
    }

    #[test]
    fn handles_character_class() {
        assert_eq!(vec![
                   Expression::Token(Token::Class(vec!['x', 'y', 'z']),
                                     Multiplicity::one())
        ], parse_expressions("[xyz]").unwrap());

        assert!(parse_expressions("]").is_err());
        assert!(parse_expressions("[").is_err());
        assert!(parse_expressions("[]").is_ok());
    }
}


