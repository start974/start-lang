mod location;
mod source_id;
mod span;

pub use location::Located;
pub use location::LocatedSet;
pub use location::Location;
pub use source_id::SourceId;
pub use span::Span;
pub use span::GetSpan;
pub use span::SetSpan;

// ==========================================================================
// Report
// ==========================================================================
pub type Report<'id, 'a> = ariadne::Report<'a, Location<'id>>;
//pub type ReportBuilder<'a> = ariadne::ReportBuilder<'a, Location>;
