//! Multi-locale month name parser: "15 January 2024", "15. Januar 2024", etc.
//!
//! Handles date expressions with month names across all supported locales.
//! European locales use little endian format (day month year).

use crate::components::Component;
use crate::context::ParsingContext;
use crate::dictionaries::Locale;
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use crate::scanner::TokenType;
use chrono::Datelike;
use regex::Regex;
use std::sync::LazyLock;

// Locale-specific patterns (little endian: day month year)
static EN_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(?:^|\W)(\d{1,2})(?:st|nd|rd|th)?\s+(jan(?:uary)?|feb(?:ruary)?|mar(?:ch)?|apr(?:il)?|may|june?|july?|aug(?:ust)?|sep(?:t(?:ember)?)?|oct(?:ober)?|nov(?:ember)?|dec(?:ember)?)\s*,?\s*(\d{2,4})?").unwrap()
});

static DE_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(?:^|\W)(\d{1,2})\.?\s+(januar|jänner|janner|jan|februar|feber|feb|märz|maerz|mär|mrz|april|apr|mai|juni|jun|juli|jul|august|aug|september|sep|sept|oktober|okt|november|nov|dezember|dez)\s*,?\s*(\d{2,4})?").unwrap()
});

static ES_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(?:^|\W)(?:el\s+)?(\d{1,2})(?:\s+de)?\s+(enero|ene|febrero|feb|marzo|mar|abril|abr|mayo|may|junio|jun|julio|jul|agosto|ago|septiembre|sep|sept|octubre|oct|noviembre|nov|diciembre|dic)(?:\s+(?:de(?:l)?\s+)?(\d{2,4}))?").unwrap()
});

static FR_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(?:^|\W)(?:le\s+)?(\d{1,2})(?:er|ème|eme|e)?\s+(janvier|janv?|février|fevrier|févr?|fevr?|mars|avril|avr|mai|juin|juillet|juil?|août|aout|aou|septembre|sept?|octobre|oct|novembre|nov|décembre|decembre|déc|dec)\s*(\d{2,4})?").unwrap()
});

static IT_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(?:^|\W)(?:il\s+)?(\d{1,2})(?:°)?\s+(gennaio|gen|febbraio|feb|marzo|mar|aprile|apr|maggio|mag|giugno|giu|luglio|lug|agosto|ago|settembre|set|sett|ottobre|ott|novembre|nov|dicembre|dic)\s*(\d{2,4})?").unwrap()
});

static JA_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(\d{2,4})?年?\s*(1|2|3|4|5|6|7|8|9|10|11|12|一|二|三|四|五|六|七|八|九|十|十一|十二)\s*月\s*(\d{1,2}|一|二|三|四|五|六|七|八|九|十|十一|十二|十三|十四|十五|十六|十七|十八|十九|二十|二十一|二十二|二十三|二十四|二十五|二十六|二十七|二十八|二十九|三十|三十一)\s*[日号]").unwrap()
});

static NL_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(?:^|\W)(\d{1,2})(?:e|ste|de)?\s+(januari|jan|februari|feb|maart|mrt|april|apr|mei|juni|jun|juli|jul|augustus|aug|september|sep|sept|oktober|okt|november|nov|december|dec)\s*(\d{2,4})?").unwrap()
});

static PT_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(?:^|\W)(\d{1,2})(?:\s+de)?\s+(janeiro|jan|fevereiro|fev|março|marco|mar|abril|abr|maio|mai|junho|jun|julho|jul|agosto|ago|setembro|set|outubro|out|novembro|nov|dezembro|dez)(?:\s+(?:de\s+)?(\d{2,4}))?").unwrap()
});

static RU_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(?:^|\W)(\d{1,2})(?:-?[ео]?е?|го)?\s+(января|янв|февраля|фев|марта|мар|апреля|апр|мая|июня|июн|июля|июл|августа|авг|сентября|сен|сент|октября|окт|ноября|ноя|декабря|дек)\s*(\d{2,4})?").unwrap()
});

static SV_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(?:^|\W)(?:den\s+)?(\d{1,2})(?::?[ae]?)?\s+(januari|jan|februari|feb|mars|april|apr|maj|juni|jun|juli|jul|augusti|aug|september|sep|sept|oktober|okt|november|nov|december|dec)\s*(\d{2,4})?").unwrap()
});

static UK_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(?:^|\W)(\d{1,2})(?:-?[еиого]?)?\s+(січня|січ|лютого|лют|березня|бер|квітня|квіт|травня|трав|червня|черв|липня|лип|серпня|серп|вересня|вер|жовтня|жовт|листопада|лист|грудня|груд)\s*(\d{2,4})?").unwrap()
});

static ZH_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(\d{2,4})?年?\s*(1|2|3|4|5|6|7|8|9|10|11|12|一|二|三|四|五|六|七|八|九|十|十一|十二)\s*月\s*(\d{1,2}|一|二|三|四|五|六|七|八|九|十|十一|十二|十三|十四|十五|十六|十七|十八|十九|二十|二十一|二十二|二十三|二十四|二十五|二十六|二十七|二十八|二十九|三十|三十一)\s*[日号號]").unwrap()
});

/// Multi-locale month name parser
pub struct MultiLocaleMonthNameParser {
    locale: Locale,
}

