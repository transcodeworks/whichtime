//! Option A: Aho-Corasick based multi-pattern scanner with multi-locale support
//!
//! This scans the input text once and identifies all potential date-related
//! tokens (months, weekdays, time units, casual words) in a single pass.
//!
//! Benefits:
//! - Single pass over input text
//! - O(n + m + z) complexity (n=text, m=patterns, z=matches)
//! - SIMD-optimized by the aho-corasick crate

use aho_corasick::{AhoCorasick, MatchKind};
use std::sync::LazyLock;

use crate::dictionaries::Locale;

/// Categories of token the scanner can emit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    // Date-related
    Month,
    Weekday,
    CasualDate,
    CasualTime,

    // Time units
    TimeUnit,

    // Modifiers
    RelativeModifier,

    // Keywords
    Ago,
    Later,
    Within,
    At,
    In,
    On,
    From,
    Before,
    After,
}

/// A token found by the scanner pre-pass.
#[derive(Debug, Clone)]
pub struct Token {
    /// Semantic category of the matched token.
    pub token_type: TokenType,
    /// Start byte index in the scanned text.
    pub start: usize,
    /// End byte index in the scanned text.
    pub end: usize,
    /// Index of the pattern within the locale automaton.
    pub pattern_id: usize,
}

/// Pattern entry for the Aho-Corasick automaton
struct PatternEntry {
    pattern: &'static str,
    token_type: TokenType,
}

// ============================================================================
// English patterns
// ============================================================================

static EN_PATTERNS: &[PatternEntry] = &[
    // Months (full)
    PatternEntry {
        pattern: "january",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "february",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "march",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "april",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "may",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "june",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "july",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "august",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "september",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "october",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "november",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "december",
        token_type: TokenType::Month,
    },
    // Months (abbreviated)
    PatternEntry {
        pattern: "jan",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "feb",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "mar",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "apr",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "jun",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "jul",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "aug",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "sep",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "sept",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "oct",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "nov",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "dec",
        token_type: TokenType::Month,
    },
    // Weekdays (full)
    PatternEntry {
        pattern: "sunday",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "monday",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "tuesday",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "wednesday",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "thursday",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "friday",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "saturday",
        token_type: TokenType::Weekday,
    },
    // Weekdays (abbreviated)
    PatternEntry {
        pattern: "sun",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "mon",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "tue",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "wed",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "thu",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "thur",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "thurs",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "fri",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "sat",
        token_type: TokenType::Weekday,
    },
    // Casual date
    PatternEntry {
        pattern: "now",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "today",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "tonight",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "tomorrow",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "tmr",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "tmrw",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "yesterday",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "overmorrow",
        token_type: TokenType::CasualDate,
    },
    // Casual time
    PatternEntry {
        pattern: "noon",
        token_type: TokenType::CasualTime,
    },
    PatternEntry {
        pattern: "midday",
        token_type: TokenType::CasualTime,
    },
    PatternEntry {
        pattern: "midnight",
        token_type: TokenType::CasualTime,
    },
    PatternEntry {
        pattern: "morning",
        token_type: TokenType::CasualTime,
    },
    PatternEntry {
        pattern: "afternoon",
        token_type: TokenType::CasualTime,
    },
    PatternEntry {
        pattern: "evening",
        token_type: TokenType::CasualTime,
    },
    PatternEntry {
        pattern: "night",
        token_type: TokenType::CasualTime,
    },
    // Time units (full)
    PatternEntry {
        pattern: "second",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "seconds",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "minute",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "minutes",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "hour",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "hours",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "day",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "days",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "week",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "weeks",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "month",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "months",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "year",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "years",
        token_type: TokenType::TimeUnit,
    },
    // Time units (abbreviated)
    PatternEntry {
        pattern: "sec",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "secs",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "min",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "mins",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "hr",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "hrs",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "mo",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "yr",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "yrs",
        token_type: TokenType::TimeUnit,
    },
    // Time units (single letter - for patterns like "1h", "5m")
    PatternEntry {
        pattern: "h",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "m",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "s",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "d",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "w",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "y",
        token_type: TokenType::TimeUnit,
    },
    // Relative modifiers
    PatternEntry {
        pattern: "this",
        token_type: TokenType::RelativeModifier,
    },
    PatternEntry {
        pattern: "next",
        token_type: TokenType::RelativeModifier,
    },
    PatternEntry {
        pattern: "last",
        token_type: TokenType::RelativeModifier,
    },
    PatternEntry {
        pattern: "past",
        token_type: TokenType::RelativeModifier,
    },
    PatternEntry {
        pattern: "previous",
        token_type: TokenType::RelativeModifier,
    },
    // Keywords
    PatternEntry {
        pattern: "ago",
        token_type: TokenType::Ago,
    },
    PatternEntry {
        pattern: "before",
        token_type: TokenType::Before,
    },
    PatternEntry {
        pattern: "earlier",
        token_type: TokenType::Ago,
    },
    PatternEntry {
        pattern: "later",
        token_type: TokenType::Later,
    },
    PatternEntry {
        pattern: "after",
        token_type: TokenType::After,
    },
    PatternEntry {
        pattern: "from now",
        token_type: TokenType::Later,
    },
    PatternEntry {
        pattern: "within",
        token_type: TokenType::Within,
    },
    PatternEntry {
        pattern: "in",
        token_type: TokenType::In,
    },
    PatternEntry {
        pattern: "at",
        token_type: TokenType::At,
    },
    PatternEntry {
        pattern: "on",
        token_type: TokenType::On,
    },
    PatternEntry {
        pattern: "from",
        token_type: TokenType::From,
    },
];

