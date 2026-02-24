use super::Cst;
use location::GetSpan;
use pp::pretty::*;

#[derive(Debug, Clone, Default)]
pub struct Sequence {
    elements: Vec<Cst>,
}

impl Sequence {
    /// sequence is empty
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// lenght of sequence
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// append an element to the sequence
    /// merge tokens if possible
    pub fn append(mut self, elem: Cst) -> Self {
        use super::Token;
        use location::{SetSpan as _, Span};

        match elem {
            Cst::Token(token_2) => match self.elements.pop() {
                Some(Cst::Token(token_1)) if token_1.span().end() == token_2.span().start() => {
                    let span = Span::new(token_1.span().start(), token_2.span().end());
                    let content = token_1.content + &token_2.content;
                    let token = Token::from(content.as_str()).with_span(span);
                    self.elements.push(Cst::Token(token));
                }
                Some(cst) => {
                    self.elements.push(cst);
                    self.elements.push(Cst::Token(token_2));
                }
                _ => {
                    self.elements.push(Cst::Token(token_2));
                }
            },
            _ => self.elements.push(elem),
        };
        self
    }

    /// iterator over elements
    pub fn iter(&self) -> std::slice::Iter<'_, Cst> {
        self.elements.iter()
    }
}

impl FromIterator<Cst> for Sequence {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Cst>,
    {
        iter.into_iter().fold(Sequence::default(), Sequence::append)
    }
}

impl GetSpan for Sequence {
    fn span(&self) -> location::Span {
        use location::Span;
        Span::from_iter(self.iter().map(|cst| cst.span()))
    }
}

impl Pretty for Sequence {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        Doc::intersperse(self.iter().map(|cst| cst.pretty(theme)), Doc::nil())
    }
}

#[cfg(test)]
mod test {
    use location::{GetSpan, SetSpan, Span};

    use crate::cst::{Cst, Sequence, Token};

    #[test]
    fn append_tokens() {
        let mut seq = Sequence::default();
        assert!(seq.is_empty());
        {
            seq = seq.append(Token::from("test1").with_span(Span::new(0, 4)).into());
            let mut it = seq.iter();
            match it.next().unwrap(){
                Cst::Token(token) => {
                    assert_eq!(token.content, "test1");
                    assert_eq!(token.span(), Span::new(0, 4));
                },
                _ => panic!("Expected token"),
            }
            assert!(it.next().is_none());
        }
        {
            seq = seq.append(Token::from("test2").with_span(Span::new(4, 8)).into());
            let mut it = seq.iter();
            match it.next().unwrap(){
                Cst::Token(token) => {
                    assert_eq!(token.content, "test1test2");
                    assert_eq!(token.span(), Span::new(0, 8));
                },
                _ => panic!("Expected token"),
            }
            assert!(it.next().is_none());
        }
        {
            seq = seq.append(Cst::nil());
            let mut it = seq.iter();
            match it.next().unwrap(){
                Cst::Token(token) => {
                    assert_eq!(token.content, "test1test2");
                    assert_eq!(token.span(), Span::new(0, 8));
                },
                _ => panic!("Expected token"),
            }

            assert!(matches!(it.next().unwrap(), Cst::Nil));
        }
    }
}
