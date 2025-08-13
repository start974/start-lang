use super::diff::print_diff;
use super::error::ErrorFileWrite;
use super::Mode;
use crate::file_interpreter::error::ErrorFileRead;
use crate::parser::CommandOrEnd;
use crate::utils::error::{ErrorCode, ErrorPrint};
use crate::utils::location::{Located as _, SourceId};
use crate::utils::pretty::Pretty as _;
use crate::utils::theme::Theme;
use crate::{lexer, parser};
use ariadne::{Source, Span as _};
use std::path::{Path, PathBuf};

pub struct Formatter {
    path: PathBuf,
    content: String,
    pub err_code: i32,
    theme: Theme,
}

impl Formatter {
    /// make a new formatter
    pub fn new(path: &Path) -> Self {
        let mut formatter = Self {
            path: path.to_path_buf(),
            content: String::new(),
            err_code: 0,
            theme: Theme::default_theme(),
        };
        match std::fs::read_to_string(path) {
            Ok(content) => {
                formatter.content = content;
            }
            Err(_) => formatter.fail(ErrorFileRead::new(path.to_path_buf())),
        };
        formatter
    }

    /// content at offset
    fn source_id(&self) -> SourceId {
        SourceId::File(self.path.clone())
    }

    /// fail with error
    pub fn fail<E>(&mut self, error: E)
    where
        E: ErrorPrint + ErrorCode,
    {
        let mut cache = (self.source_id(), Source::from(&self.content));
        error.eprint(&self.theme, &mut cache).unwrap();
        self.err_code = if self.err_code == 0 { error.code() } else { 1 };
    }

    /// lexing content
    fn lex(&mut self, content: &str, offset_source: usize) -> Vec<lexer::token::MetaToken> {
        let source_id = self.source_id();
        match lexer::lex(source_id.clone(), offset_source, content) {
            Ok(tokens) => tokens,
            Err(errs) => {
                for err in errs {
                    self.fail(err);
                }
                Vec::new()
            }
        }
    }

    /// parse command with lexer tokens
    fn parse(&mut self, tokens: &[lexer::token::MetaToken]) -> Option<CommandOrEnd> {
        let source_id = self.source_id();
        match parser::parse(source_id.clone(), tokens) {
            Ok(cmd) => Some(cmd),
            Err(errs) => {
                for err in errs {
                    self.fail(err);
                }
                None
            }
        }
    }

    /// run the interpreter
    fn parse_content(&mut self) -> Option<parser::cst::File> {
        let mut offset = 0;
        let content_copy = self.content.clone();
        let mut cst_file = parser::cst::File::default();

        loop {
            let content = &content_copy[offset..];
            if content.is_empty() {
                break;
            }
            let tokens = self.lex(content, offset);
            match tokens.last() {
                None => break,
                Some(last_token) => {
                    match self.parse(&tokens) {
                        None => {
                            break;
                        }
                        Some(CommandOrEnd::Command(cmd)) => cst_file.add_command(*cmd),
                        Some(CommandOrEnd::End(end)) => {
                            cst_file.set_end(end);
                            break;
                        }
                    }
                    offset = last_token.loc().end();
                }
            }
        }
        if self.err_code == 0 {
            Some(cst_file)
        } else {
            None
        }
    }

    /// run formatter with mode
    pub fn run(&mut self, mode: &Mode) {
        if self.err_code != 0 {
            return;
        }
        let Some(cst_file) = self.parse_content() else {
            return;
        };
        let formatted = cst_file.make_string(&Theme::default());

        match mode {
            Mode::Overwrite if std::fs::write(&self.path, &formatted).is_err() => {
                self.fail(ErrorFileWrite::new(self.path.clone()))
            }
            Mode::Diff if print_diff(&self.content, &formatted) => {
                self.err_code = 1;
            }
            Mode::Print => print!("{formatted}"),
            _ => (),
        }
    }
}
