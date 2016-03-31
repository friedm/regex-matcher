use ::nfa::{State, Edge, NFA};

#[cfg(test)] mod spec;

pub struct Matcher {
    states: Vec<(State, usize, String)>,
    nfa: NFA
}

impl Matcher {
    pub fn new(nfa: NFA, text: &str) -> Matcher {
        let start = nfa.get_start();

        let states = match start {
            None => Vec::new(),
            Some(state) => vec![
               (state, 0, text.to_owned())
            ]
        };

        Matcher {
            states: states,
            nfa: nfa
        }
    }

    pub fn run(&mut self) -> bool {

        if self.states.is_empty() { // regex is empty
            return true;
        }

        while !self.states.is_empty() {
            if self.step_and_check_all() {
                return true;
            }
        }

        false
    }

    fn step_and_check_all(&mut self) -> bool {
        let mut new_states = Vec::new();

        for state in self.states.clone() {
            let (ref state, offset, ref remaining_text) = state;
            let next_step = Self::step(state.clone(), offset, remaining_text);

            match next_step {
                None => return true,
                Some(step) => new_states.push(step)
            }
        }

        self.states = new_states;

        false
    }

    fn step(state: State, offset: usize, remaining_text: &str) -> Option<(State, usize, String)> {
        match state.clone() {
            State::State{edge, out} => {
                match edge {
                    None => Self::next_state(out, offset, remaining_text),
                    Some(c) => {
                        if remaining_text.len() == 0 {
                            None
                        } else if c == remaining_text.to_owned().into_bytes()[0] as char { // can consume the next char
                            Self::next_state(out, offset, remaining_text)
                        } else { // no longer valid state
                            None
                        }
                    }
                }
            },
            State::Split{s1, out1, s2, out2} => {
                None
            }
        }
    }

    fn next_state(edge: Edge, offset: usize, remaining_text: &str) -> Option<(State, usize, String)> {

        match edge {
            Edge::End => None,
            _ => panic!("invalid edge in complete NFA")
        }
    }
}

