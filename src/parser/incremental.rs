use super::error::Error;
use crate::utils::location::SourceId;
use chumsky::Parser;

use super::ast::Command;

pub struct IncrementalParser<'src> {
    /// source
    source: &'src str,
    /// source id
    source_id: SourceId,
    /// end of source
    end: bool,
    /// offset
    offset : usize,
}

impl<'src> IncrementalParser<'src> {
    /// Create a new incremental parser
    pub fn new(source: &'src str, source_id: SourceId) -> Self {
        Self {
            source,
            source_id,
            end: false,
            offset: 0,
        }
    }

    /// parse next command
    pub fn parse(&mut self) -> Option<Result<Command, Vec<Error>>> {
        if self.end {
            None
        } else {
            let parser = super::parsing::command_offset(self.source_id.clone());
            match parser.parse(self.source).into_result() {
                Ok((command, offset)) => {
                    self.source_id.add_offset(offset);
                    self.offset += offset;
                    self.source = &self.source[offset..];
                    if self.source.is_empty() {
                        self.end = true;
                    }
                    Some(Ok(command))
                }
                Err(errs) => {
                    self.end = true;
                    let errs = errs.iter().map(|err| {
                        Error::new(err.clone(), self.source_id.clone())
                    }).collect::<Vec<_>>();
                    Some(Err(errs))
                }
            }
        }
    }
}
