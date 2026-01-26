use cst::{
    ExpressionDefinition,
    expression::{Expression, Expression0, Expression1},
    expression_definition::TypedBy,
    operator,
};

use super::*;
/// parse constant
/// ```ebfn
/// constant :=
/// | NUMBER
/// | CHARACTER
///```
pub fn constant() -> impl Parser<'static, TokenStream, cst::Constant, ExtraChumsky> {
    use cst::constant::{BuiltinT, CharacterT, Constant, NumberT};
    let number = select! {ref meta @ Meta{ value: TokenKind::Number(ref n), ..} =>
            meta.clone().map(|_| NumberT::from(n.clone()))
    }
    .map(Constant::from);

    let builtin = select! {
        ref meta @ Meta{ value: TokenKind::Identifier(ref b), ..} if b == "__Constant_true__" =>
            meta.clone().map(|_| BuiltinT::True),
        ref meta @ Meta{ value: TokenKind::Identifier(ref b), ..} if b == "__Constant_false__" =>
            meta.clone().map(|_| BuiltinT::False)
    }
    .map(Constant::from);

    let character = select! {ref meta @ Meta{ value: TokenKind::Character(ref c), ..} =>
        meta.clone().map(|_| CharacterT::from(*c))
    }
    .map(Constant::from);

    choice((builtin, number, character)).labelled("constant")
}

/// parse variable
/// ```ebfn
/// variable := IDENTIFIER
///```
pub fn variable() -> impl Parser<'static, TokenStream, cst::expression::Variable, ExtraChumsky> {
    use cst::expression::VariableT;
    select! {ref meta @ Meta{ value: TokenKind::Identifier(ref s), ..} =>
            meta.clone().map(|_| VariableT::from(s.clone()))
    }
}

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

/// parse expression definition
/// ```ebfn
/// expr_definition := pattern (colon type)? EQ_DEF expression
///```
pub fn expression_definition()
-> impl Parser<'static, TokenStream, ExpressionDefinition, ExtraChumsky> {
    let typed_by = {
        let colon = operator(Operator::Colon, operator::ColonT()).labelled(":");
        colon.then(ty()).map(|(colon, ty)| TypedBy { colon, ty })
    };
    let eq_def = operator(Operator::EqDef, operator::EqDefT()).labelled(":=");
    pattern()
        .then(typed_by.or_not())
        .then(eq_def)
        .then(expression())
        .map(
            |(((pattern, typed_by), eq_def), body)| ExpressionDefinition {
                pattern,
                typed_by,
                eq_def,
                body,
            },
        )
}
