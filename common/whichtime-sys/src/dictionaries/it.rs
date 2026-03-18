//! Italian (Italiano) locale dictionaries

use super::{CasualDateType, CasualTimeType, RelativeModifier, TimeUnit, Weekday};
use phf::phf_map;

/// Weekday dictionary
pub static WEEKDAY_MAP: phf::Map<&'static str, Weekday> = phf_map! {
    "domenica" => Weekday::Sunday,
    "dom" => Weekday::Sunday,
    "dom." => Weekday::Sunday,
    "lunedì" => Weekday::Monday,
    "lunedi" => Weekday::Monday,
    "lun" => Weekday::Monday,
    "lun." => Weekday::Monday,
    "martedì" => Weekday::Tuesday,
    "martedi" => Weekday::Tuesday,
    "mar" => Weekday::Tuesday,
    "mar." => Weekday::Tuesday,
    "mercoledì" => Weekday::Wednesday,
    "mercoledi" => Weekday::Wednesday,
    "merc" => Weekday::Wednesday,
    "merc." => Weekday::Wednesday,
    "mer" => Weekday::Wednesday,
    "mer." => Weekday::Wednesday,
    "giovedì" => Weekday::Thursday,
    "giovedi" => Weekday::Thursday,
    "giov" => Weekday::Thursday,
    "giov." => Weekday::Thursday,
    "gio" => Weekday::Thursday,
    "gio." => Weekday::Thursday,
    "venerdì" => Weekday::Friday,
    "venerdi" => Weekday::Friday,
    "ven" => Weekday::Friday,
    "ven." => Weekday::Friday,
    "sabato" => Weekday::Saturday,
    "sab" => Weekday::Saturday,
    "sab." => Weekday::Saturday,
};

/// Month dictionary
pub static MONTH_MAP: phf::Map<&'static str, u32> = phf_map! {
    "gennaio" => 1,
    "gen" => 1,
    "gen." => 1,
    "febbraio" => 2,
    "febraio" => 2,
    "feb" => 2,
    "feb." => 2,
    "febb" => 2,
    "febb." => 2,
    "marzo" => 3,
    "mar" => 3,
    "mar." => 3,
    "aprile" => 4,
    "apr" => 4,
    "apr." => 4,
    "maggio" => 5,
    "mag" => 5,
    "mag." => 5,
    "giugno" => 6,
    "giu" => 6,
    "giu." => 6,
    "luglio" => 7,
    "lug" => 7,
    "lug." => 7,
    "lugl" => 7,
    "lugl." => 7,
    "agosto" => 8,
    "ago" => 8,
    "ago." => 8,
    "settembre" => 9,
    "set" => 9,
    "set." => 9,
    "sett" => 9,
    "sett." => 9,
    "ottobre" => 10,
    "ott" => 10,
    "ott." => 10,
    "novembre" => 11,
    "nov" => 11,
    "nov." => 11,
    "dicembre" => 12,
    "dic" => 12,
    "dic." => 12,
    "dice" => 12,
};

/// Integer words dictionary
pub static INTEGER_WORD_MAP: phf::Map<&'static str, f64> = phf_map! {
    "uno" => 1.0,
    "una" => 1.0,
    "un" => 1.0,
    "due" => 2.0,
    "tre" => 3.0,
    "quattro" => 4.0,
    "cinque" => 5.0,
    "sei" => 6.0,
    "sette" => 7.0,
    "otto" => 8.0,
    "nove" => 9.0,
    "dieci" => 10.0,
    "undici" => 11.0,
    "dodici" => 12.0,
};

/// Ordinal words dictionary
pub static ORDINAL_WORD_MAP: phf::Map<&'static str, u32> = phf_map! {
    "primo" => 1,
    "secondo" => 2,
    "terzo" => 3,
    "quarto" => 4,
    "quinto" => 5,
    "sesto" => 6,
    "settimo" => 7,
    "ottavo" => 8,
    "nono" => 9,
    "decimo" => 10,
    "undicesimo" => 11,
    "dodicesimo" => 12,
    "tredicesimo" => 13,
    "quattordicesimo" => 14,
    "quindicesimo" => 15,
    "sedicesimo" => 16,
    "diciassettesimo" => 17,
    "diciottesimo" => 18,
    "diciannovesimo" => 19,
    "ventesimo" => 20,
    "ventunesimo" => 21,
    "ventiduesimo" => 22,
    "ventitreesimo" => 23,
    "ventiquattresimo" => 24,
    "venticinquesimo" => 25,
    "ventiseiesimo" => 26,
    "ventisettesimo" => 27,
    "ventottesimo" => 28,
    "ventinovesimo" => 29,
    "trentesimo" => 30,
    "trentunesimo" => 31,
};

