use crate::frontend::error::{ErrorKind, ParsingErr};

use super::position::Position;
use super::utils::TokenResult;
use super::FilePosition;
use super::TokenCont;

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    content: TokenCont,
    pos: Option<FilePosition>,
}

impl Token {
    pub fn init(content: &TokenCont, pos: Option<FilePosition>) -> Self {
        if let TokenCont::Word(s) = content {
            assert!(!s.is_empty())
        };
        Token {
            content: content.clone(),
            pos,
        }
    }

    pub fn new(content: &TokenCont, position: FilePosition) -> Self {
        if let TokenCont::Word(s) = content {
            let word_len = s.len();
            let start = position.start().clone();
            let end = position.end();
            if start.line() == end.line() {
                let pos_len = end.offset() - start.offset() + 1;
                assert_eq!(
                    word_len, pos_len,
                    "test len equal position length - 
                    word: \"{s}\", {position:?} -- (w_len: {word_len:?}) != (p_len: {pos_len:?})\n",
                );
            } else {
                assert!(false, "word cannot be on 2 lines");
            };
        }
        Self::init(content, Some(position))
    }

    pub fn unkown(content: &TokenCont) -> Self {
        Self::init(content, None)
    }

    pub fn position(&self) -> &Option<FilePosition> {
        &self.pos
    }

    pub fn word(&self) -> Option<&String> {
        match &self.content {
            TokenCont::Word(s) => Some(s),
            _ => None,
        }
    }

    pub fn error<T>(&self, kind: ErrorKind) -> TokenResult<T> {
        ParsingErr::result(kind, self.clone())
    }

    /// check if token is white spaces
    pub fn whitespaces(&self) -> TokenResult<()> {
        match self.content {
            TokenCont::WSpace => Ok(()),
            _ => self.error(ErrorKind::EOL),
        }
    }

    //check end of line
    pub fn eol(&self) -> TokenResult<()> {
        match self.content {
            TokenCont::EOL => Ok(()),
            _ => self.error(ErrorKind::EOL),
        }
    }

    //check end of file
    pub fn eof(&self) -> TokenResult<()> {
        match self.content {
            TokenCont::EOF => Ok(()),
            _ => self.error(ErrorKind::EOF),
        }
    }

    ///check if token is a tag
    pub fn tag<'a>(&self, tag: &'a str) -> TokenResult<()> {
        match self.word() {
            Some(s) if s == tag => Ok(()),
            _ => self.error(ErrorKind::Tag(tag.to_string())),
        }
    }

    /// match char return it and return new token
    pub fn predicate<P>(&self, pred: P, context: &str) -> TokenResult<(char, Option<Self>)>
    where
        P: Fn(char) -> bool,
    {
        match self.word() {
            Some(s) => {
                let c = s.chars().next().unwrap();
                if pred(c) {
                    let content = s[1..].to_string();
                    let token = if content.is_empty() {
                        None
                    } else {
                        let pos = self.pos.clone().map(|pos| pos.succ());
                        let token = Token {
                            content: TokenCont::Word(content),
                            pos,
                        };
                        Some(token)
                    };
                    Ok((c, token))
                } else {
                    self.error(ErrorKind::Context(context.to_string()))
                }
            }
            _ => self.error(ErrorKind::Context(context.to_string())),
        }
    }

    pub fn char(&self, c: char) -> TokenResult<(char, Option<Self>)> {
        let context = c.to_string();
        self.predicate(|c0| c0 == c, &context)
    }

    pub fn alpha_lower(&self) -> TokenResult<(char, Option<Self>)> {
        self.predicate(|c| c.is_ascii_lowercase(), "ALPHA_LOWER")
    }

    pub fn alpha_upper(&self) -> TokenResult<(char, Option<Self>)> {
        self.predicate(|c| c.is_ascii_uppercase(), "ALPHA_UPPER")
    }

    pub fn alpha(&self) -> TokenResult<(char, Option<Self>)> {
        self.predicate(|c| c.is_ascii_alphabetic(), "ALPHA")
    }

    pub fn alpha_num(&self) -> TokenResult<(char, Option<Self>)> {
        self.predicate(|c| c.is_ascii_alphanumeric(), "ALPHA_NUM")
    }

    pub fn digit(&self) -> TokenResult<(char, Option<Self>)> {
        self.predicate(|c| c.is_ascii_digit(), "DIGIT")
    }

    pub fn oct_digit(&self) -> TokenResult<(char, Option<Self>)> {
        self.predicate(|c| ('0'..'7').contains(&c), "OCT_DIGIT")
    }

    pub fn hex_digit(&self) -> TokenResult<(char, Option<Self>)> {
        self.predicate(|c| c.is_ascii_hexdigit(), "HEX_DIGIT")
    }
}

