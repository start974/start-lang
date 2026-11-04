//! Prints on a terminal: outputs on stdout, errors on stderr with their
//! place in the source; used by `startlang run` and the REPL.

use crate::compile::Output;
use crate::repl::summary::SummaryDefinition;
use ariadne::Source;
use errors::{Error, Errors};
use ir::location::SourceId;
use pp::{Pretty, Theme};

pub struct Printer {
    source: SourceId,
    /// whole text of the source, to show the errors in it
    content: String,
    theme: Theme,
    /// print the summary of each definition (REPL)
    summary: bool,
    /// exit code so far: the code of the only error, `1` for several
    code: i32,
}

impl Printer {
    pub fn new(source: SourceId) -> Self {
        Self {
            source,
            content: String::new(),
            theme: Theme::colored(),
            summary: false,
            code: 0,
        }
    }

    /// colors of the outputs and errors
    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "used by the terminal UI (TODO step 3)")
    )]
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// print the summary of each definition
    pub fn with_summary(mut self, summary: bool) -> Self {
        self.summary = summary;
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

    /// forget the errors before (next REPL input)
    pub fn reset_code(&mut self) {
        self.code = 0;
    }

    pub fn output(&self, output: &Output<'_>) {
        match output {
            Output::Defined(def) if self.summary => {
                println!(
                    "       {}",
                    SummaryDefinition::from(*def).make_string(&self.theme)
                );
            }
            Output::Defined(_) => {}
            Output::Value(value, _) => self.print(value),
            Output::Type(ty, _) => self.print(*ty),
            Output::Help(help) => self.print(*help),
        }
    }

    fn print(&self, doc: &impl Pretty) {
        println!("{}", doc.make_string(&self.theme));
    }

    /// text of an error, with its place in the source
    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "used by the terminal UI (TODO step 3)")
    )]
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
