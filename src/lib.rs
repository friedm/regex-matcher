#![feature(unicode)]
#![feature(test)]

pub mod regex;

mod expr;
mod nfa;
mod matcher;

pub use regex::Regex;
pub use expr::Expr;

#[cfg(test)] mod spec;

