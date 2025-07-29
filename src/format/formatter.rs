use super::diff::print_diff;
use super::error::ErrorFileWrite;
use super::Mode;
use crate::file_interpreter::error::ErrorFileRead;
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
    commands: Vec<parser::ast::Command>,
}

impl Formatter {
    /// make a new formatter
    pub fn new(path: &Path) -> Self {
        let mut formatter = Self {
            path: path.to_path_buf(),
            content: String::new(),
            err_code: 0,
            theme: Theme::default_theme(),
            commands: Vec::new(),
        };
        match std::fs::read_to_string(path) {
            Ok(content) => {
                formatter.content = content;
            }
            Err(_) => formatter.fail(ErrorFileRead::new(path.to_path_buf())),
        };
        formatter
    }

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
    fn lex(&mut self, content: &str, offset: usize) -> Option<(Vec<lexer::Token>, usize)> {
        match lexer::lex(self.source_id(), offset, content) {
            Ok((tokens, _)) if tokens.is_empty() => None,
            Ok((tokens, end_offset)) => Some((tokens, end_offset)),
            Err(errs) => {
                self.fail(errs);
                None
            }
        }
    }

    /// parse command with lexer tokens
    fn parse(&mut self, tokens: &[lexer::Token]) -> Option<parser::ast::Command> {
        match parser::parse(tokens) {
            Ok(cmd) => Some(cmd),
            Err(errs) => {
                self.fail(errs);
                None
            }
        }
    }

    /// run the interpreter
    fn parse_content(&mut self) {
        let mut offset = 0;
        let mut content = self.content.to_string();

        while !content.is_empty() && self.err_code == 0 {
            match self.lex(&content, offset) {
                Some((tokens, add_offset)) => {
                    offset += add_offset;
                    content = content[add_offset..].to_string();
                    if let Some(cmd) = self.parse(&tokens) {
                        self.commands.push(cmd)
                    }
                }
                None => break,
            }
        }
    }

    /// format content after run
    fn format_content(&self) -> String {
        let mut result = String::new();
        let mut last_end = 0;
        let theme = Theme::default();

        for cmd in self.commands.iter() {
            let loc = cmd.loc();
            let start = loc.start();
            let end = loc.end();
            if start > last_end {
                result.push_str(&self.content[last_end..start]);
            }
            result.push_str(&cmd.to_string(&theme));
            last_end = end;
        }

        if last_end < self.content.len() {
            result.push_str(&self.content[last_end..]);
        }

        result
    }

    /// run formatter with mode
    pub fn run(&mut self, mode: &Mode) {
        self.parse_content();
        if self.err_code != 0 {
            return;
        }

        let formatted = self.format_content();
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
