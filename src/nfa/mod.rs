use ::expr::Expr;

#[cfg(test)] mod spec;

impl Expr {
    pub fn is_match(&self, expr: &str) -> bool {

        if expr.len() == 0 { 
            return match self {
                &Expr::Optional(_) => true,
                _ => false
            }; 
        } 

        let mut chars = expr.chars();
        let mut current = chars.next().unwrap();
        let matches = match self {
            &Expr::Single(c) => {
                current == c
            },
            &Expr::Sequence(ref left, ref right) => {
                left.is_match(expr) && right.is_match(&expr[1..])
            },
            &Expr::Or(ref left, ref right) => {
                left.is_match(expr) || right.is_match(expr)
            },
            &Expr::Optional(ref item) => {
                true
            },
            _ => panic!()
        };

        matches || self.is_match(&expr[1..])
    }
}