// ============================================================================
// German patterns
// ============================================================================

static DE_PATTERNS: &[PatternEntry] = &[
    // Months
    PatternEntry {
        pattern: "januar",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "februar",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "märz",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "maerz",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "april",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "mai",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "juni",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "juli",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "august",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "september",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "oktober",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "november",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "dezember",
        token_type: TokenType::Month,
    },
    // Weekdays
    PatternEntry {
        pattern: "sonntag",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "so",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "montag",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "mo",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "dienstag",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "di",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "mittwoch",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "mi",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "donnerstag",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "do",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "freitag",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "fr",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "samstag",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "sa",
        token_type: TokenType::Weekday,
    },
    // Casual date
    PatternEntry {
        pattern: "jetzt",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "heute",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "morgen",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "gestern",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "übermorgen",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "uebermorgen",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "vorgestern",
        token_type: TokenType::CasualDate,
    },
    // Time units
    PatternEntry {
        pattern: "sekunde",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "sekunden",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "minute",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "minuten",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "min",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "stunde",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "stunden",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "tag",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "tage",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "tagen",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "woche",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "wochen",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "monat",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "monate",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "monaten",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "monats",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "jahr",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "jahre",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "jahren",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "jahres",
        token_type: TokenType::TimeUnit,
    },
    // Keywords
    PatternEntry {
        pattern: "vor",
        token_type: TokenType::Ago,
    },
    PatternEntry {
        pattern: "in",
        token_type: TokenType::In,
    },
    PatternEntry {
        pattern: "um",
        token_type: TokenType::At,
    },
    PatternEntry {
        pattern: "am",
        token_type: TokenType::On,
    },
];

// ============================================================================
// Spanish patterns
// ============================================================================

