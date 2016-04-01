use ::expr::Expr;
use super::{State, Edge, NFA, ConditionChar};

#[test]
fn build_single() {
    let nfa = NFA::from_expr(&Expr::Single('a'));

    assert_eq!(vec![State::state(ConditionChar::One('a'), Edge::End)], nfa.states);
    assert_eq!(0, nfa.start);
}

#[test]
fn build_sequence() {
    let nfa = NFA::from_expr(&Expr::sequence(Expr::Single('a'),Expr::Single('b')));
    
    assert_eq!(vec![State::state(ConditionChar::One('a'), Edge::Id(1)), State::state(ConditionChar::One('b'), Edge::End)], nfa.states);
    assert_eq!(0, nfa.start);
}

#[test]
fn build_option() {
    let nfa = NFA::from_expr(&Expr::optional(Expr::Single('a')));

    assert_eq!(vec![State::state(ConditionChar::One('a'), Edge::End), State::split(ConditionChar::None, Edge::Id(0), ConditionChar::None, Edge::End)],
        nfa.states);
    assert_eq!(1, nfa.start);
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
        State::state(ConditionChar::One('a'), Edge::Id(1)),
        State::state(ConditionChar::One('b'), Edge::Id(3)),
        State::split(ConditionChar::None, Edge::Id(0),
                     ConditionChar::None, Edge::Id(3)),
        State::state(ConditionChar::One('a'), Edge::End)
    ], nfa.states);
    assert_eq!(2, nfa.start);
}

#[test]
fn build_one_or_more() {
    let nfa = NFA::from_expr(
            &Expr::one_or_more(Expr::Single('a')));
    // nfa for 'a+'

    assert_eq!(vec![
        State::state(ConditionChar::One('a'), Edge::Id(1)),
        State::split(ConditionChar::None, Edge::Id(0), ConditionChar::None, Edge::End)
    ], nfa.states);
    assert_eq!(0, nfa.start);
}

#[test]
fn build_more_complex_one_or_more() {
    let nfa = NFA::from_expr(&"a+a+b".parse::<Expr>().unwrap());

    assert_eq!(vec![
        State::state(ConditionChar::One('a'), Edge::Id(1)),
        State::split(ConditionChar::None, Edge::Id(0), ConditionChar::None, Edge::Id(2)),
        State::state(ConditionChar::One('a'), Edge::Id(3)),
        State::split(ConditionChar::None, Edge::Id(2), ConditionChar::None, Edge::Id(4)),
        State::state(ConditionChar::One('b'), Edge::End)
    ], nfa.states);
    assert_eq!(0, nfa.start);
}

#[test]
fn build_zero_or_more() {
    let nfa = NFA::from_expr(
        &Expr::zero_or_more(Expr::Single('a')));
    // 'a*'
    
    assert_eq!(vec![
        State::state(ConditionChar::One('a'), Edge::Id(1)),
        State::split(ConditionChar::None, Edge::Id(0), ConditionChar::None, Edge::End)
    ], nfa.states);
    assert_eq!(1, nfa.start);
}

#[test]
fn build_more_complex_zero_or_more() {
    let nfa = NFA::from_expr(&"b*cd*".parse::<Expr>().unwrap());

    assert_eq!(vec![
        State::state(ConditionChar::One('b'), Edge::Id(1)), // 0
        State::split(ConditionChar::None, Edge::Id(0), ConditionChar::None, Edge::Id(2)), // 1
        State::state(ConditionChar::One('c'), Edge::Id(4)), // 2
        State::state(ConditionChar::One('d'), Edge::Id(4)), // 3
        State::split(ConditionChar::None, Edge::Id(3), ConditionChar::None, Edge::End)// 4
    ], nfa.states);
    assert_eq!(1, nfa.start);
}

#[test]
fn build_any() {
    let nfa = NFA::from_expr(&Expr::Any);

    assert_eq!(vec![
        State::state(ConditionChar::Any, Edge::End)
    ], nfa.states);
}

