//! French (Français) locale dictionaries

use super::{CasualDateType, CasualTimeType, RelativeModifier, TimeUnit, Weekday};
use phf::phf_map;

/// Weekday dictionary
pub static WEEKDAY_MAP: phf::Map<&'static str, Weekday> = phf_map! {
    "dimanche" => Weekday::Sunday,
    "dim" => Weekday::Sunday,
    "dim." => Weekday::Sunday,
    "lundi" => Weekday::Monday,
    "lun" => Weekday::Monday,
    "lun." => Weekday::Monday,
    "mardi" => Weekday::Tuesday,
    "mar" => Weekday::Tuesday,
    "mar." => Weekday::Tuesday,
    "mercredi" => Weekday::Wednesday,
    "mer" => Weekday::Wednesday,
    "mer." => Weekday::Wednesday,
    "jeudi" => Weekday::Thursday,
    "jeu" => Weekday::Thursday,
    "jeu." => Weekday::Thursday,
    "vendredi" => Weekday::Friday,
    "ven" => Weekday::Friday,
    "ven." => Weekday::Friday,
    "samedi" => Weekday::Saturday,
    "sam" => Weekday::Saturday,
    "sam." => Weekday::Saturday,
};

/// Month dictionary
pub static MONTH_MAP: phf::Map<&'static str, u32> = phf_map! {
    "janvier" => 1,
    "jan" => 1,
    "jan." => 1,
    "janv" => 1,
    "janv." => 1,
    "février" => 2,
    "fevrier" => 2,
    "fév" => 2,
    "fev" => 2,
    "fév." => 2,
    "fev." => 2,
    "mars" => 3,
    "mar" => 3,
    "mar." => 3,
    "avril" => 4,
    "avr" => 4,
    "avr." => 4,
    "mai" => 5,
    "juin" => 6,
    "jun" => 6,
    "jun." => 6,
    "juillet" => 7,
    "juil" => 7,
    "juil." => 7,
    "jul" => 7,
    "jul." => 7,
    "août" => 8,
    "aout" => 8,
    "aoû" => 8,
    "aou" => 8,
    "septembre" => 9,
    "sep" => 9,
    "sep." => 9,
    "sept" => 9,
    "sept." => 9,
    "octobre" => 10,
    "oct" => 10,
    "oct." => 10,
    "novembre" => 11,
    "nov" => 11,
    "nov." => 11,
    "décembre" => 12,
    "decembre" => 12,
    "déc" => 12,
    "dec" => 12,
    "déc." => 12,
    "dec." => 12,
};

/// Integer words dictionary
pub static INTEGER_WORD_MAP: phf::Map<&'static str, f64> = phf_map! {
    "un" => 1.0,
    "une" => 1.0,
    "deux" => 2.0,
    "trois" => 3.0,
    "quatre" => 4.0,
    "cinq" => 5.0,
    "six" => 6.0,
    "sept" => 7.0,
    "huit" => 8.0,
    "neuf" => 9.0,
    "dix" => 10.0,
    "onze" => 11.0,
    "douze" => 12.0,
    "treize" => 13.0,
};

/// Time unit dictionary
pub static TIME_UNIT_MAP: phf::Map<&'static str, TimeUnit> = phf_map! {
    "sec" => TimeUnit::Second,
    "seconde" => TimeUnit::Second,
    "secondes" => TimeUnit::Second,
    "min" => TimeUnit::Minute,
    "mins" => TimeUnit::Minute,
    "minute" => TimeUnit::Minute,
    "minutes" => TimeUnit::Minute,
    "h" => TimeUnit::Hour,
    "hr" => TimeUnit::Hour,
    "hrs" => TimeUnit::Hour,
    "heure" => TimeUnit::Hour,
    "heures" => TimeUnit::Hour,
    "jour" => TimeUnit::Day,
    "jours" => TimeUnit::Day,
    "semaine" => TimeUnit::Week,
    "semaines" => TimeUnit::Week,
    "mois" => TimeUnit::Month,
    "trimestre" => TimeUnit::Quarter,
    "trimestres" => TimeUnit::Quarter,
    "an" => TimeUnit::Year,
    "ans" => TimeUnit::Year,
    "année" => TimeUnit::Year,
    "annee" => TimeUnit::Year,
    "années" => TimeUnit::Year,
    "annees" => TimeUnit::Year,
};

/// Casual date keywords
pub static CASUAL_DATE_MAP: phf::Map<&'static str, CasualDateType> = phf_map! {
    "maintenant" => CasualDateType::Now,
    "aujourd'hui" => CasualDateType::Today,
    "aujourdhui" => CasualDateType::Today,
    "ce soir" => CasualDateType::Tonight,
    "ce matin" => CasualDateType::ThisMorning,
    "cet après-midi" => CasualDateType::ThisAfternoon,
    "cet apres-midi" => CasualDateType::ThisAfternoon,
    "cet aprem" => CasualDateType::ThisAfternoon,
    "demain" => CasualDateType::Tomorrow,
    "hier" => CasualDateType::Yesterday,
    "après-demain" => CasualDateType::Overmorrow,
    "apres-demain" => CasualDateType::Overmorrow,
    "après demain" => CasualDateType::Overmorrow,
    "apres demain" => CasualDateType::Overmorrow,
    "avant-hier" => CasualDateType::DayBeforeYesterday,
    "avant hier" => CasualDateType::DayBeforeYesterday,
};

/// Casual time keywords
pub static CASUAL_TIME_MAP: phf::Map<&'static str, CasualTimeType> = phf_map! {
    "midi" => CasualTimeType::Noon,
    "minuit" => CasualTimeType::Midnight,
    "matin" => CasualTimeType::Morning,
    "matinée" => CasualTimeType::Morning,
    "matinee" => CasualTimeType::Morning,
    "après-midi" => CasualTimeType::Afternoon,
    "apres-midi" => CasualTimeType::Afternoon,
    "après midi" => CasualTimeType::Afternoon,
    "apres midi" => CasualTimeType::Afternoon,
    "soir" => CasualTimeType::Evening,
    "soirée" => CasualTimeType::Evening,
    "soiree" => CasualTimeType::Evening,
    "nuit" => CasualTimeType::Night,
};

/// Relative modifiers
pub static RELATIVE_MODIFIER_MAP: phf::Map<&'static str, RelativeModifier> = phf_map! {
    "ce" => RelativeModifier::This,
    "cette" => RelativeModifier::This,
    "cet" => RelativeModifier::This,
    "ces" => RelativeModifier::This,
    "prochain" => RelativeModifier::Next,
    "prochaine" => RelativeModifier::Next,
    "prochains" => RelativeModifier::Next,
    "prochaines" => RelativeModifier::Next,
    "suivant" => RelativeModifier::Next,
    "suivante" => RelativeModifier::Next,
    "dernier" => RelativeModifier::Last,
    "dernière" => RelativeModifier::Last,
    "derniere" => RelativeModifier::Last,
    "derniers" => RelativeModifier::Last,
    "dernières" => RelativeModifier::Last,
    "dernieres" => RelativeModifier::Last,
    "passé" => RelativeModifier::Last,
    "passe" => RelativeModifier::Last,
    "passée" => RelativeModifier::Last,
    "passee" => RelativeModifier::Last,
    "précédent" => RelativeModifier::Last,
    "precedent" => RelativeModifier::Last,
    "précédente" => RelativeModifier::Last,
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
        s if s.contains("quelques") => return 3.0,
        s if s.contains("demi") => return 0.5,
        _ => {}
    }

    text.parse::<f64>().unwrap_or(0.0)
}
