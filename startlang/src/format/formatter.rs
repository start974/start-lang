use super::{Mode, diff::print_diff};
use crate::error;
use ariadne::Source;
use errors::Error;
use ir::location::SourceId;
use ir::pir;
use pass::lexer::Lexer;
use pass::parser::Parser;
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
            theme: Theme::colored(),
        };
        match std::fs::read_to_string(path) {
            Ok(content) => {
                formatter.content = content;
            }
            Err(_) => formatter.fail(error::read_file(path)),
        }
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

    /// lex and parse the whole content
    fn parse_content(&mut self) -> Option<pir::File> {
        let parsed = Lexer::builder()
            .build()
            .run(&self.content)
            .and_then(|tokens| Parser::builder().build().run(tokens));
        match parsed {
            Ok(file) => Some(file),
            Err(errs) => {
                for err in errs {
                    self.fail(err);
                }
                None
            }
        }
    }

    /// run formatter with mode
    pub fn run(&mut self, mode: Mode) {
        if self.err_code != 0 {
            return;
        }
        let Some(pir_file) = self.parse_content() else {
            return;
        };
        let formatted = pir_file.make_string(&Theme::default());

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
