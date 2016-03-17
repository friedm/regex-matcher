pub mod regex;
mod token;

pub use regex::Regex;

#[cfg(test)]
mod lib_spec {
    use ::{Regex};

    #[test]
    fn creates_new() {
        assert_eq!(Regex{
            pattern: String::from("pattern")
        }, Regex::from("pattern"));
    }

    #[test]
    fn matches() {
        let regex = Regex::from("a");
        assert_eq!(true, regex.is_match("baaa"));
    }

    #[test]
    fn finds_first() {
        assert_eq!(Some((1, 2)), Regex::from(r"a").first("baaa"));
    }

    #[test]
    fn first_is_none() {
        assert_eq!(None, Regex::from(r"z").first("baaa"));
    }

    #[test]
    fn matches_optionally() {
        assert!(Regex::from("a?").is_match("b"));
    }
}
