use ::expr::Expr;
use ::nfa::NFA;
use ::matcher::Matcher;

#[cfg(test)] mod spec;
#[cfg(test)] mod bench;

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
        self.match_offset(text).is_some()
    }

    pub fn match_offset(&self, text: &str) -> Option<usize> {
        Matcher::new(self.nfa.clone(), text).run()
    }
}

