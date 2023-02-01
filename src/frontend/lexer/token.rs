use core::fmt;
use std::rc::Rc;

use super::{FilePosition, Rule};


#[derive(Debug, Clone)]
pub struct Token {
    rule: Rc<Rule>,
    content: String,
    pos: Option<FilePosition>,
}

impl Token {
    fn init(rule: &Rc<Rule>, content: &str, pos: Option<FilePosition>) -> Self {
        assert!(!content.is_empty());
        Token {
            content: content.to_string(),
            rule: rule.clone(),
            pos,
        }
    }

    pub fn new(rule: &Rc<Rule>, content: &str, position: FilePosition) -> Self {
        let word_len = content.len();
        let start = position.start().clone();
        let end = position.end();
        if start.line() == end.line() {
            let pos_len = end.offset() - start.offset() + 1;
            assert_eq!(
                word_len, pos_len,
                "test len equal position length - 
                token: \"{content}\", {position:?} -- (w_len: {word_len:?}) != (p_len: {pos_len:?})\n",
            );
        } 
        Self::init(rule, content, Some(position))
    }

    pub fn unkown(rule: &Rc<Rule>, content: &str) -> Self {
        Self::init(rule, content, None)
    }

    pub fn position(&self) -> &Option<FilePosition> {
        &self.pos
    }

    // pub fn error<T>(&self, kind: ErrorKind) -> TokenResult<T> {
    //     ParsingErr::result(kind, self.clone())
    // }

}

impl fmt::Display for Token {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}({})", self.rule.name(), self.content)
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Token) -> bool {
        self.pos == other.pos &&
        self.content == other.content &&
        self.rule.name() == self.rule.name()
    }
}