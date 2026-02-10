use cst::{
    ExpressionDefinition,
    expression::{Expression, Expression0, Expression1},
    expression_definition::TypedBy,
    operator,
};

pub use super::*;

mod constant;
mod definition;
mod variable;

pub use constant::constant;
pub use definition::definition as expression_definition;
pub use variable::variable;

/// parse expression0
/// ```ebfn
/// expression0 :=
/// | constant
/// | variable
/// | "(" expression ")"
///```
fn expression0(
    expr: impl Parser<'static, TokenStream, Expression, ExtraChumsky>,
) -> impl Parser<'static, TokenStream, Expression0, ExtraChumsky> {
    use cst::parenthesis::Parenthesed;

    let constant = constant().map(Expression0::Constant);
    let variable = variable().map(Expression0::Variable);
    let parens = {
        let l_paren = operator(Operator::LParen, operator::LParenT()).labelled("(");
        let r_paren = operator(Operator::RParen, operator::RParenT()).labelled(")");
        l_paren
            .then(expr.map(Box::new))
            .then(r_paren)
            .map(|((l_paren, expr), r_paren)| Parenthesed::new(l_paren, expr, r_paren))
            .map(Expression0::Paren)
    };

    choice((constant, variable, parens))
}

/// parse expression1
/// ```ebfn
/// expr@1 :=
/// | expr@0 COLON ty
/// | expr@0
///```
fn expression1(
    expr0: impl Parser<'static, TokenStream, Expression0, ExtraChumsky> + Clone,
) -> impl Parser<'static, TokenStream, Expression1, ExtraChumsky> {
    use cst::expression::Expression1;

    let colon = operator(Operator::Colon, operator::ColonT()).labelled(":");
    let type_restriction = (expr0.clone())
        .then(colon)
        .then(ty())
        .map(|((expr, colon), ty)| Expression1::TypedExpression { expr, colon, ty });

    let expr0_in1 = expr0.map(Expression1::Expression0);

    choice((type_restriction, expr0_in1))
}
/// parse expression
/// ```ebfn
/// expr0 :=
/// | variable
/// | constant
/// | "(" expr@1 ")"
///
/// expr@1 :=
/// | expr@0 colon ty
/// | expr@0
///```
pub fn expression() -> impl Parser<'static, TokenStream, Expression, ExtraChumsky> {
    recursive(move |expr| {
        let expr0 = expression0(expr.clone()).boxed();
        expression1(expr0).boxed()
    })
    .labelled("expression")
}
