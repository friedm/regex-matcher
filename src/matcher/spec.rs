use ::nfa::{NFA, State, Transition, Condition};
use super::{Matcher, PotentialMatch};

#[test]
fn is_match() {
    let m = PotentialMatch::new(None, "");
    assert_eq!(true, m.is_match());

    let nfa = NFA::new();
    assert_eq!(vec![m.clone()], m.advance(&nfa));
}

#[test]
fn is_state_match() {
    let m = PotentialMatch::new(Some(State::state(Condition::one('a'), Transition::End)), "");
    assert_eq!(false, m.is_match());
}

#[test]
fn is_inconclusive() {
    let m = PotentialMatch::new(Some(State::state(Condition::None, Transition::End)), "a");
    assert_eq!(false, m.is_match());
}

#[test]
fn advance_to_end() {
    let nfa = NFA::from_states(vec![
        State::state(Condition::one('a'), Transition::End)
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
        State::state(Condition::one('a'), Transition::Id(1)),
        State::state(Condition::one('b'), Transition::End)
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
}

#[test]
fn advance_option_greedily() {
    let nfa = NFA::from_states(vec![ // nfa for 'a?'
        State::split(Transition::End, Transition::Id(1)),
        State::state(Condition::one('a'), Transition::End),
    ]);

    let expected_state = nfa.get_state(1).clone().unwrap();
    let m = PotentialMatch::new(nfa.get_start(), "a");

    let actual = m.advance(&nfa);
    println!("{:#?}", actual);
    assert_eq!(PotentialMatch::new(Some(expected_state), "a"),
               actual[0]);
    assert_eq!(PotentialMatch::new(None, "a"),
               actual[1]);
}

#[test]
fn advance_greedily() {
    let nfa = NFA::from_states(vec![ // nfa for 'a?'
        State::split(Transition::Id(2), Transition::Id(1)),
        State::state(Condition::one('a'), Transition::End),
        State::state(Condition::None, Transition::End),
    ]);

    let state_1 = nfa.get_state(1).clone().unwrap();
    let state_2 = nfa.get_state(2).clone().unwrap();

    let m = PotentialMatch::new(nfa.get_start(), "a");

    let actual = m.advance(&nfa);
    println!("{:#?}", actual);
    assert_eq!(PotentialMatch::new(Some(state_1), "a"),
               actual[0]);
    assert_eq!(PotentialMatch::new(Some(state_2), "a"),
               actual[1]);
}

#[test]
fn advance_fail() {
    let nfa = NFA::from_states(vec![
        State::state(Condition::one('b'), Transition::End)
    ]);
    let m = PotentialMatch::new(Some(nfa.get_state(0).clone().unwrap()), "a");
    assert_eq!(Vec::<PotentialMatch>::new(), m.advance(&nfa));
}

#[test]
fn advances_with_split() { // '(a|b)c'
    let nfa = NFA::from_states(vec![
        State::split(Transition::Id(1), Transition::Id(2)),
        State::state(Condition::one('a'), Transition::Id(3)),
        State::state(Condition::one('b'), Transition::Id(3)),
        State::state(Condition::one('c'), Transition::End)
    ]);

    let m = PotentialMatch::new(nfa.get_start(), "ac");
    assert_eq!(vec![
               PotentialMatch::new(nfa.get_state(3), "c")
    ], m.advance(&nfa)[0].advance(&nfa));

    let m = PotentialMatch::new(nfa.get_start(), "bc");
    assert_eq!(vec![
               PotentialMatch::new(nfa.get_state(3), "c")
    ], m.advance(&nfa)[1].advance(&nfa))
}


#[test]
fn is_match_with_split() {
    let nfa = NFA::from_states(vec![
        State::split(Transition::End, Transition::End)
    ]);

    let m = PotentialMatch::new(nfa.get_start(), "a");
    assert!(m.advance(&nfa)[0].is_match());
    let m = PotentialMatch::new(nfa.get_start(), "b");
    assert!(m.advance(&nfa)[0].is_match())
}

#[test]
fn empty_nfa_matches() {
    let nfa = NFA::from_states(vec![]);

    assert!(Matcher::new(nfa, "").run().is_some());
}

#[test]
fn single_state_matches() {
    let nfa = NFA::from_states(vec![
        State::state(Condition::None, Transition::End)
    ]);

    assert!(Matcher::new(nfa, "a").run().is_some());
}

#[test]
fn single_char_nfa_matches() {
    let nfa = NFA::from_states(vec![
        State::state(Condition::one('a'), Transition::End)
    ]);

    assert!(Matcher::new(nfa.clone(), "a").run().is_some());
    assert!(!Matcher::new(nfa.clone(), "").run().is_some());
}

#[test]
fn null_edge_matches() {
    let nfa = NFA::from_states(vec![
        State::state(Condition::None, Transition::Id(1)),
        State::state(Condition::one('a'), Transition::End)
    ]);

    assert!(Matcher::new(nfa.clone(), "a").run().is_some());
    assert!(!Matcher::new(nfa.clone(), "").run().is_some());
}

#[test]
fn char_class_matches() {
    let nfa = NFA::from_states(vec![
        State::state(Condition::class(vec!['a', 'b']),
           Transition::End)]);

    assert!(Matcher::new(nfa.clone(), "a").run().is_some());
    assert!(Matcher::new(nfa, "b").run().is_some());
}


