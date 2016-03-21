pub mod regex;
mod expression;
mod tokenize;
mod nfa;
mod expr;

pub use regex::Regex;

#[cfg(test)] mod spec;

