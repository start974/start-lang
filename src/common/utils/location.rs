use ariadne::Span;
use chumsky::span::SimpleSpan;
use std::path::PathBuf;

// ==========================================================================
// FileId
// ==========================================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SourceId {
    Unknown,
    Repl,
    File(PathBuf),
    Url(String),
}

impl std::fmt::Display for SourceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            SourceId::Unknown => write!(f, "unknown"),
            SourceId::Repl => write!(f, "REPL"),
            SourceId::File(path) => write!(f, "{}", path.display()),
            SourceId::Url(url) => write!(f, "{url}"),
        }
    }
}

// ==========================================================================
// Location
// ==========================================================================
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Location {
    start: usize,
    end: usize,
    id: SourceId,
}

impl Location {
    /// Create a new location with the given start and end positions in the source.
    pub fn new(id: SourceId, start: usize, end: usize) -> Self {
        Self { start, end, id }
    }

    /// unknown location
    pub fn unknown() -> Self {
        Self {
            start: 0,
            end: 0,
            id: SourceId::Unknown,
        }
    }

    /// add offset to location
    pub fn with_offset(self, offset: usize) -> Self {
        Location {
            start: self.start + offset,
            end: self.end + offset,
            id: self.id,
        }
    }

    /// union of location
    pub fn union(self, other: Location) -> Location {
        if self.id != other.id {
            panic!("Cannot union locations from different sources");
        }
        Location {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
            id: self.id.clone(),
        }
    }

    /// to simple span
    pub fn to_simple_span(&self) -> SimpleSpan {
        SimpleSpan {
            start: self.start,
            end: self.end,
            context: (),
        }
    }
}

impl Span for Location {
    type SourceId = SourceId;
    fn start(&self) -> usize {
        self.start
    }

    fn end(&self) -> usize {
        self.end
    }

    fn source(&self) -> &Self::SourceId {
        &self.id
    }
}

// ==========================================================================
// Located
// ==========================================================================
pub trait Located {
    /// location of a node
    fn loc(&self) -> Location;
}

impl<T> Located for Box<T>
where
    T: Located,
{
    fn loc(&self) -> Location {
        self.as_ref().loc()
    }
}

pub trait LocatedSet {
    /// set location
    fn set_loc(&mut self, loc: &impl Located);

    /// with location
    fn with_loc(mut self, loc: &impl Located) -> Self
    where
        Self: Sized,
    {
        self.set_loc(loc);
        self
    }
}

// ==========================================================================
// Report
// ==========================================================================
pub type Report<'a> = ariadne::Report<'a, Location>;
//pub type ReportBuilder<'a> = ariadne::ReportBuilder<'a, Location>;
