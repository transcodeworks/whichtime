//! Multi-locale casual date parser: now, today, tomorrow, yesterday, etc.
//!
//! This parser handles casual date expressions across all supported locales
//! by using locale-specific patterns and dictionary lookups. It also handles
//! combined casual date + time expressions like "heute Morgen" (today morning).

use crate::components::Component;
use crate::context::ParsingContext;
use crate::dictionaries::{CasualDateType, CasualTimeType, Locale};
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use crate::scanner::TokenType;
use crate::types::Meridiem;
use chrono::{Datelike, Duration, Timelike};
use regex::Regex;
use std::sync::LazyLock;

// Locale-specific patterns - now with optional trailing casual time
static EN_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(now|today|tonight|tomorrow|overmorrow|tmr|tmrw|yesterday)(?:\s+(morning|afternoon|evening|night))?\b").unwrap()
});

static DE_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)\b(jetzt|heute|morgen|gestern|übermorgen|uebermorgen|vorgestern)(?:\s+(Morgen|Morgens|Vormittag|Vormittags|Nachmittag|Nachmittags|Abend|Abends|Nacht|Nachts))?\b",
    )
    .unwrap()
});

static ES_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(esta\s+mañana|esta\s+manana|esta\s+tarde|esta\s+noche|ahora|hoy|mañana|manana|ayer|pasado\s*mañana|pasado\s*manana|anteayer)(?:\s+(?:de\s+)?(mañana|manana|tarde|noche))?\b").unwrap()
});

static FR_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(ce\s+matin|cet\s+après-?midi|cet\s+apres-?midi|cet\s+aprem|ce\s+soir|maintenant|aujourd'?hui|demain|hier|après-?demain|apres-?demain|avant-?hier)(?:\s+(matin|après-?midi|apres-?midi|soir|nuit))?\b").unwrap()
});

static IT_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(adesso|ora|oggi|stanotte|stasera|stamattina|domani|ieri|dopodomani|l'?altro\s*ieri|altroieri)(?:\s+(mattina|pomeriggio|sera|notte))?\b")
        .unwrap()
});

static JA_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(今日|きょう|本日|ほんじつ|今夜|こんや|今晩|こんばん|明日|あした|あす|昨日|きのう|さくじつ|明後日|あさって|一昨日|おととい)(?:の(朝|午前|午後|夕方|夜|深夜))?").unwrap()
});

static NL_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(nu|vandaag|morgen|gisteren|overmorgen|eergisteren)(?:\s+(ochtend|middag|avond|nacht))?\b").unwrap()
});

static PT_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(agora|hoje|amanhã|amanha|ontem|depois\s*de\s*amanhã|depois\s*de\s*amanha|anteontem)(?:\s+(?:de\s+|à\s+)?(manhã|manha|tarde|noite))?\b").unwrap()
});

static RU_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(сейчас|сегодня|завтра|вчера|послезавтра|послепослезавтра|позавчера|позапозавчера)(?:\s+(утром|днём|днем|вечером|ночью))?\b").unwrap()
});

static SV_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(nu|idag|imorgon|igår|igar|i\s*övermorgon|i\s*overmorgon|förrgår|forrgar)(?:\s+(?:på\s+)?(morgonen|förmiddagen|formiddagen|eftermiddagen|kvällen|kvallen|natten|midnatt))?\b").unwrap()
});

static UK_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(зараз|сьогодні|завтра|вчора|післязавтра|післяпіслязавтра|позавчора|позапозавчора)(?:\s+(вранці|вдень|ввечері|вночі))?\b").unwrap()
});

static ZH_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(现在|現在|今天|今日|今晚|明天|明日|聽日|昨天|昨日|后天|後天|前天|而家)(早上|上午|中午|下午|傍晚|晚上)?").unwrap()
});

/// Multi-locale casual date parser
pub struct MultiLocaleCasualDateParser {
    locale: Locale,
}

impl MultiLocaleCasualDateParser {
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

    fn lookup_casual_date(&self, text: &str) -> Option<CasualDateType> {
        let lower = text.to_lowercase();
        let normalized: String = lower.split_whitespace().collect::<Vec<_>>().join(" ");

        match self.locale {
            Locale::En => crate::dictionaries::en::get_casual_date(&normalized),
            Locale::De => crate::dictionaries::de::get_casual_date(&normalized),
            Locale::Es => crate::dictionaries::es::get_casual_date(&normalized),
            Locale::Fr => crate::dictionaries::fr::get_casual_date(&normalized),
            Locale::It => crate::dictionaries::it::get_casual_date(&normalized),
            Locale::Ja => crate::dictionaries::ja::get_casual_date(&normalized)
                .or_else(|| crate::dictionaries::ja::get_casual_date(text)), // Japanese doesn't need lowercase
            Locale::Nl => crate::dictionaries::nl::get_casual_date(&normalized),
            Locale::Pt => crate::dictionaries::pt::get_casual_date(&normalized),
            Locale::Ru => crate::dictionaries::ru::get_casual_date(&normalized),
            Locale::Sv => crate::dictionaries::sv::get_casual_date(&normalized),
            Locale::Uk => crate::dictionaries::uk::get_casual_date(&normalized),
            Locale::Zh => crate::dictionaries::zh::get_casual_date(&normalized)
                .or_else(|| crate::dictionaries::zh::get_casual_date(text)), // Chinese doesn't need lowercase
        }
    }

