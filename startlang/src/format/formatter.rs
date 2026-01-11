use super::{Mode, diff::print_diff};
use crate::error;
use ariadne::Source;
use errors::Error;
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
            Err(_) => formatter.fail(error::read_file(path)),
        };
        formatter
    }

    /// content at offset
    fn source_id(&self) -> SourceId {
        SourceId::File(self.path.clone())
    }

    /// fail with error
    pub fn fail(&mut self, error: Error) {
        let mut cache = (self.source_id(), Source::from(&self.content));
        error.eprint(&self.source_id(), &self.theme, &mut cache);
        self.err_code = if self.err_code == 0 { error.code() } else { 1 };
    }

    /// lexing content
    fn lex(&mut self, content: &str, offset_source: usize) -> Option<lexer::MetaTokenStream> {
        match lexer::lex(content, offset_source) {
            Ok(tokens) => Some(tokens),
            Err(errs) => {
                errs.into_iter().for_each(|err| {
                    self.fail(err);
                });
                None
            }
        }
    }

    /// parse command with lexer tokens
    fn parse(&mut self, tokens: MetaTokenStream) -> Option<CommandOrEnd> {
        match parser::parse(tokens) {
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
                self.fail(error::write_file(&self.path))
            }
            Mode::Diff if print_diff(&self.content, &formatted) => {
                self.err_code = 1;
            }
            Mode::Print => print!("{formatted}"),
            _ => (),
        }
    }
}