static ES_PATTERNS: &[PatternEntry] = &[
    // Months
    PatternEntry {
        pattern: "enero",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "febrero",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "marzo",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "abril",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "mayo",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "junio",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "julio",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "agosto",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "septiembre",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "octubre",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "noviembre",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "diciembre",
        token_type: TokenType::Month,
    },
    // Abbreviated months
    PatternEntry {
        pattern: "ene",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "feb",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "abr",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "jun",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "jul",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "ago",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "sep",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "oct",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "nov",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "dic",
        token_type: TokenType::Month,
    },
    // Weekdays
    PatternEntry {
        pattern: "domingo",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "lunes",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "martes",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "miércoles",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "miercoles",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "jueves",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "viernes",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "sábado",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "sabado",
        token_type: TokenType::Weekday,
    },
    // Casual date
    PatternEntry {
        pattern: "ahora",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "hoy",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "mañana",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "manana",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "ayer",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "anoche",
        token_type: TokenType::CasualDate,
    },
    // Casual time
    PatternEntry {
        pattern: "mediodía",
        token_type: TokenType::CasualTime,
    },
    PatternEntry {
        pattern: "mediodia",
        token_type: TokenType::CasualTime,
    },
    PatternEntry {
        pattern: "medianoche",
        token_type: TokenType::CasualTime,
    },
    PatternEntry {
        pattern: "tarde",
        token_type: TokenType::CasualTime,
    },
    PatternEntry {
        pattern: "noche",
        token_type: TokenType::CasualTime,
    },
    // Time units
    PatternEntry {
        pattern: "segundo",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "segundos",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "minuto",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "minutos",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "hora",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "horas",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "día",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "dia",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "días",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "dias",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "semana",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "semanas",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "mes",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "meses",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "año",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "anos",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "años",
        token_type: TokenType::TimeUnit,
    },
    // Keywords
    PatternEntry {
        pattern: "hace",
        token_type: TokenType::Ago,
    },
    PatternEntry {
        pattern: "en",
        token_type: TokenType::In,
    },
    PatternEntry {
        pattern: "dentro de",
        token_type: TokenType::Within,
    },
];

// ============================================================================
// French patterns
// ============================================================================

static FR_PATTERNS: &[PatternEntry] = &[
    // Months
    PatternEntry {
        pattern: "janvier",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "février",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "fevrier",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "mars",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "avril",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "mai",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "juin",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "juillet",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "août",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "aout",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "septembre",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "octobre",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "novembre",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "décembre",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "decembre",
        token_type: TokenType::Month,
    },
    // Weekdays
    PatternEntry {
        pattern: "dimanche",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "lundi",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "mardi",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "mercredi",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "jeudi",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "vendredi",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "samedi",
        token_type: TokenType::Weekday,
    },
    // Casual date
    PatternEntry {
        pattern: "maintenant",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "aujourd'hui",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "aujourdhui",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "demain",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "hier",
        token_type: TokenType::CasualDate,
    },
    // Casual time
    PatternEntry {
        pattern: "midi",
        token_type: TokenType::CasualTime,
    },
    PatternEntry {
        pattern: "minuit",
        token_type: TokenType::CasualTime,
    },
    PatternEntry {
        pattern: "matin",
        token_type: TokenType::CasualTime,
    },
    PatternEntry {
        pattern: "soir",
        token_type: TokenType::CasualTime,
    },
    // Time units
    PatternEntry {
        pattern: "seconde",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "secondes",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "minute",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "minutes",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "heure",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "heures",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "jour",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "jours",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "semaine",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "semaines",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "mois",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "an",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "ans",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "année",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "annee",
        token_type: TokenType::TimeUnit,
    },
    // Keywords
    PatternEntry {
        pattern: "il y a",
        token_type: TokenType::Ago,
    },
    PatternEntry {
        pattern: "dans",
        token_type: TokenType::In,
    },
    PatternEntry {
        pattern: "à",
        token_type: TokenType::At,
    },
];

// ============================================================================
// Italian patterns
// ============================================================================