    fn lookup_casual_time(&self, text: &str) -> Option<CasualTimeType> {
        let lower = text.to_lowercase();
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

    fn apply_casual_time(
        &self,
        components: &mut crate::components::FastComponents,
        time_type: CasualTimeType,
        time_word: Option<&str>,
    ) {
        match time_type {
            CasualTimeType::Noon => {
                components.assign(Component::Hour, 12);
                components.assign(Component::Minute, 0);
                components.assign(Component::Meridiem, Meridiem::PM as i32);
            }
            CasualTimeType::Midnight => {
                components.assign(Component::Hour, 0);
                components.assign(Component::Minute, 0);
            }
            CasualTimeType::Morning => {
                // German "Vormittag" (forenoon) is ~9 AM, while "Morgen" is ~6 AM
                let hour = if self.locale == Locale::De {
                    match time_word {
                        Some(w) if w.starts_with("vormittag") => 9,
                        _ => 6,
                    }
                } else {
                    6
                };
                components.imply(Component::Hour, hour);
                components.assign(Component::Meridiem, Meridiem::AM as i32);
            }
            CasualTimeType::Afternoon => {
                components.imply(Component::Hour, 15);
                components.assign(Component::Meridiem, Meridiem::PM as i32);
            }
            CasualTimeType::Evening => {
                components.imply(Component::Hour, 18);
                components.assign(Component::Meridiem, Meridiem::PM as i32);
            }
            CasualTimeType::Night => {
                components.imply(Component::Hour, 22);
                components.assign(Component::Meridiem, Meridiem::PM as i32);
            }
        }
    }
}

impl Parser for MultiLocaleCasualDateParser {
    fn name(&self) -> &'static str {
        "MultiLocaleCasualDateParser"
    }

    fn should_apply(&self, context: &ParsingContext) -> bool {
        context.has_token_type(TokenType::CasualDate)
    }

    fn parse(&self, context: &ParsingContext) -> Result<Vec<ParsedResult>> {
        let mut results = Vec::new();
        let pattern = self.get_pattern();

        for caps in pattern.captures_iter(context.text) {
            let mat = caps.get(0).unwrap();
            let matched_text = mat.as_str();
            let index = mat.start();

            // Get the date part (first capture group)
            let date_text = caps.get(1).map(|m| m.as_str()).unwrap_or(matched_text);

            let Some(casual_type) = self.lookup_casual_date(date_text) else {
                continue;
            };

            // Get optional time part (second capture group)
            let time_type = caps
                .get(2)
                .and_then(|m| self.lookup_casual_time(m.as_str()));

            let mut components = context.create_components();
            let ref_date = context.reference.instant;

            match casual_type {
                CasualDateType::Now => {
                    components.assign(Component::Year, ref_date.year());
                    components.assign(Component::Month, ref_date.month() as i32);
                    components.assign(Component::Day, ref_date.day() as i32);
                    components.assign(Component::Hour, ref_date.hour() as i32);
                    components.assign(Component::Minute, ref_date.minute() as i32);
                    components.assign(Component::Second, ref_date.second() as i32);
                }
                CasualDateType::Today => {
                    components.assign(Component::Year, ref_date.year());
                    components.assign(Component::Month, ref_date.month() as i32);
                    components.assign(Component::Day, ref_date.day() as i32);
                }
                CasualDateType::Tonight => {
                    components.assign(Component::Year, ref_date.year());
                    components.assign(Component::Month, ref_date.month() as i32);
                    components.assign(Component::Day, ref_date.day() as i32);
                    components.imply(Component::Hour, 22);
                }
                CasualDateType::Tomorrow => {
                    let tomorrow = ref_date + Duration::days(1);
                    components.assign(Component::Year, tomorrow.year());
                    components.assign(Component::Month, tomorrow.month() as i32);
                    components.assign(Component::Day, tomorrow.day() as i32);
                }
                CasualDateType::Yesterday => {
                    let yesterday = ref_date - Duration::days(1);
                    components.assign(Component::Year, yesterday.year());
                    components.assign(Component::Month, yesterday.month() as i32);
                    components.assign(Component::Day, yesterday.day() as i32);
                }
                CasualDateType::Overmorrow => {
                    let day_after = ref_date + Duration::days(2);
                    components.assign(Component::Year, day_after.year());
                    components.assign(Component::Month, day_after.month() as i32);
                    components.assign(Component::Day, day_after.day() as i32);
                }
                CasualDateType::DayBeforeYesterday => {
                    let day_before = ref_date - Duration::days(2);
                    components.assign(Component::Year, day_before.year());
                    components.assign(Component::Month, day_before.month() as i32);
                    components.assign(Component::Day, day_before.day() as i32);
                }
                CasualDateType::ThisMorning => {
                    components.assign(Component::Year, ref_date.year());
                    components.assign(Component::Month, ref_date.month() as i32);
                    components.assign(Component::Day, ref_date.day() as i32);
                    components.imply(Component::Hour, 6);
                }
                CasualDateType::ThisAfternoon => {
                    components.assign(Component::Year, ref_date.year());
                    components.assign(Component::Month, ref_date.month() as i32);
                    components.assign(Component::Day, ref_date.day() as i32);
                    components.imply(Component::Hour, 15);
                }
                CasualDateType::ThisEvening => {
                    components.assign(Component::Year, ref_date.year());
                    components.assign(Component::Month, ref_date.month() as i32);
                    components.assign(Component::Day, ref_date.day() as i32);
                    components.imply(Component::Hour, 20);
                }
            }

            // Apply time component if present
            if let Some(time) = time_type {
                // Get the actual matched time word for locale-specific hour handling
                let time_word = caps.get(2).map(|m| m.as_str().to_lowercase());
                self.apply_casual_time(&mut components, time, time_word.as_deref());
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
