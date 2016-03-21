use super::tokenize_regex;
use ::expression::{Token, Multiplicity, Expression};

#[test]
fn parses_single_literal() {
    assert_eq!(vec![
               Expression::Token(
                   Token::Literal('a'),
                   Multiplicity::one()
                   )
    ], tokenize_regex("a").unwrap())
}

#[test]
fn parses_many_literals() {
    assert_eq!(6, tokenize_regex("abcxyz").unwrap().len());
}

#[test]
fn handles_optional_multiplicity() {
    assert_eq!(vec![
               Expression::Token(Token::Literal('a'),
               Multiplicity::optional())
    ], tokenize_regex("a?").unwrap());

    assert!(tokenize_regex("?").is_err());
    assert!(tokenize_regex("??").is_err());
}

#[test]
fn handles_one_or_more_multiplicity() {
    assert_eq!(vec![
               Expression::Token(Token::Literal('a'),
               Multiplicity::one_or_more())
    ], tokenize_regex("a+").unwrap());

    assert!(tokenize_regex("+").is_err());
    assert!(tokenize_regex("++").is_err());
}

#[test]
fn handles_zero_or_more_multiplicity() {
    assert_eq!(vec![
               Expression::Token(Token::Literal('a'),
               Multiplicity::zero_or_more())
    ], tokenize_regex("a*").unwrap());
}

#[test]
fn handles_character_class() {
    assert_eq!(vec![
               Expression::Token(Token::Class(vec!['x', 'y', 'z']),
               Multiplicity::one())
    ], tokenize_regex("[xyz]").unwrap());

    assert!(tokenize_regex("]").is_err());
    assert!(tokenize_regex("[").is_err());
    assert!(tokenize_regex("[]").is_ok());
}

#[test]
fn handles_dot() {
    assert_eq!(vec![
               Expression::Token(Token::Any,
                                 Multiplicity::one())
    ], tokenize_regex(".").unwrap());
}

