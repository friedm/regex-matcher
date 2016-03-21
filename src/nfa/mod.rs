use std::str::FromStr;
use std::collections::vec_deque::VecDeque;

#[cfg(test)] mod spec;

#[derive(PartialEq, Debug)]
enum Expr {
    Single(char),
    Sequence(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Optional(Box<Expr>),
    KleeneStar(Box<Expr>),
    KleenePlus(Box<Expr>)
}

impl FromStr for Expr {
    type Err = String;

    fn from_str(s: &str) -> Result<Expr,String> {

        let mut output_queue = VecDeque::<Expr>::new();
        let mut operator_stack = Vec::<char>::new();

        for c in s.chars() {
            if operators.contains(&c) {
                pop_operator(&mut operator_stack, &mut output_queue);
                operator_stack.push(c);
            } else {
                output_queue.push_back(Expr::Single(c))
            }
        }

        while !operator_stack.is_empty() {
            pop_operator(&mut operator_stack, &mut output_queue);
        }

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

static operators: &'static [char] = &['|', '?'];

fn pop_operator(operator_stack: &mut Vec<char>, output_queue: &mut VecDeque<Expr>) {
    while !operator_stack.is_empty() {
        // TODO handle prescedence
        match operator_stack.pop().unwrap() {
            '|' => { 
                let right = output_queue.pop_back().unwrap();
                let left = output_queue.pop_back().unwrap();
                output_queue.push_back(
                    Expr::Or(Box::new(left), 
                             Box::new(right)));
            },
            '?' => {
                let item = output_queue.pop_back().unwrap();
                output_queue.push_back(
                    Expr::Optional(Box::new(item)));
            },
            _ => panic!("unknown operator")
        }
    }
}

