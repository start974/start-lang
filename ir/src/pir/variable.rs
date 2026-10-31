use crate::pir::Meta;

// ============================================================================
// Variable
// ============================================================================

/// variable of an expression, a pattern, a type or a command argument;
/// its color is chosen by the node that contains it
pub type Variable = Meta<String>;

impl Meta<String> {
    /// get name of variable
    pub fn name(&self) -> &str {
        &self.value
    }
}
