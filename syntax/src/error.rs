use crate::lexer::lexing::ErrorChumsky as ErrorLexer;
use crate::parser::parsing::ErrorChumsky as ErrorParser;
use errors::{Error, Message};
use location::Span;

pub fn lexing<'src>(err: &ErrorLexer<'src>, offset: usize) -> Error {
    Error::new(201, Message::text("Lexing error"))
        .with_span({
            let span = err.span();
            Span::new(span.start, span.end).with_offset(offset)
        })
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

pub fn parsing(err: &ErrorParser) -> Error {
    use chumsky::error::RichPattern;

    let expected: Vec<_> = err
        .expected()
        .map(RichPattern::to_string)
        .filter(|s| !s.is_empty())
        .map(Message::quoted)
        .map(Message::important)
        .collect();
    Error::new(202, Message::text("Parsing error"))
        .with_span(*err.span())
        .with_text(
            Message::text("Parsing expect ")
                .append(Message::intersperse(
                    expected.clone(),
                    Message::text(" or "),
                ))
                .with_text("."),
        )
        .with_note(Message::from(
            err.found().map(|meta| meta.value.clone()).map(|found| {
                Message::text("Expected : ")
                    .append(Message::intersperse(expected, Message::text(", ")))
                    .with_line()
                    .with_text("Found    : ")
                    .append(Message::quoted(found.to_string()).important())
                    .with_text(".")
            }),
        ))
}
