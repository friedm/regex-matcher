use std::str::FromStr;
use std::collections::vec_deque::VecDeque;

#[cfg(test)] mod spec;

#[derive(PartialEq, Debug)]
pub enum Expr {
    Single(char),
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
}

impl FromStr for Expr {
    type Err = String;

    fn from_str(s: &str) -> Result<Expr,String> {

        let mut output_queue = VecDeque::<Expr>::new();
        let mut operator_stack = Vec::<char>::new();
        let mut last_was_char = false;

        for c in s.chars() {
            if c == '(' {
                if !output_queue.is_empty() && last_was_char {
                    operator_stack.push('@'); // "sequence" operator
                }
                operator_stack.push(c);
                last_was_char = false;
            } else if c == ')' {
                let mut top = operator_stack.pop().expect("mismatched parens");
                while top != '(' {
                    pop_infix_operator(top, &mut output_queue);
                    top = operator_stack.pop().unwrap();
                }
                last_was_char = false;
            } else if binary_operators.contains(&c) {
                while !operator_stack.is_empty() {
                    if operator_stack.last().unwrap() == &'(' { break; } // parens have higher prescedence than any other operator
                    pop_infix_operator(operator_stack.pop().unwrap(), &mut output_queue);
                }
                operator_stack.push(c);
                last_was_char = false;
            } else if unary_postfix_operators.contains(&c) {
                apply_postfix_operator(c, &mut output_queue);
                last_was_char = false;
            } else {
                if !output_queue.is_empty() && last_was_char {
                    operator_stack.push('@'); // "sequence" operator
                }
                output_queue.push_back(Expr::Single(c));
                last_was_char = true;
            }
        }

        while !operator_stack.is_empty() {
            pop_infix_operator(operator_stack.pop().unwrap(), &mut output_queue);
        }

        // build sequence tree from queue
        while output_queue.len() > 1 {
            let right = output_queue.pop_back().unwrap();
            let left = output_queue.pop_back().unwrap();

            output_queue.push_back(Expr::sequence(left,
                                                  right));
        }

        output_queue.pop_front().ok_or("output queue empty".to_owned())
    }
}

static unary_postfix_operators: &'static [char] = &['?', '*', '+'];
static binary_operators: &'static [char] = &['|'];

fn pop_infix_operator(operator: char, output_queue: &mut VecDeque<Expr>) {
    match operator {
        '|' => { 
            apply_binary_operator(output_queue, &|l,r| Expr::Or(l,r));
        },
        '@' => { // sequence operator (inserted between consecutive single chars)
            apply_binary_operator(output_queue, &|l,r| Expr::Sequence(l,r));
        },
        op => panic!("unknown infix operator {}", op)
    }
}

fn apply_postfix_operator(operator: char, output_queue: &mut VecDeque<Expr>) {
    match operator {
        '?' => {
            apply_unary_operator(output_queue, &|expr| Expr::Optional(expr));
        },
        '*' => {
            apply_unary_operator(output_queue, &|expr| Expr::ZeroOrMore(expr));
        },
        '+' => {
            apply_unary_operator(output_queue, &|expr| Expr::OneOrMore(expr));
        },
        _ => panic!("unknown postfix operator")
    }
}

fn apply_binary_operator(output_queue: &mut VecDeque<Expr>, 
                         constructor: &Fn(Box<Expr>, Box<Expr>) -> Expr) {

    let right = output_queue.pop_back().expect("not enough elements in queue for binary operator");
    let left = output_queue.pop_back().expect("not enough elements in queue for binary operator");

    output_queue.push_back(constructor(Box::new(left),Box::new(right)));
}

fn apply_unary_operator(output_queue: &mut VecDeque<Expr>,
                        constructor: &Fn(Box<Expr>) -> Expr) {

    let item = output_queue.pop_back().unwrap();
    output_queue.push_back(constructor(Box::new(item)));
}

