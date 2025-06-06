use ariadne::Span;

pub type SourceId = String;

pub struct Location {
    start: usize,
    end: usize,
    source: SourceId,
}

impl Span for Location {
    type SourceId = String;
    fn start(&self) -> usize {
        self.start
    }
    fn end(&self) -> usize {
        self.end
    }
    fn source(&self) -> &Self::SourceId {
        &self.source
    }
}

pub trait Located {
    /// location of a node
    fn get_location(&self) -> Location;
}
