//! Swedish (Svenska) locale dictionaries

use super::{CasualDateType, CasualTimeType, RelativeModifier, TimeUnit, Weekday};
use phf::phf_map;

/// Weekday dictionary
pub static WEEKDAY_MAP: phf::Map<&'static str, Weekday> = phf_map! {
    "söndag" => Weekday::Sunday,
    "sondag" => Weekday::Sunday,
    "sön" => Weekday::Sunday,
    "son" => Weekday::Sunday,
    "sön." => Weekday::Sunday,
    "son." => Weekday::Sunday,
    "måndag" => Weekday::Monday,
    "mandag" => Weekday::Monday,
    "mån" => Weekday::Monday,
    "man" => Weekday::Monday,
    "mån." => Weekday::Monday,
    "man." => Weekday::Monday,
    "tisdag" => Weekday::Tuesday,
    "tis" => Weekday::Tuesday,
    "tis." => Weekday::Tuesday,
    "onsdag" => Weekday::Wednesday,
    "ons" => Weekday::Wednesday,
    "ons." => Weekday::Wednesday,
    "torsdag" => Weekday::Thursday,
    "tor" => Weekday::Thursday,
    "tor." => Weekday::Thursday,
    "tors" => Weekday::Thursday,
    "tors." => Weekday::Thursday,
    "fredag" => Weekday::Friday,
    "fre" => Weekday::Friday,
    "fre." => Weekday::Friday,
    "lördag" => Weekday::Saturday,
    "lordag" => Weekday::Saturday,
    "lör" => Weekday::Saturday,
    "lor" => Weekday::Saturday,
    "lör." => Weekday::Saturday,
    "lor." => Weekday::Saturday,
};

/// Month dictionary
pub static MONTH_MAP: phf::Map<&'static str, u32> = phf_map! {
    "januari" => 1,
    "jan" => 1,
    "jan." => 1,
    "februari" => 2,
    "feb" => 2,
    "feb." => 2,
    "mars" => 3,
    "mar" => 3,
    "mar." => 3,
    "april" => 4,
    "apr" => 4,
    "apr." => 4,
    "maj" => 5,
    "juni" => 6,
    "jun" => 6,
    "jun." => 6,
    "juli" => 7,
    "jul" => 7,
    "jul." => 7,
    "augusti" => 8,
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
    "en" => 1.0,
    "ett" => 1.0,
    "två" => 2.0,
    "tva" => 2.0,
    "tre" => 3.0,
    "fyra" => 4.0,
    "fem" => 5.0,
    "sex" => 6.0,
    "sju" => 7.0,
    "åtta" => 8.0,
    "atta" => 8.0,
    "nio" => 9.0,
    "tio" => 10.0,
    "elva" => 11.0,
    "tolv" => 12.0,
};

/// Time unit dictionary
pub static TIME_UNIT_MAP: phf::Map<&'static str, TimeUnit> = phf_map! {
    "sek" => TimeUnit::Second,
    "sekund" => TimeUnit::Second,
    "sekunder" => TimeUnit::Second,
    "min" => TimeUnit::Minute,
    "minut" => TimeUnit::Minute,
    "minuter" => TimeUnit::Minute,
    "tim" => TimeUnit::Hour,
    "timme" => TimeUnit::Hour,
    "timmar" => TimeUnit::Hour,
    "dag" => TimeUnit::Day,
    "dagar" => TimeUnit::Day,
    "vecka" => TimeUnit::Week,
    "veckor" => TimeUnit::Week,
    "månad" => TimeUnit::Month,
    "manad" => TimeUnit::Month,
    "månader" => TimeUnit::Month,
    "manader" => TimeUnit::Month,
    "kvartal" => TimeUnit::Quarter,
    "år" => TimeUnit::Year,
    "ar" => TimeUnit::Year,
};

/// Casual date keywords
pub static CASUAL_DATE_MAP: phf::Map<&'static str, CasualDateType> = phf_map! {
    "nu" => CasualDateType::Now,
    "idag" => CasualDateType::Today,
    "i dag" => CasualDateType::Today,
    "ikväll" => CasualDateType::Tonight,
    "i kväll" => CasualDateType::Tonight,
    "ikvall" => CasualDateType::Tonight,
    "i kvall" => CasualDateType::Tonight,
    "imorgon" => CasualDateType::Tomorrow,
    "i morgon" => CasualDateType::Tomorrow,
    "igår" => CasualDateType::Yesterday,
    "i går" => CasualDateType::Yesterday,
    "igar" => CasualDateType::Yesterday,
    "i gar" => CasualDateType::Yesterday,
    "övermorgon" => CasualDateType::Overmorrow,
    "i övermorgon" => CasualDateType::Overmorrow,
    "overmorgon" => CasualDateType::Overmorrow,
    "i overmorgon" => CasualDateType::Overmorrow,
    "förrgår" => CasualDateType::DayBeforeYesterday,
    "i förrgår" => CasualDateType::DayBeforeYesterday,
    "forrgar" => CasualDateType::DayBeforeYesterday,
    "i forrgar" => CasualDateType::DayBeforeYesterday,
};

/// Casual time keywords
pub static CASUAL_TIME_MAP: phf::Map<&'static str, CasualTimeType> = phf_map! {
    "middag" => CasualTimeType::Noon,
    "mitt på dagen" => CasualTimeType::Noon,
    "midnatt" => CasualTimeType::Midnight,
    "morgon" => CasualTimeType::Morning,
    "på morgonen" => CasualTimeType::Morning,
    "eftermiddag" => CasualTimeType::Afternoon,
    "på eftermiddagen" => CasualTimeType::Afternoon,
    "kväll" => CasualTimeType::Evening,
    "kvall" => CasualTimeType::Evening,
    "på kvällen" => CasualTimeType::Evening,
    "natt" => CasualTimeType::Night,
    "på natten" => CasualTimeType::Night,
};

/// Relative modifiers
pub static RELATIVE_MODIFIER_MAP: phf::Map<&'static str, RelativeModifier> = phf_map! {
    "denna" => RelativeModifier::This,
    "detta" => RelativeModifier::This,
    "den här" => RelativeModifier::This,
    "det här" => RelativeModifier::This,
    "nästa" => RelativeModifier::Next,
    "nasta" => RelativeModifier::Next,
    "kommande" => RelativeModifier::Next,
    "förra" => RelativeModifier::Last,
    "forra" => RelativeModifier::Last,
    "senaste" => RelativeModifier::Last,
    "sista" => RelativeModifier::Last,
    "föregående" => RelativeModifier::Last,
    "foregaende" => RelativeModifier::Last,
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
        s if s.contains("några") || s.contains("nagra") => return 3.0,
        s if s.contains("halv") => return 0.5,
        s if s.contains("par") => return 2.0,
        s if s.contains("flera") => return 7.0,
        _ => {}
    }

    text.parse::<f64>().unwrap_or(0.0)
}