impl MultiLocaleMonthNameParser {
    pub fn new(locale: Locale) -> Self {
        Self { locale }
    }

    fn get_pattern(&self) -> &'static Regex {
        match self.locale {
            Locale::En => &EN_PATTERN,
            Locale::De => &DE_PATTERN,
            Locale::Es => &ES_PATTERN,
            Locale::Fr => &FR_PATTERN,
            Locale::It => &IT_PATTERN,
            Locale::Ja => &JA_PATTERN,
            Locale::Nl => &NL_PATTERN,
            Locale::Pt => &PT_PATTERN,
            Locale::Ru => &RU_PATTERN,
            Locale::Sv => &SV_PATTERN,
            Locale::Uk => &UK_PATTERN,
            Locale::Zh => &ZH_PATTERN,
        }
    }

    fn lookup_month(&self, text: &str) -> Option<u32> {
        let lower = text.to_lowercase();
        match self.locale {
            Locale::En => crate::dictionaries::en::get_month(&lower),
            Locale::De => crate::dictionaries::de::get_month(&lower),
            Locale::Es => crate::dictionaries::es::get_month(&lower),
            Locale::Fr => crate::dictionaries::fr::get_month(&lower),
            Locale::It => crate::dictionaries::it::get_month(&lower),
            Locale::Ja => crate::dictionaries::ja::get_month(text)
                .or_else(|| crate::dictionaries::ja::get_month(&lower)),
            Locale::Nl => crate::dictionaries::nl::get_month(&lower),
            Locale::Pt => crate::dictionaries::pt::get_month(&lower),
            Locale::Ru => crate::dictionaries::ru::get_month(&lower),
            Locale::Sv => crate::dictionaries::sv::get_month(&lower),
            Locale::Uk => crate::dictionaries::uk::get_month(&lower),
            Locale::Zh => crate::dictionaries::zh::get_month(text)
                .or_else(|| crate::dictionaries::zh::get_month(&lower)),
        }
    }

    fn parse_day(&self, text: &str) -> Option<i32> {
        // Try direct numeric parse first
        if let Ok(n) = text.parse::<i32>() {
            return Some(n);
        }

        // Try locale-specific number parsing
        let num = match self.locale {
            Locale::Ja => crate::dictionaries::ja::parse_number_pattern(text),
            Locale::Zh => crate::dictionaries::zh::parse_number_pattern(text),
            _ => return None,
        };

        if num > 0.0 { Some(num as i32) } else { None }
    }
}

impl Parser for MultiLocaleMonthNameParser {
    fn name(&self) -> &'static str {
        "MultiLocaleMonthNameParser"
    }

    fn should_apply(&self, context: &ParsingContext) -> bool {
        context.has_token_type(TokenType::Month)
    }

    fn parse(&self, context: &ParsingContext) -> Result<Vec<ParsedResult>> {
        let mut results = Vec::new();
        let pattern = self.get_pattern();
        let ref_date = context.reference.instant;

        for mat in pattern.find_iter(context.text) {
            let matched_text = mat.as_str();
            let index = mat.start();

            let Some(caps) = pattern.captures(matched_text) else {
                continue;
            };

            // Extract day, month, year - order depends on locale
            let (day, month, year_str) = match self.locale {
                Locale::Ja | Locale::Zh => {
                    // Year month day order (optional year)
                    let year_str = caps.get(1).map(|m| m.as_str());
                    let month_str = caps.get(2).map(|m| m.as_str()).unwrap_or_default();
                    let day_str = caps.get(3).map(|m| m.as_str()).unwrap_or_default();

                    let month = self
                        .lookup_month(month_str)
                        .or_else(|| month_str.parse::<u32>().ok());
                    let day = self.parse_day(day_str);

                    (day, month, year_str)
                }
                _ => {
                    // Day month year order (European style)
                    let day_str = caps.get(1).map(|m| m.as_str()).unwrap_or_default();
                    let month_str = caps.get(2).map(|m| m.as_str()).unwrap_or_default();
                    let year_str = caps.get(3).map(|m| m.as_str());

                    let day: Option<i32> = day_str.parse().ok();
                    let month = self.lookup_month(month_str);

                    (day, month, year_str)
                }
            };

            let Some(month) = month else {
                continue;
            };

            let day = day.unwrap_or(1);

            if !(1..=31).contains(&day) {
                continue;
            }

            let year = if let Some(y) = year_str {
                parse_year(y)
            } else {
                // Determine year based on whether month is in future or past
                let current_month = ref_date.month() as i32;
                if (month as i32) < current_month {
                    ref_date.year() + 1
                } else {
                    ref_date.year()
                }
            };

            let mut components = context.create_components();
            components.assign(Component::Year, year);
            components.assign(Component::Month, month as i32);
            components.assign(Component::Day, day);

            if !components.is_valid_date() {
                continue;
            }

            // Find actual text bounds
            let actual_start = matched_text
                .find(|c: char| c.is_alphanumeric())
                .unwrap_or(0);
            results.push(context.create_result(
                index + actual_start,
                index + matched_text.len(),
                components,
                None,
            ));
        }

        Ok(results)
    }
}

fn parse_year(s: &str) -> i32 {
    let year: i32 = s.parse().unwrap_or(0);
    if year < 100 {
        if year > 50 { 1900 + year } else { 2000 + year }
    } else {
        year
    }
}
