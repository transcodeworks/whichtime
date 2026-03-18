//! German (Deutsch) locale dictionaries

use super::{CasualDateType, CasualTimeType, RelativeModifier, TimeUnit, Weekday};
use phf::phf_map;

/// Weekday dictionary
pub static WEEKDAY_MAP: phf::Map<&'static str, Weekday> = phf_map! {
    "sonntag" => Weekday::Sunday,
    "so" => Weekday::Sunday,
    "so." => Weekday::Sunday,
    "montag" => Weekday::Monday,
    "mo" => Weekday::Monday,
    "mo." => Weekday::Monday,
    "dienstag" => Weekday::Tuesday,
    "di" => Weekday::Tuesday,
    "di." => Weekday::Tuesday,
    "mittwoch" => Weekday::Wednesday,
    "mi" => Weekday::Wednesday,
    "mi." => Weekday::Wednesday,
    "donnerstag" => Weekday::Thursday,
    "do" => Weekday::Thursday,
    "do." => Weekday::Thursday,
    "freitag" => Weekday::Friday,
    "fr" => Weekday::Friday,
    "fr." => Weekday::Friday,
    "samstag" => Weekday::Saturday,
    "sa" => Weekday::Saturday,
    "sa." => Weekday::Saturday,
};

/// Month dictionary
pub static MONTH_MAP: phf::Map<&'static str, u32> = phf_map! {
    "januar" => 1,
    "jänner" => 1,
    "janner" => 1,
    "jan" => 1,
    "jan." => 1,
    "februar" => 2,
    "feber" => 2,
    "feb" => 2,
    "feb." => 2,
    "märz" => 3,
    "maerz" => 3,
    "mär" => 3,
    "mär." => 3,
    "mrz" => 3,
    "mrz." => 3,
    "april" => 4,
    "apr" => 4,
    "apr." => 4,
    "mai" => 5,
    "juni" => 6,
    "jun" => 6,
    "jun." => 6,
    "juli" => 7,
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
    "oktober" => 10,
    "okt" => 10,
    "okt." => 10,
    "november" => 11,
    "nov" => 11,
    "nov." => 11,
    "dezember" => 12,
    "dez" => 12,
    "dez." => 12,
};

/// Integer words dictionary
pub static INTEGER_WORD_MAP: phf::Map<&'static str, f64> = phf_map! {
    "eins" => 1.0,
    "eine" => 1.0,
    "einem" => 1.0,
    "einen" => 1.0,
    "einer" => 1.0,
    "ein" => 1.0,
    "zwei" => 2.0,
    "drei" => 3.0,
    "vier" => 4.0,
    "fünf" => 5.0,
    "fuenf" => 5.0,
    "sechs" => 6.0,
    "sieben" => 7.0,
    "acht" => 8.0,
    "neun" => 9.0,
    "zehn" => 10.0,
    "elf" => 11.0,
    "zwölf" => 12.0,
    "zwoelf" => 12.0,
};

/// Time unit dictionary
pub static TIME_UNIT_MAP: phf::Map<&'static str, TimeUnit> = phf_map! {
    "sek" => TimeUnit::Second,
    "sekunde" => TimeUnit::Second,
    "sekunden" => TimeUnit::Second,
    "min" => TimeUnit::Minute,
    "minute" => TimeUnit::Minute,
    "minuten" => TimeUnit::Minute,
    "h" => TimeUnit::Hour,
    "std" => TimeUnit::Hour,
    "stunde" => TimeUnit::Hour,
    "stunden" => TimeUnit::Hour,
    "tag" => TimeUnit::Day,
    "tage" => TimeUnit::Day,
    "tagen" => TimeUnit::Day,
    "woche" => TimeUnit::Week,
    "wochen" => TimeUnit::Week,
    "monat" => TimeUnit::Month,
    "monate" => TimeUnit::Month,
    "monaten" => TimeUnit::Month,
    "monats" => TimeUnit::Month,
    "quartal" => TimeUnit::Quarter,
    "quartals" => TimeUnit::Quarter,
    "quartale" => TimeUnit::Quarter,
    "quartalen" => TimeUnit::Quarter,
    "a" => TimeUnit::Year,
    "j" => TimeUnit::Year,
    "jr" => TimeUnit::Year,
    "jahr" => TimeUnit::Year,
    "jahre" => TimeUnit::Year,
    "jahren" => TimeUnit::Year,
    "jahres" => TimeUnit::Year,
};

/// Casual date keywords
pub static CASUAL_DATE_MAP: phf::Map<&'static str, CasualDateType> = phf_map! {
    "jetzt" => CasualDateType::Now,
    "heute" => CasualDateType::Today,
    "heute nacht" => CasualDateType::Tonight,
    "morgen" => CasualDateType::Tomorrow,
    "gestern" => CasualDateType::Yesterday,
    "übermorgen" => CasualDateType::Overmorrow,
    "uebermorgen" => CasualDateType::Overmorrow,
    "vorgestern" => CasualDateType::DayBeforeYesterday,
};

/// Casual time keywords
pub static CASUAL_TIME_MAP: phf::Map<&'static str, CasualTimeType> = phf_map! {
    "mittag" => CasualTimeType::Noon,
    "mitternacht" => CasualTimeType::Midnight,
    "morgen" => CasualTimeType::Morning,
    "morgens" => CasualTimeType::Morning,
    "vormittag" => CasualTimeType::Morning,
    "vormittags" => CasualTimeType::Morning,
    "nachmittag" => CasualTimeType::Afternoon,
    "nachmittags" => CasualTimeType::Afternoon,
    "abend" => CasualTimeType::Evening,
    "abends" => CasualTimeType::Evening,
    "nacht" => CasualTimeType::Night,
    "nachts" => CasualTimeType::Night,
};

/// Relative modifiers
pub static RELATIVE_MODIFIER_MAP: phf::Map<&'static str, RelativeModifier> = phf_map! {
    "dieser" => RelativeModifier::This,
    "diese" => RelativeModifier::This,
    "diesen" => RelativeModifier::This,
    "dieses" => RelativeModifier::This,
    "nächster" => RelativeModifier::Next,
    "nächste" => RelativeModifier::Next,
    "nächsten" => RelativeModifier::Next,
    "nächstes" => RelativeModifier::Next,
    "naechster" => RelativeModifier::Next,
    "naechste" => RelativeModifier::Next,
    "naechsten" => RelativeModifier::Next,
    "naechstes" => RelativeModifier::Next,
    "kommender" => RelativeModifier::Next,
    "kommende" => RelativeModifier::Next,
    "kommenden" => RelativeModifier::Next,
    "kommendes" => RelativeModifier::Next,
    "letzter" => RelativeModifier::Last,
    "letzte" => RelativeModifier::Last,
    "letzten" => RelativeModifier::Last,
    "letztes" => RelativeModifier::Last,
    "vergangener" => RelativeModifier::Last,
    "vergangene" => RelativeModifier::Last,
    "vergangenen" => RelativeModifier::Last,
    "vergangenes" => RelativeModifier::Last,
    "voriger" => RelativeModifier::Last,
    "vorige" => RelativeModifier::Last,
    "vorigen" => RelativeModifier::Last,
    "voriges" => RelativeModifier::Last,
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
        s if s.contains("wenige") => return 2.0,
        s if s.contains("paar") => return 2.0,
        s if s.contains("halb") || s.contains("halben") => return 0.5,
        s if s.contains("einig") => return 3.0,
        s if s.contains("mehre") => return 7.0,
        _ => {}
    }

    text.parse::<f64>().unwrap_or(0.0)
}