static IT_PATTERNS: &[PatternEntry] = &[
    // Casual date
    PatternEntry {
        pattern: "adesso",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "ora",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "oggi",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "stanotte",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "stasera",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "stamattina",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "domani",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "ieri",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "dopodomani",
        token_type: TokenType::CasualDate,
    },
    // Casual time
    PatternEntry {
        pattern: "mezzogiorno",
        token_type: TokenType::CasualTime,
    },
    PatternEntry {
        pattern: "mezzanotte",
        token_type: TokenType::CasualTime,
    },
    PatternEntry {
        pattern: "mattina",
        token_type: TokenType::CasualTime,
    },
    PatternEntry {
        pattern: "mattino",
        token_type: TokenType::CasualTime,
    },
    PatternEntry {
        pattern: "pomeriggio",
        token_type: TokenType::CasualTime,
    },
    PatternEntry {
        pattern: "sera",
        token_type: TokenType::CasualTime,
    },
    PatternEntry {
        pattern: "notte",
        token_type: TokenType::CasualTime,
    },
    // Months
    PatternEntry {
        pattern: "gennaio",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "febbraio",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "marzo",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "aprile",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "maggio",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "giugno",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "luglio",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "agosto",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "settembre",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "ottobre",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "novembre",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "dicembre",
        token_type: TokenType::Month,
    },
    // Weekdays
    PatternEntry {
        pattern: "domenica",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "lunedì",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "lunedi",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "martedì",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "martedi",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "mercoledì",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "mercoledi",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "giovedì",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "giovedi",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "venerdì",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "venerdi",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "sabato",
        token_type: TokenType::Weekday,
    },
    // Time units
    PatternEntry {
        pattern: "secondo",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "secondi",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "minuto",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "minuti",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "ora",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "ore",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "giorno",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "giorni",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "settimana",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "settimane",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "mese",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "mesi",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "anno",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "anni",
        token_type: TokenType::TimeUnit,
    },
    // Keywords
    PatternEntry {
        pattern: "fa",
        token_type: TokenType::Ago,
    },
    PatternEntry {
        pattern: "in",
        token_type: TokenType::In,
    },
    PatternEntry {
        pattern: "tra",
        token_type: TokenType::Within,
    },
    PatternEntry {
        pattern: "fra",
        token_type: TokenType::Within,
    },
    PatternEntry {
        pattern: "alle",
        token_type: TokenType::At,
    },
];

// ============================================================================
// Japanese patterns
// ============================================================================

static JA_PATTERNS: &[PatternEntry] = &[
    // Casual date
    PatternEntry {
        pattern: "今日",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "きょう",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "本日",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "ほんじつ",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "今夜",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "こんや",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "今晩",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "こんばん",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "今夕",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "こんゆう",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "明日",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "あした",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "あす",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "昨日",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "きのう",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "さくじつ",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "明後日",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "あさって",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "一昨日",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "おととい",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "今朝",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "けさ",
        token_type: TokenType::CasualDate,
    },
    // Casual time
    PatternEntry {
        pattern: "午前",
        token_type: TokenType::CasualTime,
    },
    PatternEntry {
        pattern: "午後",
        token_type: TokenType::CasualTime,
    },
    PatternEntry {
        pattern: "正午",
        token_type: TokenType::CasualTime,
    },
    // Weekdays
    PatternEntry {
        pattern: "日曜日",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "月曜日",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "火曜日",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "水曜日",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "木曜日",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "金曜日",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "土曜日",
        token_type: TokenType::Weekday,
    },
    // Time units
    PatternEntry {
        pattern: "秒",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "分",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "時間",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "日",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "週間",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "月",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "年",
        token_type: TokenType::TimeUnit,
    },
];

// ============================================================================
// Dutch patterns
// ============================================================================

static NL_PATTERNS: &[PatternEntry] = &[
    // Casual date
    PatternEntry {
        pattern: "nu",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "vandaag",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "vanavond",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "morgen",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "gisteren",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "overmorgen",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "eergisteren",
        token_type: TokenType::CasualDate,
    },
    // Compound casual dates
    PatternEntry {
        pattern: "vanochtend",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "vanmiddag",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "morgenochtend",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "morgenmiddag",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "morgenavond",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "gisterenochtend",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "gisterenmiddag",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "gisterenavond",
        token_type: TokenType::CasualDate,
    },
    // Months
    PatternEntry {
        pattern: "januari",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "februari",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "maart",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "april",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "mei",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "juni",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "juli",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "augustus",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "september",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "oktober",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "november",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "december",
        token_type: TokenType::Month,
    },
    // Weekdays
    PatternEntry {
        pattern: "zondag",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "maandag",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "dinsdag",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "woensdag",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "donderdag",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "vrijdag",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "zaterdag",
        token_type: TokenType::Weekday,
    },
    // Time units
    PatternEntry {
        pattern: "seconde",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "seconden",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "minuut",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "minuten",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "uur",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "uren",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "dag",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "dagen",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "week",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "weken",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "maand",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "maanden",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "jaar",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "jaren",
        token_type: TokenType::TimeUnit,
    },
    // Keywords
    PatternEntry {
        pattern: "geleden",
        token_type: TokenType::Ago,
    },
    PatternEntry {
        pattern: "over",
        token_type: TokenType::Within,
    },
];

