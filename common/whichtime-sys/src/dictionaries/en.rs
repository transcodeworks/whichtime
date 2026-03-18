//! English locale dictionaries

use super::{CasualDateType, CasualTimeType, RelativeModifier, TimeUnit, Weekday};
use phf::phf_map;

/// Weekday dictionary - maps lowercase weekday names to Weekday enum
pub static WEEKDAY_MAP: phf::Map<&'static str, Weekday> = phf_map! {
    "sunday" => Weekday::Sunday,
    "sun" => Weekday::Sunday,
    "sun." => Weekday::Sunday,
    "monday" => Weekday::Monday,
    "mon" => Weekday::Monday,
    "mon." => Weekday::Monday,
    "tuesday" => Weekday::Tuesday,
    "tue" => Weekday::Tuesday,
    "tue." => Weekday::Tuesday,
    "tues" => Weekday::Tuesday,
    "wednesday" => Weekday::Wednesday,
    "wed" => Weekday::Wednesday,
    "wed." => Weekday::Wednesday,
    "thursday" => Weekday::Thursday,
    "thu" => Weekday::Thursday,
    "thu." => Weekday::Thursday,
    "thur" => Weekday::Thursday,
    "thur." => Weekday::Thursday,
    "thurs" => Weekday::Thursday,
    "thurs." => Weekday::Thursday,
    "friday" => Weekday::Friday,
    "fri" => Weekday::Friday,
    "fri." => Weekday::Friday,
    "saturday" => Weekday::Saturday,
    "sat" => Weekday::Saturday,
    "sat." => Weekday::Saturday,
};

/// Month dictionary - maps lowercase month names to 1-based month numbers
pub static MONTH_MAP: phf::Map<&'static str, u32> = phf_map! {
    "january" => 1,
    "jan" => 1,
    "jan." => 1,
    "february" => 2,
    "feb" => 2,
    "feb." => 2,
    "march" => 3,
    "mar" => 3,
    "mar." => 3,
    "april" => 4,
    "apr" => 4,
    "apr." => 4,
    "may" => 5,
    "june" => 6,
    "jun" => 6,
    "jun." => 6,
    "july" => 7,
    "jul" => 7,
    "jul." => 7,
    "august" => 8,
    "aug" => 8,
    "aug." => 8,
    "september" => 9,
    "sep" => 9,
    "sep." => 9,
    "sept" => 9,
    "sept." => 9,
    "october" => 10,
    "oct" => 10,
    "oct." => 10,
    "november" => 11,
    "nov" => 11,
    "nov." => 11,
    "december" => 12,
    "dec" => 12,
    "dec." => 12,
};

/// Integer words dictionary
pub static INTEGER_WORD_MAP: phf::Map<&'static str, f64> = phf_map! {
    "one" => 1.0,
    "two" => 2.0,
    "three" => 3.0,
    "four" => 4.0,
    "five" => 5.0,
    "six" => 6.0,
    "seven" => 7.0,
    "eight" => 8.0,
    "nine" => 9.0,
    "ten" => 10.0,
    "eleven" => 11.0,
    "twelve" => 12.0,
};

/// Ordinal words dictionary
pub static ORDINAL_WORD_MAP: phf::Map<&'static str, u32> = phf_map! {
    "first" => 1,
    "second" => 2,
    "third" => 3,
    "fourth" => 4,
    "fifth" => 5,
    "sixth" => 6,
    "seventh" => 7,
    "eighth" => 8,
    "ninth" => 9,
    "tenth" => 10,
    "eleventh" => 11,
    "twelfth" => 12,
    "thirteenth" => 13,
    "fourteenth" => 14,
    "fifteenth" => 15,
    "sixteenth" => 16,
    "seventeenth" => 17,
    "eighteenth" => 18,
    "nineteenth" => 19,
    "twentieth" => 20,
    "twenty first" => 21,
    "twenty-first" => 21,
    "twenty second" => 22,
    "twenty-second" => 22,
    "twenty third" => 23,
    "twenty-third" => 23,
    "twenty fourth" => 24,
    "twenty-fourth" => 24,
    "twenty fifth" => 25,
    "twenty-fifth" => 25,
    "twenty sixth" => 26,
    "twenty-sixth" => 26,
    "twenty seventh" => 27,
    "twenty-seventh" => 27,
    "twenty eighth" => 28,
    "twenty-eighth" => 28,
    "twenty ninth" => 29,
    "twenty-ninth" => 29,
    "thirtieth" => 30,
    "thirty first" => 31,
    "thirty-first" => 31,
};

