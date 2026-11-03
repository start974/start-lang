use crate::lir::{Expression, Type};
use pp::pretty::*;
use std::fmt;

// ============================================================================
// Global Identifier
// ============================================================================

/// identifier of a global, dense: the `n`-th global defined gets `n`, so a
/// backend can store the globals in a vector
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlobalId(pub usize);

impl fmt::Display for GlobalId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "@{}", self.0)
    }
}

// ============================================================================
// Global
// ============================================================================

/// global value: `@id symbol : ty := init`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Global {
    pub id: GlobalId,
    /// name for debugging (`a__0`), never used to find the global
    pub symbol: String,
    pub ty: Type,
    /// value, computed once when the global is defined
    pub init: Expression,
}

impl Pretty for Global {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        Doc::nil()
            .append(theme.keyword(&"global"))
            .append(Doc::space())
            .append(theme.def_var(&self.id))
            .append(Doc::space())
            .append(theme.comment(&self.symbol))
            .append(Doc::space())
            .append(theme.operator(&":"))
            .append(Doc::space())
            .append(theme.ty_var(&self.ty.symbol()))
            .append(Doc::space())
            .append(theme.operator(&":="))
            .append(Doc::line().append(self.init.pretty(theme)).nest(2))
            .group()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lir::Constant;

    #[test]
    fn global_pretty() {
        let global = Global {
            id: GlobalId(0),
            symbol: "a__0".to_string(),
            ty: Type::Nat,
            init: Expression::Constant(Constant::Nat(3u8.into())),
        };
        let theme = Theme::default();
        assert_eq!(global.make_string(&theme), "global @0 a__0 : ℕ := 3");
    }
}
