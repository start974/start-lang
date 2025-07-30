use crate::parser::ast::Variable;
use crate::utils::error::{ErrorCode, ErrorReport, Message};
use crate::utils::location::{Located, Location, Report, ReportBuilder};

// =======================================================================
// Unknown Option
// =======================================================================

pub struct UnknownOption {
    option: Variable,
}

impl From<Variable> for UnknownOption {
    fn from(option: Variable) -> Self {
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
            .quoted(self.option.to_string())
            .text(" is unknown.")
    }
}
