use super::*;

/**
lex comment
```ebnf
COMMENT := "(*" <ANY>* "*)"
```
*/
pub fn comment<'src>() -> impl Parser<'src, &'src str, Comment, Err<ErrorChumsky<'src>>> {
    let start = just("(*")
        .ignore_then(just("*").or_not())
        .map(|opt| opt.is_some());

    start
        .then(
            any()
                .and_is(just("*)").not())
                .repeated()
                .collect::<String>(),
        )
        .then_ignore(just("*)"))
        .map(|(is_doc, str)| Comment::from(str).with_is_doc(is_doc))
        .labelled("comment")
}

#[cfg(test)]
mod tests {
    use super::comment;
    use chumsky::Parser;
    use pp::{Pretty as _, Theme};

    #[test]
    fn lex_comment() {
        let parser = comment();
        let theme = Theme::default();

        let cases = vec![
            (
                "(*         This is a comment      *)",
                "(* This is a comment *)",
            ),
            ("(**Doc comment *)", "(** Doc comment *)"),
        ];

        for (input, expected_content) in cases {
            let result = parser.parse(input).into_result();
            let comment_str = result.unwrap().make_string(&theme);
            assert_eq!(comment_str, expected_content);
        }
    }
}