// ============================================================================
// Portuguese patterns
// ============================================================================

static PT_PATTERNS: &[PatternEntry] = &[
    // Casual date
    PatternEntry {
        pattern: "agora",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "hoje",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "amanhã",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "amanha",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "ontem",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "anteontem",
        token_type: TokenType::CasualDate,
    },
    // Months
    PatternEntry {
        pattern: "janeiro",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "fevereiro",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "março",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "marco",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "abril",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "maio",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "junho",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "julho",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "agosto",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "setembro",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "outubro",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "novembro",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "dezembro",
        token_type: TokenType::Month,
    },
    // Weekdays
    PatternEntry {
        pattern: "domingo",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "segunda-feira",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "segunda",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "terça-feira",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "terca-feira",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "terça",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "terca",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "quarta-feira",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "quarta",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "quinta-feira",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "quinta",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "sexta-feira",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "sexta",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "sábado",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "sabado",
        token_type: TokenType::Weekday,
    },
    // Time units
    PatternEntry {
        pattern: "segundo",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "segundos",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "minuto",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "minutos",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "hora",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "horas",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "dia",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "dias",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "semana",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "semanas",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "mês",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "mes",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "meses",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "ano",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "anos",
        token_type: TokenType::TimeUnit,
    },
    // Keywords
    PatternEntry {
        pattern: "atrás",
        token_type: TokenType::Ago,
    },
    PatternEntry {
        pattern: "atras",
        token_type: TokenType::Ago,
    },
    PatternEntry {
        pattern: "em",
        token_type: TokenType::In,
    },
];

// ============================================================================
// Russian patterns
// ============================================================================

static RU_PATTERNS: &[PatternEntry] = &[
    // Casual date
    PatternEntry {
        pattern: "сейчас",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "сегодня",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "завтра",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "вчера",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "послезавтра",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "послепослезавтра",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "позавчера",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "позапозавчера",
        token_type: TokenType::CasualDate,
    },
    // Casual time
    PatternEntry {
        pattern: "утром",
        token_type: TokenType::CasualTime,
    },
    PatternEntry {
        pattern: "вечером",
        token_type: TokenType::CasualTime,
    },
    PatternEntry {
        pattern: "ночью",
        token_type: TokenType::CasualTime,
    },
    PatternEntry {
        pattern: "полдень",
        token_type: TokenType::CasualTime,
    },
    PatternEntry {
        pattern: "полночь",
        token_type: TokenType::CasualTime,
    },
    // Months
    PatternEntry {
        pattern: "январь",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "января",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "февраль",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "февраля",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "март",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "марта",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "апрель",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "апреля",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "май",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "мая",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "июнь",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "июня",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "июль",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "июля",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "август",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "августа",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "сентябрь",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "сентября",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "октябрь",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "октября",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "ноябрь",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "ноября",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "декабрь",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "декабря",
        token_type: TokenType::Month,
    },
    // Weekdays
    PatternEntry {
        pattern: "воскресенье",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "понедельник",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "вторник",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "среда",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "среду",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "четверг",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "пятница",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "пятницу",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "суббота",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "субботу",
        token_type: TokenType::Weekday,
    },
    // Time units
    PatternEntry {
        pattern: "секунда",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "секунды",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "секунд",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "минута",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "минуты",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "минуту",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "минут",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "час",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "часа",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "часов",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "день",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "дня",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "дней",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "неделя",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "недели",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "недель",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "месяц",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "месяца",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "месяцев",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "год",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "года",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "лет",
        token_type: TokenType::TimeUnit,
    },
    // Keywords
    PatternEntry {
        pattern: "назад",
        token_type: TokenType::Ago,
    },
    PatternEntry {
        pattern: "через",
        token_type: TokenType::Within,
    },
    PatternEntry {
        pattern: "в течение",
        token_type: TokenType::Within,
    },
    PatternEntry {
        pattern: "в течении",
        token_type: TokenType::Within,
    },
];

// ============================================================================
// Swedish patterns
// ============================================================================

