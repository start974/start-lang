use super::ast::{Documentation, Identifier, Type, Typed};
use crate::typer::error::ErrorVariableNotFound;
use crate::utils::location::{Located, LocatedSet, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};
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
    /// location of varable
    loc: Location,
}

impl Variable {
    /// get the identifier of the variable
    pub fn identifier(&self) -> &Identifier {
        &self.identifier
    }
}

mod sealed_mut_ty {
    use crate::typer::ast::TypedMut;

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

impl Located for Variable {
    fn loc(&self) -> Location {
        self.loc.clone()
    }
}

impl LocatedSet for Variable {
    fn set_loc(&mut self, loc: &impl Located) {
        self.loc = loc.loc().clone();
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
    /// location of alias
    loc: Location,
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

impl Located for Alias {
    fn loc(&self) -> Location {
        self.loc.clone()
    }
}

impl LocatedSet for Alias {
    fn set_loc(&mut self, loc: &impl Located) {
        self.loc = loc.loc();
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
    /// definition location
    pub loc_def: Location,
    /// location of references
    pub loc_refs: Vec<Location>,
}

// ==========================================================================
// Help Variable
// ==========================================================================

#[derive(Debug)]
pub struct Help {
    id: Rc<Identifier>,
    ty: Type,
    loc: Location,
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
            IdentifierKind::Type => {
                match &self.ty {
                    Type::Builtin(_) => theme.comment(&"(builtin)"),
                    ty => Doc::nil()
                        .append(theme.operator(&":="))
                        .append(Doc::softline())
                        .append(ty.pretty(theme).group()),
                }
            }
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

impl Located for Help {
    fn loc(&self) -> Location {
        self.loc.clone()
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
    fn add(&mut self, id: Rc<Identifier>, loc_def: Location, ty: Type, kind: IdentifierKind) {
        let info = IdentifierInfo {
            id: id.clone(),
            doc: None,
            kind,
            ty,
            loc_def,
            loc_refs: Vec::new(),
        };
        self.table.insert(id, info);
    }

    /// add expression variable definition
    pub fn add_expr_def(&mut self, id: Rc<Identifier>, ty: Type, loc_def: Location) {
        self.add(id, loc_def, ty, IdentifierKind::Expr);
    }

    /// add type definition
    pub fn add_type_def(&mut self, id: Rc<Identifier>, ty: Type, loc_def: Location) {
        self.add(id, loc_def, ty, IdentifierKind::Type);
    }

    /// set documentation for identifier
    pub fn set_doc(&mut self, id: &Identifier, doc: Documentation) {
        if let Some(info) = self.table.get_mut(id) {
            info.doc = Some(doc);
        }
    }

    /// get variable by identifier
    pub fn get_expr_var(
        &mut self,
        id: &Identifier,
        loc: Location,
    ) -> Result<Variable, ErrorVariableNotFound> {
        match self.table.get_mut(id) {
            Some(info) if info.kind == IdentifierKind::Expr => {
                info.loc_refs.push(loc.clone());
                Ok(Variable {
                    identifier: id.clone(),
                    ty: info.ty.clone(),
                    loc,
                })
            }
            _ => Err(ErrorVariableNotFound::new(
                id.clone(),
                Some(IdentifierKind::Expr),
                loc,
            )),
        }
    }

    /// get type alias
    pub fn get_alias_ty(
        &mut self,
        id: &Identifier,
        loc: Location,
    ) -> Result<Alias, ErrorVariableNotFound> {
        match self.table.get_mut(id) {
            Some(info) if info.kind == IdentifierKind::Type => {
                info.loc_refs.push(loc.clone());
                Ok(Alias {
                    name: id.clone(),
                    ty: Box::new(info.ty.clone()),
                    loc,
                })
            }
            _ => Err(ErrorVariableNotFound::new(
                id.clone(),
                Some(IdentifierKind::Type),
                loc,
            )),
        }
    }

    pub fn get_help(
        &mut self,
        id: &Identifier,
        loc: Location,
    ) -> Result<Help, ErrorVariableNotFound> {
        match self.table.get(id) {
            Some(info) => {
                Ok(Help {
                    id: info.id.clone(),
                    ty: info.ty.clone(),
                    loc,
                    kind: info.kind,
                    doc: info.doc.clone(),
                })
            }
            None => Err(ErrorVariableNotFound::new(
                id.clone(),
                None,
                loc,
            )),
        }
    }

    /// iternate over all identifiers
    pub fn iter(&self) -> impl Iterator<Item = &IdentifierInfo> {
        self.table.values()
    }
}
