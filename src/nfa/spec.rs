use ::expr::Expr;
use super::{State, Edge, NFA};

#[test]
fn build_single() {
    let nfa = NFA::from_expr(&Expr::Single('a'));

    assert_eq!(vec![State::state(Some('a'), Edge::End)], nfa.states);
}

#[test]
fn build_sequence() {
    let nfa = NFA::from_expr(&Expr::sequence(Expr::Single('a'),Expr::Single('b')));
    
    assert_eq!(vec![State::state(Some('a'), Edge::Id(1)), State::state(Some('b'), Edge::End)], nfa.states);
}

#[test]
fn build_option() {
    let nfa = NFA::from_expr(&Expr::optional(Expr::Single('a')));

    assert_eq!(vec![State::state(Some('a'), Edge::End), State::split(None, Edge::Id(0), None, Edge::End)],
        nfa.states);
}

#[test]
fn build_complex_option() {
    let nfa = NFA::from_expr(
        &Expr::sequence(
            Expr::optional(Expr::sequence(
                Expr::Single('a'),
                Expr::Single('b'))),
             Expr::Single('a')));

    assert_eq!(vec![
        State::state(Some('a'), Edge::Id(1)),
        State::state(Some('b'), Edge::Id(3)),
        State::split(None, Edge::Id(0),
                     None, Edge::Id(3)),
        State::state(Some('a'), Edge::End)
    ], nfa.states);
}


