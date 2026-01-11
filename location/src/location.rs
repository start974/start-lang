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
    pub fn union(self, other: Location<'a>) -> Option<Location<'a>> {
        if self.id == other.id {
            Some(Location {
                span: self.span.union(other.span),
                id: self.id,
            })
        } else {
            None
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
        self.span().end()
    }

    fn source(&self) -> &Self::SourceId {
        self.id()
    }
}

// ==========================================================================
// Test
// ==========================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use crate::SourceId;

    #[test]
    fn location_union() {
        let source_id = SourceId::File(std::path::PathBuf::from("test.rs"));
        let source_id2 = SourceId::File(std::path::PathBuf::from("other.rs"));
        let loc1 = Location::new(&source_id, Span::new(0, 5));
        let loc2 = Location::new(&source_id, Span::new(3, 10));
        let loc3 = Location::new(&source_id2, Span::new(0, 5));
        assert_eq!(loc1.clone().union(loc2).unwrap().span(), &Span::new(0, 10));
        assert!(loc1.union(loc3).is_none());
    }

    #[test]
    fn location_offset() {
        let source_id = SourceId::File(std::path::PathBuf::from("test.rs"));
        let loc = Location::new(&source_id, Span::new(5, 15));
        let offset_loc = loc.with_offset(10);
        assert_eq!(offset_loc.span(), &Span::new(15, 25));
    }
}
