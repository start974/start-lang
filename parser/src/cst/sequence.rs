use super::Cst;
use location::GetSpan;
use pp::pretty::*;

#[derive(Debug, Clone)]
pub struct Sequence {
    elements: Vec<Cst>,
}

impl Sequence {
    /// make a new sequence
    pub fn new() -> Self {
        Sequence {
            elements: Vec::new(),
        }
    }

    /// sequence is empty
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// lenght of sequence
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// append an element to the sequence
    pub fn append(mut self, elem: Cst) -> Self {
        self.elements.push(elem);
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
        iter.into_iter().fold(Sequence::new(), Sequence::append)
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

//TODO: make a token compression
