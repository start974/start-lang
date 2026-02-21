use super::*;
use crate::lexer::token::Operator;

/**
lex operators
```ebnf
OPERATOR := "?:" | "?" | ":=" | ":" | "$" | "(" | ")"
```
*/
pub fn operator<'src>() -> impl Parser<'src, &'src str, Operator, Err<ErrorChumsky<'src>>> {
    choice((
        just('(').to(Operator::LParen),
        just(')').to(Operator::RParen),
        just("?:").to(Operator::TypeOf),
        just("?").to(Operator::Help),
        just(":=").to(Operator::EqDef),
        just(':').to(Operator::Colon),
        just('$').to(Operator::Eval),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chumsky::Parser;

    #[test]
    fn lex_operator() {
        let parser = operator();

        let result = parser.parse("?:").into_result();
        assert_eq!(result.unwrap(), Operator::TypeOf);

        let result = parser.parse("?").into_result();
        assert_eq!(result.unwrap(), Operator::Help);

        let result = parser.parse(":=").into_result();
        assert_eq!(result.unwrap(), Operator::EqDef);

        let result = parser.parse(":").into_result();
        assert_eq!(result.unwrap(), Operator::Colon);

        let result = parser.parse("$").into_result();
        assert_eq!(result.unwrap(), Operator::Eval);

        let result = parser.parse("(").into_result();
        assert_eq!(result.unwrap(), Operator::LParen);

        let result = parser.parse(")").into_result();
        assert_eq!(result.unwrap(), Operator::RParen);
    }
}
