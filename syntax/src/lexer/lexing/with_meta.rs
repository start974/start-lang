use super::*;
use chumsky::text::{newline, whitespace};
use cst::{Meta, meta::CommentOrLines};

// ===========================================================================
// Meta
// ===========================================================================
pub trait WithMeta<'src, T>: Parser<'src, &'src str, T, Err<ErrorChumsky<'src>>> + Sized {
    /// meta(rule) = (LINE{2,} | WS* COMMENT)* WS* rule
    fn with_meta(
        self,
        offset: usize,
    ) -> impl Parser<'src, &'src str, Meta<T>, Err<ErrorChumsky<'src>>> {
        let lines = newline().repeated().at_least(2).to(CommentOrLines::Lines);
        let comment = whitespace()
            .ignore_then(comment())
            .map(CommentOrLines::Comment);

        let meta_items = (lines.or(comment))
            .repeated()
            .collect::<Vec<CommentOrLines>>();

        // Ajoute la location à la rule
        let rule_loc = self.map_with(move |value, e| {
            let span: SimpleSpan = e.span();
            (value, Span::new(span.start, span.end).with_offset(offset))
        });

        // Consomme les derniers espaces/lignes avant le rule
        meta_items
            .then_ignore(whitespace())
            .then(rule_loc)
            .map(move |(comments, (value, loc))| Meta::new(value, loc).with_items(&comments))
    }
}

impl<'src, T, P> WithMeta<'src, T> for P where
    P: Parser<'src, &'src str, T, Err<ErrorChumsky<'src>>> + Sized
{
}

#[cfg(test)]
mod tests {
    use super::*;
    use chumsky::prelude::*;
    use location::Spanned as _;

    #[test]
    fn with_meta_comment() {
        let parser = identifier().with_meta(0);

        let src = "(* This is a comment *)\n\nmyIdentifier'''";
        let result = parser.parse(src).into_result();

        assert!(result.is_ok());
        let meta = result.unwrap();
        assert_eq!(meta.value, "myIdentifier'''");
        assert_eq!(meta.span(), Span::new(25, 40));
        assert!(meta.has_comment());
        assert!(meta.get_doc().is_none());
    }

    #[test]
    fn with_meta_doc() {
        let parser = identifier().with_meta(10);
        let src = "(** This is a comment *)\nmyIdentifier'''";
        let result = parser.parse(src).into_result();

        assert!(result.is_ok());
        let meta = result.unwrap();
        assert_eq!(meta.value, "myIdentifier'''");
        assert_eq!(meta.span(), Span::new(35, 50));
        assert!(meta.has_comment());
        assert!(meta.get_doc().is_some());
    }

    #[test]
    fn with_meta_lines() {
        let parser = identifier().with_meta(0);

        let src = "\n\n\nmyIdentifier";
        let result = parser.parse(src).into_result();

        assert!(result.is_ok());
        let meta = result.unwrap();
        assert_eq!(meta.value, "myIdentifier");
        assert_eq!(meta.span(), Span::new(3, 15));
        assert!(!meta.has_comment());
        assert!(meta.get_doc().is_none());
    }
}