/// Time unit dictionary
pub static TIME_UNIT_MAP: phf::Map<&'static str, TimeUnit> = phf_map! {
    "sec" => TimeUnit::Second,
    "secondo" => TimeUnit::Second,
    "secondi" => TimeUnit::Second,
    "min" => TimeUnit::Minute,
    "mins" => TimeUnit::Minute,
    "minuto" => TimeUnit::Minute,
    "minuti" => TimeUnit::Minute,
    "h" => TimeUnit::Hour,
    "hr" => TimeUnit::Hour,
    "o" => TimeUnit::Hour,
    "ora" => TimeUnit::Hour,
    "ore" => TimeUnit::Hour,
    "giorno" => TimeUnit::Day,
    "giorni" => TimeUnit::Day,
    "settimana" => TimeUnit::Week,
    "settimane" => TimeUnit::Week,
    "mese" => TimeUnit::Month,
    "mesi" => TimeUnit::Month,
    "trimestre" => TimeUnit::Quarter,
    "trimestri" => TimeUnit::Quarter,
    "anno" => TimeUnit::Year,
    "anni" => TimeUnit::Year,
};

/// Casual date keywords
pub static CASUAL_DATE_MAP: phf::Map<&'static str, CasualDateType> = phf_map! {
    "adesso" => CasualDateType::Now,
    "ora" => CasualDateType::Now,
    "oggi" => CasualDateType::Today,
    "stasera" => CasualDateType::Tonight,
    "stanotte" => CasualDateType::Tonight,
    "stamattina" => CasualDateType::ThisMorning,
    "domani" => CasualDateType::Tomorrow,
    "ieri" => CasualDateType::Yesterday,
    "dopodomani" => CasualDateType::Overmorrow,
    "l'altro ieri" => CasualDateType::DayBeforeYesterday,
    "altroieri" => CasualDateType::DayBeforeYesterday,
};

/// Casual time keywords
pub static CASUAL_TIME_MAP: phf::Map<&'static str, CasualTimeType> = phf_map! {
    "mezzogiorno" => CasualTimeType::Noon,
    "mezzanotte" => CasualTimeType::Midnight,
    "mattina" => CasualTimeType::Morning,
    "mattino" => CasualTimeType::Morning,
    "pomeriggio" => CasualTimeType::Afternoon,
    "sera" => CasualTimeType::Evening,
    "notte" => CasualTimeType::Night,
};

/// Relative modifiers
pub static RELATIVE_MODIFIER_MAP: phf::Map<&'static str, RelativeModifier> = phf_map! {
    "questo" => RelativeModifier::This,
    "questa" => RelativeModifier::This,
    "questi" => RelativeModifier::This,
    "queste" => RelativeModifier::This,
    "prossimo" => RelativeModifier::Next,
    "prossima" => RelativeModifier::Next,
    "prossimi" => RelativeModifier::Next,
    "prossime" => RelativeModifier::Next,
    "seguente" => RelativeModifier::Next,
    "scorso" => RelativeModifier::Last,
    "scorsa" => RelativeModifier::Last,
    "scorsi" => RelativeModifier::Last,
    "scorse" => RelativeModifier::Last,
    "passato" => RelativeModifier::Last,
    "passata" => RelativeModifier::Last,
    "ultimo" => RelativeModifier::Last,
    "ultima" => RelativeModifier::Last,
    "precedente" => RelativeModifier::Last,
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

/// Parse a number pattern
pub fn parse_number_pattern(text: &str) -> f64 {
    let lower = text.to_lowercase();

    if let Some(val) = get_integer_word(&lower) {
        return val;
    }

    match lower.as_str() {
        s if s.contains("alcuni") => return 3.0,
        s if s.contains("metà") || s.contains("meta") => return 0.5,
        s if s.contains("paio") => return 2.0,
        s if s.contains("molti") => return 7.0,
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
        .trim_end_matches("°")
        .trim_end_matches("imo")
        .trim_end_matches("esimo");

    cleaned.parse::<u32>().ok()
}
