use chumsky::error::Rich;
use errors::{Error, Message};
use location::{Location, SourceId};

pub fn error_lexing<'src>(err: Rich<'src, char>, source_id: SourceId, offset: usize) -> Error {
    Error::new(201, Message::text("Lexing error"))
        .with_location({
            let span = err.span();
            Location::new(source_id, span.start, span.end).with_offset(offset)
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
