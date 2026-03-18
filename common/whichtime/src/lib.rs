//! WhichTime - Natural language date parsing library
//!
//! High-level Rust API built on [`whichtime-sys`]. Use the same types as
//! whichtime-sys: [`ParsedResult`], [`Locale`], [`Error`], etc. This crate
//! adds a multi-locale parser holder ([`WhichTime`], [`LocaleParser`]) for
//! convenient reuse across locales.
//!
//! FFI bindings (UniFFI for Swift, Kotlin, Python) live in the internal
//! `whichtime-ffi` crate.

use std::sync::OnceLock;

pub use whichtime_sys::{
    Component, Error, FastComponents, Locale, ParsedResult, ReferenceWithTimezone, Result,
    WhichTime as Parser,
};

/// Parser holder with lazy initialization per locale.
/// Uses OnceLock for thread-safe lazy initialization.
pub struct LocaleParser {
    parser: OnceLock<Parser>,
    locale: Locale,
}

impl LocaleParser {
    pub fn new(locale: Locale) -> Self {
        Self {
            parser: OnceLock::new(),
            locale,
        }
    }

    pub fn parse(
        &self,
        text: &str,
        reference: Option<chrono::DateTime<chrono::Local>>,
    ) -> Result<Vec<ParsedResult>> {
        self.parser
            .get_or_init(|| Parser::with_locale(self.locale))
            .parse(text, reference)
    }

    pub fn parse_date(
        &self,
        text: &str,
        reference: Option<chrono::DateTime<chrono::Local>>,
    ) -> Result<Option<chrono::DateTime<chrono::Local>>> {
        self.parser
            .get_or_init(|| Parser::with_locale(self.locale))
            .parse_date(text, reference)
    }

    pub fn locale(&self) -> &Locale {
        &self.locale
    }
}

/// Multi-locale parser: holds lazy-initialized parsers per locale for reuse.
pub struct WhichTime {
    en_parser: LocaleParser,
    de_parser: LocaleParser,
    es_parser: LocaleParser,
    fr_parser: LocaleParser,
    it_parser: LocaleParser,
    ja_parser: LocaleParser,
    nl_parser: LocaleParser,
    pt_parser: LocaleParser,
    ru_parser: LocaleParser,
    sv_parser: LocaleParser,
    uk_parser: LocaleParser,
    zh_parser: LocaleParser,
}

impl WhichTime {
    pub fn new() -> Self {
        Self {
            en_parser: LocaleParser::new(Locale::En),
            de_parser: LocaleParser::new(Locale::De),
            es_parser: LocaleParser::new(Locale::Es),
            fr_parser: LocaleParser::new(Locale::Fr),
            it_parser: LocaleParser::new(Locale::It),
            ja_parser: LocaleParser::new(Locale::Ja),
            nl_parser: LocaleParser::new(Locale::Nl),
            pt_parser: LocaleParser::new(Locale::Pt),
            ru_parser: LocaleParser::new(Locale::Ru),
            sv_parser: LocaleParser::new(Locale::Sv),
            uk_parser: LocaleParser::new(Locale::Uk),
            zh_parser: LocaleParser::new(Locale::Zh),
        }
    }

    /// Get the parser for a specific locale
    pub fn get_locale_parser(&self, locale: Locale) -> &LocaleParser {
        match locale {
            Locale::En => &self.en_parser,
            Locale::De => &self.de_parser,
            Locale::Es => &self.es_parser,
            Locale::Fr => &self.fr_parser,
            Locale::It => &self.it_parser,
            Locale::Ja => &self.ja_parser,
            Locale::Nl => &self.nl_parser,
            Locale::Pt => &self.pt_parser,
            Locale::Ru => &self.ru_parser,
            Locale::Sv => &self.sv_parser,
            Locale::Uk => &self.uk_parser,
            Locale::Zh => &self.zh_parser,
        }
    }

    pub fn parse(
        &self,
        text: &str,
        reference: Option<chrono::DateTime<chrono::Local>>,
    ) -> Result<Vec<ParsedResult>> {
        self.en().parse(text, reference)
    }

    pub fn parse_date(
        &self,
        text: &str,
        reference: Option<chrono::DateTime<chrono::Local>>,
    ) -> Result<Option<chrono::DateTime<chrono::Local>>> {
        self.en().parse_date(text, reference)
    }

    pub fn en(&self) -> &LocaleParser {
        &self.en_parser
    }

    pub fn de(&self) -> &LocaleParser {
        &self.de_parser
    }

    pub fn es(&self) -> &LocaleParser {
        &self.es_parser
    }

    pub fn fr(&self) -> &LocaleParser {
        &self.fr_parser
    }

    pub fn it(&self) -> &LocaleParser {
        &self.it_parser
    }

    pub fn ja(&self) -> &LocaleParser {
        &self.ja_parser
    }

    pub fn nl(&self) -> &LocaleParser {
        &self.nl_parser
    }

    pub fn pt(&self) -> &LocaleParser {
        &self.pt_parser
    }

    pub fn ru(&self) -> &LocaleParser {
        &self.ru_parser
    }

    pub fn sv(&self) -> &LocaleParser {
        &self.sv_parser
    }

    pub fn uk(&self) -> &LocaleParser {
        &self.uk_parser
    }

    pub fn zh(&self) -> &LocaleParser {
        &self.zh_parser
    }
}

impl Default for WhichTime {
    fn default() -> Self {
        Self::new()
    }
}
