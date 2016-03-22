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

        let state = Self::build_expr(expr);
        Self::with_outputs(state, State::End)
    }

    fn build_expr(expr: &Expr) -> State {
        match expr {
            &Expr::Single(c) => {
                State::state(c, State::Detached)
            },
            &Expr::Sequence(ref left, ref right) => {

                let left = Self::build_expr(left);
                let right = Self::build_expr(right);

                Self::with_outputs(left, right)
            },
            &Expr::Or(ref left, ref right) => {

                let left = Self::build_expr(left);
                let right = Self::build_expr(right);

                State::split(left, right)
            },
            &Expr::Optional(ref item) => {
                State::End
            },
            _ => panic!()
        }
    }

    // recursively replace all detached outputs with the given state
    fn with_outputs(self, new_state: State) -> State {
        match self {
            State::State{edge, out} => 
                State::State{edge: edge,  
                             out: Box::new(Self::with_outputs(*out, new_state.clone()))},
            State::Split{state1, state2} =>
                State::Split{state1: Box::new(Self::with_outputs(*state1, new_state.clone())),
                             state2: Box::new(Self::with_outputs(*state2, new_state.clone()))},
            State::Detached => new_state,
            State::End => State::End
        }
    }
}

