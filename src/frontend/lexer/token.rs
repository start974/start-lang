use core::fmt;

use super::FilePosition;


#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    rule_name: String,
    content: String,
    pos: Option<FilePosition>,
}

impl Token {
    pub fn init(rule_name: &str, content: &str, pos: Option<FilePosition>) -> Self {
        assert!(!content.is_empty());
        Token {
            rule_name: rule_name.to_string(),
            content: content.to_string(),
            pos,
        }
    }

    pub fn new(rule_name: &str, content: &str, position: FilePosition) -> Self {
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
        Self::init(rule_name, content, Some(position))
    }

    pub fn unkown(rule_name: &str, content: &str) -> Self {
        Self::init(rule_name, content, None)
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
        write!(f, "{}({})", self.rule_name, self.content)
    }
}