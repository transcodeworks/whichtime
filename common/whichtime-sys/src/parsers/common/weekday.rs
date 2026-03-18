//! Multi-locale weekday parser: Monday, next Friday, last Tuesday, etc.
//!
//! Handles weekday expressions across all supported locales.

use crate::components::Component;
use crate::context::ParsingContext;
use crate::dictionaries::{Locale, RelativeModifier, Weekday};
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use crate::scanner::TokenType;
use chrono::{Datelike, Duration};
use regex::Regex;
use std::sync::LazyLock;

// Locale-specific patterns
static EN_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(?:^|\W)(?:(this|next|last|past|previous)\s+)?(sun(?:day)?|mon(?:day)?|tue(?:s(?:day)?)?|wed(?:nesday)?|thu(?:rs(?:day)?)?|fri(?:day)?|sat(?:urday)?)(?:\W|$)").unwrap()
});

static DE_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(?:^|\W)(?:(dieser?|diese[nms]?|nächster?|nächste[nms]?|naechster?|naechste[nms]?|letzter?|letzte[nms]?|kommender?|kommende[nms]?|vergangener?|vergangene[nms]?|voriger?|vorige[nms]?)\s+)?(sonntag|so|montag|mo|dienstag|di|mittwoch|mi|donnerstag|do|freitag|fr|samstag|sa)(?:\W|$)").unwrap()
});

static ES_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(?:^|\W)(?:(?:el\s+)?(este|próximo|proximo|pasado|último|ultimo)\s+)?(domingo|lunes|martes|miércoles|miercoles|jueves|viernes|sábado|sabado)(?:\W|$)").unwrap()
});

static FR_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(?:^|\W)(?:(ce|prochain|dernier|passé|passee)\s+)?(dimanche|lundi|mardi|mercredi|jeudi|vendredi|samedi)(?:\W|$)").unwrap()
});

static IT_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    // Italian supports both "modifier weekday" and "weekday modifier" orders
    Regex::new(r"(?i)(?:^|\W)(?:(?:(questo|prossimo|scorso|passato)\s+)?(domenica|lunedì|lunedi|martedì|martedi|mercoledì|mercoledi|giovedì|giovedi|venerdì|venerdi|sabato)(?:\s+(prossimo|prossima|scorso|scorsa|passato|passata))?)(?:\W|$)").unwrap()
});

static JA_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?:(今週|来週|先週|前週)の?)?(日曜日?|月曜日?|火曜日?|水曜日?|木曜日?|金曜日?|土曜日?)",
    )
    .unwrap()
});

static NL_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(?:^|\W)(?:(deze|volgende|vorige|afgelopen|komende)\s+)?(zondag|maandag|dinsdag|woensdag|donderdag|vrijdag|zaterdag)(?:\W|$)").unwrap()
});

static PT_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(?:^|\W)(?:(este|próximo|proximo|passado|último|ultimo)\s+)?(domingo|segunda(?:-feira)?|terça(?:-feira)?|terca(?:-feira)?|quarta(?:-feira)?|quinta(?:-feira)?|sexta(?:-feira)?|sábado|sabado)(?:\W|$)").unwrap()
});

static RU_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(?:^|\W)(?:(?:в\s+)?(этот|эту|следующий|следующую|прошлый|прошлую|предыдущий|предыдущую)\s+)?(воскресенье|понедельник|вторник|среду?|четверг|пятницу?|субботу?)(?:\W|$)").unwrap()
});

static SV_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(?:^|\W)(?:(?:på\s+)?(denna|nästa|nasta|förra|forra|kommande)\s+)?(söndag|sondag|måndag|mandag|tisdag|onsdag|torsdag|fredag|lördag|lordag)(?:\W|$)").unwrap()
});

static UK_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(?:^|\W)(?:(?:у\s+|в\s+)?(цей|цю|наступний|наступну|минулий|минулу|попередній|попередню)\s+)?(неділю?|понеділок|вівторок|середу?|четвер|п'ятницю?|п'ятниця|субот[уа]?)(?:\W|$)").unwrap()
});

static ZH_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(这个?|這個?|下个?|下個?|上个?|上個?)?(星期[日一二三四五六]|周[日一二三四五六]|週[日一二三四五六]|礼拜[日天一二三四五六]|禮拜[日天一二三四五六])").unwrap()
});

/// Multi-locale weekday parser
pub struct MultiLocaleWeekdayParser {
    locale: Locale,
}

impl MultiLocaleWeekdayParser {
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

    fn lookup_weekday(&self, text: &str) -> Option<Weekday> {
        let lower = text.to_lowercase();
        match self.locale {
            Locale::En => crate::dictionaries::en::get_weekday(&lower),
            Locale::De => crate::dictionaries::de::get_weekday(&lower),
            Locale::Es => crate::dictionaries::es::get_weekday(&lower),
            Locale::Fr => crate::dictionaries::fr::get_weekday(&lower),
            Locale::It => crate::dictionaries::it::get_weekday(&lower),
            Locale::Ja => crate::dictionaries::ja::get_weekday(text)
                .or_else(|| crate::dictionaries::ja::get_weekday(&lower)),
            Locale::Nl => crate::dictionaries::nl::get_weekday(&lower),
            Locale::Pt => crate::dictionaries::pt::get_weekday(&lower),
            Locale::Ru => crate::dictionaries::ru::get_weekday(&lower),
            Locale::Sv => crate::dictionaries::sv::get_weekday(&lower),
            Locale::Uk => crate::dictionaries::uk::get_weekday(&lower),
            Locale::Zh => crate::dictionaries::zh::get_weekday(text)
                .or_else(|| crate::dictionaries::zh::get_weekday(&lower)),
        }
    }

