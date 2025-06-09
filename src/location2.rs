use std::path::Path;

use ariadne::Span;

pub struct Location<'path> {
    start: usize,
    end: usize,
    path: &'path Path,
}

impl<'path> Location<'path> {
    /// Create a new location with the given start and end positions in the source.
    pub fn new(start: usize, end: usize, path: &'path Path) -> Self {
        Self { start, end, path }
    }
}

impl<'path> Span for Location<'path> {
    type SourceId = &'path Path;
    fn start(&self) -> usize {
        self.start
    }
    fn end(&self) -> usize {
        self.end
    }
    fn source(&self) -> &Self::SourceId {
        &self.path
    }
}

pub trait Located {
    /// location of a node
    fn get_location(&self) -> Location;
}
