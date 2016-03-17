use std::str::FromStr;

#[derive(PartialEq,Debug)]
pub struct Regex {
    pub pattern: String
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

    pub fn first(&self, text: &str) -> Option<(usize, usize)> {
        if text.contains(&self.pattern) {
            Some((1, 2))
        } else {
            None
        }
    }
}


