use crate::lexer::token::Token;
use crate::lexer::MetaToken;
use crate::utils::error::{ErrorCode, ErrorReport, Message};
use crate::utils::location::{Located, Location, Report, ReportBuilder, SourceId};
use crate::utils::pretty::Pretty;
use crate::utils::theme::Theme;
use ariadne::Label;
use chumsky::error::{Rich, RichPattern};

pub struct Error {
    loc: Location,
    expected: Vec<String>,
    found: Option<Token>,
}

impl Error {
    /// make a new error
    pub fn new(err: Rich<'_, MetaToken>, source_id: SourceId) -> Self {
        let span = err.span();
        let loc = Location::new(source_id, span.start, span.end);
        Self {
            loc,
            expected: {
                err.expected()
                    .map(RichPattern::to_string)
                    .filter(|s| !s.is_empty())
                    .collect()
            },
            found: err.found().map(|meta| meta.value.clone()),
        }
    }
}

impl ErrorCode for Error {
    fn code(&self) -> i32 {
        202
    }
}

impl Located for Error {
    fn loc(&self) -> Location {
        self.loc.clone()
    }
}

impl ErrorReport for Error {
    fn finalize<'a>(&self, theme: &Theme, report: ReportBuilder<'a>) -> Report<'a> {
        let mut msg = Message::nil().text("Expected ");
        let mut use_or = false;
        for expect in self.expected.iter() {
            if use_or {
                msg = msg.text(" or ");
            } else {
                use_or = true;
            }
            msg = msg.quoted(expect);
        }
        if let Some(found) = &self.found {
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
