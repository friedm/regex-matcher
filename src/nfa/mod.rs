use ::expr::Expr;

#[cfg(test)] mod spec;


#[derive(PartialEq,Clone,Debug)]
pub enum State {
    State{edge: char, out: Box<State>},
    Split{state1: Box<State>, state2: Box<State>},
    Detached,
    End
}

impl State {
    pub fn state(edge: char, out: State) -> State {
        State::State{edge: edge, out: Box::new(out)}
    }

    pub fn split(state1: State, state2: State) -> State {
        State::Split{state1: Box::new(state1),
                     state2: Box::new(state2)}
    }

    pub fn from_expr(expr: &Expr) -> State {
        let expr = Self::build_expr(expr);
        expr.with_outputs(State::End)
    }

    fn build_expr(expr: &Expr) -> State {
        match expr {
            &Expr::Single(c) => {
                State::state(c, State::Detached)
            },
            _ => panic!()
        }
    }

    fn with_outputs(self, new_state: State) -> Self {
        match self {
            State::State{edge, out} => {
                State::state(edge, 
                             match *out {
                                 State::Detached => {
                                     new_state
                                 },
                                 _ => *out
                             })
            },
            _ => self
        }
    }
}

