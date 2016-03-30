use ::expr::Expr;
use ::nfa::NFA;

#[cfg(test)] mod spec;

#[derive(PartialEq,Debug)]
pub struct Regex {
    nfa: NFA
}

impl Regex {
    pub fn from(pattern: &str) -> Result<Regex, String> {
        let expr = pattern.parse::<Expr>();
        expr.map(|expr| Regex {
            nfa: NFA::from_expr(&expr)
        })
    }

    pub fn is_match(&self, text: &str) -> bool {
        self.nfa.run(text)
    }
}

