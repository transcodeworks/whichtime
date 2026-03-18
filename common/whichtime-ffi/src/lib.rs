//! Internal FFI crate for WhichTime: UniFFI types, conversions, and bindings generation.
//!
//! This internal crate holds FFI-friendly types and the UniFFI export surface
//! (parser object and top-level functions) for Swift, Kotlin, and Python.

use std::sync::Arc;
use whichtime::{Component, Error, FastComponents, Locale, ParsedResult, ReferenceWithTimezone};

uniffi::include_scaffolding!("whichtime");

// =============================================================================
// ParsedComponents
// =============================================================================

/// Simplified date/time components for FFI export.
#[derive(Debug, Clone, Default, uniffi::Record)]
pub struct ParsedComponents {
    pub year: Option<i32>,
    pub month: Option<i32>,
    pub day: Option<i32>,
    pub hour: Option<i32>,
    pub minute: Option<i32>,
    pub second: Option<i32>,
    pub millisecond: Option<i32>,
    pub weekday: Option<i32>,
    pub timezone_offset: Option<i32>,
}

impl ParsedComponents {
    pub fn from_fast_components(fc: &FastComponents) -> Self {
        Self {
            year: fc.get(Component::Year),
            month: fc.get(Component::Month),
            day: fc.get(Component::Day),
            hour: fc.get(Component::Hour),
            minute: fc.get(Component::Minute),
            second: fc.get(Component::Second),
            millisecond: fc.get(Component::Millisecond),
            weekday: fc.get(Component::Weekday),
            timezone_offset: fc.get(Component::TimezoneOffset),
        }
    }
}

// =============================================================================
// WhichTimeResult
// =============================================================================

#[derive(Debug, Clone, uniffi::Record)]
pub struct WhichTimeResult {
    pub index: u32,
    pub end_index: u32,
    pub text: String,
    pub date_millis: Option<i64>,
    pub start: ParsedComponents,
    pub end: Option<ParsedComponents>,
}

impl WhichTimeResult {
    pub fn from_parsed_result(pr: &ParsedResult) -> Self {
        let ref_tz = ReferenceWithTimezone::new(pr.ref_date, None);
        let date_millis = pr
            .start
            .to_datetime(&ref_tz)
            .map(|dt| dt.timestamp_millis());

        Self {
            index: pr.index as u32,
            end_index: pr.end_index as u32,
            text: pr.text.clone(),
            date_millis,
            start: ParsedComponents::from_fast_components(&pr.start),
            end: pr.end.as_ref().map(ParsedComponents::from_fast_components),
        }
    }
}

// =============================================================================
// WhichTimeError
// =============================================================================

#[derive(Debug, Clone, uniffi::Error)]
pub enum WhichTimeError {
    LocaleNotFound { locale: String },
    RegexError { message: String },
    InvalidDateTime { message: String },
    ParseError { message: String },
}

impl From<Error> for WhichTimeError {
    fn from(err: Error) -> Self {
        match err {
            Error::LocaleNotFound(locale) => WhichTimeError::LocaleNotFound { locale },
            Error::Regex(e) => WhichTimeError::RegexError {
                message: e.to_string(),
            },
            Error::InvalidDateTime(msg) => WhichTimeError::InvalidDateTime { message: msg },
            Error::Parse(msg) => WhichTimeError::ParseError { message: msg },
        }
    }
}

impl std::fmt::Display for WhichTimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WhichTimeError::LocaleNotFound { locale } => {
                write!(f, "Locale not found: {}", locale)
            }
            WhichTimeError::RegexError { message } => write!(f, "Regex error: {}", message),
            WhichTimeError::InvalidDateTime { message } => {
                write!(f, "Invalid date/time: {}", message)
            }
            WhichTimeError::ParseError { message } => write!(f, "Parse error: {}", message),
        }
    }
}

impl std::error::Error for WhichTimeError {}

// =============================================================================
// WhichTimeLocale
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, uniffi::Enum)]
pub enum WhichTimeLocale {
    En,
    De,
    Es,
    Fr,
    It,
    Ja,
    Nl,
    Pt,
    Ru,
    Sv,
    Uk,
    Zh,
}

