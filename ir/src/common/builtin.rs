use pp::pretty::*;

// ============================================================================
// Builtin Type
// ============================================================================

/// type known by the compiler, written `__Type_Nat__`, … in the source
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuiltinType {
    /// natural numbers
    Nat,
    /// booleans
    Bool,
    /// characters
    Char,
}

impl BuiltinType {
    /// name as written in the source
    pub fn source_name(self) -> &'static str {
        match self {
            BuiltinType::Nat => "__Type_Nat__",
            BuiltinType::Bool => "__Type_Bool__",
            BuiltinType::Char => "__Type_Char__",
        }
    }

    /// symbol shown to the user once typed (`ℕ`, `𝔹`, `Char`)
    pub fn symbol(self) -> &'static str {
        match self {
            BuiltinType::Nat => "ℕ",
            BuiltinType::Bool => "𝔹",
            BuiltinType::Char => "Char",
        }
    }
}

/// printed as written in the source
impl Pretty for BuiltinType {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        theme.ty_var(&self.source_name())
    }
}

// ============================================================================
// Builtin Constant
// ============================================================================

/// constant known by the compiler, written `__Constant_true__`, … in the
/// source
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuiltinConstant {
    True,
    False,
}

impl BuiltinConstant {
    /// name as written in the source
    pub fn source_name(self) -> &'static str {
        match self {
            BuiltinConstant::True => "__Constant_true__",
            BuiltinConstant::False => "__Constant_false__",
        }
    }

    /// boolean value of the constant
    pub fn value(self) -> bool {
        match self {
            BuiltinConstant::True => true,
            BuiltinConstant::False => false,
        }
    }
}

/// printed as written in the source
impl Pretty for BuiltinConstant {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        theme.expr_var(&self.source_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_type_names() {
        let theme = Theme::default();
        assert_eq!(BuiltinType::Nat.make_string(&theme), "__Type_Nat__");
        assert_eq!(BuiltinType::Bool.symbol(), "𝔹");
    }

    #[test]
    fn builtin_constant_names_and_values() {
        let theme = Theme::default();
        assert_eq!(
            BuiltinConstant::False.make_string(&theme),
            "__Constant_false__"
        );
        assert!(BuiltinConstant::True.value());
        assert!(!BuiltinConstant::False.value());
    }
}
