pub mod regex;
mod token;

pub use regex::Regex;

#[cfg(test)]
mod lib_spec {
    use ::{Regex};

    #[test]
    fn creates_new() {
        assert_eq!(Regex{
            expressions: vec![]
        }, Regex::from("").unwrap());
    }

    #[test]
    fn returns_err_on_invalid_regex() {
        assert!(Regex::from("[").is_err());
    }

    #[test]
    fn matches() {
        let regex = Regex::from("a").unwrap();
        assert_eq!(true, regex.is_match("baaa"));
    }

    #[test]
    fn finds_first() {
        assert_eq!(Some((1, 2)), Regex::from("a").unwrap().first("baaa"));
    }

    #[test]
    fn first_is_none() {
        assert_eq!(None, Regex::from("z").unwrap().first("baaa"));
    }

    #[test]
    fn matches_optionally() {
        assert!(Regex::from("a?").unwrap().is_match("b"));
    }
}
