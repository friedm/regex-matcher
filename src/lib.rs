#![feature(unicode)]
#![feature(test)]

pub mod regex;

mod matcher;
mod nfa;
mod expr;

pub use regex::Regex;
pub use expr::Expr;

#[cfg(test)] mod spec;

