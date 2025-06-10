use std::collections::HashMap;

use ariadne::{Cache, Source, Span};

// ==========================================================================
// FileId
// ==========================================================================
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SourceId {
    Unknown,
    Repl,
    Path(std::path::PathBuf),
}

impl std::fmt::Display for SourceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SourceId::Unknown => write!(f, "unknown"),
            SourceId::Repl => write!(f, "REPL"),
            SourceId::Path(path) => write!(f, "{}", path.display()),
        }
    }
}

// ==========================================================================
// FileCache
// ==========================================================================

pub struct SourceCache {
    /// cache of sources
    sources: HashMap<SourceId, Source>,
    /// cache
    repl_content: String,
}

impl SourceCache {
    pub fn new() -> Self {
        Self {
            sources: HashMap::new(),
            repl_content: String::new(), // Initialize with empty REPL content
        }
    }

    // A method to update the REPL content, if applicable
    pub fn update_repl_content(&mut self, content: String) {
        self.repl_content = content;
        // Invalidate the cache for FileId::Repl so it's reloaded next time
        self.sources.remove(&SourceId::Repl);
    }
}

impl Cache<SourceId> for SourceCache {
    type Storage = String;

    fn display<'a>(&self, id: &'a SourceId) -> std::option::Option<impl std::fmt::Display + 'a> {
        Some(Box::new(id.clone()))
    }

    fn fetch(&mut self, id: &SourceId) -> Result<&Source<Self::Storage>, impl std::fmt::Debug> {
        // If the source is not already in the cache, load it
        if !self.sources.contains_key(id) {
            let source_content = match id {
                SourceId::Unknown => String::new(), // An empty string for unknown source
                SourceId::Repl => self.repl_content.clone(), // Clone the REPL content
                SourceId::Path(path) => {
                    // Read file content, map to String, or map error to a boxed Debug trait object
                    std::fs::read_to_string(path)
                        .map_err(|e| Box::new(e) as Box<dyn std::fmt::Debug>)?
                }
            };
            // Create a Source from the content and insert into cache
            self.sources
                .insert(id.clone(), Source::from(source_content));
        }
        // Return a reference to the cached Source
        Ok::<&Source, Box<dyn std::fmt::Debug>>(self.sources.get(id).unwrap())
    }
}

// ==========================================================================
// Location
// ==========================================================================
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Location {
    start: usize,
    end: usize,
    source: SourceId,
}

impl Location {
    /// Create a new location with the given start and end positions in the source.
    pub fn new(start: usize, end: usize, source: SourceId) -> Self {
        Self { start, end, source }
    }

    /// unknown location
    pub fn unknown() -> Self {
        Self {
            start: 0,
            end: 0,
            source: SourceId::Unknown,
        }
    }

    /// union of location (fail if path is different)
    pub fn union(&self, other: &Self) -> Self {
        if self.source != other.source {
            panic!(
                "Cannot union locations from different sources ({} ≠ {})",
                self.source, other.source
            );
        }
        Self {
            source: self.source.clone(),
            start: self.start.min(other.start),
            end: self.end.max(other.end),
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
        &self.source
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

// ==========================================================================
// Localised
// ==========================================================================
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

impl<T> Located for Loc<T> {
    fn loc(&self) -> &Location {
        &self.loc
    }
}

// ==========================================================================
// Report
// ==========================================================================
pub type Report<'a> = ariadne::Report<'a, Location>;
pub type ReportBuilder<'a> = ariadne::ReportBuilder<'a, Location>;
