#[derive(Debug, Clone)]
pub enum Value {
    N(u32)
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::N(n) => write!(f, "{n}"),
        }
    }
}
