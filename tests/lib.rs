#![feature(test)]

extern crate test;
use self::test::Bencher;

extern crate regexmatcher;
use self::regexmatcher::Expr;

#[bench]
fn bench_regex(b: &mut Bencher) {
    b.iter(|| {
        let expr = "a?a".parse::<Expr>().unwrap();
        assert!(expr.is_match("a"));
    });
}

