//! Common types for whichtime-sys

use serde::{Deserialize, Serialize};

/// Weekday enumeration (0 = Sunday)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum Weekday {
    /// Sunday.
    Sunday = 0,
    /// Monday.
    Monday = 1,
    /// Tuesday.
    Tuesday = 2,
    /// Wednesday.
    Wednesday = 3,
    /// Thursday.
    Thursday = 4,
    /// Friday.
    Friday = 5,
    /// Saturday.
    Saturday = 6,
}

/// Meridiem (AM/PM) enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum Meridiem {
    /// Ante meridiem.
    AM = 0,
    /// Post meridiem.
    PM = 1,
}

/// Time unit for duration calculations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimeUnit {
    /// Calendar year.
    Year,
    /// Quarter of a year.
    Quarter,
    /// Calendar month.
    Month,
    /// Seven-day week.
    Week,
    /// Calendar day.
    Day,
    /// Hour.
    Hour,
    /// Minute.
    Minute,
    /// Second.
    Second,
    /// Millisecond.
    Millisecond,
}

/// Duration with support for both calendar and clock units.
#[derive(Debug, Clone, Default)]
pub struct Duration {
    /// Number of years to add.
    pub year: Option<f64>,
    /// Number of quarters to add.
    pub quarter: Option<f64>,
    /// Number of months to add.
    pub month: Option<f64>,
    /// Number of weeks to add.
    pub week: Option<f64>,
    /// Number of days to add.
    pub day: Option<f64>,
    /// Number of hours to add.
    pub hour: Option<f64>,
    /// Number of minutes to add.
    pub minute: Option<f64>,
    /// Number of seconds to add.
    pub second: Option<f64>,
    /// Number of milliseconds to add.
    pub millisecond: Option<f64>,
}

impl Duration {
    /// Create a new empty duration
    pub const fn new() -> Self {
        Self {
            year: None,
            quarter: None,
            month: None,
            week: None,
            day: None,
            hour: None,
            minute: None,
            second: None,
            millisecond: None,
        }
    }

    /// Return `true` if the duration contains clock-based units.
    pub fn has_time_component(&self) -> bool {
        self.hour.is_some()
            || self.minute.is_some()
            || self.second.is_some()
            || self.millisecond.is_some()
    }

    /// Return `true` if the duration contains calendar-based units.
    pub fn has_date_component(&self) -> bool {
        self.year.is_some()
            || self.quarter.is_some()
            || self.month.is_some()
            || self.week.is_some()
            || self.day.is_some()
    }

    /// Return a copy of the duration with every populated value negated.
    pub fn reversed(&self) -> Self {
        Self {
            year: self.year.map(|v| -v),
            quarter: self.quarter.map(|v| -v),
            month: self.month.map(|v| -v),
            week: self.week.map(|v| -v),
            day: self.day.map(|v| -v),
            hour: self.hour.map(|v| -v),
            minute: self.minute.map(|v| -v),
            second: self.second.map(|v| -v),
            millisecond: self.millisecond.map(|v| -v),
        }
    }
}

/// Add a parsed duration to a local datetime.
pub fn add_duration(
    mut date: chrono::DateTime<chrono::Local>,
    duration: &Duration,
) -> chrono::DateTime<chrono::Local> {
    use chrono::{Datelike, Duration as ChronoDuration};

    // Handle year
    if let Some(years) = duration.year {
        let y = years.floor() as i32;
        if let Some(new_date) = date.with_year(date.year() + y) {
            date = new_date;
        }
    }

    // Handle quarter (as 3 months)
    if let Some(quarters) = duration.quarter {
        let months = (quarters * 3.0).floor() as i32;
        date = add_months(date, months);
    }

    // Handle month
    if let Some(months) = duration.month {
        let m = months.floor() as i32;
        date = add_months(date, m);
    }

    // Handle week
    if let Some(weeks) = duration.week {
        let days = (weeks * 7.0).floor() as i64;
        date += ChronoDuration::days(days);
    }

    // Handle day
    if let Some(days) = duration.day {
        let d = days.floor() as i64;
        date += ChronoDuration::days(d);
    }

    // Handle hour
    if let Some(hours) = duration.hour {
        let h = hours.floor() as i64;
        date += ChronoDuration::hours(h);
    }

    // Handle minute
    if let Some(minutes) = duration.minute {
        let m = minutes.floor() as i64;
        date += ChronoDuration::minutes(m);
    }

    // Handle second
    if let Some(seconds) = duration.second {
        let s = seconds.floor() as i64;
        date += ChronoDuration::seconds(s);
    }

    // Handle millisecond
    if let Some(ms) = duration.millisecond {
        let m = ms.floor() as i64;
        date += ChronoDuration::milliseconds(m);
    }

    date
}

/// Add months to a date, handling overflow correctly
fn add_months(
    date: chrono::DateTime<chrono::Local>,
    months: i32,
) -> chrono::DateTime<chrono::Local> {
    use chrono::Datelike;

    let current_month = date.month() as i32;
    let new_month = current_month + months;

    if new_month <= 0 {
        let years_back = (-new_month) / 12 + 1;
        let final_month = 12 - ((-new_month) % 12);
        let new_date = date.with_year(date.year() - years_back).unwrap_or(date);
        new_date.with_month(final_month as u32).unwrap_or(new_date)
    } else if new_month > 12 {
        let years_forward = (new_month - 1) / 12;
        let final_month = ((new_month - 1) % 12) + 1;
        let new_date = date.with_year(date.year() + years_forward).unwrap_or(date);
        new_date.with_month(final_month as u32).unwrap_or(new_date)
    } else {
        date.with_month(new_month as u32).unwrap_or(date)
    }
}
