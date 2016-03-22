use ::expr::Expr;

#[cfg(test)] mod spec;

struct NFA {
    states: Vec<State>
}

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
        state.with_outputs(State::End)
    }

    fn build_expr(expr: &Expr) -> State {
        match expr {
            &Expr::Single(c) => {
                State::state(c, State::Detached)
            },
            &Expr::Sequence(ref left, ref right) => {

                let left = Self::build_expr(left);
                let right = Self::build_expr(right);

                left.with_outputs(right)
            },
            &Expr::Or(ref left, ref right) => {

                let left = Self::build_expr(left);
                let right = Self::build_expr(right);

                State::split(left, right)
            },
            &Expr::Optional(ref item) => {

                let item = Self::build_expr(item);
                State::split(item, State::Detached)
            },
            //&Expr::ZeroOrMore(ref item) => {

                //let mut split = State::split(State::Detached, State::Detached);
                //let item = Self::build_expr(item);
                //let boxed_item = item.with_outputs_to_box(Box::new(split));

                //State::Detached

            //}
            _ => panic!()
        }
    }

    // recursively replace all detached outputs with the given state
    fn with_outputs(self, new_state: State) -> State {
        match self {
            State::State{edge, out} => 
                State::State{edge: edge,  
                             out: Box::new((*out).with_outputs(new_state.clone()))},
            State::Split{state1, state2} =>
                State::Split{state1: Box::new((*state1).with_outputs(new_state.clone())),
                             state2: Box::new((*state2).with_outputs(new_state.clone()))},
            State::Detached => new_state,
            State::End => State::End
        }
    }

    //fn with_outputs_to_box(self, new_state: Box<State>) -> Box<State> {
        //match self {
            //State::State{edge, out} => 
                //Box::new(State::State{edge: edge,  
                    //out: (*out).with_outputs_to_box(new_state)}),
            //State::Split{state1, state2} =>
                //Box::new(State::Split{state1: (*state1).with_outputs_to_box(new_state),
                             //state2: (*state2).with_outputs_to_box(new_state)}),
            //State::Detached => new_state,
            //State::End => Box::new(State::End)
        //}
    //}
}

