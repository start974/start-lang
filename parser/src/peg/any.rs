use location::{GetSpan, SetSpan, Span};
use pp::pretty::*;

#[derive(Debug, Clone, Default)]
pub struct Any(Span);

impl PartialEq for Any {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl Eq for Any {}

impl std::hash::Hash for Any {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u8(0);
    }
}

impl GetSpan for Any {
    fn span(&self) -> Span {
        self.0
    }
}

impl SetSpan for Any {
    fn set_span(&mut self, span: Span) {
        self.0 = span
    }
}

impl Pretty for Any {
    fn pretty(&self, _theme: &Theme) -> Doc<'_> {
        Doc::text(".")
    }
}
