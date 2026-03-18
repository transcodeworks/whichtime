//! Chinese (中文) locale dictionaries
//! Supports both Simplified (Hans) and Traditional (Hant) Chinese

use super::{CasualDateType, CasualTimeType, RelativeModifier, TimeUnit, Weekday};
use phf::phf_map;

/// Weekday dictionary (both simplified and traditional)
pub static WEEKDAY_MAP: phf::Map<&'static str, Weekday> = phf_map! {
    // Simplified
    "星期日" => Weekday::Sunday,
    "星期天" => Weekday::Sunday,
    "周日" => Weekday::Sunday,
    "週日" => Weekday::Sunday,
    "礼拜日" => Weekday::Sunday,
    "禮拜日" => Weekday::Sunday,
    "礼拜天" => Weekday::Sunday,
    "禮拜天" => Weekday::Sunday,
    "星期一" => Weekday::Monday,
    "周一" => Weekday::Monday,
    "週一" => Weekday::Monday,
    "礼拜一" => Weekday::Monday,
    "禮拜一" => Weekday::Monday,
    "星期二" => Weekday::Tuesday,
    "周二" => Weekday::Tuesday,
    "週二" => Weekday::Tuesday,
    "礼拜二" => Weekday::Tuesday,
    "禮拜二" => Weekday::Tuesday,
    "星期三" => Weekday::Wednesday,
    "周三" => Weekday::Wednesday,
    "週三" => Weekday::Wednesday,
    "礼拜三" => Weekday::Wednesday,
    "禮拜三" => Weekday::Wednesday,
    "星期四" => Weekday::Thursday,
    "周四" => Weekday::Thursday,
    "週四" => Weekday::Thursday,
    "礼拜四" => Weekday::Thursday,
    "禮拜四" => Weekday::Thursday,
    "星期五" => Weekday::Friday,
    "周五" => Weekday::Friday,
    "週五" => Weekday::Friday,
    "礼拜五" => Weekday::Friday,
    "禮拜五" => Weekday::Friday,
    "星期六" => Weekday::Saturday,
    "周六" => Weekday::Saturday,
    "週六" => Weekday::Saturday,
    "礼拜六" => Weekday::Saturday,
    "禮拜六" => Weekday::Saturday,
};

/// Month dictionary
pub static MONTH_MAP: phf::Map<&'static str, u32> = phf_map! {
    "一月" => 1,
    "1月" => 1,
    "二月" => 2,
    "2月" => 2,
    "三月" => 3,
    "3月" => 3,
    "四月" => 4,
    "4月" => 4,
    "五月" => 5,
    "5月" => 5,
    "六月" => 6,
    "6月" => 6,
    "七月" => 7,
    "7月" => 7,
    "八月" => 8,
    "8月" => 8,
    "九月" => 9,
    "9月" => 9,
    "十月" => 10,
    "10月" => 10,
    "十一月" => 11,
    "11月" => 11,
    "十二月" => 12,
    "12月" => 12,
};

/// Chinese number characters
pub static NUMBER_MAP: phf::Map<&'static str, u32> = phf_map! {
    "零" => 0,
    "〇" => 0,
    "一" => 1,
    "二" => 2,
    "两" => 2,
    "兩" => 2,
    "三" => 3,
    "四" => 4,
    "五" => 5,
    "六" => 6,
    "七" => 7,
    "八" => 8,
    "九" => 9,
    "十" => 10,
    "百" => 100,
    "千" => 1000,
};

/// Time unit dictionary
pub static TIME_UNIT_MAP: phf::Map<&'static str, TimeUnit> = phf_map! {
    "秒" => TimeUnit::Second,
    "秒钟" => TimeUnit::Second,
    "秒鐘" => TimeUnit::Second,
    "分" => TimeUnit::Minute,
    "分钟" => TimeUnit::Minute,
    "分鐘" => TimeUnit::Minute,
    "小时" => TimeUnit::Hour,
    "小時" => TimeUnit::Hour,
    "钟头" => TimeUnit::Hour,
    "鐘頭" => TimeUnit::Hour,
    "点" => TimeUnit::Hour,
    "點" => TimeUnit::Hour,
    "天" => TimeUnit::Day,
    "日" => TimeUnit::Day,
    "号" => TimeUnit::Day,
    "號" => TimeUnit::Day,
    "周" => TimeUnit::Week,
    "週" => TimeUnit::Week,
    "星期" => TimeUnit::Week,
    "礼拜" => TimeUnit::Week,
    "禮拜" => TimeUnit::Week,
    "月" => TimeUnit::Month,
    "个月" => TimeUnit::Month,
    "個月" => TimeUnit::Month,
    "季度" => TimeUnit::Quarter,
    "季" => TimeUnit::Quarter,
    "年" => TimeUnit::Year,
};

