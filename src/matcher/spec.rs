use ::nfa::{NFA, State, Edge};
use super::{Matcher, PotentialMatch};

#[test]
fn is_fail() {
    let m = PotentialMatch::new(None, "a");
    assert_eq!(false, m.is_match());
    assert_eq!(true, m.is_fail());
}

#[test]
fn is_match() {
    let m = PotentialMatch::new(None, "");
    assert_eq!(true, m.is_match());
    assert_eq!(false, m.is_fail());
}

#[test]
fn empty_nfa_matches() {
    let nfa = NFA::from_states(vec![]);

    assert!(Matcher::new(nfa, "").run());
}

#[test]
#[ignore]
fn single_state_matches() {
    let nfa = NFA::from_states(vec![
        State::state(None, Edge::End)
    ]);

    assert!(Matcher::new(nfa, "a").run());
}

#[test]
fn single_char_nfa_matches() {
    let nfa = NFA::from_states(vec![
        State::state(Some('a'), Edge::End)
    ]);

    assert!(Matcher::new(nfa.clone(), "a").run());
    assert!(!Matcher::new(nfa.clone(), "").run());
}

#[test]
#[ignore]
fn null_edge_matches() {
    let nfa = NFA::from_states(vec![
        State::state(None, Edge::Id(1)),
        State::state(Some('a'), Edge::End)
    ]);

    assert!(Matcher::new(nfa.clone(), "a").run());
    assert!(!Matcher::new(nfa.clone(), "").run());
}

