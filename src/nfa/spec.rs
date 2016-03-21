use ::expr::Expr;

#[test]
fn matches_single() {
    assert!(!Expr::Single('a').is_match("b"));
    assert!(Expr::Single('a').is_match("a"));
}

#[test]
fn matches_none() {
    assert!(!Expr::Single('a').is_match(""));
}

#[test]
fn matches_sequence() {
    let exp = Expr::Sequence(Box::new(Expr::Single('a')), Box::new(Expr::Single('b')));
    assert!(!exp.is_match(""));
    assert!(!exp.is_match("a"));
    assert!(!exp.is_match("b"));
    assert!(exp.is_match("ab"));
    assert!(exp.is_match("bab"));
}
