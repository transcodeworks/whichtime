//! Portuguese (Português) locale dictionaries

use super::{CasualDateType, CasualTimeType, RelativeModifier, TimeUnit, Weekday};
use phf::phf_map;

/// Weekday dictionary
pub static WEEKDAY_MAP: phf::Map<&'static str, Weekday> = phf_map! {
    "domingo" => Weekday::Sunday,
    "dom" => Weekday::Sunday,
    "dom." => Weekday::Sunday,
    "segunda" => Weekday::Monday,
    "segunda-feira" => Weekday::Monday,
    "seg" => Weekday::Monday,
    "seg." => Weekday::Monday,
    "terça" => Weekday::Tuesday,
    "terca" => Weekday::Tuesday,
    "terça-feira" => Weekday::Tuesday,
    "terca-feira" => Weekday::Tuesday,
    "ter" => Weekday::Tuesday,
    "ter." => Weekday::Tuesday,
    "quarta" => Weekday::Wednesday,
    "quarta-feira" => Weekday::Wednesday,
    "qua" => Weekday::Wednesday,
    "qua." => Weekday::Wednesday,
    "quinta" => Weekday::Thursday,
    "quinta-feira" => Weekday::Thursday,
    "qui" => Weekday::Thursday,
    "qui." => Weekday::Thursday,
    "sexta" => Weekday::Friday,
    "sexta-feira" => Weekday::Friday,
    "sex" => Weekday::Friday,
    "sex." => Weekday::Friday,
    "sábado" => Weekday::Saturday,
    "sabado" => Weekday::Saturday,
    "sab" => Weekday::Saturday,
    "sab." => Weekday::Saturday,
};

/// Month dictionary
pub static MONTH_MAP: phf::Map<&'static str, u32> = phf_map! {
    "janeiro" => 1,
    "jan" => 1,
    "jan." => 1,
    "fevereiro" => 2,
    "fev" => 2,
    "fev." => 2,
    "março" => 3,
    "marco" => 3,
    "mar" => 3,
    "mar." => 3,
    "abril" => 4,
    "abr" => 4,
    "abr." => 4,
    "maio" => 5,
    "mai" => 5,
    "mai." => 5,
    "junho" => 6,
    "jun" => 6,
    "jun." => 6,
    "julho" => 7,
    "jul" => 7,
    "jul." => 7,
    "agosto" => 8,
    "ago" => 8,
    "ago." => 8,
    "setembro" => 9,
    "set" => 9,
    "set." => 9,
    "outubro" => 10,
    "out" => 10,
    "out." => 10,
    "novembro" => 11,
    "nov" => 11,
    "nov." => 11,
    "dezembro" => 12,
    "dez" => 12,
    "dez." => 12,
};

/// Integer words dictionary
pub static INTEGER_WORD_MAP: phf::Map<&'static str, f64> = phf_map! {
    "um" => 1.0,
    "uma" => 1.0,
    "dois" => 2.0,
    "duas" => 2.0,
    "três" => 3.0,
    "tres" => 3.0,
    "quatro" => 4.0,
    "cinco" => 5.0,
    "seis" => 6.0,
    "sete" => 7.0,
    "oito" => 8.0,
    "nove" => 9.0,
    "dez" => 10.0,
    "onze" => 11.0,
    "doze" => 12.0,
};

/// Time unit dictionary
pub static TIME_UNIT_MAP: phf::Map<&'static str, TimeUnit> = phf_map! {
    "seg" => TimeUnit::Second,
    "segundo" => TimeUnit::Second,
    "segundos" => TimeUnit::Second,
    "min" => TimeUnit::Minute,
    "mins" => TimeUnit::Minute,
    "minuto" => TimeUnit::Minute,
    "minutos" => TimeUnit::Minute,
    "h" => TimeUnit::Hour,
    "hr" => TimeUnit::Hour,
    "hora" => TimeUnit::Hour,
    "horas" => TimeUnit::Hour,
    "dia" => TimeUnit::Day,
    "dias" => TimeUnit::Day,
    "semana" => TimeUnit::Week,
    "semanas" => TimeUnit::Week,
    "mês" => TimeUnit::Month,
    "mes" => TimeUnit::Month,
    "meses" => TimeUnit::Month,
    "trimestre" => TimeUnit::Quarter,
    "trimestres" => TimeUnit::Quarter,
    "ano" => TimeUnit::Year,
    "anos" => TimeUnit::Year,
};

/// Casual date keywords
pub static CASUAL_DATE_MAP: phf::Map<&'static str, CasualDateType> = phf_map! {
    "agora" => CasualDateType::Now,
    "hoje" => CasualDateType::Today,
    "hoje à noite" => CasualDateType::Tonight,
    "hoje a noite" => CasualDateType::Tonight,
    "amanhã" => CasualDateType::Tomorrow,
    "amanha" => CasualDateType::Tomorrow,
    "ontem" => CasualDateType::Yesterday,
    "depois de amanhã" => CasualDateType::Overmorrow,
    "depois de amanha" => CasualDateType::Overmorrow,
    "anteontem" => CasualDateType::DayBeforeYesterday,
};

/// Casual time keywords
pub static CASUAL_TIME_MAP: phf::Map<&'static str, CasualTimeType> = phf_map! {
    "meio-dia" => CasualTimeType::Noon,
    "meio dia" => CasualTimeType::Noon,
    "meia-noite" => CasualTimeType::Midnight,
    "meia noite" => CasualTimeType::Midnight,
    "manhã" => CasualTimeType::Morning,
    "manha" => CasualTimeType::Morning,
    "tarde" => CasualTimeType::Afternoon,
    "noite" => CasualTimeType::Night,
};

/// Relative modifiers
pub static RELATIVE_MODIFIER_MAP: phf::Map<&'static str, RelativeModifier> = phf_map! {
    "este" => RelativeModifier::This,
    "esta" => RelativeModifier::This,
    "isto" => RelativeModifier::This,
    "próximo" => RelativeModifier::Next,
    "proximo" => RelativeModifier::Next,
    "próxima" => RelativeModifier::Next,
    "proxima" => RelativeModifier::Next,
    "seguinte" => RelativeModifier::Next,
    "passado" => RelativeModifier::Last,
    "passada" => RelativeModifier::Last,
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
        s if s.contains("alguns") || s.contains("algumas") => return 3.0,
        s if s.contains("meia") || s.contains("meio") => return 0.5,
        s if s.contains("vários") || s.contains("varios") => return 7.0,
        _ => {}
    }

    text.parse::<f64>().unwrap_or(0.0)
}
