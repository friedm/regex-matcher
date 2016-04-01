use ::expr::Expr;
use super::{State, Edge, NFA, ConditionChar};

#[test]
fn build_single() {
    let nfa = NFA::from_expr(&Expr::Single('a'));

    assert_eq!(vec![State::state(ConditionChar::one('a'), Edge::End)], nfa.states);
    assert_eq!(0, nfa.start);
}

#[test]
fn build_sequence() {
    let nfa = NFA::from_expr(&Expr::sequence(Expr::Single('a'),Expr::Single('b')));
    
    assert_eq!(vec![State::state(ConditionChar::one('a'), Edge::Id(1)), State::state(ConditionChar::one('b'), Edge::End)], nfa.states);
    assert_eq!(0, nfa.start);
}

#[test]
fn build_option() {
    let nfa = NFA::from_expr(&Expr::optional(Expr::Single('a')));

    assert_eq!(vec![State::state(ConditionChar::one('a'), Edge::End), State::split(ConditionChar::None, Edge::Id(0), ConditionChar::None, Edge::End)],
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
        State::state(ConditionChar::one('a'), Edge::Id(1)),
        State::state(ConditionChar::one('b'), Edge::Id(3)),
        State::split(ConditionChar::None, Edge::Id(0),
                     ConditionChar::None, Edge::Id(3)),
        State::state(ConditionChar::one('a'), Edge::End)
    ], nfa.states);
    assert_eq!(2, nfa.start);
}

#[test]
fn build_one_or_more() {
    let nfa = NFA::from_expr(
            &Expr::one_or_more(Expr::Single('a')));
    // nfa for 'a+'

    assert_eq!(vec![
        State::state(ConditionChar::one('a'), Edge::Id(1)),
        State::split(ConditionChar::None, Edge::Id(0), ConditionChar::None, Edge::End)
    ], nfa.states);
    assert_eq!(0, nfa.start);
}

#[test]
fn build_more_complex_one_or_more() {
    let nfa = NFA::from_expr(&"a+a+b".parse::<Expr>().unwrap());

    assert_eq!(vec![
        State::state(ConditionChar::one('a'), Edge::Id(1)),
        State::split(ConditionChar::None, Edge::Id(0), ConditionChar::None, Edge::Id(2)),
        State::state(ConditionChar::one('a'), Edge::Id(3)),
        State::split(ConditionChar::None, Edge::Id(2), ConditionChar::None, Edge::Id(4)),
        State::state(ConditionChar::one('b'), Edge::End)
    ], nfa.states);
    assert_eq!(0, nfa.start);
}

#[test]
fn build_zero_or_more() {
    let nfa = NFA::from_expr(
        &Expr::zero_or_more(Expr::Single('a')));
    // 'a*'
    
    assert_eq!(vec![
        State::state(ConditionChar::one('a'), Edge::Id(1)),
        State::split(ConditionChar::None, Edge::Id(0), ConditionChar::None, Edge::End)
    ], nfa.states);
    assert_eq!(1, nfa.start);
}

#[test]
fn build_more_complex_zero_or_more() {
    let nfa = NFA::from_expr(&"b*cd*".parse::<Expr>().unwrap());

    assert_eq!(vec![
        State::state(ConditionChar::one('b'), Edge::Id(1)), // 0
        State::split(ConditionChar::None, Edge::Id(0), ConditionChar::None, Edge::Id(2)), // 1
        State::state(ConditionChar::one('c'), Edge::Id(4)), // 2
        State::state(ConditionChar::one('d'), Edge::Id(4)), // 3
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

#[test]
fn prioritizes_state() {
    let s = State::state(ConditionChar::one('a'), Edge::End);
    assert_eq!(97, s.get_priority_key(&NFA::new()));

    let s = State::state(ConditionChar::Any, Edge::End);
    assert_eq!(0, s.get_priority_key(&NFA::new()));

    let s = State::state(ConditionChar::None, Edge::End);
    assert_eq!(usize::max_value(), s.get_priority_key(&NFA::new()));

    // recursive
    let s = State::state(ConditionChar::None, Edge::Id(0));
    assert_eq!(98, s.get_priority_key(&NFA::from_states(vec![
        State::state(ConditionChar::None, Edge::Id(1)),
        State::state(ConditionChar::one('b'), Edge::End)
    ])));

    // does not infinitely loop (assuming no free cycles)
    let s = State::state(ConditionChar::None, Edge::Id(0));
    assert_eq!(0, s.get_priority_key(&NFA::from_states(vec![
        State::state(ConditionChar::Any, Edge::Id(0))
    ])));
}

#[test]
fn prioritizes_split() {
    let s = State::split(ConditionChar::one('a'), Edge::End, ConditionChar::None, Edge::End);
    assert_eq!(97, s.get_priority_key(&NFA::new()));

    let s = State::split(ConditionChar::None, Edge::End, ConditionChar::None, Edge::End);
    assert_eq!(usize::max_value(), s.get_priority_key(&NFA::new()));

    // recursive
    let s = State::split(ConditionChar::None, Edge::Id(0), ConditionChar::None, Edge::End);
    assert_eq!(0, s.get_priority_key(&NFA::from_states(vec![
        State::split(ConditionChar::None, Edge::Id(1), ConditionChar::None, Edge::Id(2)),
        State::state(ConditionChar::one('a'), Edge::End),
        State::state(ConditionChar::Any, Edge::End)
    ])));
}

