//! Parsing results

use crate::components::FastComponents;
use chrono::{DateTime, Local};

/// Reference date/time with timezone information.
///
/// Relative expressions are resolved against this instant and optional
/// timezone offset.
#[derive(Debug, Clone)]
pub struct ReferenceWithTimezone {
    /// The reference instant used as the base for relative parsing.
    pub instant: DateTime<Local>,
    /// Explicit timezone offset in minutes.
    ///
    /// `None` means the system timezone of `instant` should be used.
    pub timezone_offset: Option<i32>,
}

impl ReferenceWithTimezone {
    /// Create a new reference value from an instant and optional offset.
    pub fn new(instant: DateTime<Local>, timezone_offset: Option<i32>) -> Self {
        Self {
            instant,
            timezone_offset,
        }
    }

    /// Create a reference value using the current local time.
    pub fn now() -> Self {
        Self {
            instant: Local::now(),
            timezone_offset: None,
        }
    }

    /// Return the effective timezone offset in minutes.
    pub fn get_timezone_offset(&self) -> i32 {
        self.timezone_offset
            .unwrap_or_else(|| -self.instant.offset().utc_minus_local() / 60)
    }

    /// Compute the adjustment between the current system offset and a target offset.
    pub fn get_system_timezone_adjustment(
        &self,
        date: Option<DateTime<Local>>,
        override_offset: Option<i32>,
    ) -> i32 {
        let date = date.unwrap_or_else(Local::now);
        let current_offset = -date.offset().utc_minus_local() / 60;
        let target_offset = override_offset.unwrap_or_else(|| self.get_timezone_offset());
        current_offset - target_offset
    }
}

/// A parsed result representing a matched date/time expression.
#[derive(Debug, Clone)]
pub struct ParsedResult {
    /// Reference date used while resolving this result.
    pub ref_date: DateTime<Local>,
    /// Start byte index of the matched text.
    pub index: usize,
    /// End byte index of the matched text.
    pub end_index: usize,
    /// Original text slice that produced this result.
    pub text: String,
    /// Parsed components for the start of the match.
    pub start: FastComponents,
    /// Parsed components for the end of the match, when the result is a range.
    pub end: Option<FastComponents>,
}

impl ParsedResult {
    /// Create a parsed result from a matched slice and component set.
    pub fn new(
        reference: &ReferenceWithTimezone,
        index: usize,
        text: impl Into<String>,
        start: FastComponents,
        end: Option<FastComponents>,
    ) -> Self {
        let text = text.into();
        let end_index = index + text.len();
        Self {
            ref_date: reference.instant,
            index,
            end_index,
            text,
            start,
            end,
        }
    }

    /// Resolve the start components to a concrete local datetime.
    pub fn date(&self, reference: &ReferenceWithTimezone) -> Option<DateTime<Local>> {
        self.start.to_datetime(reference)
    }
}
