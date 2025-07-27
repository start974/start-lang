use crate::file_interpreter::error::ErrorFileRead;
use crate::parser;
use crate::utils::error::{ErrorCode, ErrorPrint};
use crate::utils::location::{Located as _, SourceId};
use crate::utils::pretty::Pretty;
use crate::utils::theme::Theme;
use ariadne::{Source, Span as _};
use chumsky::Parser as _;
use colored::Colorize as _;
use error::ErrorFileWrite;
use similar::{ChangeTag, TextDiff};
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::Path;

mod error;

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Overwrite,
    Diff,
    Print,
}

fn diff(original: &str, formatted: &str) -> bool {
    let diff = TextDiff::from_lines(original, formatted);
    let mut has_diff = false;
    for change in diff.iter_all_changes() {
        match change.tag() {
            ChangeTag::Delete => {
                print!("{}", format!("-{change}").red());
                has_diff = true;
            }
            ChangeTag::Insert => {
                print!("{}", format!("+{change}").green());
                has_diff = true;
            }
            ChangeTag::Equal => {
                print!(" {change}");
            }
        }
    }
    has_diff
}

fn write_file(path: &Path, content: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(path)?;
    write!(file, "{content}")
}

/// run formatter on file
pub fn run(path: &Path, mode: Mode) -> i32 {
    let mut interpreter = Formatter::new(path);
    interpreter.run();
    if let Some(formatted) = interpreter.formatted_content() {
        match mode {
            Mode::Overwrite => {
                if write_file(path, &formatted).is_err() {
                    interpreter.fail(ErrorFileWrite::new(path.to_path_buf()))
                }
            }
            Mode::Diff => {
                if diff(&interpreter.content, &formatted) {
                    interpreter.err_code = 1;
                }
            }
            Mode::Print => print!("{formatted}"),
        }
    }
    interpreter.err_code
}

struct Formatter {
    source_id: SourceId,
    content: String,
    err_code: i32,
    theme: Theme,
    commands: Vec<parser::ast::Command>,
}

impl Formatter {
    fn new(path: &Path) -> Self {
        let mut interpreter = Formatter {
            source_id: SourceId::File {
                path: path.to_path_buf(),
            },
            content: String::new(),
            err_code: 0,
            theme: Theme::default_theme(),
            commands: Vec::new(),
        };
        match read_to_string(path) {
            Ok(content) => {
                interpreter.content = content;
            }
            Err(_) => interpreter.fail(ErrorFileRead::new(path.to_path_buf())),
        };
        interpreter
    }

    fn parse_command<'src>(
        &mut self,
        content: &'src str,
        offset: usize,
    ) -> Result<(parser::ast::Command, usize), Vec<parser::Error<'src>>> {
        let parser = parser::parse::command_offset(self.source_id.clone(), offset);
        parser.parse(content).into_result().map_err(|errs| {
            errs.iter()
                .map(|err| parser::Error::new(err.clone(), self.source_id.clone(), offset))
                .collect::<Vec<_>>()
        })
    }

    /// fail with error
    fn fail<E>(&mut self, error: E)
    where
        E: ErrorPrint + ErrorCode,
    {
        let mut cache = (self.source_id.clone(), Source::from(&self.content));
        error.eprint(&self.theme, &mut cache).unwrap();
        self.err_code = if self.err_code == 0 { error.code() } else { 1 };
    }

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

    pub fn formatted_content(&self) -> Option<String> {
        if self.err_code == 0 {
            Some(self.format_content())
        } else {
            None
        }
    }

    /// run formatter
    fn run(&mut self) {
        let mut offset = 0;
        let mut content = self.content.clone();

        while !content.is_empty() && self.err_code == 0 {
            match self.parse_command(&content, offset) {
                Ok((cmd, add_offset)) => {
                    content = content[add_offset..].to_string();
                    offset += add_offset;
                    self.commands.push(cmd);
                }
                Err(errs) => {
                    self.fail(errs);
                }
            }
        }
    }
}
