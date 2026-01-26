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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::TokenKind;
    use chumsky::prelude::*;
    use cst::{AsCharacter as _, AsNumber as _};

    #[test]
    fn number() {
        use num_bigint::BigUint;
        let parser = constant();

        let n = BigUint::from(42u32);

        let tokens_kinds = [TokenKind::Number(n.clone())];
        let tokens = tokens_kinds
            .iter()
            .map(move |x| Meta::new(x.clone(), Span::default()));
        let stream = TokenStream::from_iter(tokens);

        let cst = parser.parse(stream).unwrap();
        let v = match &cst {
            Constant::Number(num) => num.value.as_number(),
            _ => panic!("Expected Number constant"),
        };
        assert_eq!(v, &n);
    }

    #[test]
    fn character() {
        let parser = constant();

        let c = 'z';

        let tokens_kinds = [TokenKind::Character(c)];
        let tokens = tokens_kinds
            .iter()
            .map(move |x| Meta::new(x.clone(), Span::default()));
        let stream = TokenStream::from_iter(tokens);

        let cst = parser.parse(stream).unwrap();
        let v = match &cst {
            Constant::Character(chr) => chr.value.as_character(),
            _ => panic!("Expected Number constant"),
        };
        assert_eq!(v, c);
    }
}
