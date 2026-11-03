use crate::lexer::ErrorChumsky;
use errors::{Error, Message};
use ir::location::Span;

pub fn error_lexing(err: &ErrorChumsky<'_>, offset: usize) -> Error {
    Error::new(201, Message::text("Lexing error"))
        .with_span({
            let span = err.span();
            Span::new(span.start, span.end).with_offset(offset)
        })
        .with_text({
            let mut expected = err.expected();
            let msg = match (expected.next(), expected.next()) {
                (Some(expected), None) => Message::text("Lexer expected ")
                    .append(Message::quoted(expected.to_string()))
                    .append_if(err.found().is_some(), || Message::text(", found ")),
                _ => Message::text("Lexer unknow token "),
            };
            msg.append_opt(
                err.found()
                    .map(|found| Message::quoted(found.to_string().escape_default()).important()),
            )
            .with_text(".")
        })
        .with_recovery(Message::text("the character is skipped"))
        .with_help(Message::text("remove or replace this character"))
}
