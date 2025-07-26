use crate::parser::ast::Identifier;
use crate::utils::error::{ErrorCode, ErrorReport, Message};
use crate::utils::location::{Located, Location, Report, ReportBuilder};

// =======================================================================
// Unknown Option
// =======================================================================

pub struct UnknownOption {
    option: Identifier,
}

impl From<Identifier> for UnknownOption {
    fn from(option: Identifier) -> Self {
        Self { option }
    }
}

impl ErrorCode for UnknownOption {
    fn code(&self) -> i32 {
        103
    }
}
impl Located for UnknownOption {
    fn loc(&self) -> &Location {
        self.option.loc()
    }
}

impl ErrorReport for UnknownOption {
    fn finalize<'a>(
        &self,
        _: &crate::utils::theme::Theme,
        report: ReportBuilder<'a>,
    ) -> Report<'a> {
        report.finish()
    }
    fn message(&self) -> Message {
        Message::nil()
            .text("Option ")
            .quoted(self.option.name())
            .text(" is unknown.")
    }
}