/// Casual date keywords
pub static CASUAL_DATE_MAP: phf::Map<&'static str, CasualDateType> = phf_map! {
    // Simplified Chinese
    "现在" => CasualDateType::Now,
    "今天" => CasualDateType::Today,
    "今晚" => CasualDateType::Tonight,
    "明天" => CasualDateType::Tomorrow,
    "昨天" => CasualDateType::Yesterday,
    "后天" => CasualDateType::Overmorrow,
    "前天" => CasualDateType::DayBeforeYesterday,
    // Traditional Chinese
    "現在" => CasualDateType::Now,
    "今日" => CasualDateType::Today,
    "今夜" => CasualDateType::Tonight,
    "明日" => CasualDateType::Tomorrow,
    "昨日" => CasualDateType::Yesterday,
    "後天" => CasualDateType::Overmorrow,
    // Cantonese
    "而家" => CasualDateType::Now,
    "聽日" => CasualDateType::Tomorrow,
    "尋日" => CasualDateType::Yesterday,
    "琴日" => CasualDateType::Yesterday,
};

/// Casual time keywords
pub static CASUAL_TIME_MAP: phf::Map<&'static str, CasualTimeType> = phf_map! {
    "中午" => CasualTimeType::Noon,
    "正午" => CasualTimeType::Noon,
    "午" => CasualTimeType::Noon,
    "半夜" => CasualTimeType::Midnight,
    "午夜" => CasualTimeType::Midnight,
    "凌晨" => CasualTimeType::Midnight,
    "早上" => CasualTimeType::Morning,
    "早晨" => CasualTimeType::Morning,
    "上午" => CasualTimeType::Morning,
    "下午" => CasualTimeType::Afternoon,
    "傍晚" => CasualTimeType::Evening,
    "晚上" => CasualTimeType::Evening,
    "晚间" => CasualTimeType::Evening,
    "晚間" => CasualTimeType::Evening,
    "夜里" => CasualTimeType::Night,
    "夜裡" => CasualTimeType::Night,
    "夜晚" => CasualTimeType::Night,
};

/// Relative modifiers
pub static RELATIVE_MODIFIER_MAP: phf::Map<&'static str, RelativeModifier> = phf_map! {
    "这" => RelativeModifier::This,
    "這" => RelativeModifier::This,
    "这个" => RelativeModifier::This,
    "這個" => RelativeModifier::This,
    "本" => RelativeModifier::This,
    "今" => RelativeModifier::This,
    "下" => RelativeModifier::Next,
    "下个" => RelativeModifier::Next,
    "下個" => RelativeModifier::Next,
    "明" => RelativeModifier::Next,
    "上" => RelativeModifier::Last,
    "上个" => RelativeModifier::Last,
    "上個" => RelativeModifier::Last,
    "去" => RelativeModifier::Last,
    "前" => RelativeModifier::Last,
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

/// Convert Chinese string to number
/// Handles Chinese number characters like 一, 二, 三, 十, 百, etc.
pub fn zh_string_to_number(text: &str) -> u32 {
    let mut result = 0u32;
    let mut current = 0u32;

    for char in text.chars() {
        let char_str = char.to_string();
        if let Some(val) = get_number(&char_str) {
            match val {
                10 => {
                    // 十 (ten)
                    if current == 0 {
                        current = 10;
                    } else {
                        current *= 10;
                    }
                }
                100 => {
                    // 百 (hundred)
                    if current == 0 {
                        current = 100;
                    } else {
                        result += current * 100;
                        current = 0;
                    }
                }
                1000 => {
                    // 千 (thousand)
                    if current == 0 {
                        current = 1000;
                    } else {
                        result += current * 1000;
                        current = 0;
                    }
                }
                _ => {
                    // Single digit
                    if current >= 10 {
                        current += val;
                    } else {
                        current = val;
                    }
                }
            }
        }
    }

    result + current
}

/// Parse full-width numbers to half-width
pub fn fullwidth_to_halfwidth(text: &str) -> String {
    text.chars()
        .map(|c| {
            match c {
                // Full-width digits (０-９) to half-width (0-9)
                '\u{FF10}'..='\u{FF19}' => char::from_u32(c as u32 - 0xFEE0).unwrap_or(c),
                // Full-width space to half-width
                '\u{3000}' => ' ',
                _ => c,
            }
        })
        .collect()
}

/// Parse a number pattern (handles Chinese numbers, digits)
pub fn parse_number_pattern(text: &str) -> f64 {
    // Convert full-width digits first
    let normalized = fullwidth_to_halfwidth(text);

    // Try parsing as regular digit
    if let Ok(n) = normalized.parse::<f64>() {
        return n;
    }

    // Try Chinese number conversion
    let zh_num = zh_string_to_number(text);
    if zh_num > 0 {
        return zh_num as f64;
    }

    // Try single character
    if let Some(n) = get_number(text) {
        return n as f64;
    }

    0.0
}
