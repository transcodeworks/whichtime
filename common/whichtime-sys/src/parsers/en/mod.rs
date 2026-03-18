//! English locale parsers

mod casual_date;
mod casual_time;
mod iso_format;
mod month_name;
mod relative;
mod slash_date;
mod time_expression;
mod time_unit_ago;
mod time_unit_later;
mod time_unit_within;
mod weekday;

pub use casual_date::CasualDateParser;
pub use casual_time::CasualTimeParser;
pub use iso_format::ISOFormatParser;
pub use month_name::MonthNameParser;
pub use relative::RelativeDateParser;
pub use slash_date::SlashDateParser;
pub use time_expression::TimeExpressionParser;
pub use time_unit_ago::TimeUnitAgoParser;
pub use time_unit_later::TimeUnitLaterParser;
pub use time_unit_within::TimeUnitWithinParser;
pub use weekday::WeekdayParser;

use crate::Configuration;
use crate::dictionaries::Locale;

/// Create the default casual configuration for English
pub fn create_casual_configuration() -> Configuration {
    Configuration {
        parsers: vec![
            Box::new(ISOFormatParser),
            Box::new(SlashDateParser::new(false)),
            Box::new(MonthNameParser),
            Box::new(WeekdayParser),
            Box::new(TimeExpressionParser),
            Box::new(CasualDateParser),
            Box::new(CasualTimeParser),
            Box::new(TimeUnitAgoParser),
            Box::new(TimeUnitLaterParser),
            Box::new(TimeUnitWithinParser),
            Box::new(RelativeDateParser),
        ],
        refiners: vec![
            Box::new(crate::refiners::OverlapRemovalRefiner),
            Box::new(crate::refiners::MergeDateTimeRefiner),
            Box::new(crate::refiners::ForwardDateRefiner),
        ],
        locale: Locale::En,
    }
}

/// Create strict configuration (no casual patterns)
pub fn create_strict_configuration() -> Configuration {
    Configuration {
        parsers: vec![
            Box::new(ISOFormatParser),
            Box::new(SlashDateParser::new(false)),
            Box::new(MonthNameParser),
            Box::new(TimeExpressionParser),
        ],
        refiners: vec![
            Box::new(crate::refiners::OverlapRemovalRefiner),
            Box::new(crate::refiners::MergeDateTimeRefiner),
        ],
        locale: Locale::En,
    }
}