#[cfg(test)]
mod test {

    use super::{FilePosition, Position, Token, TokenCont};

    #[test]
    fn test_whitespaces() {
        let token = Token::unkown(&TokenCont::WSpace);
        assert!(token.whitespaces().is_ok());

        let token = Token::unkown(&TokenCont::EOF);
        assert!(token.whitespaces().is_err())
    }

    #[test]
    fn test_eol() {
        let token = Token::unkown(&TokenCont::EOL);
        assert!(token.eol().is_ok());

        let token = Token::unkown(&TokenCont::EOF);
        assert!(token.eol().is_err())
    }

    #[test]
    fn test_eof() {
        let token = Token::unkown(&TokenCont::EOF);
        assert!(token.eof().is_ok());

        let token = Token::unkown(&TokenCont::EOL);
        assert!(token.eof().is_err())
    }

    #[test]
    fn test_tag() {
        let token = Token::unkown(&TokenCont::word("test"));
        assert!(token.tag("test").is_ok());
        assert!(token.tag("err").is_err())
    }

    #[test]
    fn test_char_unknown() {
        let token_src = Token::unkown(&TokenCont::word("test"));
        let token_dst = Token::unkown(&TokenCont::word("est"));
        assert_eq!(token_src.char('t').unwrap(), ('t', Some(token_dst)));
        assert!(token_src.char('e').is_err())
    }

    #[test]
    fn test_char() {
        let pos_src = FilePosition::new("test", Position::new(1, 1), Position::new(1, 4));
        let mut pos_dst = pos_src.clone();
        pos_dst.incr();

        let token_src = Token::new(&TokenCont::word("test"), pos_src);
        let token_dst = Token::new(&TokenCont::word("est"), pos_dst);
        assert_eq!(token_src.char('t').unwrap(), ('t', Some(token_dst)));
        assert!(token_src.char('e').is_err())
    }

    #[test]
    fn test_alpha_lower_unknown() {
        let token_src = Token::unkown(&TokenCont::word("test"));
        let token_dst = Token::unkown(&TokenCont::word("est"));
        assert_eq!(token_src.alpha_lower().unwrap(), ('t', Some(token_dst)));
    }
    #[test]
    fn test_alpha_upper_unknown() {
        let token_src = Token::unkown(&TokenCont::word("Test"));
        let token_dst = Token::unkown(&TokenCont::word("est"));
        assert_eq!(token_src.alpha_upper().unwrap(), ('T', Some(token_dst)));
    }

    #[test]
    fn test_alpha_unknown() {
        let token_src = Token::unkown(&TokenCont::word("Test"));
        let token_dst = Token::unkown(&TokenCont::word("est"));
        assert_eq!(token_src.alpha().unwrap(), ('T', Some(token_dst)));

        let token_src = Token::unkown(&TokenCont::word("t"));
        assert_eq!(token_src.alpha().unwrap(), ('t', None));
    }

    #[test]
    fn test_alpha_num_unknown() {
        let token_src = Token::unkown(&TokenCont::word("T"));
        assert_eq!(token_src.alpha_num().unwrap(), ('T', None));

        let token_src = Token::unkown(&TokenCont::word("a"));
        assert_eq!(token_src.alpha_num().unwrap(), ('a', None));
    }

    #[test]
    fn test_digit_unknown() {
        let token_src = Token::unkown(&TokenCont::word("9"));
        assert_eq!(token_src.digit().unwrap(), ('9', None));
    }

    #[test]
    fn test_oct_digit_unknown() {
        let token_src = Token::unkown(&TokenCont::word("9"));
        assert!(token_src.oct_digit().is_err());

        let token_src = Token::unkown(&TokenCont::word("0"));
        assert_eq!(token_src.oct_digit().unwrap(), ('0', None));
    }

    #[test]
    fn test_hex_digit_unknown() {
        let token_src = Token::unkown(&TokenCont::word("a"));
        assert_eq!(token_src.hex_digit().unwrap(), ('a', None));
    }
}
