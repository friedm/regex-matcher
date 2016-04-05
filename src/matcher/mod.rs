use std::collections::HashSet;

use ::nfa::{State, Edge, NFA, ConditionChar};

#[cfg(test)] mod spec;

#[derive(Clone,PartialEq,Eq,Hash,Debug)]
struct PotentialMatch {
    current_state: Option<State>,
    text: Vec<u8>
}

impl PotentialMatch {
    pub fn advance(&self, nfa: &NFA) -> Vec<PotentialMatch> {
        if self.current_state.is_none() {
            return vec![self.clone()];
        }

        let current_state = self.current_state.clone().unwrap();

        match current_state {
            State::State{ref condition, ref out} => {

                let mut result = Vec::new();
                Self::push_option(
                    &mut result,
                    self.next_for_edge(nfa, condition, out));
                result
            },
            State::Split{ref out1, ref out2} => {
                let c1_next = self.next_for_edge(nfa, &ConditionChar::None, out1);
                let c2_next = self.next_for_edge(nfa, &ConditionChar::None, out2);

                let mut result = Vec::new();
                Self::push_option(&mut result, c1_next);
                Self::push_option(&mut result, c2_next);

                result.sort_by_key(|item| {
                    match &item.current_state {
                        &None => usize::max_value(), // this is an end state
                        &Some(ref state) => state.get_priority_key(&nfa)
                    }
                });

                result
            }
        }
    }

    fn next_for_edge(&self, nfa: &NFA, condition: &ConditionChar, out: &Edge) -> Option<PotentialMatch> {
        match condition {
            &ConditionChar::One(val) => {
                if self.text.is_empty() {
                    // no character to consume, this potential match cannot continue
                    return None;
                }

                if val == self.text[0] {
                    // can consume char and advance along edge
                    match out {
                        &Edge::End => {
                            Some(self.with_state_and_increment(None))
                        },
                        &Edge::Id(id) => {
                            Some(self.with_state_and_increment(nfa.get_state(id)))
                        },
                        _ => panic!()
                    }
                } else { 
                    // potential match cannot proceed, mismatched character
                    None
                }
            },
            &ConditionChar::None => {
                // can advance along empty edge
                match out {
                    &Edge::End => {
                        Some(self.with_state(None))
                    },
                    &Edge::Id(id) => {
                        Some(self.with_state(nfa.get_state(id)))
                    },
                    _ => panic!("cannot evaluate incomplete NFA")
                }
            },
            &ConditionChar::Any => {
                if self.text.is_empty() {
                    return None;
                }

                if self.text[0] as char == '\n' {
                    return None; // `.` should not match newline
                }

                match out {
                    &Edge::End => {
                        Some(self.with_state_and_increment(None))
                    },
                    &Edge::Id(id) => {
                        Some(self.with_state_and_increment(nfa.get_state(id)))
                    },
                    _ => panic!()
                }
            },
            &ConditionChar::Class(ref chars) => {
                if self.text.is_empty() {
                    // no character to consume, this potential match cannot continue
                    return None;
                }

                if chars.contains(&self.text[0]) {
                    // can consume char and advance along edge
                    match out {
                        &Edge::End => {
                            Some(self.with_state_and_increment(None))
                        },
                        &Edge::Id(id) => {
                            Some(self.with_state_and_increment(nfa.get_state(id)))
                        },
                        _ => panic!()
                    }
                } else { 
                    // potential match cannot proceed, mismatched character
                    None
                }
            }
        }
    }

    pub fn push_option<T>(vec: &mut Vec<T>, item: Option<T>) {
        match item {
            Some(item) => vec.push(item),
            None => ()
        }
    }

    pub fn is_match(&self) -> bool {
        self.current_state.is_none()
    }

    pub fn new(state: Option<State>, text: &str) -> Self {
        PotentialMatch {
            current_state: state,
            text: Vec::from(text.to_owned().as_bytes())
        }
    }

    fn with_state(&self, state: Option<State>) -> Self {
        PotentialMatch {
            current_state: state,
            text: self.text.clone()
        }
    }

    fn with_state_and_increment(&self, state: Option<State>) -> Self {
        PotentialMatch {
            current_state: state,
            text: Vec::from(&self.text[1..])
        }
    }
}


pub struct Matcher {
    nfa: NFA,
    text: String
}

impl Matcher {
    pub fn new(nfa: NFA, text: &str) -> Matcher {
        Matcher {
            nfa: nfa,
            text: text.to_owned()
        }
    }

    pub fn run(&mut self) -> Option<usize> { // return optional end offset of match

        if self.nfa.num_states() == 0 { // regex is empty
            return Some(0);
        }

        let first_potential_match = PotentialMatch::new(
            Some(self.nfa.get_start().unwrap()),
                 &self.text);


        let mut set = HashSet::new();
        set.insert(first_potential_match.clone());
        let mut states = vec![first_potential_match];

        while states.len() > 0 {
            let state = states.pop().unwrap();

            if state.is_match() {
                let num_chars_remaining = state.text.len();
                return Some(self.text.len() - num_chars_remaining);
            }

            let mut new_states = state.advance(&self.nfa);
            // states are in order of greediness
            new_states.reverse(); 

            for state in new_states {

                if !set.contains(&state) {
                    states.push(state.clone());
                    set.insert(state);
                }
            }
        }

        None
    }
}

