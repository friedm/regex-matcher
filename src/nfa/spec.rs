use ::expr::Expr;
use super::*;

#[test]
fn build_single() {
    let nfa = State::from_expr(&Expr::Single('a'));

    assert_eq!(State::state('a', State::End), nfa);
}

#[test]
fn build_sequence() {
    let nfa = State::from_expr(&Expr::sequence(Expr::Single('a'), Expr::Single('b')));

    assert_eq!(State::state('a', State::state('b', State::End)),
               nfa);
}

#[test]
fn build_or() {
    let nfa = State::from_expr(&Expr::or(Expr::Single('a'), Expr::Single('b')));

    assert_eq!(State::split(State::state('a', State::End),
                            State::state('b', State::End)),
               nfa);
}

