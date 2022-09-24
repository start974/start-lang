use std::fs::File;
use std::io::{self, BufRead, BufReader};
use stringreader::StringReader;

use super::{Token, FilePosition};
use super::TokenCont;
use super::position::Position;

pub struct Lexer<'r> {
    reader: Box<dyn io::BufRead + 'r>,
    path: String,
    pos : Position,
    current_char: Option<char>,
    content: String,
    white_space: String,
    eof: bool,
}

impl<'r> Lexer<'r> {
    fn new(path: &str, reader: Box<dyn io::BufRead + 'r>) -> Self {
        Lexer {
            path: path.to_string(),
            current_char: None,
            content: String::new(),
            reader,
            pos: Position::new(1, 1),
            white_space: "\t ".to_string(),
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

    /// add whitespace char
    /// return false if already present
    pub fn add_whitespace(&mut self, c: char) -> bool {
        let not_contain = !self.white_space.contains(c);
        if not_contain {
            self.white_space.push(c);
        };
        not_contain
    }

    /// remove whitespace char
    /// return false if not already present
    pub fn remove_whitespace(&mut self, c: char) -> bool {
        let contain = self.white_space.contains(c);
        if contain {
            self.white_space = self.white_space.replace(c, "");
        }
        contain
    }

    /// return true if current char is whitespace
    pub fn is_whitespace(&self) -> bool {
        match self.current_char {
            Some(c) if self.white_space.contains(c) => true,
            _ => false,
        }
    }

    /// return true if current char is end of line
    pub fn is_eol(&self) -> bool {
        match self.current_char {
            Some(c) if c == '\n' => true,
            _ => false,
        }
    }
    /// return true if current char is end of line
    pub fn is_char(&self) -> bool {
        !(self.eof || self.is_eol() || self.is_whitespace())
    }

    /// consume next_token char
    fn next_char(&mut self) {
        if self.eof {
            return;
        } else if self.content.is_empty() {
            self.next_line();
        } else {
            self.pos.incr();
            self.current_char = Some(self.content.remove(0));
        }
    }

    /// consume next_token line
    fn next_line(&mut self) {
        assert!(self.content.is_empty());
        self.content.clear();
        if self.reader.read_line(&mut self.content).unwrap() != 0 {
            // if is not first line
            if !self.current_char.is_none() {
                self.pos.next_line();
            }
            self.current_char = Some(self.content.remove(0));
        } else {
            self.eof = true;
            self.pos.incr();
            self.current_char = None;
        }
    }

    /// consume whitespace and return number of whitespace
    fn skip_whitespaces(&mut self) {
        while self.is_whitespace() {
            self.next_char();
        }
    }

    fn word(&mut self) -> String {
        let mut res = String::new();
        while self.is_char() {
            res.push(self.current_char.unwrap());
            self.next_char();
        }
        res
    }

    /// compute next_token token
    pub fn next_token(&mut self) -> Token {
        // if is at start of line
        if self.current_char.is_none() && !self.eof {
            self.next_char()
        };

        let start = self.pos;
        let (end, tok_content) = if self.eof {
            (start, TokenCont::EOF)
        } else if self.is_whitespace() {
            self.skip_whitespaces();
            (self.pos.pred(), TokenCont::WSpace)
        } else if self.is_eol() {
            self.next_line();
            (start, TokenCont::EOL)
        } else {
            let word = self.word();
            assert!(!word.is_empty());
            (self.pos.pred(), TokenCont::Word(word))
        };
        let f_pos = FilePosition::new(&self.path, start, end);
        Token::new(&tok_content, f_pos)
    }
}

#[cfg(test)]
mod test {


    use super::{FilePosition, Position, Token, TokenCont};

    use super::Lexer;

    #[test]
    fn test_iter() {
        let data = "test1   test2\n \t test3";
        let mut lexer = Lexer::from_string(data);
        {
            let content = &TokenCont::Word("test1".to_string());
            let position = FilePosition::new("<<string>>", 
                Position::new(1, 1), Position::new(1, 5));
            let token = Token::new(content, position);
            assert_eq!(lexer.next_token(), token);
        }
        {
            let content = &&TokenCont::WSpace;
            let position = FilePosition::new("<<string>>", 
                Position::new(1, 6), Position::new(1, 8));
            let token = Token::new(content, position);
            assert_eq!(lexer.next_token(), token);
        }
        {
            let content = &TokenCont::Word("test2".to_string());
            let position = FilePosition::new("<<string>>", 
                Position::new(1, 9), Position::new(1, 13));
            let token = Token::new(content, position);
            assert_eq!(lexer.next_token(), token);
        }
        {
            let content = &TokenCont::EOL;
            let position = FilePosition::new("<<string>>", 
                Position::new(1, 14), Position::new(1, 14));
            let token = Token::new(content, position);
            assert_eq!(lexer.next_token(), token);
        }
        {
            let content = &TokenCont::WSpace;
            let position = FilePosition::new("<<string>>", 
                Position::new(2, 1), Position::new(2, 3));
            let token = Token::new(content, position);
            assert_eq!(lexer.next_token(), token);
        }
        {
            let content = &TokenCont::Word("test3".to_string());
            let position = FilePosition::new("<<string>>", 
                Position::new(2, 4), Position::new(2, 8));
            let token = Token::new(content, position);
            assert_eq!(lexer.next_token(), token);
        }
        {
            let content = &TokenCont::EOF;
            let position = FilePosition::new("<<string>>", 
                Position::new(2, 9), Position::new(2, 9));
            let token = Token::new(content, position);
            assert_eq!(lexer.next_token(), token);
        }
        // assert_eq!(lexer.next_token(), None);
    }
}
