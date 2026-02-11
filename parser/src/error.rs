use errors::{Error, Message};
use location::GetSpan as _;

use crate::peg::{Peg, RefRule};

// ===========================================================================
// Error rule construction
// ===========================================================================

/// undefined grammar rule error
pub fn not_defined(ref_rule: &RefRule) -> Error {
    Error::new(201, Message::text("Gramar rule not defined."))
        .with_text(
            Message::text("Grammar rule ")
                .with_quoted(ref_rule.name.clone())
                .with_text("does not exist."),
        )
        .with_span(ref_rule.span())
}

/// nullable repetition error
pub fn nullable_repetition(rule: &Peg) -> Error {
    Error::new(202, Message::text("Nullable repetition."))
        .with_text(
            Message::text("rule")
                .with_pretty(rule)
                .with_text("cannot be nullable."),
        )
        .with_span(rule.span())
}

// ===========================================================================
// Parser Error
// ===========================================================================
/*#[derive(Debug)]*/
/*pub struct Expected {*/
/*/// The syntax that was expected at this point.*/
/*pub expected: String,*/

/*/// position*/
/*pub position: usize,*/
/*}*/

/*#[derive(Debug, Default)]*/
/*pub struct ParserError {*/
/*/// expected syntax at the point of failure, used for error reporting*/
/*expected: Vec<Expected>,*/
/*}*/
