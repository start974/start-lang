use chumsky::{error::Rich, extra::Err, input::ValueInput, span::SimpleSpan};
use lexer::MetaToken;

pub type ErrorChumsky<'tokens> = Rich<'tokens, MetaToken>;
pub trait Input<'tokens> = ValueInput<'tokens, Token = MetaToken, Span = SimpleSpan>;
pub trait Parser<'tokens, I: Input<'tokens>, T> =
    chumsky::Parser<'tokens, I, T, Err<ErrorChumsky<'tokens>>>;