static SV_PATTERNS: &[PatternEntry] = &[
    // Casual date
    PatternEntry {
        pattern: "nu",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "idag",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "ikväll",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "i kväll",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "imorgon",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "igår",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "igar",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "förrgår",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "forrgar",
        token_type: TokenType::CasualDate,
    },
    // Months
    PatternEntry {
        pattern: "januari",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "februari",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "mars",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "april",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "maj",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "juni",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "juli",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "augusti",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "september",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "oktober",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "november",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "december",
        token_type: TokenType::Month,
    },
    // Weekdays
    PatternEntry {
        pattern: "söndag",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "sondag",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "måndag",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "mandag",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "tisdag",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "onsdag",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "torsdag",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "fredag",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "lördag",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "lordag",
        token_type: TokenType::Weekday,
    },
    // Time units
    PatternEntry {
        pattern: "sekund",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "sekunder",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "minut",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "minuter",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "timme",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "timmar",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "dag",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "dagar",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "vecka",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "veckor",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "månad",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "månader",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "år",
        token_type: TokenType::TimeUnit,
    },
    // Keywords
    PatternEntry {
        pattern: "sedan",
        token_type: TokenType::Ago,
    },
    PatternEntry {
        pattern: "om",
        token_type: TokenType::Within,
    },
];

// ============================================================================
// Ukrainian patterns
// ============================================================================

static UK_PATTERNS: &[PatternEntry] = &[
    // Casual date
    PatternEntry {
        pattern: "зараз",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "сьогодні",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "завтра",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "вчора",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "післязавтра",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "післяпіслязавтра",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "позавчора",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "позапозавчора",
        token_type: TokenType::CasualDate,
    },
    // Casual time
    PatternEntry {
        pattern: "вранці",
        token_type: TokenType::CasualTime,
    },
    PatternEntry {
        pattern: "ввечері",
        token_type: TokenType::CasualTime,
    },
    PatternEntry {
        pattern: "вночі",
        token_type: TokenType::CasualTime,
    },
    PatternEntry {
        pattern: "опівдні",
        token_type: TokenType::CasualTime,
    },
    PatternEntry {
        pattern: "опівночі",
        token_type: TokenType::CasualTime,
    },
    // Months
    PatternEntry {
        pattern: "січень",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "січня",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "лютий",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "лютого",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "березень",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "березня",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "квітень",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "квітня",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "травень",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "травня",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "червень",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "червня",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "липень",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "липня",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "серпень",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "серпня",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "вересень",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "вересня",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "жовтень",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "жовтня",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "листопад",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "листопада",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "грудень",
        token_type: TokenType::Month,
    },
    PatternEntry {
        pattern: "грудня",
        token_type: TokenType::Month,
    },
    // Weekdays
    PatternEntry {
        pattern: "неділя",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "понеділок",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "вівторок",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "середа",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "середу",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "четвер",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "п'ятниця",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "п'ятницю",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "субота",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "суботу",
        token_type: TokenType::Weekday,
    },
    // Time units
    PatternEntry {
        pattern: "секунда",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "секунди",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "секунд",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "хвилина",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "хвилини",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "хвилин",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "година",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "години",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "годин",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "день",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "дня",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "днів",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "тиждень",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "тижня",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "тижнів",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "місяць",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "місяця",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "місяців",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "рік",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "року",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "років",
        token_type: TokenType::TimeUnit,
    },
    // Keywords
    PatternEntry {
        pattern: "тому",
        token_type: TokenType::Ago,
    },
    PatternEntry {
        pattern: "через",
        token_type: TokenType::Within,
    },
];

// ============================================================================
// Chinese patterns
// ============================================================================

