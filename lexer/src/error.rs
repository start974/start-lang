use crate::ErrorChumsky;
use chumsky::span::Span;
use errors::{Error, Message};
use location::{Location, SourceId};

pub fn error_lexing<'src>(err: ErrorChumsky<'src>, offset: usize) -> Error {
    Error::new(201, Message::text("Lexing error"))
        .with_span({ err.span().into().with_offset() })
        .with_text({
            let msg = if err.expected().len() == 1 {
                Message::text("Lexer expected ")
                    .append(Message::quoted(err.expected().next().unwrap().to_string()))
                    .append_if(err.found().is_some(), || Message::text(", found "))
            } else {
                Message::text("Lexer unknow token ")
            };
            msg.append_opt(
                err.found()
                    .map(|found| Message::quoted(found.to_string().escape_default()).important()),
            )
            .with_text(".")
        })
}
