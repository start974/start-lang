use location::{GetSpan, SetSpan, Span};
use pp::pretty::*;

/// End of input
#[derive(Debug, Clone, Default)]
pub struct Eoi(Span);

impl PartialEq for Eoi {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl Eq for Eoi {}

impl std::hash::Hash for Eoi {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u8(0);
    }
}

impl GetSpan for Eoi {
    fn span(&self) -> Span {
        self.0
    }
}

impl SetSpan for Eoi {
    fn set_span(&mut self, span: Span) {
        self.0 = span
    }
}

impl Pretty for Eoi {
    fn pretty(&self, _theme: &Theme) -> Doc<'_> {
        Doc::text("$")
    }
}
