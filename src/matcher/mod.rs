use ::nfa::{State, Edge, NFA};

#[cfg(test)] mod spec;

pub struct Matcher {
    state_list: Vec<(State, usize, String)>,
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
            state_list: states,
            nfa: nfa
        }
    }

    pub fn run(&mut self) -> bool {

        if self.state_list.is_empty() { // regex is empty
            return true;
        }

        while !self.state_list.is_empty() {
            if self.step_all() {
                return true;
            }
        }

        false
    }

    fn step_and_check_all(&mut self) -> bool {
        for i in 0..state_list.len() {
            let &(ref state, ref offset, ref remaining_text) = state;
            let next_step = Self::step(state, offset, remaining_text);
            if self.step_and_check(state, offset, remaining_text) {
                return true;
            }
        }

        false

        false
        self.state_list = self.state_list.iter()
            .map(|state| {
            }).collect::<Vec<_>>();
    }

    fn step_and_check(state: &State, offset: usize, remaining_text: String) -> bool {
        match state {
            &State::State{edge: None, out: ref edge} => { // this edge has no cost, just advance
            },
            &State::State{edge: Some(val), out: ref edge} => {
            },
            &State::Split{s1: s1, out1: ref out1, s2: s2, out2: ref out2} => {
            }
        };
        (State::state(Some('a'),Edge::End), 1, "a".to_owned())
    }
}

