use cst::Constant;

use super::*;
/// parse constant
/// ```ebfn
/// constant :=
/// | NUMBER
/// | CHARACTER
///```
pub fn constant() -> impl Parser<'static, TokenStream, Constant, ExtraChumsky> {
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
