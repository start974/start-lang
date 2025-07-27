use crate::file_interpreter::error::ErrorFileRead;
use crate::parser::{self, ast::Command};
use crate::utils::error::{ErrorCode, ErrorPrint};
use crate::utils::location::SourceId;
use crate::utils::pretty::Pretty;
use crate::utils::theme::Theme;
use ariadne::Source;
use chumsky::Parser as _;
use colored::Colorize as _;
use error::ErrorFileWrite;
use parsed_file::ParsedFile;
use similar::{ChangeTag, TextDiff};
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::Path;

mod error;
mod parsed_file;

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
    if let Some(parsed_file) = interpreter.parsed_file() {
        let theme = Theme::default();
        let str = parsed_file.to_string(&theme);
        match mode {
            Mode::Overwrite => {
                if write_file(path, &str).is_err() {
                    interpreter.fail(ErrorFileWrite::new(path.to_path_buf()))
                }
            }
            Mode::Diff => {
                if diff(&interpreter.content, &str) {
                    interpreter.err_code = 1;
                }
            }
            Mode::Print => println!("{str}"),
        }
    }
    interpreter.err_code
}

struct Formatter {
    content: String,
    err_code: i32,
    theme: Theme,
    file_parse: ParsedFile,
}

impl Formatter {
    fn new(path: &Path) -> Self {
        let source_id = SourceId::File {
            path: path.to_path_buf(),
        };
        let mut interpreter = Formatter {
            content: String::new(),
            err_code: 0,
            theme: Theme::default_theme(),
            file_parse: ParsedFile::new(source_id),
        };
        match read_to_string(path) {
            Ok(content) => {
                interpreter.content = content;
            }
            Err(_) => interpreter.fail(ErrorFileRead::new(path.to_path_buf())),
        };
        interpreter
    }

    /// get parsed file
    fn parsed_file(&self) -> Option<&ParsedFile> {
        if self.err_code == 0 {
            Some(&self.file_parse)
        } else {
            None
        }
    }

    fn source_id(&self) -> &SourceId {
        self.file_parse.source_id()
    }

    fn parse_command<'src>(
        &mut self,
        content: &'src str,
        offset: usize,
    ) -> Result<(Command, usize), Vec<parser::Error<'src>>> {
        let parser = parser::parse::command_offset(self.source_id().clone(), offset);
        parser.parse(content).into_result().map_err(|errs| {
            errs.iter()
                .map(|err| parser::Error::new(err.clone(), self.source_id().clone(), offset))
                .collect::<Vec<_>>()
        })
    }

    /// fail with error
    fn fail<E>(&mut self, error: E)
    where
        E: ErrorPrint + ErrorCode,
    {
        let mut cache = (self.source_id().clone(), Source::from(&self.content));
        error.eprint(&self.theme, &mut cache).unwrap();
        self.err_code = if self.err_code == 0 { error.code() } else { 1 };
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
                    self.file_parse.add_command(cmd);
                }
                Err(errs) => {
                    self.fail(errs);
                }
            }
        }
    }
}
