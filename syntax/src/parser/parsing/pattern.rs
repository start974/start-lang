use cst::pattern::{Pattern, VariableT};

use super::*;
/// parse pattern
/// ```ebfn
/// pattern :=
/// | IDENTIFIER
///```
pub fn pattern() -> impl Parser<'static, TokenStream, Pattern, ExtraChumsky> {
    let variable = select! {ref meta @ Meta{ value: TokenKind::Identifier(ref s), ..} =>
            meta.clone().map(|_| VariableT::from(s.clone()))
    }
    .labelled("pattern variable")
    .map(Pattern::from);

    variable.labelled("pattern")
}
