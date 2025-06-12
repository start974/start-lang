use super::super::error::ErrorUnexpectedType;
use super::expression::{Expression, ExpressionDefinition};
use super::identifier::{Identifier, IdentifierBuilder};
use super::ty::{Ty, TyBuiltin, Typed};
use super::variable::VariableEnv;
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};
use std::collections::HashMap;
use std::rc::Rc;

// =======================================================================
// Program
// =======================================================================

pub struct Program {
    env: HashMap<Rc<Identifier>, ExpressionDefinition>,
    ordered_env: Vec<Rc<Identifier>>,
    main: Option<Expression>,
}

impl Program {
    fn push_definition(&mut self, def: ExpressionDefinition) {
        let identifier = Rc::new(def.name().clone());
        self.env.insert(identifier.clone(), def);
        self.ordered_env.push(identifier);
    }

    /// iter on environment (without main)
    pub fn iter(&self) -> impl Iterator<Item = &ExpressionDefinition> {
        self.ordered_env.iter().map(|id| self.env.get(id).unwrap())
    }

    /// get main expression definition
    pub fn get_main(&self) -> Option<&Expression> {
        self.main.as_ref()
    }
}

impl Pretty for Program {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        Doc::intersperse(
            self.iter().map(|item| item.pretty(theme)),
            Doc::line_().append(Doc::line_()),
        )
    }
}

// =======================================================================
// Program Builder
// =======================================================================
pub struct ProgramBuilder {
    program: Program,
    indent_builder: IdentifierBuilder,
}

impl ProgramBuilder {
    /// make a new program builder
    pub fn nil() -> Self {
        let program = Program {
            env: HashMap::new(),
            ordered_env: Vec::new(),
            main: None,
        };
        Self {
            program,
            indent_builder: IdentifierBuilder::nil(),
        }
    }

    /// add definition to program
    pub fn add_definition(&mut self, def: ExpressionDefinition) {
        let identifier = Rc::new(def.name().clone());
        self.program.env.insert(identifier, def);
    }

    /// set main expression
    pub fn set_main(&mut self, expr: Expression) -> Result<(), Box<ErrorUnexpectedType>> {
        let main_ty = Ty::Builtin(TyBuiltin::N);
        let expr = expr.constraint_ty(main_ty)?;
        self.program.main = Some(expr);
        Ok(())
    }

    /// build the program
    pub fn build(self) -> Program {
        self.program
    }
}

impl VariableEnv for ProgramBuilder {
    fn add_definition(&mut self, def: ExpressionDefinition) {
        self.program.push_definition(def);
    }

    fn get_ty(&self, identifier: &Identifier) -> Option<&Ty> {
        self.program
            .env
            .get(&Rc::new(identifier.clone()))
            .map(|def| def.ty())
    }
}
