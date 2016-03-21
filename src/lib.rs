pub mod regex;
mod expression;
mod tokenize;
mod nfa;

pub use regex::Regex;

#[cfg(test)] mod spec;

