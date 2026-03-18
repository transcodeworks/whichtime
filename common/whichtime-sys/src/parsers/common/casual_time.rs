//! Multi-locale casual time parser: noon, midnight, morning, afternoon, evening, night
//!
//! Handles casual time expressions across all supported locales.

use crate::components::Component;
use crate::context::ParsingContext;
use crate::dictionaries::{CasualTimeType, Locale, RelativeModifier};
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use crate::scanner::TokenType;
use crate::types::Meridiem;
use chrono::{Datelike, Duration, Timelike};
use regex::Regex;
use std::sync::LazyLock;

// Locale-specific patterns
static EN_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(?:(this|last|next|past|previous)\s+)?(noon|midday|midnight|morning|afternoon|evening|night)\b").unwrap()
});

static DE_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(?:(dieser?|diese[nms]?|letzter?|letzte[nms]?|nächster?|nächste[nms]?|naechster?|naechste[nms]?)\s+)?(mittag|mitternacht|morgens?|vormittags?|nachmittags?|abends?|nachts?)\b").unwrap()
});

static ES_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(?:(este|esta|pasado|pasada|próximo|próxima|proximo|proxima)\s+)?(mediodía|mediodia|medianoche|mañana|manana|tarde|noche)(?:\s+pasad[ao]|)?\b").unwrap()
});

static FR_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(?:(ce|cette|dernier|dernière|derniere|prochain|prochaine)\s+)?(midi|minuit|matin|après-midi|apres-midi|soir|nuit)\b").unwrap()
});

static IT_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(?:(questo|questa|scorso|scorsa|prossimo|prossima)\s+)?(mezzogiorno|mezzanotte|mattina|mattino|pomeriggio|sera|notte)\b").unwrap()
});

static JA_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(正午|真夜中|朝|午前|午後|夕方|夜|深夜)").unwrap());

static NL_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(?:(deze|vorige|volgende|afgelopen|komende)\s+)?(middag|middernacht|ochtend|'s\s*ochtends|'s\s*middags|'s\s*avonds|avond|nacht|'s\s*nachts)\b").unwrap()
});

static PT_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(?:(este|esta|passado|passada|próximo|próxima|proximo|proxima)\s+)?(meio-dia|meio\s*dia|meia-noite|meia\s*noite|manhã|manha|tarde|noite)\b").unwrap()
});

static RU_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(?:(этот|эта|прошлый|прошлая|следующий|следующая)\s+)?(полдень|в\s*полдень|полночь|в\s*полночь|утром?|днём|днем|вечером?|ночью?)\b").unwrap()
});

static SV_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(?:(denna|förra|forra|nästa|nasta)\s+)?(middag|midnatt|morgon(?:en)?|på\s*morgonen|förmiddag(?:en)?|formiddag(?:en)?|eftermiddag(?:en)?|kväll(?:en)?|kvall(?:en)?|natt(?:en)?)\b").unwrap()
});

static UK_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(?:(цей|ця|минулий|минула|наступний|наступна)\s+)?(полудень|опівдні|опівночі|вранці|ранок|вдень|ввечері|вночі)\b").unwrap()
});

static ZH_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(中午|正午|午夜|凌晨|早上|上午|下午|傍晚|晚上|深夜)").unwrap());

/// Multi-locale casual time parser
pub struct MultiLocaleCasualTimeParser {
    locale: Locale,
}

impl MultiLocaleCasualTimeParser {
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

