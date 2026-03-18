//! Dutch (Nederlands) locale dictionaries

use super::{CasualDateType, CasualTimeType, RelativeModifier, TimeUnit, Weekday};
use phf::phf_map;

/// Weekday dictionary
pub static WEEKDAY_MAP: phf::Map<&'static str, Weekday> = phf_map! {
    "zondag" => Weekday::Sunday,
    "zon" => Weekday::Sunday,
    "zo" => Weekday::Sunday,
    "zo." => Weekday::Sunday,
    "maandag" => Weekday::Monday,
    "maan" => Weekday::Monday,
    "ma" => Weekday::Monday,
    "ma." => Weekday::Monday,
    "dinsdag" => Weekday::Tuesday,
    "dins" => Weekday::Tuesday,
    "di" => Weekday::Tuesday,
    "di." => Weekday::Tuesday,
    "woensdag" => Weekday::Wednesday,
    "woens" => Weekday::Wednesday,
    "wo" => Weekday::Wednesday,
    "wo." => Weekday::Wednesday,
    "donderdag" => Weekday::Thursday,
    "donder" => Weekday::Thursday,
    "do" => Weekday::Thursday,
    "do." => Weekday::Thursday,
    "vrijdag" => Weekday::Friday,
    "vrij" => Weekday::Friday,
    "vr" => Weekday::Friday,
    "vr." => Weekday::Friday,
    "zaterdag" => Weekday::Saturday,
    "zater" => Weekday::Saturday,
    "za" => Weekday::Saturday,
    "za." => Weekday::Saturday,
};

/// Month dictionary
pub static MONTH_MAP: phf::Map<&'static str, u32> = phf_map! {
    "januari" => 1,
    "jan" => 1,
    "jan." => 1,
    "februari" => 2,
    "feb" => 2,
    "feb." => 2,
    "maart" => 3,
    "mrt" => 3,
    "mrt." => 3,
    "april" => 4,
    "apr" => 4,
    "apr." => 4,
    "mei" => 5,
    "juni" => 6,
    "jun" => 6,
    "jun." => 6,
    "juli" => 7,
    "jul" => 7,
    "jul." => 7,
    "augustus" => 8,
    "aug" => 8,
    "aug." => 8,
    "september" => 9,
    "sep" => 9,
    "sep." => 9,
    "sept" => 9,
    "sept." => 9,
    "oktober" => 10,
    "okt" => 10,
    "okt." => 10,
    "november" => 11,
    "nov" => 11,
    "nov." => 11,
    "december" => 12,
    "dec" => 12,
    "dec." => 12,
};

/// Integer words dictionary
pub static INTEGER_WORD_MAP: phf::Map<&'static str, f64> = phf_map! {
    "een" => 1.0,
    "één" => 1.0,
    "twee" => 2.0,
    "drie" => 3.0,
    "vier" => 4.0,
    "vijf" => 5.0,
    "zes" => 6.0,
    "zeven" => 7.0,
    "acht" => 8.0,
    "negen" => 9.0,
    "tien" => 10.0,
    "elf" => 11.0,
    "twaalf" => 12.0,
};

/// Time unit dictionary
pub static TIME_UNIT_MAP: phf::Map<&'static str, TimeUnit> = phf_map! {
    "sec" => TimeUnit::Second,
    "seconde" => TimeUnit::Second,
    "seconden" => TimeUnit::Second,
    "secondes" => TimeUnit::Second,
    "min" => TimeUnit::Minute,
    "mins" => TimeUnit::Minute,
    "minuut" => TimeUnit::Minute,
    "minuten" => TimeUnit::Minute,
    "u" => TimeUnit::Hour,
    "uur" => TimeUnit::Hour,
    "uren" => TimeUnit::Hour,
    "dag" => TimeUnit::Day,
    "dagen" => TimeUnit::Day,
    "week" => TimeUnit::Week,
    "weken" => TimeUnit::Week,
    "maand" => TimeUnit::Month,
    "maanden" => TimeUnit::Month,
    "kwartaal" => TimeUnit::Quarter,
    "kwartalen" => TimeUnit::Quarter,
    "jaar" => TimeUnit::Year,
    "jaren" => TimeUnit::Year,
};

/// Casual date keywords
pub static CASUAL_DATE_MAP: phf::Map<&'static str, CasualDateType> = phf_map! {
    "nu" => CasualDateType::Now,
    "vandaag" => CasualDateType::Today,
    "vanavond" => CasualDateType::Tonight,
    "morgen" => CasualDateType::Tomorrow,
    "gisteren" => CasualDateType::Yesterday,
    "overmorgen" => CasualDateType::Overmorrow,
    "eergisteren" => CasualDateType::DayBeforeYesterday,
};

/// Casual time keywords
pub static CASUAL_TIME_MAP: phf::Map<&'static str, CasualTimeType> = phf_map! {
    "middag" => CasualTimeType::Noon,
    "twaalf uur" => CasualTimeType::Noon,
    "middernacht" => CasualTimeType::Midnight,
    "ochtend" => CasualTimeType::Morning,
    "'s ochtends" => CasualTimeType::Morning,
    "namiddag" => CasualTimeType::Afternoon,
    "'s middags" => CasualTimeType::Afternoon,
    "avond" => CasualTimeType::Evening,
    "'s avonds" => CasualTimeType::Evening,
    "nacht" => CasualTimeType::Night,
    "'s nachts" => CasualTimeType::Night,
};

/// Relative modifiers
pub static RELATIVE_MODIFIER_MAP: phf::Map<&'static str, RelativeModifier> = phf_map! {
    "deze" => RelativeModifier::This,
    "dit" => RelativeModifier::This,
    "volgende" => RelativeModifier::Next,
    "volgend" => RelativeModifier::Next,
    "komende" => RelativeModifier::Next,
    "komend" => RelativeModifier::Next,
    "afgelopen" => RelativeModifier::Last,
    "vorige" => RelativeModifier::Last,
    "vorig" => RelativeModifier::Last,
    "laatste" => RelativeModifier::Last,
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

/// Parse a number pattern
pub fn parse_number_pattern(text: &str) -> f64 {
    let lower = text.to_lowercase();

    if let Some(val) = get_integer_word(&lower) {
        return val;
    }

    match lower.as_str() {
        s if s.contains("paar") => return 2.0,
        s if s.contains("half") => return 0.5,
        s if s.contains("enkele") || s.contains("enkel") => return 3.0,
        s if s.contains("meerdere") || s.contains("verschillende") => return 7.0,
        _ => {}
    }

    text.parse::<f64>().unwrap_or(0.0)
}
