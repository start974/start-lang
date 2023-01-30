use regex::Regex;
use std::fmt;

#[derive(Debug)]
pub struct Rule {
    name: String,
    description: String,
    regex: Regex,
    skip: bool,
}

impl Rule {
    pub fn new(name: &str, description: &str, skip: bool) -> Self {
        Rule {
            name: name.to_string(),
            description: description.to_string(),
            regex: Regex::new(&format!("^({})", description)).unwrap(),
            skip,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_skip(&self) -> bool {
        self.skip
    }

    pub fn match_rule(&self, text: &str) -> Option<usize> {
        self.regex.find(text).map(|m| m.end())
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.name, self.description)
    }
}
