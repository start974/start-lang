mod span;
mod location;
mod source_id;

pub use span::Span;
pub use span::Spanned;
pub use span::SpannedSet;
pub use source_id::SourceId;
pub use location::Location;
pub use location::Located;
pub use location::LocatedSet;

// ==========================================================================
// Report
// ==========================================================================
pub type Report<'a> = ariadne::Report<'a, Location>;
//pub type ReportBuilder<'a> = ariadne::ReportBuilder<'a, Location>;
