use super::*;
use chumsky::text::{newline, whitespace};
use cst::{
    Meta,
    meta_info::{CommentOrLines, MetaInfo, SetMetaInfo},
};

// ===========================================================================
// Meta
// ===========================================================================
/**
trait to add meta information (comments or empty lines) before a rule
and location information to it
*/
pub trait WithMeta<'src, T>: Parser<'src, &'src str, T, Err<ErrorChumsky<'src>>> + Sized {
    /**
      parses a rule with optional meta information (comments or empty lines) before it
      and adds location information to it
      ```ebnf
      meta(rule) = (LINE{2,} | WS* COMMENT)* WS* rule
      ```
    */
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
        let rule_loc =
            self.map_with(move |value, e| (value, Span::from(e.span()).with_offset(offset)));

        // Consomme les derniers espaces/lignes avant le rule
        meta_items
            .then_ignore(whitespace())
            .then(rule_loc)
            .map(move |(comments, (value, loc))| {
                let meta_info = MetaInfo::default().with_items(&comments);
                Meta::new(value, loc).with_meta_info(meta_info)
            })
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
    use cst::meta_info::GetMetaInfo as _;
    use location::GetSpan as _;

    #[test]
    fn with_meta_comment() {
        let parser = identifier().with_meta(0);

        let src = "(* This is a comment *)\n\nmyIdentifier'''";
        let result = parser.parse(src).into_result();

        assert!(result.is_ok());
        let meta = result.unwrap();
        assert_eq!(meta.value, "myIdentifier'''");
        assert_eq!(meta.span(), Span::new(25, 40));
        assert!(meta.meta_info().has_comment());
        assert!(meta.meta_info().get_doc().is_none());
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
        assert!(meta.meta_info().has_comment());
        assert!(meta.meta_info().get_doc().is_some());
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
        assert!(meta.meta_info().has_comment());
        assert!(meta.meta_info().get_doc().is_some());
    }
}
