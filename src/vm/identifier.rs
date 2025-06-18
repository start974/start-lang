use crate::typing::ast;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    name: String,
    id: usize,
}

impl From<&ast::Identifier> for Identifier {
    fn from(value: &ast::Identifier) -> Self {
        Identifier {
            name: value.name().to_string(),
            id: value.id(),
        }
    }
}

impl From<&ast::Variable> for Identifier {
    fn from(value: &ast::Variable) -> Self {
        Self::from(value.identifier())
    }
}