    fn lookup_casual_time(&self, text: &str) -> Option<CasualTimeType> {
        let lower = text.to_lowercase();
        // Normalize whitespace
        let normalized: String = lower.split_whitespace().collect::<Vec<_>>().join(" ");

        match self.locale {
            Locale::En => crate::dictionaries::en::get_casual_time(&normalized),
            Locale::De => crate::dictionaries::de::get_casual_time(&normalized),
            Locale::Es => crate::dictionaries::es::get_casual_time(&normalized),
            Locale::Fr => crate::dictionaries::fr::get_casual_time(&normalized),
            Locale::It => crate::dictionaries::it::get_casual_time(&normalized),
            Locale::Ja => crate::dictionaries::ja::get_casual_time(text)
                .or_else(|| crate::dictionaries::ja::get_casual_time(&normalized)),
            Locale::Nl => crate::dictionaries::nl::get_casual_time(&normalized),
            Locale::Pt => crate::dictionaries::pt::get_casual_time(&normalized),
            Locale::Ru => crate::dictionaries::ru::get_casual_time(&normalized),
            Locale::Sv => crate::dictionaries::sv::get_casual_time(&normalized),
            Locale::Uk => crate::dictionaries::uk::get_casual_time(&normalized),
            Locale::Zh => crate::dictionaries::zh::get_casual_time(text)
                .or_else(|| crate::dictionaries::zh::get_casual_time(&normalized)),
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

    fn is_digit_like(ch: char) -> bool {
        ch.is_ascii_digit()
            || ('０'..='９').contains(&ch)
            || matches!(
                ch,
                '〇' | '一' | '二' | '三' | '四' | '五' | '六' | '七' | '八' | '九' | '十'
            )
    }

    fn has_trailing_number(text: &str, idx: usize) -> bool {
        if idx >= text.len() {
            return false;
        }

        let chars = text[idx..].chars();
        for ch in chars {
            if ch.is_whitespace() {
                continue;
            }
            return Self::is_digit_like(ch);
        }
        false
    }
}

impl Parser for MultiLocaleCasualTimeParser {
    fn name(&self) -> &'static str {
        "MultiLocaleCasualTimeParser"
    }

    fn should_apply(&self, context: &ParsingContext) -> bool {
        context.has_token_type(TokenType::CasualTime)
    }

    fn parse(&self, context: &ParsingContext) -> Result<Vec<ParsedResult>> {
        let mut results = Vec::new();
        let pattern = self.get_pattern();
        let ref_date = context.reference.instant;

        for mat in pattern.find_iter(context.text) {
            let matched_text = mat.as_str();
            let index = mat.start();
            if matches!(self.locale, Locale::Ja)
                && Self::has_trailing_number(context.text, mat.end())
            {
                continue;
            }

            let Some(caps) = pattern.captures(matched_text) else {
                continue;
            };

            // Try to get modifier and time word from captures
            let (modifier_str, time_word) = match self.locale {
                Locale::Ja | Locale::Zh => {
                    (None, caps.get(1).map(|m| m.as_str()).unwrap_or_default())
                }
                _ => (
                    caps.get(1).map(|m| m.as_str()),
                    caps.get(2).map(|m| m.as_str()).unwrap_or_default(),
                ),
            };

            let Some(time_type) = self.lookup_casual_time(time_word) else {
                continue;
            };

            let modifier = modifier_str.and_then(|s| self.lookup_relative_modifier(s));

            let mut components = context.create_components();

            // Calculate target date based on modifier
            let target_date = match modifier {
                Some(RelativeModifier::Last) => {
                    if ref_date.hour() <= 6 && matches!(time_type, CasualTimeType::Night) {
                        ref_date
                    } else {
                        ref_date - Duration::days(1)
                    }
                }
                Some(RelativeModifier::Next) => ref_date + Duration::days(1),
                Some(RelativeModifier::This) | None => ref_date,
            };

            // Set date components
            components.assign(Component::Year, target_date.year());
            components.assign(Component::Month, target_date.month() as i32);
            components.assign(Component::Day, target_date.day() as i32);

            // Set time components based on time_type
            match time_type {
                CasualTimeType::Noon => {
                    components.assign(Component::Hour, 12);
                    components.assign(Component::Minute, 0);
                    components.assign(Component::Second, 0);
                    components.assign(Component::Meridiem, Meridiem::PM as i32);
                }
                CasualTimeType::Midnight => {
                    if matches!(modifier, Some(RelativeModifier::Last)) {
                        components.assign(Component::Day, target_date.day() as i32);
                    } else if modifier.is_none() {
                        let next_day = ref_date + Duration::days(1);
                        components.assign(Component::Year, next_day.year());
                        components.assign(Component::Month, next_day.month() as i32);
                        components.assign(Component::Day, next_day.day() as i32);
                    }
                    components.assign(Component::Hour, 0);
                    components.assign(Component::Minute, 0);
                    components.assign(Component::Second, 0);
                }
                CasualTimeType::Morning => {
                    components.imply(Component::Hour, 6);
                    components.imply(Component::Minute, 0);
                    components.assign(Component::Meridiem, Meridiem::AM as i32);
                }
                CasualTimeType::Afternoon => {
                    components.imply(Component::Hour, 15);
                    components.imply(Component::Minute, 0);
                    components.assign(Component::Meridiem, Meridiem::PM as i32);
                }
                CasualTimeType::Evening => {
                    components.imply(Component::Hour, 20);
                    components.imply(Component::Minute, 0);
                    components.assign(Component::Meridiem, Meridiem::PM as i32);
                }
                CasualTimeType::Night => {
                    components.imply(Component::Hour, 22);
                    components.imply(Component::Minute, 0);
                    components.assign(Component::Meridiem, Meridiem::PM as i32);
                }
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
