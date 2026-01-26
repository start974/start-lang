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
