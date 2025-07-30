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
    fn lex(&mut self, content: &str, offset_source: usize) -> Vec<lexer::token::TokenSpanned> {
        let source_id = self.source_id();
        match lexer::lex(source_id.clone(), offset_source, content) {
            Ok(tokens) => tokens,
            Err(errs) => {
                self.fail(errs);
                Vec::new()
            }
        }
    }

    /// parse command with lexer tokens
    fn parse(&mut self, tokens: &[lexer::token::TokenSpanned]) -> Option<parser::ast::Command> {
        let source_id = self.source_id();
        match parser::parse(source_id.clone(), tokens) {
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
        let content_copy = self.content.clone();

        loop {
            let content = &content_copy[offset..];
            if content.is_empty() {
                break;
            }
            let tokens = self.lex(content, offset);
            match tokens.last() {
                None => break,
                Some(last_token) => {
                    if let Some(cmd) = self.parse(&tokens) {
                        self.commands.push(cmd)
                    }
                    offset = last_token.span.end;
                }
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
            result.push_str(&cmd.make_string(&theme));
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
