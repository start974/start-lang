//! Prints on a terminal: outputs on stdout, errors on stderr with their
//! place in the source (`startlang run`); gives the text of an error (the
//! TUI).

use crate::compile::Output;
use ariadne::Source;
use errors::{Error, Errors};
use ir::location::SourceId;
use pp::{Pretty, Theme};

pub struct Printer {
    source: SourceId,
    /// whole text of the source, to show the errors in it
    content: String,
    theme: Theme,
    /// exit code so far: the code of the only error, `1` for several
    code: i32,
}

impl Printer {
    pub fn new(source: SourceId) -> Self {
        Self {
            source,
            content: String::new(),
            theme: Theme::colored(),
            code: 0,
        }
    }

    /// colors of the outputs and errors
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// whole text of the source
    pub fn set_content(&mut self, content: impl Into<String>) {
        self.content = content.into();
    }

    /// exit code so far
    pub fn code(&self) -> i32 {
        self.code
    }

    pub fn output(&self, output: &Output<'_>) {
        match output {
            Output::Defined => {}
            Output::Value(value, _) => self.print(value),
            Output::Type(ty, _) => self.print(*ty),
            Output::Help(help) => self.print(*help),
        }
    }

    fn print(&self, doc: &impl Pretty) {
        println!("{}", doc.make_string(&self.theme));
    }

    /// text of an error, with its place in the source
    pub fn render_error(&self, err: &Error) -> String {
        let cache = (self.source.clone(), Source::from(&self.content));
        let mut text = Vec::new();
        err.report(&self.source, &self.theme)
            .write(cache, &mut text)
            .expect("writing to memory does not fail");
        String::from_utf8(text).expect("ariadne writes UTF-8")
    }

    /// print every error
    pub fn errors(&mut self, errs: Errors) {
        for err in errs.sorted() {
            self.fail(&err);
        }
    }

    /// print the error and update the exit code
    pub fn fail(&mut self, err: &Error) {
        let mut cache = (self.source.clone(), Source::from(&self.content));
        err.eprint(&self.source, &self.theme, &mut cache);
        self.code = if self.code == 0 { err.code() } else { 1 };
    }
}
