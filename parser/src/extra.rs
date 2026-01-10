use chumsky::{error::Rich, extra::Err, input::Stream};
use lexer::MetaToken;
use std::vec::IntoIter;

pub type ErrorChumsky<'tokens> = Rich<'tokens, MetaToken>;
type SteamToken = Stream<IntoIter<MetaToken>>;
pub trait Parser<'tokens, T> = chumsky::Parser<'tokens, SteamToken, T, Err<ErrorChumsky<'tokens>>>;
