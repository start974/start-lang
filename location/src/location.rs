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

    /// unknown location
    pub fn unknown() -> Self {
        Location {
            span: Span::default(),
            id: &SourceId::Unknown,
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
    fn location(&self) -> Location<'a>;
}

pub trait LocatedSet<'a>: Sized {
    /// set location
    fn set_location(&mut self, location: Location<'a>);

    /// with location
    fn with_location(mut self, location: Location<'a>) -> Self {
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

    struct TestLocated<'a>(Location<'a>);
    impl<'a> Default for TestLocated<'a> {
        fn default() -> Self {
            Self(Location::unknown())
        }
    }

    impl<'a> Located<'a> for TestLocated<'a> {
        fn location(&self) -> Location<'a> {
            self.0.clone()
        }
    }

    impl<'a> LocatedSet<'a> for TestLocated<'a> {
        fn set_location(&mut self, location: Location<'a>) {
            self.0 = location
        }
    }

    #[test]
    fn location_union() {
        let source_id = SourceId::File(std::path::PathBuf::from("test.st"));
        let source_id2 = SourceId::File(std::path::PathBuf::from("other.st"));
        let loc1 = Location::new(&source_id, Span::new(0, 5));
        let loc2 = Location::new(&source_id, Span::new(3, 10));
        let loc3 = Location::new(&source_id2, Span::new(0, 5));
        assert_eq!(loc1.clone().union(loc2).unwrap().span(), &Span::new(0, 10));
        assert!(loc1.union(loc3).is_none());
    }

    #[test]
    fn location_offset() {
        let source_id = SourceId::File(std::path::PathBuf::from("test.st"));
        let loc = Location::new(&source_id, Span::new(5, 15));
        let offset_loc = loc.with_offset(10);
        assert_eq!(offset_loc.span(), &Span::new(15, 25));
    }

    #[test]
    fn located_traits() {
        let source_id = SourceId::File(std::path::PathBuf::from("test.st"));
        let loc1 = Location::new(&source_id, Span::new(0, 10));
        let loc2 = Location::new(&source_id, Span::new(10, 20));

        let mut test_located = TestLocated::default();
        assert_eq!(test_located.location(), Location::unknown());

        test_located.set_location(loc1.clone());
        assert_eq!(test_located.location(), loc1);

        assert_eq!(test_located.with_location(loc2.clone()).location(), loc2);
    }
}
