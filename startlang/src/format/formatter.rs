use super::Mode;
use super::diff::print_diff;
use super::error::ErrorFileWrite;
use crate::file_interpreter::error::ErrorFileRead;
use ariadne::Source;
use error::{ErrorCode, ErrorPrint};
use lexer::MetaTokenStream;
use location::SourceId;
use parser::CommandOrEnd;
use pp::pretty::Pretty as _;
use pp::theme::Theme;
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
    fn lex(&mut self, content: &str, offset_source: usize) -> Option<lexer::MetaTokenStream> {
        let source_id = self.source_id();
        match lexer::lex(source_id.clone(), offset_source, content) {
            Ok(tokens) => Some(tokens),
            Err(errs) => {
                for err in errs {
                    self.fail(err);
                }
                None
            }
        }
    }

    /// parse command with lexer tokens
    fn parse(&mut self, tokens: MetaTokenStream) -> Option<CommandOrEnd> {
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
    fn parse_content(&mut self) -> Option<cst::File> {
        let mut offset = 0;
        let content_copy = self.content.clone();
        let mut cst_file = cst::File::default();

        loop {
            let content = &content_copy[offset..];
            if content.is_empty() {
                break;
            }
            match self.lex(content, offset) {
                None => break,
                Some(tokens) => {
                    let offset_end = tokens.last_offset();
                    match self.parse(tokens) {
                        None => {
                            break;
                        }
                        Some(CommandOrEnd::Command(cmd)) => cst_file.add_command(*cmd),
                        Some(CommandOrEnd::End(end)) => {
                            cst_file.set_end(end);
                            break;
                        }
                    }
                    offset = offset_end
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
