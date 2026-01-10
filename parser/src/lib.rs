use chumsky::{Parser as _, input::Input as _, span::SimpleSpan};
use cst::{Command, EndOfFile};
use lexer::MetaTokenStream;
use location::{Located as _, SourceId};

pub mod error;
pub mod parsing;

pub use error::Error;
pub use parsing::parser;

pub enum CommandOrEnd {
    Command(Box<Command>),
    End(EndOfFile),
}

/// parse tokens
pub fn parse(source_id: SourceId, tokens: MetaTokenStream) -> Result<CommandOrEnd, Vec<Error>> {
    let tokens_spanned = tokens
        .into_iter()
        .map(|token| (token.clone(), token.loc().to_simple_span()))
        .collect::<Vec<_>>();
    let span_end: SimpleSpan = tokens_spanned.last().unwrap().1;
    let input = tokens_spanned.map(span_end, |(t, s)| (t, s));

    parser().parse(input).into_result().map_err(|errs| {
        errs.iter()
            .map(|e| Error::new(e.clone(), source_id.clone()))
            .collect()
    })
}