static ZH_PATTERNS: &[PatternEntry] = &[
    // Casual date - Simplified
    PatternEntry {
        pattern: "现在",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "今天",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "今晚",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "明天",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "昨天",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "后天",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "前天",
        token_type: TokenType::CasualDate,
    },
    // Casual date - Traditional
    PatternEntry {
        pattern: "現在",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "今日",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "明日",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "昨日",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "後天",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "聽日",
        token_type: TokenType::CasualDate,
    },
    PatternEntry {
        pattern: "而家",
        token_type: TokenType::CasualDate,
    },
    // Casual time
    PatternEntry {
        pattern: "上午",
        token_type: TokenType::CasualTime,
    },
    PatternEntry {
        pattern: "下午",
        token_type: TokenType::CasualTime,
    },
    PatternEntry {
        pattern: "早上",
        token_type: TokenType::CasualTime,
    },
    PatternEntry {
        pattern: "晚上",
        token_type: TokenType::CasualTime,
    },
    PatternEntry {
        pattern: "中午",
        token_type: TokenType::CasualTime,
    },
    // Weekdays
    PatternEntry {
        pattern: "星期日",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "星期一",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "星期二",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "星期三",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "星期四",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "星期五",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "星期六",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "周日",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "周一",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "周二",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "周三",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "周四",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "周五",
        token_type: TokenType::Weekday,
    },
    PatternEntry {
        pattern: "周六",
        token_type: TokenType::Weekday,
    },
    // Time units
    PatternEntry {
        pattern: "秒",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "分",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "分钟",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "分鐘",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "小时",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "小時",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "天",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "日",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "周",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "週",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "星期",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "月",
        token_type: TokenType::TimeUnit,
    },
    PatternEntry {
        pattern: "年",
        token_type: TokenType::TimeUnit,
    },
    // Keywords
    PatternEntry {
        pattern: "前",
        token_type: TokenType::Ago,
    },
    PatternEntry {
        pattern: "后",
        token_type: TokenType::Later,
    },
    PatternEntry {
        pattern: "後",
        token_type: TokenType::Later,
    },
    PatternEntry {
        pattern: "内",
        token_type: TokenType::Within,
    },
    PatternEntry {
        pattern: "內",
        token_type: TokenType::Within,
    },
];

// Lazily initialized automaton per locale
static EN_AUTOMATON: LazyLock<LocaleAutomaton> =
    LazyLock::new(|| LocaleAutomaton::new(EN_PATTERNS));
static DE_AUTOMATON: LazyLock<LocaleAutomaton> =
    LazyLock::new(|| LocaleAutomaton::new(DE_PATTERNS));
static ES_AUTOMATON: LazyLock<LocaleAutomaton> =
    LazyLock::new(|| LocaleAutomaton::new(ES_PATTERNS));
static FR_AUTOMATON: LazyLock<LocaleAutomaton> =
    LazyLock::new(|| LocaleAutomaton::new(FR_PATTERNS));
static IT_AUTOMATON: LazyLock<LocaleAutomaton> =
    LazyLock::new(|| LocaleAutomaton::new(IT_PATTERNS));
static JA_AUTOMATON: LazyLock<LocaleAutomaton> =
    LazyLock::new(|| LocaleAutomaton::new(JA_PATTERNS));
static NL_AUTOMATON: LazyLock<LocaleAutomaton> =
    LazyLock::new(|| LocaleAutomaton::new(NL_PATTERNS));
static PT_AUTOMATON: LazyLock<LocaleAutomaton> =
    LazyLock::new(|| LocaleAutomaton::new(PT_PATTERNS));
static RU_AUTOMATON: LazyLock<LocaleAutomaton> =
    LazyLock::new(|| LocaleAutomaton::new(RU_PATTERNS));
static SV_AUTOMATON: LazyLock<LocaleAutomaton> =
    LazyLock::new(|| LocaleAutomaton::new(SV_PATTERNS));
static UK_AUTOMATON: LazyLock<LocaleAutomaton> =
    LazyLock::new(|| LocaleAutomaton::new(UK_PATTERNS));
static ZH_AUTOMATON: LazyLock<LocaleAutomaton> =
    LazyLock::new(|| LocaleAutomaton::new(ZH_PATTERNS));

/// Locale-specific automaton
struct LocaleAutomaton {
    ac: AhoCorasick,
    patterns: &'static [PatternEntry],
}

impl LocaleAutomaton {
    fn new(patterns: &'static [PatternEntry]) -> Self {
        let pattern_strs: Vec<&str> = patterns.iter().map(|p| p.pattern).collect();
        let ac = AhoCorasick::builder()
            .match_kind(MatchKind::LeftmostLongest)
            .build(&pattern_strs)
            .expect("Failed to build Aho-Corasick automaton");
        Self { ac, patterns }
    }

