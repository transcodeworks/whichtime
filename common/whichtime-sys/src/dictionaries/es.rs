//! Spanish (Español) locale dictionaries

use super::{CasualDateType, CasualTimeType, RelativeModifier, TimeUnit, Weekday};
use phf::phf_map;

/// Weekday dictionary
pub static WEEKDAY_MAP: phf::Map<&'static str, Weekday> = phf_map! {
    "domingo" => Weekday::Sunday,
    "dom" => Weekday::Sunday,
    "dom." => Weekday::Sunday,
    "lunes" => Weekday::Monday,
    "lun" => Weekday::Monday,
    "lun." => Weekday::Monday,
    "martes" => Weekday::Tuesday,
    "mar" => Weekday::Tuesday,
    "mar." => Weekday::Tuesday,
    "miércoles" => Weekday::Wednesday,
    "miercoles" => Weekday::Wednesday,
    "mié" => Weekday::Wednesday,
    "mie" => Weekday::Wednesday,
    "mié." => Weekday::Wednesday,
    "mie." => Weekday::Wednesday,
    "jueves" => Weekday::Thursday,
    "jue" => Weekday::Thursday,
    "jue." => Weekday::Thursday,
    "viernes" => Weekday::Friday,
    "vie" => Weekday::Friday,
    "vie." => Weekday::Friday,
    "sábado" => Weekday::Saturday,
    "sabado" => Weekday::Saturday,
    "sáb" => Weekday::Saturday,
    "sab" => Weekday::Saturday,
    "sáb." => Weekday::Saturday,
    "sab." => Weekday::Saturday,
};

/// Month dictionary
pub static MONTH_MAP: phf::Map<&'static str, u32> = phf_map! {
    "enero" => 1,
    "ene" => 1,
    "ene." => 1,
    "febrero" => 2,
    "feb" => 2,
    "feb." => 2,
    "marzo" => 3,
    "mar" => 3,
    "mar." => 3,
    "abril" => 4,
    "abr" => 4,
    "abr." => 4,
    "mayo" => 5,
    "may" => 5,
    "may." => 5,
    "junio" => 6,
    "jun" => 6,
    "jun." => 6,
    "julio" => 7,
    "jul" => 7,
    "jul." => 7,
    "agosto" => 8,
    "ago" => 8,
    "ago." => 8,
    "septiembre" => 9,
    "setiembre" => 9,
    "sep" => 9,
    "sep." => 9,
    "set" => 9,
    "set." => 9,
    "octubre" => 10,
    "oct" => 10,
    "oct." => 10,
    "noviembre" => 11,
    "nov" => 11,
    "nov." => 11,
    "diciembre" => 12,
    "dic" => 12,
    "dic." => 12,
};

/// Integer words dictionary
pub static INTEGER_WORD_MAP: phf::Map<&'static str, f64> = phf_map! {
    "uno" => 1.0,
    "una" => 1.0,
    "un" => 1.0,
    "dos" => 2.0,
    "tres" => 3.0,
    "cuatro" => 4.0,
    "cinco" => 5.0,
    "seis" => 6.0,
    "siete" => 7.0,
    "ocho" => 8.0,
    "nueve" => 9.0,
    "diez" => 10.0,
    "once" => 11.0,
    "doce" => 12.0,
    "trece" => 13.0,
};

/// Time unit dictionary
pub static TIME_UNIT_MAP: phf::Map<&'static str, TimeUnit> = phf_map! {
    "sec" => TimeUnit::Second,
    "segundo" => TimeUnit::Second,
    "segundos" => TimeUnit::Second,
    "min" => TimeUnit::Minute,
    "mins" => TimeUnit::Minute,
    "minuto" => TimeUnit::Minute,
    "minutos" => TimeUnit::Minute,
    "h" => TimeUnit::Hour,
    "hr" => TimeUnit::Hour,
    "hrs" => TimeUnit::Hour,
    "hora" => TimeUnit::Hour,
    "horas" => TimeUnit::Hour,
    "día" => TimeUnit::Day,
    "dia" => TimeUnit::Day,
    "días" => TimeUnit::Day,
    "dias" => TimeUnit::Day,
    "semana" => TimeUnit::Week,
    "semanas" => TimeUnit::Week,
    "mes" => TimeUnit::Month,
    "meses" => TimeUnit::Month,
    "trimestre" => TimeUnit::Quarter,
    "trimestres" => TimeUnit::Quarter,
    "año" => TimeUnit::Year,
    "ano" => TimeUnit::Year,
    "años" => TimeUnit::Year,
    "anos" => TimeUnit::Year,
};

/// Casual date keywords
pub static CASUAL_DATE_MAP: phf::Map<&'static str, CasualDateType> = phf_map! {
    "ahora" => CasualDateType::Now,
    "hoy" => CasualDateType::Today,
    "esta noche" => CasualDateType::Tonight,
    "esta mañana" => CasualDateType::ThisMorning,
    "esta manana" => CasualDateType::ThisMorning,
    "esta tarde" => CasualDateType::ThisAfternoon,
    "mañana" => CasualDateType::Tomorrow,
    "manana" => CasualDateType::Tomorrow,
    "ayer" => CasualDateType::Yesterday,
    "pasado mañana" => CasualDateType::Overmorrow,
    "pasado manana" => CasualDateType::Overmorrow,
    "anteayer" => CasualDateType::DayBeforeYesterday,
    "antes de ayer" => CasualDateType::DayBeforeYesterday,
};

/// Casual time keywords
pub static CASUAL_TIME_MAP: phf::Map<&'static str, CasualTimeType> = phf_map! {
    "mediodía" => CasualTimeType::Noon,
    "mediodia" => CasualTimeType::Noon,
    "medianoche" => CasualTimeType::Midnight,
    "mañana" => CasualTimeType::Morning,
    "manana" => CasualTimeType::Morning,
    "tarde" => CasualTimeType::Afternoon,
    "noche" => CasualTimeType::Night,
};

/// Relative modifiers
pub static RELATIVE_MODIFIER_MAP: phf::Map<&'static str, RelativeModifier> = phf_map! {
    "este" => RelativeModifier::This,
    "esta" => RelativeModifier::This,
    "estos" => RelativeModifier::This,
    "estas" => RelativeModifier::This,
    "próximo" => RelativeModifier::Next,
    "proximo" => RelativeModifier::Next,
    "próxima" => RelativeModifier::Next,
    "proxima" => RelativeModifier::Next,
    "siguiente" => RelativeModifier::Next,
    "pasado" => RelativeModifier::Last,
    "pasada" => RelativeModifier::Last,
    "último" => RelativeModifier::Last,
    "ultimo" => RelativeModifier::Last,
    "última" => RelativeModifier::Last,
    "ultima" => RelativeModifier::Last,
    "anterior" => RelativeModifier::Last,
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
        s if s.contains("algunos") || s.contains("unos") => return 3.0,
        s if s.contains("media") => return 0.5,
        _ => {}
    }

    text.parse::<f64>().unwrap_or(0.0)
}