    fn lookup_relative_modifier(&self, text: &str) -> Option<RelativeModifier> {
        let lower = text.to_lowercase();
        match self.locale {
            Locale::En => crate::dictionaries::en::get_relative_modifier(&lower),
            Locale::De => crate::dictionaries::de::get_relative_modifier(&lower),
            Locale::Es => crate::dictionaries::es::get_relative_modifier(&lower),
            Locale::Fr => crate::dictionaries::fr::get_relative_modifier(&lower),
            Locale::It => crate::dictionaries::it::get_relative_modifier(&lower),
            Locale::Ja => crate::dictionaries::ja::get_relative_modifier(text)
                .or_else(|| crate::dictionaries::ja::get_relative_modifier(&lower)),
            Locale::Nl => crate::dictionaries::nl::get_relative_modifier(&lower),
            Locale::Pt => crate::dictionaries::pt::get_relative_modifier(&lower),
            Locale::Ru => crate::dictionaries::ru::get_relative_modifier(&lower),
            Locale::Sv => crate::dictionaries::sv::get_relative_modifier(&lower),
            Locale::Uk => crate::dictionaries::uk::get_relative_modifier(&lower),
            Locale::Zh => crate::dictionaries::zh::get_relative_modifier(text)
                .or_else(|| crate::dictionaries::zh::get_relative_modifier(&lower)),
        }
    }
}

impl Parser for MultiLocaleWeekdayParser {
    fn name(&self) -> &'static str {
        "MultiLocaleWeekdayParser"
    }

    fn should_apply(&self, context: &ParsingContext) -> bool {
        context.has_token_type(TokenType::Weekday)
    }

    fn parse(&self, context: &ParsingContext) -> Result<Vec<ParsedResult>> {
        let mut results = Vec::new();
        let pattern = self.get_pattern();
        let ref_date = context.reference.instant;
        let ref_weekday = ref_date.weekday().num_days_from_sunday();

        for mat in pattern.find_iter(context.text) {
            let matched_text = mat.as_str();
            let index = mat.start();

            let Some(caps) = pattern.captures(matched_text) else {
                continue;
            };

            // Group 1: Pre-modifier (before weekday), Group 2: Weekday, Group 3: Post-modifier (Italian)
            let pre_modifier_str = caps.get(1).map(|m| m.as_str());
            let weekday_str = caps.get(2).map(|m| m.as_str()).unwrap_or_default();
            let post_modifier_str = caps.get(3).map(|m| m.as_str());

            let Some(weekday) = self.lookup_weekday(weekday_str) else {
                continue;
            };

            // Use pre-modifier if available, otherwise use post-modifier (Italian word order)
            let modifier = pre_modifier_str
                .and_then(|s| self.lookup_relative_modifier(s))
                .or_else(|| post_modifier_str.and_then(|s| self.lookup_relative_modifier(s)));

            // Calculate days offset
            let days_offset = match modifier {
                Some(RelativeModifier::Next) => {
                    let diff = (weekday as i64) - (ref_weekday as i64);
                    if diff <= 0 { diff + 7 } else { diff }
                }
                Some(RelativeModifier::Last) => {
                    let diff = (weekday as i64) - (ref_weekday as i64);
                    if diff >= 0 { diff - 7 } else { diff }
                }
                Some(RelativeModifier::This) | None => {
                    // Find closest occurrence (past or future)
                    let diff = (weekday as i64) - (ref_weekday as i64);
                    if diff == 0 {
                        0 // Same day
                    } else if diff > 0 {
                        // Target is ahead in the week
                        if diff <= 3 {
                            diff // Go forward
                        } else {
                            diff - 7 // Go back to previous week
                        }
                    } else {
                        // diff < 0, target is behind in the week
                        if diff >= -3 {
                            diff // Go back
                        } else {
                            diff + 7 // Go to next week
                        }
                    }
                }
            };

            let target_date = ref_date + Duration::days(days_offset);

            let mut components = context.create_components();
            components.assign(Component::Year, target_date.year());
            components.assign(Component::Month, target_date.month() as i32);
            components.assign(Component::Day, target_date.day() as i32);
            components.assign(Component::Weekday, weekday as i32);

            // Find actual text bounds
            let actual_start = matched_text
                .find(|c: char| c.is_alphanumeric())
                .unwrap_or(0);
            let actual_end = matched_text
                .rfind(|c: char| c.is_alphanumeric())
                .map(|i| i + matched_text[i..].chars().next().map_or(1, char::len_utf8))
                .unwrap_or(matched_text.len());

            results.push(context.create_result(
                index + actual_start,
                index + actual_end,
                components,
                None,
            ));
        }

        Ok(results)
    }
}
