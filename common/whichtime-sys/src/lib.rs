//! Lower-level parsing engine for the `whichtime` project.
//!
//! `whichtime-sys` exposes the lower-level Rust parser engine that powers the
//! higher-level [`whichtime`] crate and the project's FFI bindings. The main
//! entry point is [`WhichTime`], which coordinates a locale-specific parser
//! pipeline:
//!
//! 1. Pre-scan the input for date-related tokens.
//! 2. Run matching parsers for the active locale.
//! 3. Post-process matches with refiners.
//! 4. Return [`ParsedResult`] values or the first parsed date.
//!
//! Supported locales: `en`, `de`, `es`, `fr`, `it`, `ja`, `nl`, `pt`, `ru`,
//! `sv`, `uk`, and `zh`.

pub mod components;
pub mod context;
pub mod dictionaries;
pub mod error;
pub mod parsers;
pub mod refiners;
pub mod results;
pub mod scanner;
pub mod types;

// Re-export main types
pub use components::{Component, ComponentFlags, FastComponents};
pub use context::ParsingContext;
pub use dictionaries::{CasualDateType, CasualTimeType, Locale, RelativeModifier};
pub use error::{Error, Result};
pub use results::{ParsedResult, ReferenceWithTimezone};
pub use types::{Meridiem, Weekday};

use parsers::Parser;
use refiners::Refiner;

/// Locale-specific parser configuration.
///
/// This bundles the parser and refiner pipeline that will be used by
/// [`WhichTime`]. Most callers should use [`WhichTime::new`] or
/// [`WhichTime::with_locale`] instead of constructing this directly.
pub struct Configuration {
    /// Parsers that attempt to extract date and time mentions from text.
    pub parsers: Vec<Box<dyn Parser>>,
    /// Refiners that merge, filter, or adjust parser output.
    pub refiners: Vec<Box<dyn Refiner>>,
    /// Locale that the configuration is intended for.
    pub locale: Locale,
}

/// The main natural-language date parser.
///
/// A `WhichTime` instance owns a locale-specific set of parsers and refiners
/// and can be reused across multiple parse calls.
pub struct WhichTime {
    parsers: Vec<Box<dyn Parser>>,
    refiners: Vec<Box<dyn Refiner>>,
    locale: Locale,
}

impl WhichTime {
    /// Create a parser with the default English configuration.
    pub fn new() -> Self {
        Self::with_locale(Locale::En)
    }

    /// Create a parser configured for a specific locale.
    pub fn with_locale(locale: Locale) -> Self {
        let config = create_configuration_for_locale(locale);
        Self {
            parsers: config.parsers,
            refiners: config.refiners,
            locale: config.locale,
        }
    }

    /// Create a parser from a custom [`Configuration`].
    pub fn with_configuration(config: Configuration) -> Self {
        Self {
            parsers: config.parsers,
            refiners: config.refiners,
            locale: config.locale,
        }
    }

    /// Return the parser locale.
    pub fn locale(&self) -> Locale {
        self.locale
    }

    /// Parse text and return every detected date/time match.
    ///
    /// If `reference` is `None`, the current local time is used as the
    /// reference instant for relative expressions such as "tomorrow" or
    /// "next week".
    pub fn parse(
        &self,
        text: &str,
        reference: Option<chrono::DateTime<chrono::Local>>,
    ) -> Result<Vec<ParsedResult>> {
        let reference = reference.unwrap_or_else(chrono::Local::now);
        let ref_tz = ReferenceWithTimezone::new(reference, None);
        let context = ParsingContext::with_locale(text, &ref_tz, self.locale);

        let mut results = Vec::new();

        for parser in &self.parsers {
            if !parser.should_apply(&context) {
                continue;
            }
            let parsed = parser.parse(&context)?;
            results.extend(parsed);
        }

        results.sort_by_key(|r| r.index);

        for refiner in &self.refiners {
            results = refiner.refine(&context, results);
            results.sort_by_key(|r| r.index);
        }

        Ok(results)
    }

    /// Parse text and return the first resolved date, if any.
    ///
    /// This is a convenience wrapper around [`WhichTime::parse`] for callers
    /// that only care about the earliest match.
    pub fn parse_date(
        &self,
        text: &str,
        reference: Option<chrono::DateTime<chrono::Local>>,
    ) -> Result<Option<chrono::DateTime<chrono::Local>>> {
        let results = self.parse(text, reference)?;
        if let Some(result) = results.first() {
            let ref_dt = reference.unwrap_or_else(chrono::Local::now);
            let ref_tz = ReferenceWithTimezone::new(ref_dt, None);
            Ok(result.start.to_datetime(&ref_tz))
        } else {
            Ok(None)
        }
    }
}

