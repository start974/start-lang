use crate::sir::Node;

// ============================================================================
// Variable
// ============================================================================

/// variable of an expression, a pattern, a type, or a command argument
pub type Variable = Node<String>;

impl Node<String> {
    /// get name of variable
    pub fn name(&self) -> &str {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::location::{Span, Spanned};

    #[test]
    fn variable_name_and_span() {
        let var = Variable::new("x".to_string(), Span::new(2, 3));
        assert_eq!(var.name(), "x");
        assert_eq!(var.span(), Span::new(2, 3));
    }
}
