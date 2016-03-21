use ::expression::{Token, Expression, Multiplicity};

#[cfg(test)] mod spec;

pub fn tokenize_regex(text: &str) -> Result<Vec<Expression>,&str> {
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

