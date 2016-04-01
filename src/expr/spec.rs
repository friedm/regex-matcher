use super::Expr;

#[test]
fn parse_single() {
    assert_eq!(Expr::Single('a'),
    "a".parse::<Expr>().unwrap());
}

#[test]
fn parse_empty() {
    assert!("".parse::<Expr>().is_err());
}

#[test]
fn parse_sequence() {
    assert_eq!(Expr::sequence(Expr::Single('a'),
                              Expr::Single('b')),
               "ab".parse::<Expr>().unwrap());

    assert_eq!(Expr::sequence(Expr::Single('a'),
                              Expr::Single('b')),
               "(a)b".parse::<Expr>().unwrap());

    assert_eq!(Expr::sequence(Expr::or(Expr::Single('a'),Expr::Single('b')),
                              Expr::Single('b')),
                "(a|b)b".parse::<Expr>().unwrap());

    assert_eq!(Expr::sequence(Expr::Single('a'),
                              Expr::Single('b')),
               "((a)b)".parse::<Expr>().unwrap());
}

#[test]
fn parse_nested_sequence() {
    assert_eq!(Expr::sequence(Expr::Single('a'),
                              Expr::sequence(Expr::Single('b'),
                                             Expr::Single('c'))),
               "abc".parse::<Expr>().unwrap());
}

#[test]
fn parse_or() {
    assert_eq!(Expr::or(Expr::Single('a'),
                        Expr::Single('b')),
               "a|b".parse::<Expr>().unwrap());

    assert_eq!(Expr::or(Expr::sequence(Expr::Single('a'), Expr::Single('b')),
                        Expr::sequence(Expr::Single('c'), Expr::Single('d'))),
               "ab|cd".parse::<Expr>().unwrap());
}

#[test]
fn parse_optional() {
    assert_eq!(Expr::optional(Expr::Single('a')),
    "a?".parse::<Expr>().unwrap());
}

#[test]
fn parse_kleene_star() {
    assert_eq!(Expr::zero_or_more(Expr::Single('a')),
    "a*".parse::<Expr>().unwrap());
    assert_eq!(Expr::sequence(Expr::Single('a'),
                Expr::zero_or_more(Expr::Single('b'))),
    "ab*".parse::<Expr>().unwrap());

}

#[test]
fn parse_kleene_plus() {
    assert_eq!(Expr::OneOrMore(Box::new(Expr::Single('a'))),
    "a+".parse::<Expr>().unwrap());
}

#[test]
fn parse_combination() {
    assert_eq!(
        Expr::sequence(Expr::Single('a'),
                       Expr::sequence(
                               Expr::OneOrMore(Box::new(Expr::Single('b'))),
                               Expr::Single('c'))),
               "ab+c".parse::<Expr>().unwrap());
}

#[test]
fn parse_parens() {
    assert_eq!(Expr::OneOrMore(Box::new(Expr::sequence(Expr::Single('a'),
                                             Expr::Single('b')))),
               "(ab)+".parse::<Expr>().unwrap());

    assert_eq!(
        Expr::sequence(
            Expr::OneOrMore(Box::new(Expr::sequence(Expr::Single('a'),
                                          Expr::Single('b')))),
            Expr::Single('c')),
               "(ab)+c".parse::<Expr>().unwrap());
    
    assert_eq!(Expr::Single('a'),
        "(a)".parse::<Expr>().unwrap());

    assert_eq!(Expr::Single('a'),
        "()a".parse::<Expr>().unwrap());

    assert_eq!(Expr::Single('a'),
        "((a))".parse::<Expr>().unwrap());
}

#[test]
fn parse_parens_with_or() {
    assert_eq!(Expr::Or(Box::new(Expr::Single('a')), Box::new(Expr::Single('b'))),
               "(a|b)".parse::<Expr>().unwrap());
}

#[test]
fn parse_dot() {
    assert_eq!(Expr::Any,
               ".".parse::<Expr>().unwrap());
}

#[test]
fn parse_parens_with_kleene_star() {
    "(ab)*".parse::<Expr>().unwrap();
}

#[test]
fn parse_complex() {
    assert_eq!(Expr::one_or_more(
            Expr::sequence(
            Expr::Single('a'),
            Expr::Single('b'))),
            "((a)b)+".parse::<Expr>().unwrap());

    assert_eq!(
        Expr::one_or_more(
            Expr::sequence(
                Expr::or(
                    Expr::Single('a'),
                    Expr::Single('b')),
                Expr::Single('b'))),
            "((a|b)b)+".parse::<Expr>().unwrap());
}

