use super::utils::TokenResult;
use super::token::Token;

/// -------------------------------------------------------------------------------
///                                 Token matcher 
/// -------------------------------------------------------------------------------
pub fn white_space(token: Token) -> TokenResult<()> {
    token.whitespaces()
}

pub fn eol(token: Token)-> TokenResult<()> {
    token.eol()
}

pub fn eof(token :Token) -> TokenResult<()> {
    token.eof()
}

pub fn tag<'a>(tag: &str) -> impl FnOnce(Token) -> TokenResult<()> + '_ {
    |token| token.tag(tag)
}

pub fn char<'a>(c: char) -> impl FnOnce(Token) -> TokenResult<(char, Option<Token>)> {
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
pub fn opt<'a, F, O>(f: F) -> impl FnOnce(Token) -> Option<O>
where
    F: FnOnce(Token) -> TokenResult<O>
{
    |token| f(token).ok()
}

fn fold<'s, F>(s: &'s mut String, f: F) -> 
    impl FnOnce(Token) -> Option<Token> + 's
where
    F: FnOnce(Token) -> TokenResult<(char, Option<Token>)> + 's + Copy
{
    move |token| f(token).ok().and_then(|(c, opt_tok)| {
            s.push(c);
            opt_tok.and_then(|tok| fold(s, f)(tok))
        })
}

pub fn many0<F>(f: F) -> impl Fn(Token) -> Option<(String, Option<Token>)>
where
    F: Fn(Token) -> TokenResult<(char, Option<Token>)> + Copy
{
    move |token|{
        let mut buffer = String::new();
        let opt_token = fold(&mut buffer, f)(token);
        if buffer.is_empty(){
            assert_eq!(opt_token, None);
            None
        } else {
            Some((buffer, opt_token))
        }
    }
}

pub fn many1<F>(f: F) -> impl Fn(Token) -> TokenResult<(String, Option<Token>)>
where
    F: Fn(Token) -> TokenResult<(char, Option<Token>)> + Copy
{
    move |token|
        f(token).map(|(c0, tok_opt)|{
        let mut buffer = c0.to_string();
        let opt_token = tok_opt.and_then(fold(&mut buffer, f));
        (buffer, opt_token)
    })
}

#[cfg(test)]
mod test{
    use crate::frontend::lexer::{Token, TokenCont, combinator::{tag, many1, digit, many0}};

    use super::opt;

    #[test]
    fn test_opt(){
        let token = Token::unkown(&TokenCont::word("test"));
        assert!(opt(tag("test"))(token).is_some());

        let token = Token::unkown(&TokenCont::word("test"));
        assert!(opt(tag("couc"))(token).is_none());
    }

    #[test]
    fn test_many1(){
        let token = Token::unkown(&TokenCont::word("1234a"));
        assert_eq!(many1(digit)(token).unwrap().0, "1234".to_string() );

        let token = Token::unkown(&TokenCont::word("a"));
        assert!(many1(digit)(token).is_err());
    }

    #[test]
    fn test_many0(){
        let token = Token::unkown(&TokenCont::word("1234a"));
        assert_eq!(many0(digit)(token).unwrap().0, "1234".to_string() );

        let token = Token::unkown(&TokenCont::word("a"));
        assert!(many0(digit)(token).is_none());
    }
}