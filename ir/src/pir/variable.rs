use crate::token::Token;

// ============================================================================
// Variable
// ============================================================================

/// variable of an expression, a pattern, a type or a command argument;
/// its color is chosen by the node that contains it
pub type Variable = Token<String>;
