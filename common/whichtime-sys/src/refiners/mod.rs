//! Refiner infrastructure and implementations

use crate::context::ParsingContext;
use crate::results::ParsedResult;

/// Trait for post-processing parser output.
///
/// Refiners can merge adjacent matches, remove overlaps, or resolve
/// ambiguities after the initial parse step.
pub trait Refiner: Send + Sync {
    /// Transform the current list of parsed results.
    fn refine(&self, context: &ParsingContext, results: Vec<ParsedResult>) -> Vec<ParsedResult>;
}

mod forward_date;
mod merge_date_range;
mod merge_datetime;
mod merge_weekday_date;
mod overlap_removal;

pub use forward_date::ForwardDateRefiner;
pub use merge_date_range::MergeDateRangeRefiner;
pub use merge_datetime::MergeDateTimeRefiner;
pub use merge_weekday_date::MergeWeekdayDateRefiner;
pub use overlap_removal::OverlapRemovalRefiner;
