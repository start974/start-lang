mod located;
mod source_id;
mod span;

pub use located::Located;
pub use located::LocatedSet;
pub use located::Location;
pub use source_id::SourceId;
pub use span::Span;
pub use span::Spanned;
pub use span::SpannedSet;

// ==========================================================================
// Report
// ==========================================================================
pub type Report<'id, 'a> = ariadne::Report<'a, Location<'id>>;
//pub type ReportBuilder<'a> = ariadne::ReportBuilder<'a, Location>;
