use cst::operator::{ColonT, EqDefT};

use super::*;

/// parse expression definition
/// ```ebfn
/// expr_definition := pattern (colon type)? EQ_DEF expression
///```
pub fn definition() -> impl Parser<'static, TokenStream, ExpressionDefinition, ExtraChumsky> {
    let typed_by = {
        let colon = operator(Operator::Colon, ColonT()).labelled(":");
        colon.then(ty()).map(|(colon, ty)| TypedBy { colon, ty })
    };
    let eq_def = operator(Operator::EqDef, EqDefT()).labelled(":=");
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
