pub mod regex;
mod token;

pub use regex::Regex;

#[cfg(test)]
mod regex_spec {
    use ::{Regex};

    #[test]
    fn creates_new() {
        assert_eq!(Regex{
            pattern: String::from(r"pattern")
        }, Regex::from(r"pattern"));
    }

    #[test]
    fn matches() {
        let regex = Regex::from(r"a");
        assert_eq!(true, regex.is_match(r"baaa"));
    }

    #[test]
    fn finds_first() {
        assert_eq!(Some((1, 2)), Regex::from(r"a").first("baaa"))
    }

    #[test]
    fn first_is_none() {
        assert_eq!(None, Regex::from(r"z").first("baaa"))
    }
}
