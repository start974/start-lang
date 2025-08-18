use crate::typer::ast::Identifier;
use crate::utils::location::{Located, LocatedSet, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

// ==========================================================================
// Pattern Variable
// ==========================================================================
pub struct PatternVar {
    /// identifier of the variable
    id: Identifier,
    /// location of the pattern
    loc: Location,
}

impl PatternVar {
    /// get identifier
    pub fn identifier(&self) -> &Identifier {
        &self.id
    }

}

impl From<Identifier> for PatternVar {
    fn from(id: Identifier) -> Self {
        Self {
            id,
            loc: Location::unknown(),
        }
    }
}

impl Pretty for PatternVar {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        theme.def_var(&self.id)
    }
}

impl Located for PatternVar {
    fn loc(&self) -> Location {
        self.loc.clone()
    }
}

impl LocatedSet for PatternVar {
    fn set_loc(&mut self, loc: &impl Located) {
        self.loc = loc.loc().clone();
    }
}

// ==========================================================================
// Pattern
// ==========================================================================
pub enum Pattern {
    Variable(PatternVar),
}

impl Pattern{
    /// get names on patterns
    pub fn names(&self) -> impl Iterator<Item = &Identifier> {
        match self {
            Pattern::Variable(var) => std::iter::once(&var.id),
        }
    }
}

impl Pretty for Pattern {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Pattern::Variable(var) => var.pretty(theme),
        }
    }
}

impl Located for Pattern {
    fn loc(&self) -> Location {
        match self {
            Pattern::Variable(var) => var.loc(),
        }
    }
}
