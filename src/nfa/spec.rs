use ::expr::Expr;
use super::{State};

#[test]
fn build_single() {
    let nfa = State::from_expr(&Expr::Single('a'));

    assert_eq!(State::state('a', State::End), nfa);
}

