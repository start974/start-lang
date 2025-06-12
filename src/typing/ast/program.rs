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
    /// make program
    pub fn new() -> Self {
        Self {
            env: HashMap::new(),
            ordered_env: Vec::new(),
            main: None,
        }
    }

    /// push a new definition
    /// return none if definition already exist
    pub fn with_definition(mut self, def: ExpressionDefinition) -> Option<Self> {
        let identifier = Rc::new(def.name().clone());
        match self.env.insert(identifier.clone(), def) {
            None => {
                self.ordered_env.push(identifier);
                Some(self)
            }
            Some(_) => None,
        }
    }

    /// set main expression
    pub fn set_main(&mut self, expr: Expression) -> Result<(), Box<ErrorUnexpectedType>> {
        let main_ty = Ty::Builtin(TyBuiltin::N);
        let expr = expr.constraint_ty(main_ty)?;
        self.main = Some(expr);
        Ok(())
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
