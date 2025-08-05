use crate::typer::ast;

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
