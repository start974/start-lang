use crate::sir::Node;

// ============================================================================
// Variable
// ============================================================================

/// variable of an expression, a pattern, a type, or a command argument
pub type Variable = Node<String>;
