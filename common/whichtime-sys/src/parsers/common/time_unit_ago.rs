//! Multi-locale time unit ago parser: "5 days ago", "2 hours ago", etc.
//!
//! Handles time expressions with "ago" semantics across all supported locales.

use crate::components::Component;
use crate::context::ParsingContext;
use crate::dictionaries::Locale;
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use crate::scanner::TokenType;
use crate::types::{Duration, TimeUnit, add_duration};
use chrono::Datelike;
use regex::Regex;
use std::sync::LazyLock;

// Locale-specific patterns
static EN_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(\d+|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|a|an|the|few|several|couple)\s*(seconds?|minutes?|hours?|days?|weeks?|months?|years?|mins?|hrs?|secs?)\s*(?:ago|before|earlier)\b").unwrap()
});

static DE_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(?:vor\s+)?(\d+|ein(?:e[mnrs]?)?|zwei|drei|vier|fünf|fuenf|sechs|sieben|acht|neun|zehn|elf|zwölf|zwoelf)\s*(sekunden?|minuten?|stunden?|tagen?|wochen?|monaten?|monats?|jahren?|jahres?|sek|min|std)(?:\s+(?:her|früher|frueher|zuvor))?\b").unwrap()
});

static ES_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(?:hace\s+)?(\d+|un[oa]?|dos|tres|cuatro|cinco|seis|siete|ocho|nueve|diez|once|doce)\s*(segundos?|minutos?|horas?|días?|dias?|semanas?|meses?|años?|anos?|seg|min|hr?s?)\s*(?:atrás|atras)?\b").unwrap()
});

static FR_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(?:il\s+y\s+a\s+)?(\d+|une?|deux|trois|quatre|cinq|six|sept|huit|neuf|dix|onze|douze)\s*(secondes?|minutes?|heures?|jours?|semaines?|mois|ans?|années?|annees?|sec|min|h)\b").unwrap()
});

static IT_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(\d+|un[oa']?|due|tre|quattro|cinque|sei|sette|otto|nove|dieci|undici|dodici)\s*(second[oi]|minut[oi]|or[ae]|giorn[oi]|settiman[ae]|mes[ei]|ann[oi]|sec|min|h)\s*fa\b").unwrap()
});

static JA_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(\d+|一|二|三|四|五|六|七|八|九|十)\s*(秒|分|時間|日|週間|ヶ月|か月|年)\s*前")
        .unwrap()
});

static NL_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(\d+|een|twee|drie|vier|vijf|zes|zeven|acht|negen|tien|elf|twaalf)\s*(seconden?|minuten?|uren?|dagen?|weken?|maanden?|jaren?|sec|min|u)\s*(?:geleden|terug)\b").unwrap()
});

static PT_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(?:há\s+|ha\s+)?(\d+|um[a]?|dois|duas|três|tres|quatro|cinco|seis|sete|oito|nove|dez|onze|doze)\s*(segundos?|minutos?|horas?|dias?|semanas?|meses?|anos?|seg|min|h)\s*(?:atrás|atras)?\b").unwrap()
});

static RU_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(\d+|один|одну|одна|два|две|три|четыре|пять|шесть|семь|восемь|девять|десять|одиннадцать|двенадцать)\s*(секунд[уы]?|минут[уы]?|час(?:ов|а)?|дн[яей]|недел[юиь]|месяц(?:ев|а)?|год(?:а|ов)?|лет)\s*назад\b").unwrap()
});

static SV_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(?:för\s+|for\s+)?(\d+|en|ett|två|tva|tre|fyra|fem|sex|sju|åtta|atta|nio|tio|elva|tolv)\s*(sekunder?|minuter?|timm(?:ar|e)?|dagar?|veckor?|månader?|manader?|år|ar)\s*(?:sedan)?\b").unwrap()
});

static UK_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(\d+|один|одну|одна|два|дві|три|чотири|п'ять|шість|сім|вісім|дев'ять|десять|одинадцять|дванадцять)\s*(секунд[уи]?|хвилин[уи]?|годин[уи]?|дн[яів]|тижн[яів]|місяц[яів]|рок(?:ів|у)?)\s*(?:тому|назад)\b").unwrap()
});

static ZH_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(\d+|一|二|两|兩|三|四|五|六|七|八|九|十)\s*(秒[钟鐘]?|分[钟鐘]?|[小个個]?时|時|天|[个個]?星期|周|週|[个個]?月|年)\s*(?:前|以前)").unwrap()
});

