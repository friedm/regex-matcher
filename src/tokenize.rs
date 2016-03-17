use ::expression::{Token, Expression, Multiplicity};

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
            '.' => {
                result.push(Expression::Token(Token::Any,
                                              Multiplicity::one()));
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
    use super::parse_expressions;
    use ::expression::{Token, Multiplicity, Expression};

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

    #[test]
    fn handles_dot() {
        assert_eq!(vec![
                   Expression::Token(Token::Any,
                                     Multiplicity::one())
        ], parse_expressions(".").unwrap());
    }
}


