pub mod regex;
mod nfa;
mod expr;

pub use regex::Regex;
pub use expr::Expr;

#[cfg(test)] mod spec;

