use super::*;
use crate::lexer::token::Operator;
pub use cst::Meta;

/// Operator token
pub fn operator<Op>(
    op: Operator,
    res: Op,
) -> impl Parser<'static, TokenStream, Meta<Op>, ExtraChumsky>
where
    Op: Clone,
{
    select! {ref meta @ Meta{ value: TokenKind::Operator(ref token_op), ..} if token_op == &op =>
            meta.clone().map(|_| res.clone())
    }
}

// ===========================================================================
// End of input
// ===========================================================================

/// parse end of input
pub fn end_of_input() -> impl Parser<'static, TokenStream, cst::EndOfInput, ExtraChumsky> {
    use cst::file::EndOfInputT;
    select! {meta @ Meta{ value: TokenKind::EndOfInput, ..} =>
        meta.map(|_| EndOfInputT())
    }
    .labelled("")
}

// ===========================================================================
// End of input
// ===========================================================================
