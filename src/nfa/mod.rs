use std::str::FromStr;
use std::collections::vec_deque::VecDeque;

#[derive(PartialEq, Debug)]
enum Expr {
    Single(char),
    Sequence(Vec<Box<Expr>>),
    Or(Box<Expr>, Box<Expr>),
    Optional(Box<Expr>),
    KleeneStar(Box<Expr>),
    KleenePlus(Box<Expr>)
}

impl FromStr for Expr {
    type Err = String;

    fn from_str(s: &str) -> Result<Expr,String> {

        let mut output_queue = VecDeque::<Expr>::new();
        let mut operators = Vec::<char>::new();

        for c in s.chars() {
            match c {
                '|' => {

                    
                    operators.push('|');
                },
                _ => {

                    if !operators.is_empty() && output_queue.len() >= 2 {
                        process_operators(&mut operators, &mut output_queue);
                    }
                    else {
                        output_queue.push_back(Expr::Single(c))
                    }
                }
            }
        }

        while !operators.is_empty() {
            process_operators(&mut operators, &mut output_queue);
        }

        if output_queue.len() == 1 {
            return Ok(output_queue.pop_front().unwrap());
        }

        let mut v = Vec::new();
        for expr in output_queue {
            v.push(Box::new(expr));
        }


        Ok(Expr::Sequence(v))
    }
}

fn process_operators(operators: &mut Vec<char>, output_queue: &mut VecDeque<Expr>) {
    // we can apply an operator
    let op = operators.pop().unwrap();
    let ast = match op {
        '|' => {
            let right = output_queue.pop_back().unwrap();
            let left = output_queue.pop_back().unwrap();
            Expr::Or(Box::new(left),
            Box::new(right))
        },
        _ => panic!("unknown operator")
    };

    output_queue.push_back(ast);
}


#[cfg(test)]
mod spec {
    use super::Expr;

    #[test]
    fn parse_single() {
        assert_eq!(Expr::Single('a'),
                   "a".parse::<Expr>().unwrap());
    }

    #[test]
    fn parse_sequence() {
        assert_eq!(Expr::Sequence(vec![Box::new(Expr::Single('a')),
                                       Box::new(Expr::Single('b')),
                                       Box::new(Expr::Single('c'))]),
                   "abc".parse::<Expr>().unwrap());
    }

    #[test]
    fn parse_or() {
        assert_eq!(Expr::Or(Box::new(Expr::Single('a')),
                            Box::new(Expr::Single('b'))),
                   "a|b".parse::<Expr>().unwrap());
    }

}

