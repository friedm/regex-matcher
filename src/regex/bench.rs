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

#[bench]
fn bench_30(b: &mut Bencher) {
    bench_difficult_regex(b, 30);
}

#[bench]
fn bench_100(b: &mut Bencher) {
    bench_difficult_regex(b, 100);
}

fn bench_difficult_regex(b: &mut Bencher, size: usize) {
    // 'a?a?aa' maching text "aa"

    let text: String = iter::repeat("a").take(size).collect();
    let mut regex: String = iter::repeat("a?").take(size).collect();
    regex.push_str(&text.clone());

    let regex = Regex::from(&regex).unwrap();

    b.iter(|| {
        assert!(regex.is_match(&text.clone()));
        assert!(!regex.is_match(&text[1..]));
    });
}


