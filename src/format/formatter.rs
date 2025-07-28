use super::diff::print_diff;
use super::error::ErrorFileWrite;
use super::Mode;
use crate::file_interpreter::error::ErrorFileRead;
use crate::parser;
use crate::utils::error::{ErrorCode, ErrorPrint};
use crate::utils::location::{Located as _, SourceId};
use crate::utils::pretty::Pretty as _;
use crate::utils::theme::Theme;
use ariadne::{Source, Span as _};
use chumsky::Parser as _;
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

    /// parse a command
    fn parse_command<'src>(
        &mut self,
        content: &'src str,
        offset: usize,
    ) -> Result<Option<(parser::ast::Command, usize)>, Vec<parser::Error<'src>>> {
        let source_id = SourceId::File(self.path.clone());
        let parser = parser::parse::command_offset(source_id.clone(), offset);
        parser.parse(content).into_result().map_err(|errs| {
            errs.iter()
                .map(|err| parser::Error::new(err.clone(), source_id.clone(), offset))
                .collect::<Vec<_>>()
        })
    }

    /// fail with error
    pub fn fail<E>(&mut self, error: E)
    where
        E: ErrorPrint + ErrorCode,
    {
        let source_id = SourceId::File(self.path.clone());
        let mut cache = (source_id, Source::from(&self.content));
        error.eprint(&self.theme, &mut cache).unwrap();
        self.err_code = if self.err_code == 0 { error.code() } else { 1 };
    }

    fn run_parser(&mut self) {
        let mut offset = 0;
        let mut content = self.content.clone();

        while !content.is_empty() && self.err_code == 0 {
            match self.parse_command(&content, offset) {
                Ok(Some((cmd, add_offset))) => {
                    content = content[add_offset..].to_string();
                    offset += add_offset;
                    self.commands.push(cmd);
                }
                Ok(None) => break,
                Err(errs) => {
                    self.fail(errs);
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
        self.run_parser();
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