/// Multi-locale time unit ago parser
pub struct MultiLocaleTimeUnitAgoParser {
    locale: Locale,
}

impl MultiLocaleTimeUnitAgoParser {
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

    fn parse_number(&self, text: &str) -> f64 {
        let _lower = text.to_lowercase();
        match self.locale {
            Locale::En => crate::dictionaries::en::parse_number_pattern(text),
            Locale::De => crate::dictionaries::de::parse_number_pattern(text),
            Locale::Es => crate::dictionaries::es::parse_number_pattern(text),
            Locale::Fr => crate::dictionaries::fr::parse_number_pattern(text),
            Locale::It => crate::dictionaries::it::parse_number_pattern(text),
            Locale::Ja => crate::dictionaries::ja::parse_number_pattern(text),
            Locale::Nl => crate::dictionaries::nl::parse_number_pattern(text),
            Locale::Pt => crate::dictionaries::pt::parse_number_pattern(text),
            Locale::Ru => crate::dictionaries::ru::parse_number_pattern(text),
            Locale::Sv => crate::dictionaries::sv::parse_number_pattern(text),
            Locale::Uk => crate::dictionaries::uk::parse_number_pattern(text),
            Locale::Zh => crate::dictionaries::zh::parse_number_pattern(text),
        }
    }

    fn lookup_time_unit(&self, text: &str) -> Option<TimeUnit> {
        let lower = text.to_lowercase();
        match self.locale {
            Locale::En => crate::dictionaries::en::get_time_unit(&lower),
            Locale::De => crate::dictionaries::de::get_time_unit(&lower),
            Locale::Es => crate::dictionaries::es::get_time_unit(&lower),
            Locale::Fr => crate::dictionaries::fr::get_time_unit(&lower),
            Locale::It => crate::dictionaries::it::get_time_unit(&lower),
            Locale::Ja => crate::dictionaries::ja::get_time_unit(text)
                .or_else(|| crate::dictionaries::ja::get_time_unit(&lower)),
            Locale::Nl => crate::dictionaries::nl::get_time_unit(&lower),
            Locale::Pt => crate::dictionaries::pt::get_time_unit(&lower),
            Locale::Ru => crate::dictionaries::ru::get_time_unit(&lower),
            Locale::Sv => crate::dictionaries::sv::get_time_unit(&lower),
            Locale::Uk => crate::dictionaries::uk::get_time_unit(&lower),
            Locale::Zh => crate::dictionaries::zh::get_time_unit(text)
                .or_else(|| crate::dictionaries::zh::get_time_unit(&lower)),
        }
    }
}

impl Parser for MultiLocaleTimeUnitAgoParser {
    fn name(&self) -> &'static str {
        "MultiLocaleTimeUnitAgoParser"
    }

    fn should_apply(&self, context: &ParsingContext) -> bool {
        context.has_token_type(TokenType::Ago) && context.has_token_type(TokenType::TimeUnit)
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

            let num_str = caps.get(1).map(|m| m.as_str()).unwrap_or("1");
            let unit_str = caps.get(2).map(|m| m.as_str()).unwrap_or_default();

            let num = self.parse_number(num_str);
            let Some(unit) = self.lookup_time_unit(unit_str) else {
                continue;
            };

            // Create negative duration
            let mut duration = Duration::new();
            match unit {
                TimeUnit::Second => duration.second = Some(-num),
                TimeUnit::Minute => duration.minute = Some(-num),
                TimeUnit::Hour => duration.hour = Some(-num),
                TimeUnit::Day => duration.day = Some(-num),
                TimeUnit::Week => duration.week = Some(-num),
                TimeUnit::Month => duration.month = Some(-num),
                TimeUnit::Year => duration.year = Some(-num),
                TimeUnit::Quarter => duration.quarter = Some(-num),
                TimeUnit::Millisecond => duration.millisecond = Some(-num),
            }

            let target_date = add_duration(ref_date, &duration);

            let mut components = context.create_components();
            components.assign(Component::Year, target_date.year());
            components.assign(Component::Month, target_date.month() as i32);
            components.assign(Component::Day, target_date.day() as i32);

            if duration.has_time_component() {
                use chrono::Timelike;
                components.assign(Component::Hour, target_date.hour() as i32);
                components.assign(Component::Minute, target_date.minute() as i32);
                components.assign(Component::Second, target_date.second() as i32);
            }

            results.push(context.create_result(
                index,
                index + matched_text.len(),
                components,
                None,
            ));
        }

        Ok(results)
    }
}