impl Default for WhichTime {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for WhichTime {
    fn clone(&self) -> Self {
        Self::with_locale(self.locale)
    }
}

/// Build the default parser configuration for a locale.
///
/// This is primarily useful when composing a custom [`WhichTime`] instance or
/// when inspecting the parser/refiner pipeline used by a given locale.
pub fn create_configuration_for_locale(locale: Locale) -> Configuration {
    use parsers::common::{
        MultiLocaleCasualTimeParser, MultiLocaleTimeUnitAgoParser, MultiLocaleTimeUnitWithinParser,
        MultiLocaleWeekdayParser,
    };
    use parsers::de::{
        DECasualDateParser, DECasualTimeParser, DETimeExpressionParser, DETimeUnitRelativeParser,
    };
    use parsers::en::{ISOFormatParser, SlashDateParser, TimeExpressionParser};

    match locale {
        Locale::En => parsers::en::create_casual_configuration(),
        Locale::De => {
            use parsers::de::{DEMonthNameParser, DEWeekdayParser};
            Configuration {
                parsers: vec![
                    Box::new(ISOFormatParser),
                    Box::new(SlashDateParser::new(true)), // European date format (DD/MM/YYYY)
                    Box::new(DEMonthNameParser::new()),
                    Box::new(DEWeekdayParser::new()),
                    Box::new(DETimeExpressionParser::new()),
                    Box::new(DECasualDateParser::new()),
                    Box::new(DECasualTimeParser::new()),
                    Box::new(DETimeUnitRelativeParser::new()),
                    Box::new(MultiLocaleTimeUnitAgoParser::new(locale)),
                    Box::new(MultiLocaleTimeUnitWithinParser::new(locale)),
                ],
                refiners: vec![
                    Box::new(crate::refiners::MergeWeekdayDateRefiner),
                    Box::new(crate::refiners::MergeDateRangeRefiner),
                    Box::new(crate::refiners::OverlapRemovalRefiner),
                    Box::new(crate::refiners::MergeDateTimeRefiner),
                    Box::new(crate::refiners::ForwardDateRefiner),
                ],
                locale,
            }
        }
        Locale::Es => {
            use parsers::es::{
                ESCasualDateParser, ESCasualTimeParser, ESMonthNameParser, ESTimeExpressionParser,
            };
            Configuration {
                parsers: vec![
                    Box::new(ISOFormatParser),
                    Box::new(SlashDateParser::new(true)), // DD/MM/YYYY format
                    Box::new(ESMonthNameParser::new()),
                    Box::new(MultiLocaleWeekdayParser::new(locale)),
                    Box::new(ESTimeExpressionParser::new()),
                    Box::new(ESCasualDateParser::new()),
                    Box::new(ESCasualTimeParser::new()),
                    Box::new(MultiLocaleTimeUnitAgoParser::new(locale)),
                    Box::new(MultiLocaleTimeUnitWithinParser::new(locale)),
                ],
                refiners: vec![
                    Box::new(crate::refiners::MergeWeekdayDateRefiner),
                    Box::new(crate::refiners::OverlapRemovalRefiner),
                    Box::new(crate::refiners::MergeDateTimeRefiner),
                    Box::new(crate::refiners::ForwardDateRefiner),
                ],
                locale,
            }
        }
        Locale::Fr => {
            use parsers::fr::{
                FRCasualDateParser, FRCasualTimeParser, FRMonthNameParser, FRSlashDateParser,
                FRTimeExpressionParser, FRTimeUnitRelativeParser, FRWeekdayParser,
            };
            Configuration {
                parsers: vec![
                    Box::new(ISOFormatParser),
                    Box::new(FRSlashDateParser::new()), // DD/MM/YYYY format with optional year
                    Box::new(FRMonthNameParser::new()),
                    Box::new(FRWeekdayParser::new()),
                    Box::new(FRTimeExpressionParser::new()),
                    Box::new(FRCasualDateParser::new()),
                    Box::new(FRCasualTimeParser::new()),
                    Box::new(FRTimeUnitRelativeParser::new()),
                    Box::new(MultiLocaleTimeUnitAgoParser::new(locale)),
                    Box::new(MultiLocaleTimeUnitWithinParser::new(locale)),
                ],
                refiners: vec![
                    Box::new(crate::refiners::MergeWeekdayDateRefiner),
                    Box::new(crate::refiners::OverlapRemovalRefiner),
                    Box::new(crate::refiners::MergeDateTimeRefiner),
                    Box::new(crate::refiners::ForwardDateRefiner),
                ],
                locale,
            }
        }
        Locale::It => {
            use parsers::it::{ITCasualDateParser, ITMonthNameParser, ITYearMonthDayParser};
            Configuration {
                parsers: vec![
                    Box::new(ISOFormatParser),
                    Box::new(SlashDateParser::new(true)), // DD/MM/YYYY format
                    Box::new(ITYearMonthDayParser::new()), // YYYY.MM.DD format
                    Box::new(ITMonthNameParser::new()),
                    Box::new(MultiLocaleWeekdayParser::new(locale)),
                    Box::new(TimeExpressionParser),
                    Box::new(ITCasualDateParser::new()),
                    Box::new(MultiLocaleCasualTimeParser::new(locale)),
                    Box::new(MultiLocaleTimeUnitAgoParser::new(locale)),
                    Box::new(MultiLocaleTimeUnitWithinParser::new(locale)),
                ],
                refiners: vec![
                    Box::new(crate::refiners::MergeWeekdayDateRefiner),
                    Box::new(crate::refiners::OverlapRemovalRefiner),
                    Box::new(crate::refiners::MergeDateTimeRefiner),
                    Box::new(crate::refiners::ForwardDateRefiner),
                ],
                locale,
            }
        }
        Locale::Ja => {
            use parsers::ja::{
                JACasualDateParser, JACasualTimeParser, JASlashDateParser, JAStandardDateParser,
                JATimeExpressionParser, JAWeekdayParser,
            };
            Configuration {
                parsers: vec![
                    Box::new(ISOFormatParser),
                    Box::new(JASlashDateParser::new()),
                    Box::new(JAStandardDateParser::new()),
                    Box::new(JAWeekdayParser::new()),
                    Box::new(JATimeExpressionParser::new()),
                    Box::new(JACasualDateParser::new()),
                    Box::new(JACasualTimeParser::new()),
                    Box::new(MultiLocaleTimeUnitAgoParser::new(locale)),
                    Box::new(MultiLocaleTimeUnitWithinParser::new(locale)),
                ],
                refiners: vec![
                    Box::new(crate::refiners::MergeWeekdayDateRefiner),
                    Box::new(crate::refiners::OverlapRemovalRefiner),
                    Box::new(crate::refiners::MergeDateTimeRefiner),
                    Box::new(crate::refiners::ForwardDateRefiner),
                ],
                locale,
            }
        }
        Locale::Nl => {
            use parsers::nl::{
                NLCasualDateParser, NLCasualTimeParser, NLMonthNameParser, NLWeekdayParser,
            };
            Configuration {
                parsers: vec![
                    Box::new(ISOFormatParser),
                    Box::new(SlashDateParser::new(true)), // DD/MM/YYYY format
                    Box::new(NLMonthNameParser::new()),
                    Box::new(NLWeekdayParser::new()),
                    Box::new(TimeExpressionParser),
                    Box::new(NLCasualDateParser::new()),
                    Box::new(NLCasualTimeParser::new()),
                    Box::new(MultiLocaleTimeUnitAgoParser::new(locale)),
                    Box::new(MultiLocaleTimeUnitWithinParser::new(locale)),
                ],
                refiners: vec![
                    Box::new(crate::refiners::MergeWeekdayDateRefiner),
                    Box::new(crate::refiners::MergeDateRangeRefiner),
                    Box::new(crate::refiners::OverlapRemovalRefiner),
                    Box::new(crate::refiners::MergeDateTimeRefiner),
                    Box::new(crate::refiners::ForwardDateRefiner),
                ],
                locale,
            }
        }
        Locale::Pt => {
            use parsers::pt::{
                PTCasualDateParser, PTCasualTimeParser, PTMonthNameParser, PTTimeExpressionParser,
                PTWeekdayParser,
            };
            Configuration {
                parsers: vec![
                    Box::new(ISOFormatParser),
                    Box::new(SlashDateParser::new(true)), // DD/MM/YYYY format
                    Box::new(PTMonthNameParser::new()),
                    Box::new(PTWeekdayParser::new()),
                    Box::new(PTTimeExpressionParser::new()),
                    Box::new(PTCasualDateParser::new()),
                    Box::new(PTCasualTimeParser::new()),
                    Box::new(MultiLocaleTimeUnitAgoParser::new(locale)),
                    Box::new(MultiLocaleTimeUnitWithinParser::new(locale)),
                ],
                refiners: vec![
                    Box::new(crate::refiners::MergeWeekdayDateRefiner),
                    Box::new(crate::refiners::MergeDateRangeRefiner),
                    Box::new(crate::refiners::OverlapRemovalRefiner),
                    Box::new(crate::refiners::MergeDateTimeRefiner),
                    Box::new(crate::refiners::ForwardDateRefiner),
                ],
                locale,
            }
        }
        Locale::Ru => {
            use parsers::ru::{
                RUCasualDateParser, RUCasualTimeParser, RUMonthNameParser,
                RUTimeUnitRelativeParser, RUWeekdayParser,
            };
            Configuration {
                parsers: vec![
                    Box::new(ISOFormatParser),
                    Box::new(SlashDateParser::new(true)), // DD.MM.YYYY
                    Box::new(RUMonthNameParser::new()),
                    Box::new(RUWeekdayParser::new()),
                    Box::new(TimeExpressionParser),
                    Box::new(RUCasualDateParser::new()),
                    Box::new(RUCasualTimeParser::new()),
                    Box::new(RUTimeUnitRelativeParser::new()),
                    Box::new(MultiLocaleTimeUnitWithinParser::new(locale)),
                ],
                refiners: vec![
                    Box::new(crate::refiners::MergeWeekdayDateRefiner),
                    Box::new(crate::refiners::MergeDateRangeRefiner),
                    Box::new(crate::refiners::OverlapRemovalRefiner),
                    Box::new(crate::refiners::MergeDateTimeRefiner),
                    Box::new(crate::refiners::ForwardDateRefiner),
                ],
                locale,
            }
        }
        Locale::Sv => {
            use parsers::sv::{
                SVCasualDateParser, SVMonthNameParser, SVTimeUnitRelativeParser, SVWeekdayParser,
            };
            Configuration {
                parsers: vec![
                    Box::new(ISOFormatParser),
                    Box::new(SlashDateParser::new(true)), // DD/MM/YYYY or DD.MM.YYYY
                    Box::new(SVMonthNameParser::new()),
                    Box::new(SVWeekdayParser::new()),
                    Box::new(TimeExpressionParser),
                    Box::new(SVCasualDateParser::new()),
                    Box::new(SVTimeUnitRelativeParser::new()),
                    Box::new(MultiLocaleTimeUnitAgoParser::new(locale)),
                    Box::new(MultiLocaleTimeUnitWithinParser::new(locale)),
                ],
                refiners: vec![
                    Box::new(crate::refiners::MergeWeekdayDateRefiner),
                    Box::new(crate::refiners::MergeDateRangeRefiner),
                    Box::new(crate::refiners::OverlapRemovalRefiner),
                    Box::new(crate::refiners::MergeDateTimeRefiner),
                    Box::new(crate::refiners::ForwardDateRefiner),
                ],
                locale,
            }
        }
        Locale::Uk => {
            use parsers::uk::{UKCasualDateParser, UKMonthNameParser, UKWeekdayParser};
            Configuration {
                parsers: vec![
                    Box::new(ISOFormatParser),
                    Box::new(UKMonthNameParser::new()),
                    Box::new(UKWeekdayParser::new()),
                    Box::new(TimeExpressionParser),
                    Box::new(UKCasualDateParser::new()),
                    Box::new(MultiLocaleTimeUnitAgoParser::new(locale)),
                    Box::new(MultiLocaleTimeUnitWithinParser::new(locale)),
                ],
                refiners: vec![
                    Box::new(crate::refiners::MergeWeekdayDateRefiner),
                    Box::new(crate::refiners::MergeDateRangeRefiner),
                    Box::new(crate::refiners::OverlapRemovalRefiner),
                    Box::new(crate::refiners::MergeDateTimeRefiner),
                    Box::new(crate::refiners::ForwardDateRefiner),
                ],
                locale,
            }
        }
        Locale::Zh => {
            use parsers::zh::{
                ZHCasualDateParser, ZHStandardDateParser, ZHTimeExpressionParser,
                ZHTimeUnitWithinParser, ZHWeekdayParser,
            };
            Configuration {
                parsers: vec![
                    Box::new(ISOFormatParser),
                    Box::new(ZHStandardDateParser::new()),
                    Box::new(ZHWeekdayParser::new()),
                    Box::new(ZHTimeExpressionParser::new()),
                    Box::new(ZHCasualDateParser::new()),
                    Box::new(ZHTimeUnitWithinParser::new()),
                ],
                refiners: vec![
                    Box::new(crate::refiners::MergeWeekdayDateRefiner),
                    Box::new(crate::refiners::OverlapRemovalRefiner),
                    Box::new(crate::refiners::MergeDateTimeRefiner),
                    Box::new(crate::refiners::ForwardDateRefiner),
                ],
                locale,
            }
        }
    }
}
