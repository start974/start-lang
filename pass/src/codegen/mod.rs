//! `tir` → `lir`: every definition becomes a global with a dense
//! `GlobalId`, type aliases are resolved and spans are dropped.
//!
//! The state is kept between calls, so the REPL can generate one command at a
//! time.

use crate::{Checked, CheckedResult, Partial};
use errors::{Error, Message};
use ir::tir::Typed as _;
use ir::{lir, tir};
use std::collections::HashMap;

mod error;

// ============================================================================
// Codegen
// ============================================================================

/// generates the files one after the other: the globals of the previous
/// files (stdlib, REPL inputs) are kept
#[derive(Debug, Default, Clone)]
pub struct Codegen {
    /// global of every generated definition
    globals: HashMap<tir::Identifier, lir::GlobalId>,
}

impl Codegen {
    pub fn new() -> Self {
        Self::default()
    }

    /// code of a typed file: a global per definition, an item per `Eval`;
    /// the items it could generate, and every (internal) error
    pub fn run(&mut self, file: &Checked<tir::File>) -> CheckedResult<lir::File> {
        let items: Partial<Vec<_>> = file
            .commands()
            .iter()
            .filter_map(|cmd| match &cmd.kind {
                tir::CommandKind::ExpressionDefinition(def) => {
                    Some(self.definition(def).map(lir::Item::Global))
                }
                tir::CommandKind::Eval(expr) => Some(self.expression(expr).map(lir::Item::Eval)),
                tir::CommandKind::TypeOf(_) | tir::CommandKind::Help(_) => None,
            })
            .map(|item| match item {
                Ok(item) => Partial::ok(Some(item)),
                Err(err) => Partial::ok(None)
                    .with_error(err.with_recovery(Message::text("the command is left out."))),
            })
            .collect();
        items
            .map(|items| items.into_iter().flatten().collect())
            .check()
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
                .ok_or_else(|| error::unknown_identifier(var)),
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

#[cfg(test)]
mod tests {
    use super::*;
    use ir::location::Span;
    use ir::sir::DefId;
    use ir::tir::{Env, Identifier, PatternVar};
    use std::rc::Rc;

    fn ident(name: &str, id: usize) -> Rc<Identifier> {
        Rc::new(Identifier {
            name: name.to_string(),
            id: DefId(id),
        })
    }

    /// `a := 3` then `b := a`, through the typing environment
    #[test]
    fn definitions_become_dense_globals() {
        let mut env = Env::default();
        let mut codegen = Codegen::default();
        let nat = tir::Type::Builtin(tir::ty::Builtin::nat());

        let a = ident("a", 0);
        let def_a = tir::ExpressionDefinition::new(
            tir::Pattern::Variable(PatternVar::from(a.clone())),
            tir::Expression::Constant(tir::Constant::nat(3u8.into())),
        );
        let global_a = codegen.definition(&def_a).unwrap();
        env.add_expr_def(a.clone(), nat, Span::default());

        let b = ident("b", 1);
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
        let mut env = Env::default();
        let a = ident("a", 0);
        env.add_expr_def(
            a.clone(),
            tir::Type::Builtin(tir::ty::Builtin::bool()),
            Span::default(),
        );
        let use_a = tir::Expression::Variable(env.expr_var(&a, Span::default()).unwrap());

        let codegen = Codegen::default();
        assert_eq!(
            codegen.expression(&use_a).map_err(|err| err.code()),
            Err(901)
        );
    }

    #[test]
    fn alias_is_resolved() {
        let mut env = Env::default();
        let t = ident("T", 0);
        env.add_type_def(
            t.clone(),
            tir::Type::Builtin(tir::ty::Builtin::char()),
            Span::default(),
        );
        let alias = tir::Type::Alias(env.type_alias(&t, Span::default()).unwrap());
        assert_eq!(Codegen::ty(&alias), lir::Type::Char);
    }
}
