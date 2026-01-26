use super::{Constant, Type, operator, parenthesis::Parenthesed};
use crate::{AsIdentifier, Meta};
use location::{GetSpan, Span};
use pp::pretty::*;

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

impl Pretty for Expression0 {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Expression0::Variable(var) => var.pretty(theme),
            Expression0::Constant(constant) => constant.pretty(theme),
            Expression0::Paren(parent) => parent.pretty(theme),
        }
    }
}

impl GetSpan for Expression0 {
    fn span(&self) -> Span {
        match self {
            Expression0::Variable(var) => var.span(),
            Expression0::Constant(constant) => constant.span(),
            Expression0::Paren(parent) => parent.span(),
        }
    }
}

impl Pretty for Expression1 {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Expression1::TypedExpression { expr, colon, ty } => Doc::nil()
                .append(expr.pretty(theme))
                .append(Doc::space())
                .append(colon.pretty(theme))
                .append(Doc::space())
                .append(ty.pretty(theme))
                .group(),
            Expression1::Expression0(expr) => expr.pretty(theme),
        }
    }
}

impl GetSpan for Expression1 {
    fn span(&self) -> Span {
        match self {
            Expression1::TypedExpression { expr, ty, .. } => expr.span().union(ty.span()),
            Expression1::Expression0(expr) => expr.span(),
        }
    }
}
