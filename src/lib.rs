#[derive(PartialEq,Debug)]
struct Regex;

impl Regex {
    pub fn new() -> Regex {
        Regex
    }

    pub fn is_match(&self) -> bool {
        true
    }
}



#[cfg(test)]
mod regex_spec {
    use super::{Regex};

    #[test]
    fn creates_new() {
        assert_eq!(Regex, Regex::new());
    }

    #[test]
    fn matches() {
        let regex = Regex::new();
        assert_eq!(true, regex.is_match());
    }
}
