//! Tokens → `pir::File`: the whole token stream at once.

mod error;
mod parsing;

use crate::{Checked, CheckedResult, Partial};
use ir::location::{SourceId, Spanned};
use ir::pir;
use ir::token::{Token, TokenKind, TokenStream};

pub use parsing::file;

pub type ErrorChumsky<'tokens> = chumsky::error::Rich<'tokens, Token<TokenKind>>;

/// parser of the token stream of one source
#[derive(Debug, Default, Clone)]
pub struct Parser {
    /// source of the tokens, kept in the `pir::File`
    source: SourceId,
}

impl Parser {
    pub fn builder() -> ParserBuilder {
        ParserBuilder::default()
    }

    /// parse the whole token stream: the commands it could read, and every
    /// parsing error
    // both variants hold the whole file: boxing the error saves nothing
    #[allow(clippy::result_large_err)]
    pub fn run(&self, tokens: &Checked<TokenStream>) -> CheckedResult<pir::File> {
        use chumsky::prelude::*;

        let last = tokens.last_span();
        let eoi = SimpleSpan::from(last.start()..last.end());
        let tokens = tokens
            .iter()
            .map(|token| {
                let span = token.span();
                (token.clone(), SimpleSpan::from(span.start()..span.end()))
            })
            .collect::<Vec<_>>();
        let input = tokens.map(eoi, |(token, span)| (token, span));

        let (file, errs) = file().parse(input).into_output_errors();
        let file = pir::File {
            source: self.source.clone(),
            ..file.unwrap_or_default()
        };
        Partial::ok(file)
            .with_errors(errs.iter().map(error::error_parsing))
            .check()
    }
}

/// arguments of a `Parser`
#[derive(Debug, Default, Clone)]
pub struct ParserBuilder {
    source: SourceId,
}

impl ParserBuilder {
    /// source of the tokens
    pub fn source(mut self, source: SourceId) -> Self {
        self.source = source;
        self
    }

    pub fn build(self) -> Parser {
        Parser {
            source: self.source,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use pp::MessageTheme;

    fn parse(text: &str) -> Partial<pir::File> {
        let tokens = Lexer::builder()
            .build()
            .run(text)
            .unwrap_or_else(|_| panic!("lexing error in {text}"));
        Parser::builder().build().run(&tokens).into()
    }

    #[test]
    fn whole_file() {
        let parsed = parse("Def a := 1.\nEval a.\n$ (a : ℕ).");
        assert!(parsed.is_ok());
        assert_eq!(parsed.value.commands.len(), 3);
    }

    #[test]
    fn empty_file() {
        let parsed = parse("  (* nothing *)  ");
        assert!(parsed.is_ok());
        assert!(parsed.value.commands.is_empty());
    }

    #[test]
    fn every_broken_command_is_reported() {
        // `d.` and `Def := 2.` are broken; parsing goes on after each one and
        // keeps the three valid commands
        let parsed = parse("Eval 1.\nd.\nEval 2.\nDef := 2.\nEval 3.");
        assert_eq!(parsed.errors.len(), 2);
        assert_eq!(parsed.value.commands.len(), 3);
        for err in &parsed.errors {
            let recovery = err
                .recovery()
                .map(|msg| msg.make_string(&MessageTheme::default()));
            assert_eq!(
                recovery.as_deref(),
                Some("the command is skipped up to the next \".\"")
            );
        }
    }

    #[test]
    fn broken_command_up_to_the_end() {
        let parsed = parse("Eval 1.\nEval (2");
        assert_eq!(parsed.errors.len(), 1);
        assert_eq!(parsed.value.commands.len(), 1);
    }
}
