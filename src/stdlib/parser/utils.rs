use crate::utils::location::{Location, SourceId};
use crate::parser::ast::Identifier;
use chumsky::prelude::*;
use std::rc::Rc;

/// parse unicode alphabetic characters
pub fn letter<'src>() -> impl Parser<'src, &'src str, char> {
    any().filter(|c: &char| c.is_alphabetic())
}

/// parse ascii digits
pub fn digit<'src>() -> impl Parser<'src, &'src str, char> {
    any().filter(|c: &char| c.is_ascii_digit())
}

/// parse `"_"* letter  (letter | 0..9 | _)* "'"*`
pub fn identifier<'src>(source_id: SourceId) -> impl Parser<'src, &'src str, Identifier> {
    let letter = Rc::new(letter());
    let underscore = Rc::new(just('_'));
    let digit = digit();
    let quote = just('\'');

    (underscore.clone().repeated().collect::<String>())
        .then(letter.clone())
        .then(
            letter
                .or(digit)
                .or(underscore)
                .repeated()
                .collect::<String>(),
        )
        .then(quote.repeated().collect::<String>())
        .map_with(
            move |(((underscores, first_letter), mid), apostrophes), e| {
                let span = e.span();
                let name = format!("{}{}{}{}", underscores, first_letter, mid, apostrophes);
                let loc = Location::new(span.start, span.end, source_id.clone());
                Identifier::new(&name, loc)
            },
        )
}
