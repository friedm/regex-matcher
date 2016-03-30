#![feature(test)]

extern crate test;
use self::test::Bencher;

extern crate regexmatcher;
use self::regexmatcher::Regex;

#[bench]
#[ignore]
fn bench_regex(b: &mut Bencher) {
    b.iter(|| {
        let expr = Regex::from("a?a").unwrap();
        assert!(expr.is_match("a"));
    });
}

