use ::nfa::{NFA, State, Edge};
use super::Matcher;

#[test]
fn empty_nfa_matches() {
    let nfa = NFA::from_states(vec![]);

    assert!(Matcher::new(nfa, "").run());
}

#[test]
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

    assert!(!Matcher::new(nfa.clone(), "").run());
    assert!(Matcher::new(nfa, "a").run());
}

