use super::*;
use chumsky::{Parser, text::unicode::ident};

/// lex identifier defined in
/// [Unicode Standard Annex #31](https://www.unicode.org/reports/tr31/) named `<IDENT>` in ebnf
/// follwing by quotes
/// ```ebnf
/// INDENTIFIER := <IDENT> "'"*
/// ```
pub fn identifier<'src>() -> impl Parser<'src, &'src str, String, Err<ErrorChumsky<'src>>> {
    ident()
        .then(just('\'').repeated().collect::<String>())
        .map(|(ident, quotes)| format!("{ident}{quotes}"))
        .labelled("identifier")
}

#[cfg(test)]
mod tests {
    use super::*;
    use chumsky::Parser;

    #[test]
    fn test_identifier() {
        let parser = identifier();

        let result = parser.parse("variable_name'''").into_result();
        assert_eq!(result.unwrap(), "variable_name'''".to_string());

        let result = parser.parse("αβγδεζ").into_result();
        assert_eq!(result.unwrap(), "αβγδεζ".to_string());

        let result = parser.parse("var123").into_result();
        assert_eq!(result.unwrap(), "var123".to_string());
    }
}
