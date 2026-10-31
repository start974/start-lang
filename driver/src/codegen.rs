//! `tir` → `lir`: every definition becomes a global with a dense
//! `GlobalId`, type aliases are resolved and spans are dropped.
//!
//! The state is kept between calls, so the REPL can generate one command at a
//! time.

use ir::tir::Typed as _;
use ir::{lir, tir};
use std::collections::HashMap;
use std::fmt;

// ============================================================================
// Error
// ============================================================================

/// internal error: `typing` accepted a program that cannot be generated
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// variable whose definition was never generated
    UnknownIdentifier(tir::Identifier),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::UnknownIdentifier(id) => write!(f, "codegen: unknown identifier `{id}`"),
        }
    }
}

impl std::error::Error for Error {}

/// diagnostic of an internal error: `typing` accepted the program, so this is
/// a bug of the compiler
impl From<Error> for errors::Error {
    fn from(err: Error) -> Self {
        errors::Error::new(901, errors::Message::text("Internal error."))
            .with_text(errors::Message::text(err.to_string()))
    }
}

/// so that `?` reports it from a pass that returns `Errors`
impl From<Error> for errors::Errors {
    fn from(err: Error) -> Self {
        errors::Error::from(err).into()
    }
}

// ============================================================================
// Codegen
// ============================================================================

/// generates the files one after the other: the globals of the previous
/// files (stdlib, REPL inputs) are kept
#[derive(Debug, Default)]
pub struct Codegen {
    /// global of every generated definition
    globals: HashMap<tir::Identifier, lir::GlobalId>,
}

impl Codegen {
    pub fn builder() -> CodegenBuilder {
        CodegenBuilder
    }

    /// code of a typed file: a global per definition, an item per `Eval`
    pub fn run(&mut self, file: &tir::File) -> Result<lir::File, errors::Errors> {
        let mut items = Vec::new();
        for cmd in file.commands() {
            match &cmd.kind {
                tir::CommandKind::ExpressionDefinition(def) => {
                    items.push(lir::Item::Global(self.definition(def)?));
                }
                tir::CommandKind::Eval(expr) => items.push(lir::Item::Eval(self.expression(expr)?)),
                tir::CommandKind::TypeOf(_) | tir::CommandKind::Help(_) => {}
            }
        }
        Ok(items.into_iter().collect())
    }

    /// run-time type: aliases resolved
    pub fn ty(ty: &tir::Type) -> lir::Type {
        match ty {
            tir::Type::Builtin(builtin) => builtin.kind(),
            tir::Type::Alias(alias) => Self::ty(alias.ty()),
        }
    }

    /// constant
    pub fn constant(constant: &tir::Constant) -> lir::Constant {
        match constant.kind() {
            tir::ConstantKind::Nat(n) => lir::Constant::Nat(n.clone()),
            tir::ConstantKind::Bool(b) => lir::Constant::Bool(*b),
            tir::ConstantKind::Char(c) => lir::Constant::Char(*c),
        }
    }

    /// expression
    pub fn expression(&self, expr: &tir::Expression) -> Result<lir::Expression, Error> {
        match expr {
            tir::Expression::Constant(c) => Ok(lir::Expression::Constant(Self::constant(c))),
            tir::Expression::Variable(var) => self
                .globals
                .get(var.identifier())
                .map(|id| lir::Expression::Global(*id))
                .ok_or_else(|| Error::UnknownIdentifier(var.identifier().clone())),
        }
    }

    /// definition: a new global
    pub fn definition(&mut self, def: &tir::ExpressionDefinition) -> Result<lir::Global, Error> {
        let init = self.expression(def.body())?;
        match def.pattern() {
            tir::Pattern::Variable(var) => {
                let id = lir::GlobalId::new(self.globals.len());
                self.globals.insert(var.identifier().clone(), id);
                Ok(lir::Global {
                    id,
                    symbol: var.identifier().to_string(),
                    ty: Self::ty(def.ty()),
                    init,
                })
            }
        }
    }
}

/// arguments of a `Codegen` (none yet)
#[derive(Debug, Default, Clone, Copy)]
pub struct CodegenBuilder;

impl CodegenBuilder {
    pub fn build(self) -> Codegen {
        Codegen::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ir::location::Span;
    use ir::tir::{Env, IdentifierBuilder, PatternVar};

    /// `a := 3` then `b := a`, through the typing environment
    #[test]
    fn definitions_become_dense_globals() {
        let mut ids = IdentifierBuilder::default();
        let mut env = Env::default();
        let mut codegen = Codegen::default();
        let nat = tir::Type::Builtin(tir::ty::Builtin::nat());

        let a = ids.build("a");
        let def_a = tir::ExpressionDefinition::new(
            tir::Pattern::Variable(PatternVar::from(a.clone())),
            tir::Expression::Constant(tir::Constant::nat(3u8.into())),
        );
        let global_a = codegen.definition(&def_a).unwrap();
        env.add_expr_def(a.clone(), nat, Span::default());

        let b = ids.build("b");
        let use_a = env.expr_var(&a, Span::default()).unwrap();
        let def_b = tir::ExpressionDefinition::new(
            tir::Pattern::Variable(PatternVar::from(b)),
            tir::Expression::Variable(use_a),
        );
        let global_b = codegen.definition(&def_b).unwrap();

        assert_eq!(global_a.id, lir::GlobalId::new(0));
        assert_eq!(global_a.symbol, "a__0");
        assert_eq!(global_a.ty, lir::Type::Nat);
        assert_eq!(
            global_a.init,
            lir::Expression::Constant(lir::Constant::Nat(3u8.into()))
        );
        assert_eq!(global_b.id, lir::GlobalId::new(1));
        assert_eq!(global_b.init, lir::Expression::Global(global_a.id));
    }

    #[test]
    fn unknown_identifier_is_an_error() {
        let mut ids = IdentifierBuilder::default();
        let mut env = Env::default();
        let a = ids.build("a");
        env.add_expr_def(
            a.clone(),
            tir::Type::Builtin(tir::ty::Builtin::bool()),
            Span::default(),
        );
        let use_a = tir::Expression::Variable(env.expr_var(&a, Span::default()).unwrap());

        let codegen = Codegen::default();
        assert_eq!(
            codegen.expression(&use_a),
            Err(Error::UnknownIdentifier((*a).clone()))
        );
    }

    #[test]
    fn alias_is_resolved() {
        let mut ids = IdentifierBuilder::default();
        let mut env = Env::default();
        let t = ids.build("T");
        env.add_type_def(
            t.clone(),
            tir::Type::Builtin(tir::ty::Builtin::char()),
            Span::default(),
        );
        let alias = tir::Type::Alias(env.type_alias(&t, Span::default()).unwrap());
        assert_eq!(Codegen::ty(&alias), lir::Type::Char);
    }

    #[test]
    fn internal_error_diagnostic() {
        let mut ids = IdentifierBuilder::default();
        let err = Error::UnknownIdentifier((*ids.build("a")).clone());
        let diagnostic = errors::Error::from(err.clone());
        assert_eq!(diagnostic.code(), 901);
        let errors = errors::Errors::from(err);
        assert_eq!(errors.code(), 901);
    }
}
