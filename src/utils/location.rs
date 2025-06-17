use std::{collections::HashMap, path::PathBuf};

use ariadne::{Cache, Source, Span};

// ==========================================================================
// FileId
// ==========================================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SourceId {
    Unknown,
    Repl { offset: usize },
    File { offset: usize, path: PathBuf },
}

impl SourceId {
    /// source id with offset
    pub fn add_offset(&mut self, o: usize) {
        match self {
            SourceId::Unknown => (),
            SourceId::Repl { offset } | SourceId::File { offset, .. } => {
                *offset += o;
            }
        }
    }
    pub fn offset(&self) -> usize {
        match self {
            SourceId::Unknown => 0,
            SourceId::Repl { offset } => *offset,
            SourceId::File { offset, .. } => *offset,
        }
    }
}

impl std::fmt::Display for SourceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            SourceId::Unknown => write!(f, "unknown"),
            SourceId::Repl { .. } => write!(f, "REPL"),
            SourceId::File { path, .. } => write!(f, "{}", path.display()),
        }
    }
}

// ==========================================================================
// Source Cache
// ==========================================================================

struct ReplCache {
    buffer: String,
    source: Source,
}

impl ReplCache {
    fn new() -> Self {
        Self {
            buffer: String::new(),
            source: Source::from("".to_string()),
        }
    }

    /// last offset of source
    fn last_offset(&self) -> usize {
        self.source.len()
    }

    /// add content to repl cache
    /// repl content is reset when [`update_source`]
    /// to get current repl content use [`get`]
    /// this function return current offset of source
    fn add(&mut self, content: &str) -> usize {
        self.buffer.push_str(content);
        self.buffer.push('\n');
        self.last_offset()
    }

    /// push all content add in buffer [`add`] in source
    fn update_source(&mut self) {
        self.source = Source::from(self.source.text().to_owned() + &self.buffer);
        self.buffer.clear();
    }

    /// get content with an offset
    /// if offset is lower than buffer content is get with source (but no contain buffer)
    fn get(&self, offset: &usize) -> &str {
        match offset.cmp(&self.source.len()) {
            std::cmp::Ordering::Less => &self.source.text()[*offset..],
            std::cmp::Ordering::Equal => &self.buffer,
            std::cmp::Ordering::Greater => {
                let offset = offset - self.source.len();
                &self.buffer[offset..]
            }
        }
    }
}

impl Cache<usize> for ReplCache {
    type Storage = String;
    fn display<'a>(&self, _: &'a usize) -> std::option::Option<impl std::fmt::Display + 'a> {
        None::<u8>
    }

    fn fetch(&mut self, id: &usize) -> Result<&Source<Self::Storage>, impl std::fmt::Debug> {
        if *id == self.source.len() {
            self.update_source();
        }
        if *id >= self.source.len() {
            Err(format!(
                "REPL source index {} is out of bounds (max: {})",
                id,
                self.source.len()
            ))
        } else {
            Ok(&self.source)
        }
    }
}

pub struct SourceCache {
    files: HashMap<PathBuf, Source>,
    repl: ReplCache,
    unknown: Source,
}

impl SourceCache {
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
            repl: ReplCache::new(),
            unknown: Source::from("".to_string()),
        }
    }

    /// set repl content
    /// repl is stored when fetch is apply
    /// to get actual repl content use
    pub fn add_repl(&mut self, content: &str) -> SourceId {
        let offset = self.repl.add(content);
        SourceId::Repl { offset }
    }

    /// get content of source id
    /// for repl get last content
    pub fn get(&self, id: &SourceId) -> &str {
        match id {
            SourceId::Unknown => panic!("cannot get unkow content"),
            SourceId::Repl { offset } => self.repl.get(offset),
            SourceId::File { path, .. } => {
                if let Some(source) = self.files.get(path) {
                    source.text()
                } else {
                    panic!("Source not found in cache: {}", path.display())
                }
            }
        }
    }

    /// update repl source
    pub fn update_repl(&mut self) -> SourceId {
        self.repl.update_source();
        SourceId::Repl {
            offset: self.repl.last_offset(),
        }
    }

    /// last repl source id to access to buffer
    pub fn last_repl_source_id(&self) -> SourceId {
        SourceId::Repl {
            offset: self.repl.last_offset(),
        }
    }

    /// set file content
    pub fn set_file(&mut self, path: PathBuf, content: String) -> SourceId {
        self.files.insert(path.clone(), Source::from(content));
        SourceId::File { path, offset: 0 }
    }
}

impl Default for SourceCache {
    fn default() -> Self {
        Self::new()
    }
}

impl Cache<SourceId> for SourceCache {
    type Storage = String;

    fn display<'a>(&self, id: &'a SourceId) -> std::option::Option<impl std::fmt::Display + 'a> {
        Some(Box::new(id.clone()))
    }

    fn fetch(&mut self, id: &SourceId) -> Result<&Source<Self::Storage>, impl std::fmt::Debug> {
        match id {
            SourceId::Unknown => Ok(&self.unknown),
            SourceId::Repl { offset } => self
                .repl
                .fetch(offset)
                .map_err(|e| Box::new(format!("{:?}", e))),
            SourceId::File { path, .. } => self
                .files
                .get(path)
                .ok_or(Box::new(format!("Source not found in cache: {}", id))),
        }
    }
}

impl AsMut<SourceCache> for &mut SourceCache {
    fn as_mut(&mut self) -> &mut SourceCache {
        self
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

    pub fn union(&self, other: &Location) -> Location {
        if self.source != other.source {
            panic!("Cannot union locations from different sources");
        }
        Location {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
            source: self.source.clone(),
        }
    }
}

pub const UNKNOWN_LOCATION: Location = Location {
    start: 0,
    end: 0,
    source: SourceId::Unknown,
};

impl Span for Location {
    type SourceId = SourceId;
    fn start(&self) -> usize {
        match self.source {
            SourceId::Repl { offset } => offset + self.start,
            SourceId::File { offset, .. } => offset + self.start,
            SourceId::Unknown => 0,
        }
    }
    fn end(&self) -> usize {
        match self.source {
            SourceId::Repl { offset } => offset + self.end,
            SourceId::File { offset, .. } => offset + self.end,
            SourceId::Unknown => 0,
        }
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

impl<T> LocatedSet for Loc<T> {
    fn set_loc(&mut self, loc: &impl Located) {
        self.loc = loc.loc().clone();
    }
}

// ==========================================================================
// Report
// ==========================================================================
pub type Report<'a> = ariadne::Report<'a, Location>;
pub type ReportBuilder<'a> = ariadne::ReportBuilder<'a, Location>;
