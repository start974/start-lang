use super::token;
use chumsky::extra::Err;
use chumsky::prelude::*;
use cst::Comment;
use location::Span;
use num_bigint::BigUint;
use std::rc::Rc;

mod character;
mod comment;
mod identifier;
mod number;
mod operator;
mod with_meta;

use character::character;
pub use chumsky::prelude::Parser;
pub use comment::comment;
pub use identifier::identifier;
pub use number::{
    digit, digit_bin, digit_hex, digit_oct, number, number_bin, number_dec, number_hex, number_oct,
};
pub use operator::operator;
pub use with_meta::WithMeta;

pub type ErrorChumsky<'src> = chumsky::error::Rich<'src, char>;
pub type ExtraChumsky<'src> = chumsky::extra::Err<ErrorChumsky<'src>>;

// ===========================================================================
// Lexer
// ===========================================================================
/// make a lexing with offset to token until "." (end of a command)
/// return offset rest to lexing
pub fn lexer<'src>(
    offset: usize,
) -> impl Parser<'src, &'src str, Vec<token::MetaToken>, Err<ErrorChumsky<'src>>> {
    use token::Token;

    let token = choice((
        operator().map(Token::Operator),
        identifier().map(Token::Identifier),
        number().map(Token::Number),
        character().map(Token::Character),
    ))
    .with_meta(offset);

    let token_dot = just('.')
        .to(Token::Operator(token::Operator::Dot))
        .with_meta(offset)
        .lazy();

    let token_end = end().to(Token::EndOfInput).with_meta(offset);

    token
        .repeated()
        .collect::<Vec<_>>()
        .then(choice((token_dot, token_end)))
        .map(move |(mut tokens, end)| {
            tokens.push(end);
            tokens
        })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lexer_test() {
        use token::Token::*;
        let parser = lexer(0);

        let result = parser.parse("var_name 123 'a' ?: .").into_result();
        assert!(result.is_ok());
        let tokens = result.unwrap();

        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].value, Identifier("var_name".to_string()));
        assert_eq!(tokens[1].value, Number(BigUint::from(123u32)));
        assert_eq!(tokens[2].value, Character('a'));
        assert_eq!(tokens[3].value, Operator(token::Operator::TypeOf));
        assert_eq!(tokens[4].value, Operator(token::Operator::Dot));
    }
}
