use ::nfa::{NFA, State, Edge};
use super::{Matcher, PotentialMatch};

#[test]
fn advance_fail() {
    let nfa = NFA::from_states(vec![
        State::state(Some('b'), Edge::End)
    ]);
    let m = PotentialMatch::new(Some(nfa.get_state(0).clone().unwrap()), "a");
    assert_eq!(Vec::<PotentialMatch>::new(), m.advance(&nfa));
}

#[test]
fn is_match() {
    let m = PotentialMatch::new(None, "");
    assert_eq!(true, m.is_match());
    assert_eq!(false, m.is_fail());
}

#[test]
fn is_inconclusive() {
    let m = PotentialMatch::new(Some(State::state(None, Edge::End)), "a");
    assert_eq!(false, m.is_match());
    assert_eq!(false, m.is_fail());
}

#[test]
fn advance_to_end() {
    let nfa = NFA::from_states(vec![
        State::state(Some('a'), Edge::End)
    ]);

    let m = PotentialMatch::new(nfa.get_start(),
                                "a");
    assert_eq!(vec![PotentialMatch::new(None,"")], m.advance(&nfa));
    let m = PotentialMatch::new(nfa.get_start(),
                                "");
    assert_eq!(Vec::<PotentialMatch>::new(), m.advance(&nfa));
}

#[test]
fn advance_to_next() {
    let nfa = NFA::from_states(vec![
        State::state(Some('a'), Edge::Id(1)),
        State::state(Some('b'), Edge::End)
    ]);

    let expected_state = nfa.get_state(1).clone().unwrap();
    let m = PotentialMatch::new(nfa.get_start(), "ab");
    assert_eq!(vec![PotentialMatch::new(Some(expected_state),
                                        "b")],
                                        m.advance(&nfa));

    let m = PotentialMatch::new(nfa.get_state(1), "b");
    let final_state = m.advance(&nfa);

    assert_eq!(vec![PotentialMatch::new(None, "")],
        final_state);
    assert!(final_state[0].is_match());
    assert!(!final_state[0].is_fail());
}

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

