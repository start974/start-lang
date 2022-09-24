use super::token::Token;
use super::utils::TokenResult;

/// -------------------------------------------------------------------------------
///                                 Token matcher
/// -------------------------------------------------------------------------------
pub fn white_space(token: Token) -> TokenResult<()> {
    token.whitespaces()
}

pub fn eol(token: Token) -> TokenResult<()> {
    token.eol()
}

pub fn eof(token: Token) -> TokenResult<()> {
    token.eof()
}

pub fn tag(tag: &str) -> impl FnOnce(Token) -> TokenResult<()> + '_ {
    |token| token.tag(tag)
}

pub fn char(c: char) -> impl FnOnce(Token) -> TokenResult<(char, Option<Token>)> {
    move |token| token.char(c)
}

pub fn alpha_lower(token: Token) -> TokenResult<(char, Option<Token>)> {
    token.alpha_lower()
}

pub fn alpha_upper(token: Token) -> TokenResult<(char, Option<Token>)> {
    token.alpha_upper()
}

pub fn alpha(token: Token) -> TokenResult<(char, Option<Token>)> {
    token.alpha()
}

pub fn alpha_num(token: Token) -> TokenResult<(char, Option<Token>)> {
    token.alpha_num()
}

pub fn digit(token: Token) -> TokenResult<(char, Option<Token>)> {
    token.digit()
}

pub fn oct_digit(token: Token) -> TokenResult<(char, Option<Token>)> {
    token.oct_digit()
}

pub fn hex_digit(token: Token) -> TokenResult<(char, Option<Token>)> {
    token.hex_digit()
}

/// -------------------------------------------------------------------------------
///                                 Combinator
/// -------------------------------------------------------------------------------

pub fn opt<F, O>(f: F) -> impl FnOnce(Token) -> TokenResult<Option<O>>
where
    F: FnOnce(Token) -> TokenResult<O>,
{
    |token| Ok(f(token).ok())
}

fn fold<'s, F>(s: &'s mut String, f: F) -> impl FnOnce(Token) -> Option<Token> + 's
where
    F: FnOnce(Token) -> TokenResult<(char, Option<Token>)> + 's + Copy,
{
    move |token| {
        f(token).ok().and_then(|(c, opt_tok)| {
            s.push(c);
            opt_tok.and_then(|token| fold(s, f)(token))
        })
    }
}

pub fn many0<'a, F>(f: F) -> impl Fn(Token) -> TokenResult<Option<(String, Option<Token>)>>
where
    F: Fn(Token) -> TokenResult<(char, Option<Token>)> + Copy,
{
    move |token| {
        let mut buffer = String::new();
        let opt_token = fold(&mut buffer, f)(token);
        if buffer.is_empty() {
            assert_eq!(opt_token, None);
            Ok(None)
        } else {
            Ok(Some((buffer, opt_token)))
        }
    }
}

pub fn many1<'a, F>(f: F) -> impl Fn(Token) -> TokenResult<(String, Option<Token>)>
where
    F: Fn(Token) -> TokenResult<(char, Option<Token>)> + Copy,
{
    move |token| {
        f(token).map(|(c0, tok_opt)| {
            let mut buffer = c0.to_string();
            let opt_token = tok_opt.and_then(|token| fold(&mut buffer, f)(token));
            (buffer, opt_token)
        })
    }
}

pub fn alt2<'a, O>(
    f1: impl Fn(Token) -> TokenResult<O>,
    f2: impl Fn(Token) -> TokenResult<O>,
) -> impl Fn(Token) -> TokenResult<O> {
    move |token| f1(token.clone()).or_else(|_| f2(token.clone()))
}

#[macro_export]
macro_rules! alt {

    ($f1:expr, $f2: expr) => {
        alt2($f1, $f2)
    };

    ($f1:expr, $f2: expr $(, $f3: expr)+) => {
        alt2($f1,
             alt!($f2$(, $f3)+))
    };
}

#[cfg(test)]
mod test {
    use crate::frontend::lexer::{TokenCont, token::Token};

    use super::{alpha, alpha_lower, alt2, digit, many0, many1, opt, tag};

    #[test]
    fn test_opt() {
        let token = Token::unkown(&TokenCont::word("test"));
        assert!(opt(tag("test"))(token).unwrap().is_some());

        let token = Token::unkown(&TokenCont::word("test"));
        assert!(opt(tag("couc"))(token).unwrap().is_none());
    }

    #[test]
    fn test_many1() {
        let token = Token::unkown(&TokenCont::word("1234a"));
        assert_eq!(many1(digit)(token).unwrap().0, "1234".to_string());

        let token = Token::unkown(&TokenCont::word("a"));
        assert!(many1(digit)(token).is_err());
    }

    #[test]
    fn test_many0() {
        let token = Token::unkown(&TokenCont::word("1234a"));
        assert_eq!(many0(digit)(token).unwrap().unwrap().0, "1234".to_string());

        let token = Token::unkown(&TokenCont::word("a"));
        assert!(many0(digit)(token).unwrap().is_none());
    }

    #[test]
    fn test_alt() {
        let token = Token::unkown(&TokenCont::word("A"));
        assert_eq!(alt!(digit, alpha_lower, alpha)(token).unwrap().0, 'A');
    }
}
