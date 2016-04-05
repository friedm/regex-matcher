use std::str::{FromStr, Chars};

#[cfg(test)] mod spec;

static UNARY_POSTFIX_OPERATORS: &'static [char] = &['?', '*', '+'];
static BINARY_OPERATORS: &'static [char] = &['|'];
static SPECIAL_CHARS: &'static [char] = &['.'];

#[derive(PartialEq, Debug)]
pub enum Expr {
    Single(char),
    Class(Vec<char>),
    Any,
    Sequence(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Optional(Box<Expr>),
    ZeroOrMore(Box<Expr>),
    OneOrMore(Box<Expr>)
}

impl Expr {
    pub fn sequence(left: Expr, right: Expr) -> Expr {
        Expr::Sequence(Box::new(left), Box::new(right))
    }

    pub fn or(left: Expr, right: Expr) -> Expr {
        Expr::Or(Box::new(left), Box::new(right))
    }

    pub fn optional(item: Expr) -> Expr {
        Expr::Optional(Box::new(item))
    }

    pub fn one_or_more(item: Expr) -> Expr {
        Expr::OneOrMore(Box::new(item))
    }

    pub fn zero_or_more(item: Expr) -> Expr {
        Expr::ZeroOrMore(Box::new(item))
    }
}

impl FromStr for Expr {
    type Err = String;

    fn from_str(s: &str) -> Result<Expr,String> {

        ExprBuilder::from(s).parse()
    }
}


struct ExprBuilder {
    chars: Vec<char>,
    output_stack: Vec<Expr>,
    operator_stack: Vec<char>
}

impl ExprBuilder {

    pub fn from(s: &str) -> Self {
        ExprBuilder {
            chars: s.chars().collect::<Vec<_>>(),
            output_stack: Vec::<Expr>::new(),
            operator_stack: Vec::<char>::new()
        }
    }

    pub fn parse(&mut self) -> Result<Expr,String> {
        let mut last_was_char = false;
        let mut in_char_class = false;

        let mut current_class = Vec::new();

        for c in &self.chars {
            let c = *c;

            if in_char_class {
                if c == ']' {
                    self.output_stack.push(Expr::Class(current_class));
                    current_class = Vec::new();
                    in_char_class = false;
                } else {
                    current_class.push(c);
                }

                continue;
            }

            if c == '(' {

                if !self.output_stack.is_empty() && last_was_char {
                    self.operator_stack.push('@'); // "sequence" operator
                }
                self.operator_stack.push(c);
                last_was_char = false;

            } else if c == ')' {

                let mut top = self.operator_stack.pop().expect("mismatched parens");
                while top != '(' {
                    self.pop_infix_operator(top);
                    top = self.operator_stack.pop().unwrap();
                }

            } else if c == '[' {
                last_was_char = false;
                in_char_class = true;
            } else if c == ']' {
                panic!();
            } else if BINARY_OPERATORS.contains(&c) {

                while !self.operator_stack.is_empty() {
                    if self.operator_stack.last().unwrap() == &'(' { break; } // parens have higher prescedence than any other operator
                    self.pop_infix_operator(self.operator_stack.pop().unwrap());
                }
                self.operator_stack.push(c);
                last_was_char = false;

            } else if UNARY_POSTFIX_OPERATORS.contains(&c) {

                self.apply_postfix_operator(c);
                last_was_char = false;

            } else if SPECIAL_CHARS.contains(&c) {

                if !self.output_stack.is_empty() && last_was_char {
                    self.operator_stack.push('@'); // "sequence" operator
                }
                self.output_stack.push(Expr::Any);
                last_was_char = true;

            } else { // literal char

                if !self.output_stack.is_empty() && last_was_char {
                    self.operator_stack.push('@'); // "sequence" operator
                }
                self.output_stack.push(Expr::Single(c));
                last_was_char = true;

            }
        }

        for operator in self.operator_stack {
            self.pop_infix_operator(operator);
        }

        // build sequence tree from queue
        while self.output_stack.len() > 1 {
            let right = self.output_stack.pop().unwrap();
            let left = self.output_stack.pop().unwrap();

            self.output_stack.push(Expr::sequence(left,
                                                  right));
        }

        if self.output_stack.is_empty() {
            Err("output stack empty".to_owned())
        } else {
            Ok(self.output_stack[0])
        }
    }

    fn pop_infix_operator(&mut self, operator: char) {
        match operator {
            '|' => { 
                self.apply_binary_operator(&|l,r| Expr::Or(l,r));
            },
            '@' => { // sequence operator (inserted between consecutive single chars)
                self.apply_binary_operator(&|l,r| Expr::Sequence(l,r));
            },
            op => panic!("unknown infix operator {}", op)
        }
    }

    fn apply_postfix_operator(&mut self, operator: char) {
        match operator {
            '?' => {
                self.apply_unary_operator(&|expr| Expr::Optional(expr));
            },
            '*' => {
                self.apply_unary_operator(&|expr| Expr::ZeroOrMore(expr));
            },
            '+' => {
                self.apply_unary_operator(&|expr| Expr::OneOrMore(expr));
            },
            _ => panic!("unknown postfix operator")
        }
    }

    fn apply_binary_operator(&mut self,
                             constructor: &Fn(Box<Expr>, Box<Expr>) -> Expr) {

        let right = self.output_stack.pop().expect("not enough elements in queue for binary operator");
        let left = self.output_stack.pop().expect("not enough elements in queue for binary operator");

        self.output_stack.push(constructor(Box::new(left),Box::new(right)));
    }

    fn apply_unary_operator(&mut self,
                            constructor: &Fn(Box<Expr>) -> Expr) {

        let item = self.output_stack.pop().unwrap();
        self.output_stack.push(constructor(Box::new(item)));
    }
}