impl From<WhichTimeLocale> for Locale {
    fn from(locale: WhichTimeLocale) -> Self {
        match locale {
            WhichTimeLocale::En => Locale::En,
            WhichTimeLocale::De => Locale::De,
            WhichTimeLocale::Es => Locale::Es,
            WhichTimeLocale::Fr => Locale::Fr,
            WhichTimeLocale::It => Locale::It,
            WhichTimeLocale::Ja => Locale::Ja,
            WhichTimeLocale::Nl => Locale::Nl,
            WhichTimeLocale::Pt => Locale::Pt,
            WhichTimeLocale::Ru => Locale::Ru,
            WhichTimeLocale::Sv => Locale::Sv,
            WhichTimeLocale::Uk => Locale::Uk,
            WhichTimeLocale::Zh => Locale::Zh,
        }
    }
}

impl From<Locale> for WhichTimeLocale {
    fn from(locale: Locale) -> Self {
        match locale {
            Locale::En => WhichTimeLocale::En,
            Locale::De => WhichTimeLocale::De,
            Locale::Es => WhichTimeLocale::Es,
            Locale::Fr => WhichTimeLocale::Fr,
            Locale::It => WhichTimeLocale::It,
            Locale::Ja => WhichTimeLocale::Ja,
            Locale::Nl => WhichTimeLocale::Nl,
            Locale::Pt => WhichTimeLocale::Pt,
            Locale::Ru => WhichTimeLocale::Ru,
            Locale::Sv => WhichTimeLocale::Sv,
            Locale::Uk => WhichTimeLocale::Uk,
            Locale::Zh => WhichTimeLocale::Zh,
        }
    }
}

// =============================================================================
// WhichTimeParser (UniFFI object)
// =============================================================================

#[derive(uniffi::Object, Default)]
pub struct WhichTimeParser {
    inner: whichtime::WhichTime,
}

#[uniffi::export]
impl WhichTimeParser {
    #[uniffi::constructor]
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            inner: whichtime::WhichTime::new(),
        })
    }

    pub fn parse(
        &self,
        text: String,
        locale: WhichTimeLocale,
    ) -> std::result::Result<Vec<WhichTimeResult>, WhichTimeError> {
        let parser = self.inner.get_locale_parser(locale.into());
        let results = parser.parse(&text, None)?;
        Ok(results
            .iter()
            .map(WhichTimeResult::from_parsed_result)
            .collect())
    }

    pub fn parse_date(
        &self,
        text: String,
        locale: WhichTimeLocale,
    ) -> std::result::Result<Option<i64>, WhichTimeError> {
        let parser = self.inner.get_locale_parser(locale.into());
        let date = parser.parse_date(&text, None)?;
        Ok(date.map(|dt| dt.timestamp_millis()))
    }

    pub fn parse_en(
        &self,
        text: String,
    ) -> std::result::Result<Vec<WhichTimeResult>, WhichTimeError> {
        self.parse(text, WhichTimeLocale::En)
    }

    pub fn parse_date_en(&self, text: String) -> std::result::Result<Option<i64>, WhichTimeError> {
        self.parse_date(text, WhichTimeLocale::En)
    }
}

// =============================================================================
// Top-level UniFFI exports
// =============================================================================

#[uniffi::export]
pub fn parse(text: String) -> std::result::Result<Vec<WhichTimeResult>, WhichTimeError> {
    let parser = WhichTimeParser::default();
    parser.parse(text, WhichTimeLocale::En)
}

#[uniffi::export]
pub fn parse_date(text: String) -> std::result::Result<Option<i64>, WhichTimeError> {
    let parser = WhichTimeParser::default();
    parser.parse_date(text, WhichTimeLocale::En)
}

#[uniffi::export]
pub fn parse_with_locale(
    text: String,
    locale: WhichTimeLocale,
) -> std::result::Result<Vec<WhichTimeResult>, WhichTimeError> {
    let parser = WhichTimeParser::default();
    parser.parse(text, locale)
}

#[uniffi::export]
pub fn parse_date_with_locale(
    text: String,
    locale: WhichTimeLocale,
) -> std::result::Result<Option<i64>, WhichTimeError> {
    let parser = WhichTimeParser::default();
    parser.parse_date(text, locale)
}
