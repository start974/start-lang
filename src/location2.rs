use ariadne::Span;

// ==========================================================================
// Location
// ==========================================================================
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Location<Path> {
    start: usize,
    end: usize,
    path: Path,
}

impl<Path> Location<Path> {
    /// Create a new location with the given start and end positions in the source.
    pub fn new(start: usize, end: usize, path: Path) -> Self {
        Self { start, end, path }
    }
}

impl<Path> Location<Path>
where
    Path: AsRef<std::path::Path> + Clone,
{
    /// union of location (fail if path is different)
    pub fn union(&self, other: &Self) -> Self {
        let path1 = self.path.as_ref();
        let path2 = other.path.as_ref();
        if path1 != path2 {
            panic!(
                "Cannot union locations from different sources ({} ≠ {})",
                path1.to_string_lossy(),
                path2.to_string_lossy()
            );
        }
        Self {
            path: self.path.clone(),
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }
}

impl<Path> Span for Location<Path>
where
    Path: PartialEq + ToOwned,
{
    type SourceId = Path;
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

impl<Path> Located<Path> for Location<Path> {
    fn loc(&self) -> &Location<Path> {
        self
    }
}

// ==========================================================================
// Located
// ==========================================================================
pub trait Located<Path> {
    /// location of a node
    fn loc(&self) -> &Location<Path>;
}

// ==========================================================================
// Localised
// ==========================================================================
pub struct Loc<Path, T> {
    /// location of a node
    pub loc: Location<Path>,

    /// the node itself
    pub data: T,
}

impl<Path, T> Loc<Path, T> {
    /// Create a new localised node with the given data and location.
    pub fn new(data: T, loc: Location<Path>) -> Self {
        Self { data, loc }
    }
}

impl<Path, T> Located<Path> for Loc<Path, T> {
    fn loc(&self) -> &Location<Path> {
        &self.loc
    }
}
