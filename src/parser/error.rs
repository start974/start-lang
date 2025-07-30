use crate::lexer::token::Token;
use crate::utils::error::{ErrorCode, ErrorReport, Message};
use crate::utils::location::{Located, Location, Report, ReportBuilder, SourceId};
use crate::utils::pretty::Pretty;
use crate::utils::theme::Theme;
use ariadne::Label;
use chumsky::error::{Rich, RichPattern};

pub struct Error<'tokens> {
    location: Location,
    err: Rich<'tokens, Token>,
}

impl<'tokens> Error<'tokens> {
    /// make a new error
    pub fn new(err: Rich<'tokens, Token>, source_id: SourceId) -> Self {
        let span = err.span();
        let location = Location::new(source_id, span.start, span.end);
        Self {
            location,
            err: err.clone(),
        }
    }
}

impl ErrorCode for Error<'_> {
    fn code(&self) -> i32 {
        202
    }
}

impl Located for Error<'_> {
    fn loc(&self) -> &Location {
        &self.location
    }
}

impl ErrorReport for Error<'_> {
    fn finalize<'a>(&self, theme: &Theme, report: ReportBuilder<'a>) -> Report<'a> {
        let mut msg = Message::nil().text("Expected ");
        let mut use_or = false;
        for c in self.err.expected() {
            if c == &RichPattern::SomethingElse {
                continue;
            }
            if use_or {
                msg = msg.text(" or ");
            } else {
                use_or = true;
            }
            msg = msg.quoted(c.to_string());
        }
        if let Some(found) = self.err.found() {
            msg = msg.text(", found ").quoted(found.to_string());
        }
        msg = msg.text(".");
        report
            .with_label(Label::new(self.loc().clone()).with_message(msg.make_string(theme)))
            .finish()
    }

    fn message(&self) -> Message {
        Message::nil().text("Parsing error")
    }
}
