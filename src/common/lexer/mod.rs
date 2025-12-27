use crate::utils::location::{Located as _, Location, SourceId};
use ariadne::Span as _;
use chumsky::prelude::*;

pub mod comment;
pub mod error;
pub mod lexing;
pub mod meta;
pub mod token;

pub use error::Error;
pub use meta::Meta;

pub type ErrorChumsky<'a> = chumsky::extra::Err<chumsky::error::Rich<'a, char>>;
pub use token::MetaToken;

pub struct Lexer {
    source_id: SourceId,
    content: String,
    offset: usize,
}

impl Lexer {
    pub fn new(source_id: SourceId) -> Self {
        Self {
            source_id,
            offset: 0,
            content: String::new(),
        }
    }

    //TODO: remove
    pub fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }

    /// feed content to lex
    pub fn add_content(&mut self, content: &str) {
        self.content.push_str(content);
    }

    /// feed content
    pub fn with_content(mut self, content: &str) -> Self {
        self.add_content(content);
        self
    }

    pub fn run(&mut self) -> Option<Result<Vec<MetaToken>, Vec<Error>>> {
        if self.content.is_empty() {
            None
        } else {
            let source_id = self.source_id.clone();
            let offset = self.offset;

            let res: Result<Vec<MetaToken>, Vec<Error>> = lexing::lexer()
                .parse(&self.content)
                .into_result()
                .map(|tokens| {
                    tokens
                        .into_iter()
                        .map(|token| {
                            token.map_location(|span| {
                                Location::new(source_id.clone(), span.start, span.end)
                                    .with_offset(offset)
                            })
                        })
                        .collect()
                })
                .map_err(|errs| {
                    errs.iter()
                        .map(|e| Error::new(e.clone(), source_id.clone(), offset))
                        .collect()
                });
            if let Ok(tokens) = &res {
                let last_tokens = tokens.last().unwrap();
                self.offset = last_tokens.loc().end();
                self.content = self.content[self.offset - offset..].to_string();
            };
            Some(res)
        }
    }
}
