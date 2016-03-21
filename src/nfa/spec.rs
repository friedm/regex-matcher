use super::Expr;

#[test]
fn parse_single() {
    assert_eq!(Expr::Single('a'),
    "a".parse::<Expr>().unwrap());
}

#[test]
fn parse_sequence() {
    assert_eq!(Expr::Sequence(Box::new(Expr::Single('a')),
                              Box::new(Expr::Single('b'))),
               "ab".parse::<Expr>().unwrap());
}

#[test]
fn parse_nested_sequence() {
    assert_eq!(Expr::Sequence(Box::new(Expr::Single('a')),
                              Box::new(Expr::Sequence(Box::new(Expr::Single('b')),
                                             Box::new(Expr::Single('c'))))),
               "abc".parse::<Expr>().unwrap());
}

#[test]
fn parse_or() {
    assert_eq!(Expr::Or(Box::new(Expr::Single('a')),
    Box::new(Expr::Single('b'))),
    "a|b".parse::<Expr>().unwrap());
}

#[test]
fn parse_optional() {
    assert_eq!(Expr::Optional(Box::new(Expr::Single('a'))),
    "a?".parse::<Expr>().unwrap());
}

#[test]
fn parse_kleene_star() {
    assert_eq!(Expr::ZeroOrMore(Box::new(Expr::Single('a'))),
    "a*".parse::<Expr>().unwrap());
}

#[test]
fn parse_kleene_plus() {
    assert_eq!(Expr::OneOrMore(Box::new(Expr::Single('a'))),
    "a+".parse::<Expr>().unwrap());
}

#[test]
fn parse_combination() {
    assert_eq!(
        Expr::Sequence(Box::new(Expr::Single('a')),
                       Box::new(Expr::Sequence(
                               Box::new(Expr::OneOrMore(Box::new(Expr::Single('b')))),
                               Box::new(Expr::Single('c'))))),
               "ab+c".parse::<Expr>().unwrap());
}

#[test]
fn parse_parens() {
    assert_eq!(Expr::OneOrMore(Box::new(Expr::Sequence(Box::new(Expr::Single('a')),
                                             Box::new(Expr::Single('b'))))),
               "(ab)+".parse::<Expr>().unwrap());

    assert_eq!(
        Expr::Sequence(
            Box::new(Expr::OneOrMore(Box::new(Expr::Sequence(Box::new(Expr::Single('a')),
                                          Box::new(Expr::Single('b')))))),
            Box::new(Expr::Single('c'))),
               "(ab)+c".parse::<Expr>().unwrap());
}

