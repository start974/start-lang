//! Source → tokens: the whole source at once, up to `EndOfInput`.

pub mod error;
pub mod lexing;

use crate::{CheckedResult, Partial};
use ir::token::TokenStream;

pub use lexing::lexer;

pub type ErrorChumsky<'src> = chumsky::error::Rich<'src, char>;

/// lexer of one source
#[derive(Debug, Default, Clone, Copy)]
pub struct Lexer {
    /// position of the first byte of the text in its source
    offset: usize,
}

impl Lexer {
    pub fn builder() -> LexerBuilder {
        LexerBuilder::default()
    }

    /// lex the whole text: the tokens it could read, and every lexing error
    pub fn run(&self, text: &str) -> CheckedResult<TokenStream> {
        use chumsky::Parser as _;
        let (tokens, errs) = lexer(self.offset).parse(text).into_output_errors();
        Partial::ok(TokenStream::from(tokens.unwrap_or_default()))
            .with_errors(errs.iter().map(|err| error::error_lexing(err, self.offset)))
            .check()
    }
}

/// arguments of a `Lexer`
#[derive(Debug, Default, Clone, Copy)]
pub struct LexerBuilder {
    offset: usize,
}

impl LexerBuilder {
    /// position of the first byte of the text in its source (REPL inputs
    /// follow each other in one source)
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }

    pub fn build(self) -> Lexer {
        Lexer {
            offset: self.offset,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ir::location::Spanned as _;
    use ir::token::{Operator, TokenKind};
    use pp::MessageTheme;

    fn tokens(text: &str) -> Vec<TokenKind> {
        Lexer::builder()
            .build()
            .run(text)
            .unwrap_or_else(|_| panic!("lexing error in {text}"))
            .into_inner()
            .into_iter()
            .map(|token| token.value)
            .collect()
    }

    #[test]
    fn whole_source_up_to_end_of_input() {
        let tokens = tokens("Eval 1.\nEval 2.  (* end *)\n");
        assert_eq!(tokens.len(), 7);
        assert_eq!(tokens[2], TokenKind::Operator(Operator::Dot));
        assert_eq!(tokens[5], TokenKind::Operator(Operator::Dot));
        assert_eq!(tokens[6], TokenKind::EndOfInput);
    }

    #[test]
    fn offset_moves_the_spans() {
        let tokens = Lexer::builder()
            .offset(10)
            .build()
            .run("Eval 1.")
            .ok()
            .unwrap();
        assert_eq!(tokens.iter().next().unwrap().span().start(), 10);
    }

    #[test]
    fn every_lexing_error_is_reported() {
        let lexed = Partial::from(Lexer::builder().build().run("Eval 1 # 2.\nEval 3 ~ 4."));
        assert_eq!(lexed.errors.len(), 2);
        // each error says how it was corrected
        for err in &lexed.errors {
            let recovery = err
                .recovery()
                .map(|msg| msg.make_string(&MessageTheme::default()));
            assert_eq!(recovery.as_deref(), Some("the character is skipped"));
        }
        // the tokens around the errors are kept
        let tokens: Vec<_> = lexed.value.into_iter().map(|token| token.value).collect();
        assert_eq!(tokens.len(), 9);
        assert_eq!(tokens.last(), Some(&TokenKind::EndOfInput));
    }
}
