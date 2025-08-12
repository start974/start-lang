use crate::parser::cst::expression::Variable;
use crate::parser::cst::AsIdentifier as _;
use crate::utils::error::{ErrorCode, ErrorReport, Message};
use crate::utils::location::{Located, Location};

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
    fn loc(&self) -> Location {
        self.option.loc()
    }
}

impl ErrorReport for UnknownOption {
    fn head(&self) -> Message {
        Message::text("Option unknown.")
    }

    fn text(&self) -> Option<Message> {
        let msg = Message::text("Option ")
            .append(Message::quoted(self.option.name()).important())
            .with_text(" is unknown.");
        Some(msg)
    }
}
