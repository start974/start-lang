use crate::error;
use crate::{Documentation, Identifier, Type, Typed};
use errors::Error;
use location::{GetSpan, SetSpan, Span};
use pp::pretty::*;
use std::collections::HashMap;
use std::rc::Rc;

// ==========================================================================
// Variable
// ==========================================================================

pub struct Variable {
    /// identifier of the variable
    identifier: Identifier,
    /// type of the variable
    ty: Type,
    /// span of varable
    span: Span,
}

impl Variable {
    /// get the identifier of the variable
    pub fn identifier(&self) -> &Identifier {
        &self.identifier
    }
}

mod sealed_mut_ty {
    use crate::TypedMut;

    use super::*;
    impl TypedMut for Variable {
        fn ty_mut(&mut self) -> &mut Type {
            &mut self.ty
        }
    }
}

impl Typed for Variable {
    fn ty(&self) -> &Type {
        &self.ty
    }
}

impl GetSpan for Variable {
    fn span(&self) -> Span {
        self.span
    }
}

impl SetSpan for Variable {
    fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}

impl Pretty for Variable {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        theme.expr_var(&self.identifier)
    }
}

// ==========================================================================
// alias Ty
// ==========================================================================
#[derive(Debug, Clone)]
pub struct Alias {
    /// name of alias
    name: Identifier,
    /// type of alias
    ty: Box<Type>,
    /// span of alias
    span: Span,
}

impl Typed for Alias {
    fn ty(&self) -> &Type {
        &self.ty
    }
}

impl Pretty for Alias {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        theme.ty_var(&self.name.name())
    }
}

impl GetSpan for Alias {
    fn span(&self) -> Span {
        self.span
    }
}

impl SetSpan for Alias {
    fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}
// ==========================================================================
// Identifier Kind
// ==========================================================================
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum IdentifierKind {
    /// identifier is a type
    Type,
    /// identifier is a expression variable
    Expr,
    /// unknown identifier kind
    Unknown,
}

// ==========================================================================
// Identifier Information
// ==========================================================================
/// information about identifier
/// with store documentation, location of definition and location of references
#[derive(Debug)]
pub struct IdentifierInfo {
    /// identifier
    pub id: Rc<Identifier>,
    /// documentation
    pub doc: Option<Documentation>,
    /// kind of identifier
    pub kind: IdentifierKind,
    /// type of indentifier
    pub ty: Type,
    /// definition span
    pub span_def: Span,
    /// references spans
    pub span_refs: Vec<Span>,
}

// ==========================================================================
// Help Variable
// ==========================================================================

#[derive(Debug)]
pub struct Help {
    id: Rc<Identifier>,
    ty: Type,
    span: Span,
    kind: IdentifierKind,
    doc: Option<Documentation>,
}

impl Pretty for Help {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        let doc_ty = match &self.kind {
            IdentifierKind::Expr => Doc::nil()
                .append(theme.operator(&":"))
                .append(Doc::softline())
                .append(self.ty.pretty(theme).group()),
            IdentifierKind::Type => match &self.ty {
                Type::Builtin(_) => theme.comment(&"(builtin)"),
                ty => Doc::nil()
                    .append(theme.operator(&":="))
                    .append(Doc::softline())
                    .append(ty.pretty(theme).group()),
            },
            IdentifierKind::Unknown => Doc::nil(),
        };

        let documentation = match &self.doc {
            Some(doc) => Doc::hardline().append(doc.pretty(theme)).nest(2),
            None => Doc::nil(),
        };
        theme
            .expr_var(&self.id.name())
            .append(Doc::space())
            .append(doc_ty)
            .append(documentation)
    }
}

impl GetSpan for Help {
    fn span(&self) -> Span {
        self.span
    }
}
// ==========================================================================
// Identifier table
// ==========================================================================
#[derive(Debug, Default)]
pub struct Env {
    /// map of identifiers
    table: HashMap<Rc<Identifier>, IdentifierInfo>,
}

impl Env {
    fn add(&mut self, id: Rc<Identifier>, span_def: Span, ty: Type, kind: IdentifierKind) {
        let info = IdentifierInfo {
            id: id.clone(),
            doc: None,
            kind,
            ty,
            span_def,
            span_refs: Vec::new(),
        };
        self.table.insert(id, info);
    }

    /// add expression variable definition
    pub fn add_expr_def(&mut self, id: Rc<Identifier>, ty: Type, span_def: Span) {
        self.add(id, span_def, ty, IdentifierKind::Expr);
    }

    /// add type definition
    pub fn add_type_def(&mut self, id: Rc<Identifier>, ty: Type, span_def: Span) {
        self.add(id, span_def, ty, IdentifierKind::Type);
    }

    /// set documentation for identifier
    pub fn set_doc(&mut self, id: &Identifier, doc: Documentation) {
        if let Some(info) = self.table.get_mut(id) {
            info.doc = Some(doc);
        }
    }

    /// get variable by identifier
    pub fn get_expr_var(&mut self, id: &Identifier, span: Span) -> Result<Variable, Error> {
        match self.table.get_mut(id) {
            Some(info) if info.kind == IdentifierKind::Expr => {
                info.span_refs.push(span);
                Ok(Variable {
                    identifier: id.clone(),
                    ty: info.ty.clone(),
                    span,
                })
            }
            _ => Err(error::variable_not_found(
                id.clone(),
                IdentifierKind::Expr,
                span,
            )),
        }
    }

    /// get type alias
    pub fn get_alias_ty(&mut self, id: &Identifier, span: Span) -> Result<Alias, Error> {
        match self.table.get_mut(id) {
            Some(info) if info.kind == IdentifierKind::Type => {
                info.span_refs.push(span);
                Ok(Alias {
                    name: id.clone(),
                    ty: Box::new(info.ty.clone()),
                    span,
                })
            }
            _ => Err(error::variable_not_found(
                id.clone(),
                IdentifierKind::Type,
                span,
            )),
        }
    }

    pub fn get_help(&mut self, id: &Identifier, span: Span) -> Result<Help, Error> {
        match self.table.get(id) {
            Some(info) => Ok(Help {
                id: info.id.clone(),
                ty: info.ty.clone(),
                span,
                kind: info.kind,
                doc: info.doc.clone(),
            }),
            None => Err(error::variable_not_found(
                id.clone(),
                IdentifierKind::Unknown,
                span,
            )),
        }
    }

    /// iternate over all identifiers
    pub fn iter(&self) -> impl Iterator<Item = &IdentifierInfo> {
        self.table.values()
    }
}
