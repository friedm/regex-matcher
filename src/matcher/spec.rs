use ::nfa::{NFA, State, Edge, ConditionChar};
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
    let m = PotentialMatch::new(Some(State::state(ConditionChar::one('a'), Edge::End)), "");
    assert_eq!(false, m.is_match());
}

#[test]
fn is_inconclusive() {
    let m = PotentialMatch::new(Some(State::state(ConditionChar::None, Edge::End)), "a");
    assert_eq!(false, m.is_match());
}

#[test]
fn advance_to_end() {
    let nfa = NFA::from_states(vec![
        State::state(ConditionChar::one('a'), Edge::End)
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
        State::state(ConditionChar::one('a'), Edge::Id(1)),
        State::state(ConditionChar::one('b'), Edge::End)
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
        State::split(ConditionChar::None, Edge::End, ConditionChar::None, Edge::Id(1)),
        State::state(ConditionChar::one('a'), Edge::End),
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
        State::split(ConditionChar::None, Edge::Id(2), ConditionChar::None, Edge::Id(1)),
        State::state(ConditionChar::one('a'), Edge::End),
        State::state(ConditionChar::None, Edge::End),
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
        State::state(ConditionChar::one('b'), Edge::End)
    ]);
    let m = PotentialMatch::new(Some(nfa.get_state(0).clone().unwrap()), "a");
    assert_eq!(Vec::<PotentialMatch>::new(), m.advance(&nfa));
}

#[test]
fn advances_with_split() { // '(a|b)c'
    let nfa = NFA::from_states(vec![
        State::split(ConditionChar::one('a'), Edge::Id(1), ConditionChar::one('b'), Edge::Id(1)),
        State::state(ConditionChar::one('c'), Edge::End)
    ]);

    let m = PotentialMatch::new(nfa.get_start(), "ac");
    assert_eq!(vec![
               PotentialMatch::new(nfa.get_state(1), "c"),
    ],
        m.advance(&nfa));

    let m = PotentialMatch::new(nfa.get_start(), "bc");
    assert_eq!(vec![
               PotentialMatch::new(nfa.get_state(1), "c"),
    ],
        m.advance(&nfa))
}


#[test]
fn is_match_with_split() {
    let nfa = NFA::from_states(vec![
        State::split(ConditionChar::one('a'), Edge::End, ConditionChar::one('b'), Edge::End)
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
        State::state(ConditionChar::None, Edge::End)
    ]);

    assert!(Matcher::new(nfa, "a").run().is_some());
}

#[test]
fn single_char_nfa_matches() {
    let nfa = NFA::from_states(vec![
        State::state(ConditionChar::one('a'), Edge::End)
    ]);

    assert!(Matcher::new(nfa.clone(), "a").run().is_some());
    assert!(!Matcher::new(nfa.clone(), "").run().is_some());
}

#[test]
fn null_edge_matches() {
    let nfa = NFA::from_states(vec![
        State::state(ConditionChar::None, Edge::Id(1)),
        State::state(ConditionChar::one('a'), Edge::End)
    ]);

    assert!(Matcher::new(nfa.clone(), "a").run().is_some());
    assert!(!Matcher::new(nfa.clone(), "").run().is_some());
}

#[test]
fn char_class_matches() {
    let nfa = NFA::from_states(vec![
        State::state(ConditionChar::class(vec!['a', 'b']),
           Edge::End)]);

    assert!(Matcher::new(nfa.clone(), "a").run().is_some());
    assert!(Matcher::new(nfa, "b").run().is_some());
}


