use ::expr::Expr;

#[cfg(test)] mod spec;


#[derive(PartialEq,Clone,Debug)]
pub enum State {
    State{edge: char, out: Edge},
    Split{state1: State, state2: State}
}

enum Edge {
    Id(usize),
    Detached,
    End
}

impl State {
    pub fn state(edge: char, out: Edge) -> State {
        State::State{edge: edge, out: out}
    }

    pub fn split(state1: Edge, state2: Edge) -> State {
        State::Split{state1: state1,
                     state2: state2}
    }
}

struct NFA {
    states: Vec<State>
}

impl NFA {

    pub fn new() -> NFA {
        NFA {
            states: Vec::new()
        }
    }

    pub fn from_expr(expr: &Expr) -> NFA {
        let nfa = Self::new();

        let id = nfa.build_expr(expr);
        nfa.update_outputs(id, State::End);
        nfa
    }

    fn build_expr(&mut self, expr: &Expr) -> usize {
        let id = match expr {
            &Expr::Single(c) => {
                let new_state_id = self.states.len();
                let s = State::state(c, new_state_id + 1);
                self.states.push(s);
                self.states.push(State::Detached);
                self.states.len() - 1
            },
            &Expr::Sequence(ref a, ref b) => {
                let left_id = self.build_expr(&*a);
                let right_id = self.build_expr(&*b);
                self.update_outputs(left_id, right_id);
                left_id
            },
            _ => panic!()
        };

        id
    }

    fn update_outputs(self, start_id: usize, new_id: usize) {
        let state = match self.states[start_id] {
            State::State{edge, out} => {
                State::state(edge, 
                             match self.states[out] {
                                 State::Detached => {
                                     new_id
                                 },
                                 _ => {
                                     self.update_outputs(out, new_id);
                                     out
                                 }
                             })
            },
            _ => self.states[start_id]
        };
    }
}

