//! Tokens → `pir::File`: the whole token stream at once.

mod error;
mod parsing;

use errors::Errors;
use ir::location::Spanned;
use ir::pir;
use ir::token::{MetaToken, MetaTokenStream};

pub use parsing::file;

pub type ErrorChumsky<'tokens> = chumsky::error::Rich<'tokens, MetaToken>;

/// parser of a token stream
#[derive(Debug, Default, Clone, Copy)]
pub struct Parser;

impl Parser {
    pub fn builder() -> ParserBuilder {
        ParserBuilder
    }

    /// parse the whole token stream; every parsing error is reported
    pub fn run(&self, tokens: MetaTokenStream) -> Result<pir::File, Errors> {
        use chumsky::prelude::*;

        let last = tokens.last_span();
        let eoi = SimpleSpan::from(last.start()..last.end());
        let tokens = tokens
            .into_iter()
            .map(|token| {
                let span = token.span();
                (token, SimpleSpan::from(span.start()..span.end()))
            })
            .collect::<Vec<_>>();
        let input = tokens.map(eoi, |(token, span)| (token, span));

        let (file, errs) = file().parse(input).into_output_errors();
        match file {
            Some(file) if errs.is_empty() => Ok(file),
            _ => Err(errs.iter().map(error::error_parsing).collect()),
        }
    }
}

/// arguments of a `Parser` (none yet)
#[derive(Debug, Default, Clone, Copy)]
pub struct ParserBuilder;

impl ParserBuilder {
    pub fn build(self) -> Parser {
        Parser
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    fn parse(text: &str) -> Result<pir::File, Errors> {
        let tokens = Lexer::builder()
            .build()
            .run(text)
            .unwrap_or_else(|_| panic!("lexing error in {text}"));
        Parser::builder().build().run(tokens)
    }

    #[test]
    fn whole_file() {
        let file = parse("Def a := 1.\nEval a.\n$ (a : ℕ).").ok().unwrap();
        assert_eq!(file.commands().len(), 3);
    }

    #[test]
    fn empty_file() {
        assert!(
            parse("  (* nothing *)  ")
                .ok()
                .unwrap()
                .commands()
                .is_empty()
        );
    }

    #[test]
    fn every_broken_command_is_reported() {
        // `d.` and `Def := 2.` are broken; parsing goes on after each one
        let errs = parse("Eval 1.\nd.\nEval 2.\nDef := 2.\nEval 3.")
            .err()
            .unwrap();
        assert_eq!(errs.len(), 2);
    }

    #[test]
    fn broken_command_up_to_the_end() {
        let errs = parse("Eval 1.\nEval (2").err().unwrap();
        assert_eq!(errs.len(), 1);
    }
}
