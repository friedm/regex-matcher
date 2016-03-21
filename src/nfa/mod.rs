use ::expr::Expr;

#[cfg(test)] mod spec;

impl Expr {
    fn is_match(&self, expr: &str) -> bool {

        if expr.len() == 0 { return false; } // false since the regex "" is not currently possible

        let mut chars = expr.chars();
        let mut current = chars.next().unwrap();
        let matches = match self {
            &Expr::Single(c) => {
                current == c
            },
            &Expr::Sequence(ref left, ref right) => {
                left.is_match(expr) && right.is_match(&expr[1..])
            },
            _ => panic!()
        };

        matches || self.is_match(&expr[1..])
    }
}

