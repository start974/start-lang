use crate::{SourceId, Span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Location {
    id: SourceId,
    span: Span,
}

impl Location {
    /// Create a new location with the given start and end positions in the source.
    pub fn new(id: SourceId, span: Span) -> Self {
        Self { id, span }
    }

    /// unknown location
    pub fn unknown() -> Self {
        Self {
            span: Span::new(0, 0),
            id: SourceId::Unknown,
        }
    }

    /// add offset to location
    pub fn with_offset(self, offset: usize) -> Self {
        Location {
            span: self.span.with_offset(offset),
            id: self.id,
        }
    }

    /// union of location
    pub fn union(self, other: Location) -> Location {
        if self.id != other.id {
            panic!("Cannot union locations from different sources");
        }
        Location {
            span: self.span.union(&other.span),
            id: self.id.clone(),
        }
    }

    /// get identifier
    pub fn id(&self) -> &SourceId {
        &self.id
    }

    /// start position
    pub fn start(&self) -> usize {
        self.span.start()
    }

    /// end position
    pub fn end(&self) -> usize {
        self.span.end()
    }
}

impl ariadne::Span for Location {
    type SourceId = SourceId;

    fn start(&self) -> usize {
        self.span.start()
    }

    fn end(&self) -> usize {
        self.span.end()
    }

    fn source(&self) -> &Self::SourceId {
        &self.id
    }
}

pub trait Located {
    /// get location
    fn location(&self) -> &Location;
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
