pub type NConst = u32;

#[derive(Debug, Clone)]
pub enum Constant {
    N(NConst),
}

impl Constant {
    pub fn make_n(v: NConst) -> Self {
        Self::N(v)
    }
}

impl std::fmt::Display for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::N(c) => write!(f, "{c}"),
        }
    }
}
