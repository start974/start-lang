use core::str;
use std::fs::File;
use std::io::{self, BufReader};
use std::rc::Rc;
use std::vec;
use stringreader::StringReader;

use super::{Error as ErrorLexer, FilePosition, Position, Rule, Token};

type Rules = Vec<Rc<Rule>>;

const BUFFER_SIZE: usize = 1024;
fn init_rules() -> Rules {
    vec![Rule::new("WHITESPACE", "[ \t\r\n]+", true).into()]
}

pub struct Lexer<'r> {
    path: String,
    reader: Box<dyn io::BufRead + 'r>,
    pos: Position,
    token_buffer: Vec<Token>,
    string_buffer: String,
    rules: Rules,
    eof: bool,
}

impl<'r> Lexer<'r> {
    fn new(path: &str, reader: Box<dyn io::BufRead + 'r>) -> Self {
        Lexer {
            path: path.to_string(),
            reader,
            token_buffer: Vec::new(),
            string_buffer: String::new(),
            pos: Position::default(),
            rules: init_rules(),
            eof: false,
        }
    }

    /// make a lexer on stdin
    pub fn from_stdin() -> Self {
        let file = io::stdin();
        let bufreader = BufReader::new(file);
        Self::new("<<stdin>>", Box::new(bufreader))
    }

    /// make a lexer from file
    pub fn from_file(path: &str) -> io::Result<Self> {
        let file = File::open(path)?;
        let bufreader = BufReader::new(file);
        Ok(Self::new(path, Box::new(bufreader)))
    }

    /// make a lexer from string to test
    pub fn from_string(program: &'r str) -> Self {
        let file = StringReader::new(program);
        let bufreader = BufReader::new(file);
        Self::new("<<string>>", Box::new(bufreader))
    }

    /// add with new rule and return true if not overrride
    /// if index is greater than length push at end
    pub fn add_rule(&mut self, priority: usize, name: &str, regex: &str, skip: bool) -> bool {
        let present = self.remove_rule(name);
        let rule_rc = Rule::new(name, regex, skip).into();
        if priority < self.rules.len() {
            self.rules.insert(priority, rule_rc)
        } else {
            self.rules.push(rule_rc);
        };
        !present
    }

    /// push back a new rule and return true if not overrride
    pub fn push_rule(&mut self, name: &str, regex: &str, skip: bool) -> bool {
        self.add_rule(usize::MAX, name, regex, skip)
    }

    /// wrapper to remove_skip_rule and remove_token_rule
    pub fn remove_rule(&mut self, name: &str) -> bool {
        match self.rules.iter().position(|r| r.name() == name) {
            Some(i) => {
                self.rules.remove(i);
                true
            }
            None => false,
        }
    }

    /// return rule with index in lexing
    pub fn get_rule(&self, name: &str) -> Option<(usize, Rc<Rule>)> {
        self.rules
            .iter()
            .enumerate()
            .find(|(_, r)| r.name() == name)
            .map(|(i, r)| (i, r.clone()))
    }

    /// append end buffer
    fn append_end_buffer(&mut self) -> Result<(), ErrorLexer> {
        if !self.eof {
            let mut buffer = [0u8; BUFFER_SIZE];
            let size = self.reader.read(&mut buffer).map_err(ErrorLexer::IoError)?;
            self.eof = size < BUFFER_SIZE;
            let buffer_str = str::from_utf8(&buffer[..size]).map_err(ErrorLexer::UTf8error)?;
            self.string_buffer.push_str(buffer_str);
        };
        Ok(())
    }

    /// apply rules on text
    /// return rule name and size read
    fn apply_rules(&mut self) -> Option<(Rc<Rule>, usize)> {
        self.rules
            .iter()
            .filter_map(|rule| {
                rule.match_rule(&self.string_buffer)
                    .map(|size| (rule.clone(), size))
            })
            .max_by(|(_, len1), (_, len2)| len1.cmp(len2))
    }

    fn make_token(&mut self, rule: &Rc<Rule>, len: usize) -> Token {
        let start = self.pos.clone();
        let content = &self.string_buffer.clone()[..len];
        for c in content.chars() {
            if c == '\n' {
                self.pos.next_line();
            } else {
                self.pos.incr();
            }
        }
        self.string_buffer.replace_range(..len, "");
        let pos_file = FilePosition::new(&self.path, start, self.pos.pred());
        Token::new(rule, content, pos_file)
    }

    /// tokenize current buffer
    fn tokenize(&mut self) -> Result<Token, ErrorLexer> {
        // read buffer
        self.append_end_buffer()?;
        while !self.string_buffer.is_empty() {
            // apply token rules
            match self.apply_rules() {
                Some((rule, len)) if self.string_buffer.len() != len || self.eof => {
                    let token = self.make_token(&rule, len);
                    if !rule.is_skip() {
                        self.token_buffer.push(token);
                    }
                }
                None if self.eof => {
                    return ErrorLexer::unkown(&self.path, self.pos, &self.string_buffer)
                }
                _ => break,
            }
        }
        Ok(self.token_buffer.remove(0))
    }
}

impl<'r> Iterator for Lexer<'r> {
    type Item = Result<Token, ErrorLexer>;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.token_buffer.is_empty() {
            Some(Ok(self.token_buffer.remove(0)))
        } else if !self.eof {
            Some(self.tokenize())
        } else {
            None
        }
    }
}
#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_iter() {
        let data = "test1   test12\n \t test  ";
        let mut lexer = Lexer::from_string(data);
        lexer.push_rule("TEST", "test", false);
        lexer.push_rule("DIGITS", "[0-9]+", false);
        let (_, test_rule) = lexer.get_rule("TEST").unwrap();
        let (_, digit_rule) = lexer.get_rule("DIGITS").unwrap();
        {
            let position =
                FilePosition::new("<<string>>", Position::new(1, 1), Position::new(1, 4));
            let token = Token::new(&test_rule, "test", position);
            assert_eq!(lexer.next().unwrap().unwrap(), token);
        }
        {
            let position =
                FilePosition::new("<<string>>", Position::new(1, 5), Position::new(1, 5));
            let token = Token::new(&digit_rule, "1", position);
            assert_eq!(lexer.next().unwrap().unwrap(), token);
        }
        {
            let position =
                FilePosition::new("<<string>>", Position::new(1, 9), Position::new(1, 12));
            let token = Token::new(&test_rule, "test", position);
            assert_eq!(lexer.next().unwrap().unwrap(), token);
        }
        {
            let position =
                FilePosition::new("<<string>>", Position::new(1, 13), Position::new(1, 14));
            let token = Token::new(&digit_rule, "12", position);
            assert_eq!(lexer.next().unwrap().unwrap(), token);
        }
        {
            let position =
                FilePosition::new("<<string>>", Position::new(2, 4), Position::new(2, 7));
            let token = Token::new(&test_rule, "test", position);
            assert_eq!(lexer.next().unwrap().unwrap(), token);
        }
        assert!(lexer.next().is_none())
    }
}
