use crate::ErrorChumsky;
use errors::{Error, Message};
use location::Span;

pub fn error_parsing(err: &ErrorChumsky<'_>) -> Error {
    use chumsky::error::RichPattern;

    let expected: Vec<_> = err
        .expected()
        .map(RichPattern::to_string)
        .filter(|s| !s.is_empty())
        .map(Message::quoted)
        .map(Message::important)
        .collect();
    let res = Error::new(202, Message::text("Parsing error"))
        .with_span({
            let span = err.span();
            Span::new(span.start, span.end)
        })
        .with_text(
            Message::text("Parsing expect ")
                .append(Message::intersperse(
                    expected.clone(),
                    Message::text(" or "),
                ))
                .with_text("."),
        );
    match err.found().map(|meta| meta.value.clone()) {
        None => res,
        Some(found) => res.with_note(
            Message::text("Expected : ")
                .append(Message::intersperse(expected, Message::text(", ")))
                .with_line()
                .with_text("Found    : ")
                .append(Message::quoted(found.to_string()).important())
                .with_text("."),
        ),
    }
}
