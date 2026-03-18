//! French locale-specific parsers

mod casual_date;
mod casual_time;
mod month_name;
mod slash_date;
mod time_expression;
mod time_unit_relative;
mod weekday;

pub use casual_date::FRCasualDateParser;
pub use casual_time::FRCasualTimeParser;
pub use month_name::FRMonthNameParser;
pub use slash_date::FRSlashDateParser;
pub use time_expression::FRTimeExpressionParser;
pub use time_unit_relative::FRTimeUnitRelativeParser;
pub use weekday::FRWeekdayParser;
