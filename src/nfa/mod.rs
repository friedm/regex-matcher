use ::expr::Expr;

#[cfg(test)] mod spec;


#[derive(PartialEq,Debug,Clone)]
pub enum State {
    State{edge: Option<char>, out: Edge},
    Split{s1: Option<char>, out1: Edge, s2: Option<char>, out2: Edge}
}

#[derive(PartialEq,Debug,Clone)]
pub enum Edge {
    Id(usize),
    Detached,
    End
}

impl Edge {
    pub fn is_end(&self) -> bool {
        match self {
            &Edge::End => true,
            _ => false
        }
    }
}

impl State {
    pub fn state(edge: Option<char>, out: Edge) -> State {
        State::State{edge: edge, out: out}
    }

    pub fn split(s1: Option<char>, out1: Edge, s2: Option<char>, out2: Edge) -> State {
        State::Split{s1: s1,
                     out1: out1,
                     s2: s2,
                     out2: out2}
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
        nfa.update_outputs(start, Edge::End);
        nfa.start = start;
        nfa
    }

    fn build_expr(&mut self, expr: &Expr) -> usize {
        let id = match expr {
            &Expr::Single(c) => {
                let new_state_id = self.states.len();
                let s = State::state(Some(c), Edge::Detached);
                self.states.push(s);
                self.states.len() - 1
            },
            &Expr::Sequence(ref a, ref b) => {
                let left_id = self.build_expr(a);
                let right_id = self.build_expr(b);
                self.update_outputs(left_id, Edge::Id(right_id));
                left_id
            },
            &Expr::Optional(ref expr) => {
                let expr_id = self.build_expr(expr);
                let s = State::split(None, Edge::Id(expr_id), None, Edge::Detached);
                self.states.push(s);
                self.states.len() - 1
            },
            &Expr::OneOrMore(ref expr) => {
                let expr_id = self.build_expr(expr);
                let s = State::split(None, Edge::Id(expr_id), None, Edge::Detached);

                self.states.push(s);
                let split_id = self.states.len() - 1;
                self.update_outputs(expr_id, Edge::Id(split_id));

                expr_id
            },
            _ => panic!()
        };

        id
    }

    fn update_outputs(&mut self, start_id: usize, new_edge: Edge) {
        self.update_outputs_rec(start_id, start_id, new_edge);
    }

    fn update_outputs_rec(&mut self, start_id: usize, initial_id: usize, new_edge: Edge) {
        let state = self.states[start_id].clone();
        let state = match state {
            State::State{edge, ref out} => {
                State::state(edge, 
                             self.replace_edge(out.clone(), new_edge, start_id)
)
            },
            State::Split{s1, ref out1, s2, ref out2} => {
                State::split(s1,
                             self.replace_edge(out1.clone(), new_edge.clone(), initial_id),
                             s2,
                             self.replace_edge(out2.clone(), new_edge.clone(), initial_id))
            }
        };
        self.states[start_id] = state;
    }

    fn replace_edge(&mut self, edge: Edge, replacement: Edge, start_id: usize) -> Edge {
        match &edge {
            &Edge::Detached => {
                replacement
            },
            &Edge::Id(id) => {
                if id != start_id { // don't recurse further
                    self.update_outputs_rec(id, start_id, replacement);
                }

                edge
            },
            _ => edge
        }
    }

}

