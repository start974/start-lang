use super::parenthesis::PrettyPrecedence;
use super::AsIdentifier;
use super::{operator, parenthesis::Parenthesed, Constant, Type};
use crate::lexer::meta::Meta;
use crate::utils::location::{Located, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

// ============================================================================
// Variable
// ============================================================================
#[derive(Debug, Clone)]
pub struct VariableT(String);
pub type Variable = Meta<VariableT>;

impl From<String> for VariableT {
    fn from(name: String) -> Self {
        Self(name)
    }
}

impl AsIdentifier for VariableT {
    /// get name of variable
    fn name(&self) -> &str {
        &self.0
    }
}

impl Pretty for VariableT {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        theme.expr_var(&self.0)
    }
}

// ============================================================================
// Expression
// ============================================================================

#[derive(Debug, Clone)]
pub enum Expression0 {
    /// variable
    Variable(Variable),

    /// constant
    Constant(Constant),

    /// parenthesized expression
    Paren(Parenthesed<operator::LParenT, Box<Expression>, operator::RParenT>),
}

#[derive(Debug, Clone)]
pub enum Expression1 {
    TypedExpression {
        expr: Expression0,
        colon: operator::Colon,
        ty: Type,
    },
    Expression0(Expression0),
}

pub type Expression = Expression1;

impl PrettyPrecedence for Expression0 {
    fn precedence(&self) -> u8 {
        0
    }

    fn pretty_precedence(&self, prec: u8, theme: &Theme) -> Doc<'_> {
        match self {
            Expression0::Variable(var) => var.pretty(theme),
            Expression0::Constant(constant) => constant.pretty(theme),
            Expression0::Paren(parent) => parent.pretty_precedence(prec, theme),
        }
    }
}

impl Located for Expression0 {
    fn loc(&self) -> Location {
        match self {
            Expression0::Variable(var) => var.loc(),
            Expression0::Constant(constant) => constant.loc(),
            Expression0::Paren(parent) => parent.loc(),
        }
    }
}

impl PrettyPrecedence for Expression1 {
    fn precedence(&self) -> u8 {
        1
    }

    fn pretty_precedence(&self, prec: u8, theme: &Theme) -> Doc<'_> {
        match self {
            Expression1::TypedExpression { expr, colon, ty } => Doc::nil()
                .append(expr.pretty(theme))
                .append(Doc::space())
                .append(colon.pretty(theme))
                .append(Doc::space())
                .append(ty.pretty(theme))
                .group(),
            Expression1::Expression0(expr) => expr.pretty_precedence(prec, theme),
        }
    }
}

impl Located for Expression1 {
    fn loc(&self) -> Location {
        match self {
            Expression1::TypedExpression { expr, ty, .. } => expr.loc().union(ty.loc()),
            Expression1::Expression0(expr) => expr.loc(),
        }
    }
}
