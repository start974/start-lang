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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_id_display() {
        let id_unknown = SourceId::Unknown;
        let id_repl = SourceId::Repl;
        let id_file = SourceId::File(PathBuf::from("test.st"));
        let id_url = SourceId::Url("test".to_string());

        assert_eq!(id_unknown.to_string(), "unknown");
        assert_eq!(id_repl.to_string(), "REPL");
        assert_eq!(id_file.to_string(), "test.st");
        assert_eq!(id_url.to_string(), "test");
    }
}
