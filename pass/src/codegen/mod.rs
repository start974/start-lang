//! `tir` → `lir`: every definition becomes a global with a dense
//! `GlobalId`, type aliases are resolved and spans are dropped.
//!
//! The state is kept between calls, so the REPL can generate one command at a
//! time.

use crate::{Checked, CheckedResult, Partial};
use errors::{Errors, Message};
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
    globals: HashMap<tir::DefId, lir::GlobalId>,
    /// print the `lir`; set again by `Stdlib::debug` once the stdlib is loaded
    pub(crate) debug: bool,
}

/// arguments of a `Codegen`
#[derive(Debug, Default, Clone, Copy)]
pub struct CodegenBuilder {
    debug: bool,
}

impl CodegenBuilder {
    /// print the `lir`
    pub fn debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }

    pub fn build(self) -> Codegen {
        Codegen {
            debug: self.debug,
            ..Codegen::default()
        }
    }
}

impl Codegen {
    pub fn builder() -> CodegenBuilder {
        CodegenBuilder::default()
    }

    /// code of a typed file: a global per definition, in order; the
    /// globals it could generate, and every (internal) error; the commands
    /// are generated apart (`expression`)
    pub fn run(&mut self, file: &Checked<tir::File>) -> CheckedResult<lir::File> {
        let globals: Partial<Vec<_>> = file
            .commands
            .iter()
            .filter_map(|cmd| match &cmd.kind {
                tir::CommandKind::ExpressionDefinition(def) => Some(self.definition(def)),
                tir::CommandKind::Eval(_)
                | tir::CommandKind::TypeOf(_)
                | tir::CommandKind::Help(_) => None,
            })
            .map(|global| match global {
                Ok(global) => Partial::ok(Some(global)),
                Err(errs) => Partial::ok(None).with_errors(
                    errs.into_iter()
                        .map(|err| err.with_recovery(Message::text("the command is left out"))),
                ),
            })
            .collect();
        globals
            .map(|globals| lir::File {
                globals: globals.into_iter().flatten().collect(),
            })
            .check_debug(self.debug)
    }

    /// constant
    pub fn constant(constant: &tir::Constant) -> lir::Constant {
        match constant {
            tir::Constant::Nat(n) => lir::Constant::Nat(n.clone()),
            tir::Constant::Bool(b) => lir::Constant::Bool(*b),
            tir::Constant::Char(c) => lir::Constant::Char(*c),
        }
    }

    /// expression
    pub fn expression(&self, expr: &tir::Expression) -> Result<lir::Expression, Errors> {
        match &expr.kind {
            tir::ExpressionKind::Constant(c) => Ok(lir::Expression::Constant(Self::constant(c))),
            tir::ExpressionKind::Variable(var) => self
                .globals
                .get(&var.value.id)
                .map(|id| lir::Expression::Global(*id))
                .ok_or_else(|| error::unknown_identifier(var).into()),
        }
    }

    /// definition: a new global
    pub fn definition(&mut self, def: &tir::ExpressionDefinition) -> Result<lir::Global, Errors> {
        let init = self.expression(&def.body)?;
        let tir::Pattern::Variable(var) = &def.pattern;
        let id = lir::GlobalId(self.globals.len());
        self.globals.insert(var.value.id, id);
        Ok(lir::Global {
            id,
            symbol: var.value.to_string(),
            ty: def.body.ty.builtin(),
            init,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ir::location::Span;
    use ir::tir::ty::BuiltinType;

    fn var(name: &str, id: usize) -> tir::Node<tir::Identifier> {
        tir::Node {
            value: tir::Identifier {
                name: name.to_string(),
                id: tir::DefId(id),
            },
            span: Span::default(),
        }
    }

    fn expr(kind: tir::ExpressionKind) -> tir::Expression {
        tir::Expression {
            kind,
            ty: tir::Type::Builtin(BuiltinType::Nat),
            span: Span::default(),
        }
    }

    /// `a := 3` then `b := a`
    #[test]
    fn definitions_become_dense_globals() {
        let mut codegen = Codegen::default();
        let def_a = tir::ExpressionDefinition {
            pattern: tir::Pattern::Variable(var("a", 0)),
            body: expr(tir::ExpressionKind::Constant(tir::Constant::Nat(
                3u8.into(),
            ))),
        };
        let global_a = codegen.definition(&def_a).unwrap();
        let def_b = tir::ExpressionDefinition {
            pattern: tir::Pattern::Variable(var("b", 1)),
            body: expr(tir::ExpressionKind::Variable(var("a", 0))),
        };
        let global_b = codegen.definition(&def_b).unwrap();

        assert_eq!(global_a.id, lir::GlobalId(0));
        assert_eq!(global_a.symbol, "a@0");
        assert_eq!(global_a.ty, lir::Type::Nat);
        assert_eq!(
            global_a.init,
            lir::Expression::Constant(lir::Constant::Nat(3u8.into()))
        );
        assert_eq!(global_b.id, lir::GlobalId(1));
        assert_eq!(global_b.init, lir::Expression::Global(global_a.id));
    }

    #[test]
    fn unknown_identifier_is_an_error() {
        let use_a = expr(tir::ExpressionKind::Variable(var("a", 0)));
        assert_eq!(
            Codegen::default()
                .expression(&use_a)
                .map_err(|err| err.code()),
            Err(901)
        );
    }

    #[test]
    fn alias_is_resolved() {
        let mut codegen = Codegen::default();
        let def = tir::ExpressionDefinition {
            pattern: tir::Pattern::Variable(var("c", 1)),
            body: tir::Expression {
                kind: tir::ExpressionKind::Constant(tir::Constant::Char('x')),
                ty: tir::Type::Alias {
                    name: var("T", 0).value,
                    ty: Box::new(tir::Type::Builtin(BuiltinType::Char)),
                },
                span: Span::default(),
            },
        };
        assert_eq!(codegen.definition(&def).unwrap().ty, lir::Type::Char);
    }
}
