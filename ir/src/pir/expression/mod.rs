use crate::location::{Span, Spanned};
use crate::pir::{Constant, Type, Variable, operator};
use crate::token::Token;
use parenthesis::Paren;
use pp::pretty::*;

pub mod constant;
pub mod definition;
pub mod parenthesis;
pub mod pattern;

// ============================================================================
// Expression
// ============================================================================

/// expression; the parser decides what may appear where, the precedence is
/// only used to print it back
#[derive(Debug, Clone)]
pub enum Expression {
    /// variable
    Variable(Variable),

    /// constant
    Constant(Constant),

    /// parenthesized expression
    Paren(Paren<Box<Expression>>),

    /// expression restricted to a type: `e : ty`
    Typed {
        expr: Box<Expression>,
        colon: Token<operator::Colon>,
        ty: Type,
    },
}

impl PrettyPrecedence for Expression {
    fn precedence(&self) -> u8 {
        match self {
            Expression::Variable(_) | Expression::Constant(_) => 0,
            Expression::Paren(paren) => paren.precedence(),
            Expression::Typed { .. } => 1,
        }
    }

    fn pretty_precedence(&self, prec: u8, theme: &Theme) -> Doc<'_> {
        match self {
            Expression::Variable(var) => var.pretty_with(theme, theme.expr_var(&var.value)),
            Expression::Constant(constant) => constant.pretty(theme),
            Expression::Paren(paren) => paren.pretty_precedence(prec, theme),
            Expression::Typed { expr, colon, ty } => Doc::nil()
                .append(expr.pretty_precedence(0, theme))
                .append(Doc::space())
                .append(colon.pretty(theme))
                .append(Doc::space())
                .append(ty.pretty(theme))
                .group(),
        }
    }
}

impl Spanned for Expression {
    fn span(&self) -> Span {
        match self {
            Expression::Variable(var) => var.span(),
            Expression::Constant(constant) => constant.span(),
            Expression::Paren(paren) => paren.span(),
            Expression::Typed { expr, ty, .. } => expr.span().union(ty.span()),
        }
    }
}
