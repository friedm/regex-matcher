use ::expr::Expr;

#[cfg(test)] mod spec;


#[derive(PartialEq,Debug,Clone)]
enum State {
    State{edge: Option<char>, out: Edge},
    Split{s1: Option<char>, out1: Edge, s2: Option<char>, out2: Edge}
}

#[derive(PartialEq,Debug,Clone)]
enum Edge {
    Id(usize),
    Detached,
    End
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

#[derive(Debug)]
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
        let mut nfa = Self::new();

        let id = nfa.build_expr(expr);
        nfa.update_outputs(id, Edge::End);
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
            }
            _ => panic!()
        };

        id
    }

    fn update_outputs(&mut self, start_id: usize, new_edge: Edge) {
        let state = self.states[start_id].clone();
        let state = match state {
            State::State{edge, ref out} => {
                State::state(edge, 
                             self.replace_edge(out.clone(), new_edge)
)
            },
            State::Split{s1, ref out1, s2, ref out2} => {
                State::split(s1,
                             self.replace_edge(out1.clone(), new_edge.clone()),
                             s2,
                             self.replace_edge(out2.clone(), new_edge.clone()))
            }
        };
        self.states[start_id] = state;
    }

    fn replace_edge(&mut self, edge: Edge, replacement: Edge) -> Edge {
        match &edge {
            &Edge::Detached => {
                replacement
            },
            &Edge::Id(id) => {
                self.update_outputs(id, replacement);
                edge
            },
            _ => edge
        }
    }
}

