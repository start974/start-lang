use crate::{SourceId, Span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Location<'a> {
    id: &'a SourceId,
    span: Span,
}

impl<'a> Location<'a> {
    /// Create a new location with the given start and end positions in the source.
    pub fn new(id: &'a SourceId, span: Span) -> Self {
        Self { id, span }
    }

    /// add offset to location
    pub fn with_offset(self, offset: usize) -> Self {
        Location {
            span: self.span.with_offset(offset),
            id: self.id,
        }
    }

    /// union of location
    pub fn union(self, other: Location<'a>) -> Location<'a> {
        if self.id != other.id {
            panic!("Cannot union locations from different sources");
        }
        Location {
            span: self.span.union(other.span),
            id: self.id,
        }
    }

    /// get identifier
    pub fn id(&self) -> &'a SourceId {
        self.id
    }

    /// get span
    pub fn span(&self) -> &Span {
        &self.span
    }
}

pub trait Located<'a> {
    /// get location
    fn location(&self) -> &Location<'a>;
}

pub trait LocatedSet: Sized {
    /// set location
    fn set_location(&mut self, location: Location);

    /// with location
    fn with_location(mut self, location: Location) -> Self {
        self.set_location(location);
        self
    }
}

impl<'a> ariadne::Span for Location<'a> {
    type SourceId = SourceId;

    fn start(&self) -> usize {
        self.span().start()
    }

    fn end(&self) -> usize {
        self.span().start()
    }

    fn source(&self) -> &Self::SourceId {
        self.id()
    }
}
