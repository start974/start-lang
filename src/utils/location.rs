use ariadne::Span;
use std::path::PathBuf;

// ==========================================================================
// FileId
// ==========================================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SourceId {
    Unknown,
    Repl,
    File(PathBuf),
}

impl std::fmt::Display for SourceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            SourceId::Unknown => write!(f, "unknown"),
            SourceId::Repl => write!(f, "REPL"),
            SourceId::File(path) => write!(f, "{}", path.display()),
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

    /// add offset to location
    pub fn with_offset(self, offset: usize) -> Self {
        Location {
            start: self.start + offset,
            end: self.end + offset,
            id: self.id,
        }
    }

    /// union of location
    pub fn union(&self, other: &Location) -> Location {
        if self.id != other.id {
            panic!("Cannot union locations from different sources");
        }
        Location {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
            id: self.id.clone(),
        }
    }
}

pub const UNKNOWN_LOCATION: Location = Location {
    start: 0,
    end: 0,
    id: SourceId::Unknown,
};

impl Span for Location {
    type SourceId = SourceId;
    fn start(&self) -> usize {
        match self.id {
            SourceId::Repl => self.start,
            SourceId::File { .. } => self.start,
            SourceId::Unknown => 0,
        }
    }
    fn end(&self) -> usize {
        match self.id {
            SourceId::Repl => self.end,
            SourceId::File { .. } => self.end,
            SourceId::Unknown => 0,
        }
    }
    fn source(&self) -> &Self::SourceId {
        &self.id
    }
}

impl Located for Location {
    fn loc(&self) -> &Location {
        self
    }
}

// ==========================================================================
// Located
// ==========================================================================
pub trait Located {
    /// location of a node
    fn loc(&self) -> &Location;
}

impl<T> Located for Box<T>
where
    T: Located,
{
    fn loc(&self) -> &Location {
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
// Localised
// ==========================================================================
#[derive(Debug)]
pub struct Loc<T> {
    /// location of a node
    pub loc: Location,

    /// the node itself
    pub data: T,
}

impl<T> Loc<T> {
    /// Create a new localised node with the given data and location.
    pub fn new(data: T, loc: Location) -> Self {
        Self { data, loc }
    }
}

// ==========================================================================
// Report
// ==========================================================================
pub type Report<'a> = ariadne::Report<'a, Location>;
pub type ReportBuilder<'a> = ariadne::ReportBuilder<'a, Location>;
