use ::expr::Expr;
use super::{State, Edge, NFA};

#[test]
fn build_single() {
    let nfa = NFA::from_expr(&Expr::Single('a'));

    assert_eq!(vec![State::state('a', Edge::End)], nfa.states);
}

#[test]
fn build_sequence() {
    let expr = NFA::from_expr(&Expr::sequence(Expr::Single('a'),Expr::Single('b')));
    
    assert_eq!(vec![State::state('a', Edge::Id(1)), State::state('b', Edge::End)], expr.states);
}

