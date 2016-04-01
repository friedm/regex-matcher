use std::iter;

extern crate test;
use self::test::Bencher;

use super::Regex;

#[bench]
fn bench_2(b: &mut Bencher) {
    bench_difficult_regex(b, 2);
}

#[bench]
fn bench_12(b: &mut Bencher) {
    bench_difficult_regex(b, 12);
}

fn bench_difficult_regex(b: &mut Bencher, size: usize) {
    // 'a?a?aa' maching text "aa"

    let mut text: String = iter::repeat("a").take(size).collect();
    let mut regex: String = iter::repeat("a?").take(size).collect();
    regex.push_str(&text.clone());

    b.iter(|| {
        let expr = Regex::from(&regex).unwrap();
        assert!(expr.is_match(&text));
    });
}

