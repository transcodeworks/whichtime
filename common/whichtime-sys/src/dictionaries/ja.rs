//! Japanese (日本語) locale dictionaries

use super::{CasualDateType, CasualTimeType, RelativeModifier, TimeUnit, Weekday};
use phf::phf_map;

/// Weekday dictionary
pub static WEEKDAY_MAP: phf::Map<&'static str, Weekday> = phf_map! {
    "日曜日" => Weekday::Sunday,
    "日曜" => Weekday::Sunday,
    "日" => Weekday::Sunday,
    "月曜日" => Weekday::Monday,
    "月曜" => Weekday::Monday,
    "月" => Weekday::Monday,
    "火曜日" => Weekday::Tuesday,
    "火曜" => Weekday::Tuesday,
    "火" => Weekday::Tuesday,
    "水曜日" => Weekday::Wednesday,
    "水曜" => Weekday::Wednesday,
    "水" => Weekday::Wednesday,
    "木曜日" => Weekday::Thursday,
    "木曜" => Weekday::Thursday,
    "木" => Weekday::Thursday,
    "金曜日" => Weekday::Friday,
    "金曜" => Weekday::Friday,
    "金" => Weekday::Friday,
    "土曜日" => Weekday::Saturday,
    "土曜" => Weekday::Saturday,
    "土" => Weekday::Saturday,
};

/// Month dictionary (Japanese uses numeric months but with kanji suffix)
pub static MONTH_MAP: phf::Map<&'static str, u32> = phf_map! {
    "1月" => 1,
    "一月" => 1,
    "2月" => 2,
    "二月" => 2,
    "3月" => 3,
    "三月" => 3,
    "4月" => 4,
    "四月" => 4,
    "5月" => 5,
    "五月" => 5,
    "6月" => 6,
    "六月" => 6,
    "7月" => 7,
    "七月" => 7,
    "8月" => 8,
    "八月" => 8,
    "9月" => 9,
    "九月" => 9,
    "10月" => 10,
    "十月" => 10,
    "11月" => 11,
    "十一月" => 11,
    "12月" => 12,
    "十二月" => 12,
};

/// Japanese number characters
pub static NUMBER_MAP: phf::Map<&'static str, u32> = phf_map! {
    "零" => 0,
    "〇" => 0,
    "一" => 1,
    "二" => 2,
    "三" => 3,
    "四" => 4,
    "五" => 5,
    "六" => 6,
    "七" => 7,
    "八" => 8,
    "九" => 9,
    "十" => 10,
};

/// Time unit dictionary
pub static TIME_UNIT_MAP: phf::Map<&'static str, TimeUnit> = phf_map! {
    "秒" => TimeUnit::Second,
    "秒間" => TimeUnit::Second,
    "分" => TimeUnit::Minute,
    "分間" => TimeUnit::Minute,
    "時間" => TimeUnit::Hour,
    "時" => TimeUnit::Hour,
    "日" => TimeUnit::Day,
    "日間" => TimeUnit::Day,
    "週" => TimeUnit::Week,
    "週間" => TimeUnit::Week,
    "ヶ月" => TimeUnit::Month,
    "カ月" => TimeUnit::Month,
    "か月" => TimeUnit::Month,
    "月" => TimeUnit::Month,
    "月間" => TimeUnit::Month,
    "四半期" => TimeUnit::Quarter,
    "年" => TimeUnit::Year,
    "年間" => TimeUnit::Year,
};

/// Casual date keywords
pub static CASUAL_DATE_MAP: phf::Map<&'static str, CasualDateType> = phf_map! {
    "今" => CasualDateType::Now,
    "今日" => CasualDateType::Today,
    "きょう" => CasualDateType::Today,
    "本日" => CasualDateType::Today,
    "ほんじつ" => CasualDateType::Today,
    "今夜" => CasualDateType::Tonight,
    "こんや" => CasualDateType::Tonight,
    "今晩" => CasualDateType::Tonight,
    "こんばん" => CasualDateType::Tonight,
    "今夕" => CasualDateType::Tonight,
    "こんゆう" => CasualDateType::Tonight,
    "今朝" => CasualDateType::Today,
    "けさ" => CasualDateType::Today,
    "明日" => CasualDateType::Tomorrow,
    "あした" => CasualDateType::Tomorrow,
    "あす" => CasualDateType::Tomorrow,
    "昨日" => CasualDateType::Yesterday,
    "きのう" => CasualDateType::Yesterday,
    "さくじつ" => CasualDateType::Yesterday,
    "明後日" => CasualDateType::Overmorrow,
    "あさって" => CasualDateType::Overmorrow,
    "一昨日" => CasualDateType::DayBeforeYesterday,
    "おととい" => CasualDateType::DayBeforeYesterday,
};

