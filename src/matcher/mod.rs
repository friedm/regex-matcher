use ::nfa::{State, Edge, NFA};

#[cfg(test)] mod spec;

#[derive(Clone)]
struct PotentialMatch {
    current_state: Option<State>,
    remaining_text: String
}

impl PotentialMatch {
    pub fn advance(&self, nfa: &NFA) -> Vec<PotentialMatch> {
        if self.current_state.is_none() {
            return vec![];
        }

        let current_state = self.current_state.clone().unwrap();

        match current_state {
            State::State{ref edge, ref out} => {
                match edge {
                    &Some(val) => {
                        if self.remaining_text.is_empty() {
                            return vec![];
                        }

                        if val == self.remaining_text.as_bytes()[0] as char {
                            // can consume char and advance along edge
                            match out {
                                &Edge::End => {
                                    vec![PotentialMatch::new(None, 
                                                             &self.remaining_text[1..])]
                                },
                                &Edge::Id(id) => {
                                    vec![PotentialMatch::new(nfa.get_state(id),
                                                             &self.remaining_text[1..])]
                                },
                                _ => panic!()
                            }
                        } else {
                            match out {
                                &Edge::End => {
                                    vec![PotentialMatch::new(None, 
                                                             &self.remaining_text)]
                                },
                                &Edge::Id(id) => {
                                    vec![PotentialMatch::new(nfa.get_state(id),
                                                             &self.remaining_text)]
                                },
                                _ => panic!()
                            }
                        }
                    },
                    &None => {
                        // can advance along empty edge
                        match out {
                            &Edge::End => {
                                vec![PotentialMatch::new(None,
                                                         &self.remaining_text)]
                            },
                            &Edge::Id(id) => {
                                vec![PotentialMatch::new(
                                        nfa.get_state(id),
                                        &self.remaining_text)]
                            },
                            _ => panic!()
                        }
                    }
                }
            },
            State::Split{ref s1, ref out1, ref s2, ref out2} => {
                vec![]
                //match (s1, s2) {
                //    // TODO match priority
                //    (Some(val1),Some(val2)) => {
                //        out1.advance().append(out2.advance())
                //    },
                //    (None,Some(val)) => {
                //        out2.advance().append(out1.advance())
                //    },
                //    (Some(val),None) => {
                //        out1.advance().append(out2.advance())
                //    },
                //    (None,None) => {
                //        out1.advance().append(out2.advance())
                //    }
                //}
            }
        }
    }

    pub fn is_match(&self) -> bool {
        match self.current_state {
            Some(ref current_state) => {
                match current_state {
                    &State::State{ref edge, ref out} => {
                        out.is_end() && self.remaining_text.is_empty()
                    },
                    &State::Split{ref s1, ref out1, ref s2, ref out2} => {
                        (out1.is_end() || out2.is_end()) &&
                            self.remaining_text.is_empty()
                    }
                }
            },
            None => self.remaining_text.is_empty()
        }
    }

    pub fn is_fail(&self) -> bool {
        match self.current_state {
            Some(ref current_state) => {
                match current_state {
                    &State::State{ref edge, ref out} => {
                        self.remaining_text.is_empty() &&
                            !out.is_end()
                    },
                    &State::Split{ref s1, ref out1, ref s2, ref out2} => {
                        self.remaining_text.is_empty() && 
                            !(out1.is_end() || out2.is_end())
                    }
                }
            },
            None => !self.remaining_text.is_empty()
        }
    }

    pub fn new(state: Option<State>, remaining_text: &str) -> PotentialMatch {
        PotentialMatch {
            current_state: state,
            remaining_text: remaining_text.to_owned()
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

    pub fn run(&mut self) -> bool {

        if self.nfa.num_states() == 0 { // regex is empty
            return true;
        }

        let mut states = vec![
            PotentialMatch::new(Some(self.nfa.get_start().unwrap()),
                                &self.text)
        ];

        while Self::has_valid_state(&states) {

            let mut updated_states = Vec::new();

            for state in states {
                let new_states = state.advance(&self.nfa);

                for s in new_states {
                    if s.is_fail() {
                        continue;
                    }

                    if s.is_match() {
                        return true;
                    } else {
                        updated_states.push(s);
                    }
                }
            }

            states = updated_states;
        }

        false
    }

    fn has_valid_state(states: &Vec<PotentialMatch>) -> bool {
        states.len() > 0
    }
}