    fn scan(&self, text: &str) -> Vec<Token> {
        let mut tokens = Vec::new();

        for mat in self.ac.find_iter(text) {
            let pattern_id = mat.pattern().as_usize();
            let entry = &self.patterns[pattern_id];

            let start = mat.start();
            let end = mat.end();
            let pattern_len = end - start;

            // Word boundary check with special handling for single-letter time units
            // Allow digits before single-letter time unit abbreviations (e.g., "1h", "5m")
            let valid_start = if start == 0 {
                true
            } else {
                let prev_char = text.as_bytes()[start - 1];
                if pattern_len == 1 && entry.token_type == TokenType::TimeUnit {
                    // Single-letter time units can follow digits
                    !prev_char.is_ascii_alphabetic()
                } else {
                    !prev_char.is_ascii_alphanumeric()
                }
            };

            let valid_end = end == text.len() || !text.as_bytes()[end].is_ascii_alphanumeric();

            if valid_start && valid_end {
                tokens.push(Token {
                    token_type: entry.token_type,
                    start,
                    end,
                    pattern_id,
                });
            }
        }

        tokens
    }
}

/// Locale-aware token scanner used by parser prefilters.
pub struct TokenScanner;

impl TokenScanner {
    /// Scan text using the automaton for the provided locale.
    pub fn scan_locale(text: &str, locale: Locale) -> Vec<Token> {
        let automaton = match locale {
            Locale::En => &*EN_AUTOMATON,
            Locale::De => &*DE_AUTOMATON,
            Locale::Es => &*ES_AUTOMATON,
            Locale::Fr => &*FR_AUTOMATON,
            Locale::It => &*IT_AUTOMATON,
            Locale::Ja => &*JA_AUTOMATON,
            Locale::Nl => &*NL_AUTOMATON,
            Locale::Pt => &*PT_AUTOMATON,
            Locale::Ru => &*RU_AUTOMATON,
            Locale::Sv => &*SV_AUTOMATON,
            Locale::Uk => &*UK_AUTOMATON,
            Locale::Zh => &*ZH_AUTOMATON,
        };
        automaton.scan(text)
    }

    /// Scan text using the default English automaton.
    pub fn scan(text: &str) -> Vec<Token> {
        Self::scan_locale(text, Locale::En)
    }

    /// Return `true` if the scanned text contains a token of the given type.
    pub fn contains_type(text: &str, token_type: TokenType) -> bool {
        Self::scan(text).iter().any(|t| t.token_type == token_type)
    }

    /// Return `true` if the text appears to contain date-related hints.
    ///
    /// This fast heuristic treats any digits or any scanned token as a hint.
    pub fn has_date_hint(text: &str) -> bool {
        // Quick check for digits (dates often have numbers)
        if text.bytes().any(|b| b.is_ascii_digit()) {
            return true;
        }

        // Check for any token
        !Self::scan(text).is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_months() {
        let tokens = TokenScanner::scan("meeting in january");
        assert!(tokens.iter().any(|t| t.token_type == TokenType::Month));
    }

    #[test]
    fn test_scan_weekdays() {
        let tokens = TokenScanner::scan("see you on monday");
        assert!(tokens.iter().any(|t| t.token_type == TokenType::Weekday));
    }

    #[test]
    fn test_scan_casual() {
        let tokens = TokenScanner::scan("let's meet tomorrow");
        assert!(tokens.iter().any(|t| t.token_type == TokenType::CasualDate));
    }

    #[test]
    fn test_word_boundary() {
        // "may" should not match inside "maybe"
        let tokens = TokenScanner::scan("maybe we can");
        assert!(tokens.iter().all(|t| t.token_type != TokenType::Month));
    }

    #[test]
    fn test_german_locale() {
        let tokens = TokenScanner::scan_locale("treffen wir uns morgen", Locale::De);
        assert!(tokens.iter().any(|t| t.token_type == TokenType::CasualDate));
    }

    #[test]
    fn test_spanish_locale() {
        let tokens = TokenScanner::scan_locale("nos vemos mañana", Locale::Es);
        assert!(tokens.iter().any(|t| t.token_type == TokenType::CasualDate));
    }
}
