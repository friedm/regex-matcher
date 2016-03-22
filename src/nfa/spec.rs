use ::expr::Expr;
use super::*;

#[test]
fn build_single() {
    let nfa = State::from_expr(&Expr::Single('a'));

    assert_eq!(State::state('a', State::End), nfa);
}

#[test]
fn build_sequence() {
    let expr = &Expr::sequence(Expr::Single('a'), Expr::Single('b'));

    assert_eq!(State::state('a', State::state('b', State::End)),
               State::from_expr(expr));
}

#[test]
fn build_or() {
    let expr = &Expr::or(Expr::Single('a'), Expr::Single('b'));

    assert_eq!(State::split(State::state('a', State::End),
                            State::state('b', State::End)),
               State::from_expr(expr));
}

#[test]
fn build_optional() {
    let expr = &Expr::optional(Expr::Single('a'));

    assert_eq!(State::split(State::state('a', State::End),
                            State::End),
               State::from_expr(expr));
}

#[test]
#[ignore]
fn build_zero_or_more() {
    let expr = &Expr::zero_or_more(Expr::Single('a'));

    let mut expected_state = State::split(State::state('a', State::Detached),
                                          State::End);
    expected_state = State::split(State::State{edge: 'a', out: Box::new(expected_state)},
                                  State::End);

    assert_eq!(expected_state, State::from_expr(expr));
}

