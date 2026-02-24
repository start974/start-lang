pub mod sequence;
pub mod token;

use location::GetSpan;
use pp::pretty::*;

pub use sequence::*;
pub use token::*;

/// Concrete Syntax Tree node
#[derive(Debug, Clone)]
pub enum Cst {
    /// token cst
    /// Used for parsing
    /// - Literal
    /// - Class
    /// - Any
    Token(Token),

    /// named cst
    /// Used for paring
    /// - RefRule
    Named(String, Box<Cst>),

    /// sequence cst
    /// used for parsing:
    /// - Sequence
    /// - Repetition
    Sequence(Sequence),

    /// Nil
    /// - Used for parsing
    /// - NegativeLookahead
    /// - PositiveLookahead
    Nil, //TODO: add template
}

impl From<Token> for Cst {
    fn from(token: Token) -> Self {
        Self::Token(token)
    }
}

impl From<Sequence> for Cst {
    fn from(sequence: Sequence) -> Self {
        Self::Sequence(sequence)
    }
}

impl Cst {
    /// make nil Cst
    pub fn nil() -> Self {
        Self::Nil
    }

    /// make a nil cst
    pub fn named(self, name: &str) -> Self {
        Self::Named(name.to_string(), Box::new(self))
    }
}

impl GetSpan for Cst {
    fn span(&self) -> location::Span {
        match self {
            Cst::Token(token) => token.span(),
            Cst::Named(_, cst) => cst.span(),
            Cst::Sequence(sequence) => sequence.span(),
            Cst::Nil => location::Span::default(),
        }
    }
}

impl Pretty for Cst {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Cst::Token(token) => token.pretty(theme),
            Cst::Named(_, named) => named.pretty(theme),
            Cst::Sequence(sequence) => sequence.pretty(theme),
            Cst::Nil => Doc::nil(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cst::token::Token;

    use super::*;

    #[test]
    fn pretty_cst() {
        let cst: Cst = Sequence::from_iter(vec![
            Token::from("(* This is a comment *)").into(),
            Cst::from(Token::from(" ")).named("whitespace"),
            Cst::from(Token::from("token1")).named("test1"),
            Cst::nil(),
            Cst::from(Token::from(" ")).named("whitespace"),
            Token::from("token2").into(),
            Cst::from(Token::from(" ")).named("whitespace"),
            Token::from("token3").into(),
            Token::from("token4").into(),
            Cst::from(Token::from("\n")).named("whitespace"),
        ])
        .into();

        dbg!(&cst);
        let theme = Theme::default();
        let str = cst.make_string(&theme);
        assert_eq!(str, "(* This is a comment *) token1 token2 token3token4\n");
    }
}
