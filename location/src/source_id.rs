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
