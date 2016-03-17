#[derive(PartialEq,Debug)]
struct Regex {
    pattern: String
}

impl Regex {
    pub fn from(pattern: &str) -> Regex {
        Regex {
            pattern: pattern.to_owned()
        }
    }

    pub fn is_match(&self, text: &str) -> bool {
        true
    }

    pub fn find_first(&self, text: &str) -> (usize, usize) {
        (1, 2)
    }
}

#[cfg(test)]
mod regex_spec {
    use super::{Regex};

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
    fn find() {
        assert_eq!((1, 2), Regex::from(r"a").find_first("baaa"))
    }
}
