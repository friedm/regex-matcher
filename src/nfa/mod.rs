use std::cmp;

use ::expr::Expr;

#[cfg(test)] mod spec;


#[derive(PartialEq,Debug,Clone,Eq,Hash)]
pub enum Condition {
    One(u8), // ascii encoded char
    Class(Vec<u8>), // list of valid ascii encoded chars
    Any,
    None
}

impl Condition {
    pub fn one(c: char) -> Condition {
        Condition::One(Self::to_ascii(c))
    }

    fn to_ascii(c: char) -> u8 {
        let mut buf = [0; 1];
        match c.encode_utf8(&mut buf) {
            Some(1) => buf[0],
            _ => panic!("attempted to create condition with non-ascii char")
        }
    }

    pub fn class(chars: Vec<char>) -> Condition {
        let ascii_bytes = chars.iter().map(|&c| Self::to_ascii(c)).collect::<Vec<_>>();

        Condition::Class(ascii_bytes)
    }
}


#[derive(PartialEq,Debug,Clone,Eq,Hash)]
pub enum Transition {
    Id(usize),
    Detached,
    End
}


#[derive(PartialEq,Debug,Clone,Eq,Hash)]
pub enum State {
    State{condition: Condition, out: Transition},
    Split{out1: Transition, out2: Transition}
}

impl State {
    pub fn state(condition: Condition, out: Transition) -> State {
        State::State{condition: condition, out: out}
    }

    pub fn split(out1: Transition, out2: Transition) -> State {
        State::Split{out1: out1,
                     out2: out2}
    }

    pub fn get_priority_key(&self, nfa: &NFA) -> usize { 
        // key by greediness and lexographical order of condition char
       
        match self {
            &State::State{ref condition, ref out} => {
                Self::get_transition_priority_key(condition, out, nfa)
            },
            &State::Split{ref out1, ref out2} => {
                cmp::min(
                    Self::get_transition_priority_key(&Condition::None, out1, nfa),
                    Self::get_transition_priority_key(&Condition::None, out2, nfa))
            }
        }
    }

    fn get_transition_priority_key(condition: &Condition, out: &Transition, nfa: &NFA) -> usize {
        match condition {
            &Condition::One(c) => c as usize, // there is a cost
            &Condition::Any => 0, // prioritize any
            &Condition::None => {
                match out {
                    &Transition::Id(id) => {
                        let next_state = nfa.get_state(id).unwrap();
                        next_state.get_priority_key(&nfa)
                    },
                    _ => usize::max_value() // state terminates with no cost
                }
            },
            &Condition::Class(ref chars) => 0
        }
    }
}

#[derive(PartialEq,Debug,Clone)]
pub struct NFA {
    start: usize,
    states: Vec<State>
}

impl NFA {

    pub fn new() -> NFA {
        NFA {
            start: 0,
            states: Vec::new()
        }
    }

    pub fn from_states(states: Vec<State>) -> NFA {
        NFA {
            start: 0,
            states: states
        }
    }

    pub fn get_start(&self) -> Option<State> {
        if self.start >= self.states.len() {
            None
        } else {
            Some(self.states[self.start].clone())
        }
    }

    pub fn get_state(&self, index: usize) -> Option<State> {
        if index >= self.states.len() {
            None
        } else {
            Some(self.states[index].clone())
        }

    }

    pub fn num_states(&self) -> usize {
        self.states.len()
    }

    pub fn from_expr(expr: &Expr) -> NFA {
        let mut nfa = Self::new();

        let start = nfa.build_expr(expr);
        nfa.update_outputs(start, Transition::End);
        nfa.start = start;
        nfa
    }

    fn build_expr(&mut self, expr: &Expr) -> usize {
        let id = match expr {
            &Expr::Any => {
                let s = State::state(Condition::Any, Transition::Detached);
                self.states.push(s);

                self.states.len() - 1
            },
            &Expr::Single(c) => {
                let s = State::state(Condition::one(c), Transition::Detached);
                self.states.push(s);

                self.states.len() - 1
            },
            &Expr::Class(ref chars) => {
                let s = State::state(Condition::class(chars.clone()),
                                     Transition::Detached);
                self.states.push(s);

                self.states.len() - 1
            },
            &Expr::Sequence(ref a, ref b) => {
                let left_id = self.build_expr(a);
                let right_id = self.build_expr(b);
                self.update_outputs(left_id, Transition::Id(right_id));

                left_id
            },
            &Expr::Optional(ref expr) => {
                let expr_id = self.build_expr(expr);
                let s = State::split(Transition::Id(expr_id), Transition::Detached);
                self.states.push(s);

                self.states.len() - 1
            },
            &Expr::OneOrMore(ref expr) => {
                let expr_id = self.build_expr(expr);
                let s = State::split(Transition::Id(expr_id), Transition::Detached);

                self.states.push(s);
                let split_id = self.states.len() - 1;
                self.update_outputs(expr_id, Transition::Id(split_id));

                expr_id
            },
            &Expr::ZeroOrMore(ref expr) => {
                let expr_id = self.build_expr(expr);
                let s = State::split(Transition::Id(expr_id), Transition::Detached);

                self.states.push(s);
                let split_id = self.states.len() - 1;
                self.update_outputs(expr_id, Transition::Id(split_id));

                split_id
            },
            &Expr::Or(ref expr1, ref expr2) => {
                let expr1_id = self.build_expr(expr1);
                let expr2_id = self.build_expr(expr2);

                let s = State::split(Transition::Id(expr1_id),
                                     Transition::Id(expr2_id));

                self.states.push(s);

                self.states.len() - 1
            }
        };

        id
    }

    fn update_outputs(&mut self, start_id: usize, new_edge: Transition) {
        self.update_outputs_rec(start_id, &mut vec![start_id], new_edge);
    }

    fn update_outputs_rec(&mut self, start_id: usize, visited: &mut Vec<usize>, new_edge: Transition) {
        let state = self.states[start_id].clone();
        let state = match state {
            State::State{ref condition, ref out} => {
                State::state(condition.clone(), 
                             self.replace_edge(out.clone(), new_edge, visited)
)
            },
            State::Split{ref out1, ref out2} => {
                let edge1 = self.replace_edge(out1.clone(), new_edge.clone(), visited);
                let edge2 = self.replace_edge(out2.clone(), new_edge.clone(), visited);

                State::split(edge1,
                             edge2)
            }
        };
        self.states[start_id] = state;
    }

    fn replace_edge(&mut self, edge: Transition, replacement: Transition, visited: &mut Vec<usize>) -> Transition {
        match &edge {
            &Transition::Detached => {
                replacement
            },
            &Transition::Id(id) => {
                if !visited.contains(&id) { // don't recurse further
                    visited.push(id);
                    self.update_outputs_rec(id, visited, replacement);
                }

                edge
            },
            _ => edge
        }
    }

}

