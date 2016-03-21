use std::str::FromStr;
use std::collections::vec_deque::VecDeque;

#[cfg(test)] mod spec;

#[derive(PartialEq, Debug)]
enum Expr {
    Single(char),
    Sequence(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Optional(Box<Expr>),
    ZeroOrMore(Box<Expr>),
    OneOrMore(Box<Expr>)
}

impl FromStr for Expr {
    type Err = String;

    fn from_str(s: &str) -> Result<Expr,String> {

        let mut output_queue = VecDeque::<Expr>::new();
        let mut operator_stack = Vec::<char>::new();

        for c in s.chars() {
            if binary_operators.contains(&c) {
                pop_infix_operators(&mut operator_stack, &mut output_queue);
                operator_stack.push(c);
            } else if unary_postfix_operators.contains(&c) {
                apply_postfix_operator(c, &mut output_queue);
            } else {
                output_queue.push_back(Expr::Single(c))
            }
        }

        pop_infix_operators(&mut operator_stack, &mut output_queue);

        // build sequence tree from queue
        while output_queue.len() > 1 {
            let right = output_queue.pop_back().unwrap();
            let left = output_queue.pop_back().unwrap();

            output_queue.push_back(Expr::Sequence(Box::new(left),
                                                  Box::new(right)));
        }

        output_queue.pop_front().ok_or("output queue empty".to_owned())
    }
}

static unary_postfix_operators: &'static [char] = &['?', '*', '+'];
static binary_operators: &'static [char] = &['|'];

fn pop_infix_operators(operator_stack: &mut Vec<char>, output_queue: &mut VecDeque<Expr>) {
    while !operator_stack.is_empty() {

        println!("{:?}, {:?}", output_queue, operator_stack);
        // TODO handle prescedence
        match operator_stack.pop().unwrap() {
            '|' => { 
                apply_binary_operator(output_queue, &|l,r| Expr::Or(l,r));
            },
            _ => panic!("unknown infix operator")
        }
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
        }
        _ => panic!("unknown postfix operator")
    }
}

fn apply_binary_operator(output_queue: &mut VecDeque<Expr>, 
                         constructor: &Fn(Box<Expr>, Box<Expr>) -> Expr) {

    let right = output_queue.pop_back().unwrap();
    let left = output_queue.pop_back().unwrap();

    output_queue.push_back(constructor(Box::new(left),Box::new(right)));
}

fn apply_unary_operator(output_queue: &mut VecDeque<Expr>,
                        constructor: &Fn(Box<Expr>) -> Expr) {

    let item = output_queue.pop_back().unwrap();
    output_queue.push_back(constructor(Box::new(item)));
}

