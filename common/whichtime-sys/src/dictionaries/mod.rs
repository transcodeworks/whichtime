//! Option B: Compile-time PHF dictionaries
//!
//! This module provides locale-specific dictionaries using compile-time perfect hash functions.
//! Benefits:
//! - O(1) guaranteed lookup with no runtime hashing
//! - Zero heap allocations
//! - Maps stored in read-only memory (.rodata section)

pub mod de;
pub mod en;
pub mod es;
pub mod fr;
pub mod it;
pub mod ja;
pub mod nl;
pub mod pt;
pub mod ru;
pub mod sv;
pub mod uk;
pub mod zh;

use std::str::FromStr;

// Re-export common types used across locales
pub use crate::types::{TimeUnit, Weekday};

/// Locale-independent categories of casual date expressions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CasualDateType {
    /// Immediate current instant.
    Now,
    /// Current day.
    Today,
    /// Current night.
    Tonight,
    /// Following day.
    Tomorrow,
    /// Previous day.
    Yesterday,
    /// Day after tomorrow.
    Overmorrow,
    /// Day before yesterday.
    DayBeforeYesterday,
    /// Morning of the current day.
    ThisMorning,
    /// Afternoon of the current day.
    ThisAfternoon,
    /// Evening of the current day.
    ThisEvening,
}

/// Locale-independent categories of casual time expressions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CasualTimeType {
    /// Around 12:00 PM.
    Noon,
    /// Around 12:00 AM.
    Midnight,
    /// Morning period.
    Morning,
    /// Afternoon period.
    Afternoon,
    /// Evening period.
    Evening,
    /// Night period.
    Night,
}

/// Relative modifiers used with weekdays and time units.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelativeModifier {
    /// Current period.
    This,
    /// Following period.
    Next,
    /// Previous period.
    Last,
}

/// Supported parser locales.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Locale {
    /// English.
    En,
    /// German.
    De,
    /// Spanish.
    Es,
    /// French.
    Fr,
    /// Italian.
    It,
    /// Japanese.
    Ja,
    /// Dutch.
    Nl,
    /// Portuguese.
    Pt,
    /// Russian.
    Ru,
    /// Swedish.
    Sv,
    /// Ukrainian.
    Uk,
    /// Chinese.
    Zh,
}

impl FromStr for Locale {
    type Err = crate::Error;
    /// Get locale from string code
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "en" | "english" => Ok(Locale::En),
            "de" | "german" | "deutsch" => Ok(Locale::De),
            "es" | "spanish" | "español" => Ok(Locale::Es),
            "fr" | "french" | "français" => Ok(Locale::Fr),
            "it" | "italian" | "italiano" => Ok(Locale::It),
            "ja" | "japanese" | "日本語" => Ok(Locale::Ja),
            "nl" | "dutch" | "nederlands" => Ok(Locale::Nl),
            "pt" | "portuguese" | "português" => Ok(Locale::Pt),
            "ru" | "russian" | "русский" => Ok(Locale::Ru),
            "sv" | "swedish" | "svenska" => Ok(Locale::Sv),
            "uk" | "ukrainian" | "українська" => Ok(Locale::Uk),
            "zh" | "chinese" | "中文" => Ok(Locale::Zh),
            _ => Err(Self::Err::LocaleNotFound(s.to_string())),
        }
    }
}