/// Casual time keywords
pub static CASUAL_TIME_MAP: phf::Map<&'static str, CasualTimeType> = phf_map! {
    "正午" => CasualTimeType::Noon,
    "昼" => CasualTimeType::Noon,
    "真夜中" => CasualTimeType::Midnight,
    "夜中" => CasualTimeType::Midnight,
    "朝" => CasualTimeType::Morning,
    "午前" => CasualTimeType::Morning,
    "午後" => CasualTimeType::Afternoon,
    "夕方" => CasualTimeType::Evening,
    "夜" => CasualTimeType::Night,
};

/// Relative modifiers
pub static RELATIVE_MODIFIER_MAP: phf::Map<&'static str, RelativeModifier> = phf_map! {
    "今" => RelativeModifier::This,
    "この" => RelativeModifier::This,
    "今週" => RelativeModifier::This,
    "次" => RelativeModifier::Next,
    "来" => RelativeModifier::Next,
    "次の" => RelativeModifier::Next,
    "来週" => RelativeModifier::Next,
    "前" => RelativeModifier::Last,
    "先" => RelativeModifier::Last,
    "先週" => RelativeModifier::Last,
    "去" => RelativeModifier::Last,
    "昨" => RelativeModifier::Last,
};

// ============================================================================
// Lookup functions
// ============================================================================

#[inline]
pub fn get_weekday(s: &str) -> Option<Weekday> {
    WEEKDAY_MAP.get(s).copied()
}

#[inline]
pub fn get_month(s: &str) -> Option<u32> {
    MONTH_MAP.get(s).copied()
}

#[inline]
pub fn get_number(s: &str) -> Option<u32> {
    NUMBER_MAP.get(s).copied()
}

#[inline]
pub fn get_time_unit(s: &str) -> Option<TimeUnit> {
    TIME_UNIT_MAP.get(s).copied()
}

#[inline]
pub fn get_casual_date(s: &str) -> Option<CasualDateType> {
    CASUAL_DATE_MAP.get(s).copied()
}

#[inline]
pub fn get_casual_time(s: &str) -> Option<CasualTimeType> {
    CASUAL_TIME_MAP.get(s).copied()
}

#[inline]
pub fn get_relative_modifier(s: &str) -> Option<RelativeModifier> {
    RELATIVE_MODIFIER_MAP.get(s).copied()
}

/// Convert full-width characters to half-width (hankaku)
pub fn to_hankaku(text: &str) -> String {
    text.chars()
        .map(|c| {
            match c {
                '\u{2019}' => '\u{0027}', // Right single quotation mark -> apostrophe
                '\u{201D}' => '\u{0022}', // Right double quotation mark -> quote
                '\u{3000}' => '\u{0020}', // Ideographic space -> space
                '\u{FFE5}' => '\u{00A5}', // Full-width yen sign -> yen sign
                // Full-width alphanumeric and punctuation (FF01-FF5E)
                c if ('\u{FF01}'..='\u{FF5E}').contains(&c) => {
                    char::from_u32(c as u32 - 0xFEE0).unwrap_or(c)
                }
                _ => c,
            }
        })
        .collect()
}

/// Convert Japanese string to number
/// Handles Japanese number characters like 一, 二, 三, 十, etc.
pub fn ja_string_to_number(text: &str) -> u32 {
    let mut number = 0u32;

    for char in text.chars() {
        let char_str = char.to_string();
        if let Some(val) = get_number(&char_str) {
            if char_str == "十" {
                number = if number == 0 { val } else { number * val };
            } else {
                number += val;
            }
        }
    }

    number
}

/// Parse a number pattern (handles kanji numbers, digits)
pub fn parse_number_pattern(text: &str) -> f64 {
    // Try parsing as regular digit first
    if let Ok(n) = text.parse::<f64>() {
        return n;
    }

    // Try Japanese number conversion
    let ja_num = ja_string_to_number(text);
    if ja_num > 0 {
        return ja_num as f64;
    }

    // Try single character kanji
    if let Some(n) = get_number(text) {
        return n as f64;
    }

    0.0
}
