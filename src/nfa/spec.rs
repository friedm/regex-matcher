use ::expr::Expr;
use super::{State, Transition, NFA, Condition};

#[test]
fn build_single() {
    let nfa = NFA::from_expr(&Expr::Single('a'));

    assert_eq!(vec![State::state(Condition::one('a'), Transition::End)], nfa.states);
    assert_eq!(0, nfa.start);
}

#[test]
fn build_sequence() {
    let nfa = NFA::from_expr(&Expr::sequence(Expr::Single('a'),Expr::Single('b')));
    
    assert_eq!(vec![State::state(Condition::one('a'), Transition::Id(1)), State::state(Condition::one('b'), Transition::End)], nfa.states);
    assert_eq!(0, nfa.start);
}

#[test]
fn build_option() {
    let nfa = NFA::from_expr(&Expr::optional(Expr::Single('a')));

    assert_eq!(vec![State::state(Condition::one('a'), Transition::End), State::split(Transition::Id(0), Transition::End)],
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
        State::state(Condition::one('a'), Transition::Id(1)),
        State::state(Condition::one('b'), Transition::Id(3)),
        State::split(Transition::Id(0), Transition::Id(3)),
        State::state(Condition::one('a'), Transition::End)
    ], nfa.states);
    assert_eq!(2, nfa.start);
}

#[test]
fn build_one_or_more() {
    let nfa = NFA::from_expr(
            &Expr::one_or_more(Expr::Single('a')));
    // nfa for 'a+'

    assert_eq!(vec![
        State::state(Condition::one('a'), Transition::Id(1)),
        State::split(Transition::Id(0), Transition::End)
    ], nfa.states);
    assert_eq!(0, nfa.start);
}

#[test]
fn build_more_complex_one_or_more() {
    let nfa = NFA::from_expr(&"a+a+b".parse::<Expr>().unwrap());

    assert_eq!(vec![
        State::state(Condition::one('a'), Transition::Id(1)),
        State::split(Transition::Id(0), Transition::Id(2)),
        State::state(Condition::one('a'), Transition::Id(3)),
        State::split(Transition::Id(2), Transition::Id(4)),
        State::state(Condition::one('b'), Transition::End)
    ], nfa.states);
    assert_eq!(0, nfa.start);
}

#[test]
fn build_zero_or_more() {
    let nfa = NFA::from_expr(
        &Expr::zero_or_more(Expr::Single('a')));
    // 'a*'
    
    assert_eq!(vec![
        State::state(Condition::one('a'), Transition::Id(1)),
        State::split(Transition::Id(0), Transition::End)
    ], nfa.states);
    assert_eq!(1, nfa.start);
}

#[test]
fn build_more_complex_zero_or_more() {
    let nfa = NFA::from_expr(&"b*cd*".parse::<Expr>().unwrap());

    assert_eq!(vec![
        State::state(Condition::one('b'), Transition::Id(1)), // 0
        State::split(Transition::Id(0), Transition::Id(2)), // 1
        State::state(Condition::one('c'), Transition::Id(4)), // 2
        State::state(Condition::one('d'), Transition::Id(4)), // 3
        State::split(Transition::Id(3), Transition::End)// 4
    ], nfa.states);
    assert_eq!(1, nfa.start);
}

#[test]
fn build_doesnt_infinite_loop() {
    let nfa = NFA::from_expr(&"(ab)*".parse::<Expr>().unwrap());
    let nfa = NFA::from_expr(&"..+.".parse::<Expr>().unwrap());
}

#[test]
fn build_or() {
    let nfa = NFA::from_expr(
        &Expr::or(Expr::Single('a'), Expr::Single('b')));

    assert_eq!(vec![
        State::state(Condition::one('a'), Transition::End),
        State::state(Condition::one('b'), Transition::End),
        State::split(Transition::Id(0), Transition::Id(1))
    ], nfa.states);
    assert_eq!(2, nfa.start);
}

#[test]
fn build_any() {
    let nfa = NFA::from_expr(&Expr::Any);

    assert_eq!(vec![
        State::state(Condition::Any, Transition::End)
    ], nfa.states);
}

#[test]
fn prioritizes_state() {
    let s = State::state(Condition::one('a'), Transition::End);
    assert_eq!(97, s.get_priority_key(&NFA::new()));

    let s = State::state(Condition::Any, Transition::End);
    assert_eq!(0, s.get_priority_key(&NFA::new()));

    let s = State::state(Condition::None, Transition::End);
    assert_eq!(usize::max_value(), s.get_priority_key(&NFA::new()));

    // recursive
    let s = State::state(Condition::None, Transition::Id(0));
    assert_eq!(98, s.get_priority_key(&NFA::from_states(vec![
        State::state(Condition::None, Transition::Id(1)),
        State::state(Condition::one('b'), Transition::End)
    ])));

    // does not infinitely loop (assuming no free cycles)
    let s = State::state(Condition::None, Transition::Id(0));
    assert_eq!(0, s.get_priority_key(&NFA::from_states(vec![
        State::state(Condition::Any, Transition::Id(0))
    ])));
}

#[test]
fn prioritizes_split() {
    let nfa = NFA::from_states(vec![
        State::state(Condition::one('a'), Transition::End)
    ]);
    let s = State::split(Transition::Id(0), Transition::End);
    assert_eq!(97, s.get_priority_key(&nfa));

    let s = State::split(Transition::End, Transition::End);
    assert_eq!(usize::max_value(), s.get_priority_key(&NFA::new()));

    // recursive
    let s = State::split(Transition::Id(0), Transition::End);
    assert_eq!(0, s.get_priority_key(&NFA::from_states(vec![
        State::split(Transition::Id(1), Transition::Id(2)),
        State::state(Condition::one('a'), Transition::End),
        State::state(Condition::Any, Transition::End)
    ])));
}

#[test]
fn build_char_class() {
    let nfa = NFA::from_expr(&Expr::Class(vec!['a','b']));

    assert_eq!(vec![
        State::state(Condition::class(vec!['a','b']), Transition::End)
    ], nfa.states);
}

