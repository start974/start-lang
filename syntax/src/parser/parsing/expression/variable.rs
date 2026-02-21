use cst::expression::Variable;

use super::*;

/// parse variable
/// ```ebfn
/// variable := IDENTIFIER
///```
pub fn variable() -> impl Parser<'static, TokenStream, Variable, ExtraChumsky> {
    use cst::expression::VariableT;
    select! {ref meta @ Meta{ value: TokenKind::Identifier(ref s), ..} =>
            meta.clone().map(|_| VariableT::from(s.clone()))
    }
}
