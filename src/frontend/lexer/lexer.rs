use core::str;
use std::fs::File;
use std::io::{self, BufReader};
use std::vec;
use stringreader::StringReader;

use super::{Error as ErrorLexer, FilePosition, Position, Rule, Token};

type Rules = Vec<Rule>;

const BUFFER_SIZE: usize = 1024;
fn init_rules() -> Rules {
    vec![Rule::new("WHITESPACE", "[ \t\r\n]+", true)]
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

    /// wrapper to add_skip_rule and add_token_rule
    pub fn add_rule(&mut self, priority: Option<usize>, name: &str, regex: &str, skip: bool) -> bool {
        let present = self.remove_rule(name);
        let rule = Rule::new(name, regex, skip);
        match priority {
            Some(i) if i < self.rules.len() => self.rules.insert(i, rule),
            _ => self.rules.push(rule),
        };
        !present
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
    fn apply_rules(&mut self) -> Option<(String, bool, usize)> {
        self.rules.iter()
            .filter_map(|rule| {
                rule.match_rule(&self.string_buffer)
                    .map(|size| (rule.name(), rule.is_skip(), size))
            })
            .max_by(|(_, _, len1), (_, _, len2)| len1.cmp(len2))
            .map(|(name, skip, len)| (name.to_string(), skip, len))
    }

    fn make_token(&mut self, name: &str, len: usize) -> Token {
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
        Token::new(name, content, pos_file)
    }

    /// tokenize current buffer
    fn tokenize(&mut self) -> Result<Token, ErrorLexer> {
        // read buffer 
        self.append_end_buffer()?;
        while !self.string_buffer.is_empty() {
            // apply token rules
            match self.apply_rules() {
                Some((name, skip, len)) if self.string_buffer.len() != len || self.eof => {
                    let token = self.make_token(&name, len);
                    if !skip {
                        self.token_buffer.push(token);
                    }
                }
                None if self.eof => return ErrorLexer::unkown(&self.path, self.pos, &self.string_buffer),
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

    use super::{FilePosition, Position, Token};

    use super::Lexer;

    #[test]
    fn test_iter() {
        let data = "test1   test12\n \t test  ";
        let mut lexer = Lexer::from_string(data);
        lexer.add_rule(None, "TEST", "test", false);
        lexer.add_rule(None, "DIGITS", "[0-9]+", false);
        {
            let position =
                FilePosition::new("<<string>>", Position::new(1, 1), Position::new(1, 4));
            let token = Token::new("TEST", "test", position);
            assert_eq!(lexer.next().unwrap().unwrap(), token);
        }
        {
            let position =
                FilePosition::new("<<string>>", Position::new(1, 5), Position::new(1, 5));
            let token = Token::new("DIGITS", "1", position);
            assert_eq!(lexer.next().unwrap().unwrap(), token);
        }        
        {
            let position =
                FilePosition::new("<<string>>", Position::new(1, 9), Position::new(1, 12));
            let token = Token::new("TEST", "test", position);
            assert_eq!(lexer.next().unwrap().unwrap(), token);
        }
        {
            let position =
                FilePosition::new("<<string>>", Position::new(1, 13), Position::new(1, 14));
            let token = Token::new("DIGITS", "12", position);
            assert_eq!(lexer.next().unwrap().unwrap(), token);
        }
        {
            let position =
                FilePosition::new("<<string>>", Position::new(2, 4), Position::new(2, 7));
            let token = Token::new("TEST", "test", position);
            assert_eq!(lexer.next().unwrap().unwrap(), token);
        }
        assert!(lexer.next().is_none())
    }
}