/// Time unit dictionary (with abbreviations)
pub static TIME_UNIT_MAP: phf::Map<&'static str, TimeUnit> = phf_map! {
    "second" => TimeUnit::Second,
    "seconds" => TimeUnit::Second,
    "sec" => TimeUnit::Second,
    "secs" => TimeUnit::Second,
    "s" => TimeUnit::Second,
    "minute" => TimeUnit::Minute,
    "minutes" => TimeUnit::Minute,
    "min" => TimeUnit::Minute,
    "mins" => TimeUnit::Minute,
    "m" => TimeUnit::Minute,
    "hour" => TimeUnit::Hour,
    "hours" => TimeUnit::Hour,
    "hr" => TimeUnit::Hour,
    "hrs" => TimeUnit::Hour,
    "h" => TimeUnit::Hour,
    "day" => TimeUnit::Day,
    "days" => TimeUnit::Day,
    "d" => TimeUnit::Day,
    "week" => TimeUnit::Week,
    "weeks" => TimeUnit::Week,
    "w" => TimeUnit::Week,
    "month" => TimeUnit::Month,
    "months" => TimeUnit::Month,
    "mo" => TimeUnit::Month,
    "mos" => TimeUnit::Month,
    "quarter" => TimeUnit::Quarter,
    "quarters" => TimeUnit::Quarter,
    "qtr" => TimeUnit::Quarter,
    "year" => TimeUnit::Year,
    "years" => TimeUnit::Year,
    "yr" => TimeUnit::Year,
    "yrs" => TimeUnit::Year,
    "y" => TimeUnit::Year,
};

/// Casual date keywords
pub static CASUAL_DATE_MAP: phf::Map<&'static str, CasualDateType> = phf_map! {
    "now" => CasualDateType::Now,
    "today" => CasualDateType::Today,
    "tonight" => CasualDateType::Tonight,
    "tomorrow" => CasualDateType::Tomorrow,
    "tmr" => CasualDateType::Tomorrow,
    "tmrw" => CasualDateType::Tomorrow,
    "yesterday" => CasualDateType::Yesterday,
    "overmorrow" => CasualDateType::Overmorrow,
};

/// Casual time keywords
pub static CASUAL_TIME_MAP: phf::Map<&'static str, CasualTimeType> = phf_map! {
    "noon" => CasualTimeType::Noon,
    "midday" => CasualTimeType::Noon,
    "midnight" => CasualTimeType::Midnight,
    "morning" => CasualTimeType::Morning,
    "afternoon" => CasualTimeType::Afternoon,
    "evening" => CasualTimeType::Evening,
    "night" => CasualTimeType::Night,
};

/// Relative modifiers
pub static RELATIVE_MODIFIER_MAP: phf::Map<&'static str, RelativeModifier> = phf_map! {
    "this" => RelativeModifier::This,
    "next" => RelativeModifier::Next,
    "last" => RelativeModifier::Last,
    "past" => RelativeModifier::Last,
    "previous" => RelativeModifier::Last,
    "coming" => RelativeModifier::Next,
    "following" => RelativeModifier::Next,
};

// ============================================================================
// Lookup functions
// ============================================================================

#[inline]
pub fn get_weekday(s: &str) -> Option<Weekday> {
    WEEKDAY_MAP.get(s).copied()
}

#[inline]
pub fn get_month(s: &str) -> Option<u32> {
    MONTH_MAP.get(s).copied()
}

#[inline]
pub fn get_integer_word(s: &str) -> Option<f64> {
    INTEGER_WORD_MAP.get(s).copied()
}

#[inline]
pub fn get_ordinal_word(s: &str) -> Option<u32> {
    ORDINAL_WORD_MAP.get(s).copied()
}

#[inline]
pub fn get_time_unit(s: &str) -> Option<TimeUnit> {
    TIME_UNIT_MAP.get(s).copied()
}

#[inline]
pub fn get_casual_date(s: &str) -> Option<CasualDateType> {
    CASUAL_DATE_MAP.get(s).copied()
}

#[inline]
pub fn get_casual_time(s: &str) -> Option<CasualTimeType> {
    CASUAL_TIME_MAP.get(s).copied()
}

#[inline]
pub fn get_relative_modifier(s: &str) -> Option<RelativeModifier> {
    RELATIVE_MODIFIER_MAP.get(s).copied()
}

/// Parse a number pattern (handles words, numbers, and special cases)
pub fn parse_number_pattern(text: &str) -> f64 {
    let lower = text.to_lowercase();

    if let Some(val) = get_integer_word(&lower) {
        return val;
    }

    match lower.as_str() {
        "a" | "an" | "the" => return 1.0,
        s if s.contains("few") => return 3.0,
        s if s.contains("half") => return 0.5,
        s if s.contains("couple") => return 2.0,
        s if s.contains("several") => return 7.0,
        _ => {}
    }

    text.parse::<f64>().unwrap_or(0.0)
}

/// Parse an ordinal number pattern
pub fn parse_ordinal_pattern(text: &str) -> Option<u32> {
    let lower = text.to_lowercase();

    if let Some(val) = get_ordinal_word(&lower) {
        return Some(val);
    }

    let cleaned = lower
        .trim_end_matches("st")
        .trim_end_matches("nd")
        .trim_end_matches("rd")
        .trim_end_matches("th");

    cleaned.parse::<u32>().ok()
}
