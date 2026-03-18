//! Russian (Русский) locale dictionaries

use super::{CasualDateType, CasualTimeType, RelativeModifier, TimeUnit, Weekday};
use phf::phf_map;

/// Weekday dictionary (includes various cases)
pub static WEEKDAY_MAP: phf::Map<&'static str, Weekday> = phf_map! {
    "воскресенье" => Weekday::Sunday,
    "воскресенья" => Weekday::Sunday,
    "вск" => Weekday::Sunday,
    "вск." => Weekday::Sunday,
    "вс" => Weekday::Sunday,
    "вс." => Weekday::Sunday,
    "понедельник" => Weekday::Monday,
    "понедельника" => Weekday::Monday,
    "пн" => Weekday::Monday,
    "пн." => Weekday::Monday,
    "вторник" => Weekday::Tuesday,
    "вторника" => Weekday::Tuesday,
    "вт" => Weekday::Tuesday,
    "вт." => Weekday::Tuesday,
    "среда" => Weekday::Wednesday,
    "среды" => Weekday::Wednesday,
    "среду" => Weekday::Wednesday,
    "ср" => Weekday::Wednesday,
    "ср." => Weekday::Wednesday,
    "четверг" => Weekday::Thursday,
    "четверга" => Weekday::Thursday,
    "чт" => Weekday::Thursday,
    "чт." => Weekday::Thursday,
    "пятница" => Weekday::Friday,
    "пятницу" => Weekday::Friday,
    "пятницы" => Weekday::Friday,
    "пт" => Weekday::Friday,
    "пт." => Weekday::Friday,
    "суббота" => Weekday::Saturday,
    "субботу" => Weekday::Saturday,
    "субботы" => Weekday::Saturday,
    "сб" => Weekday::Saturday,
    "сб." => Weekday::Saturday,
};

/// Month dictionary (includes various cases)
pub static MONTH_MAP: phf::Map<&'static str, u32> = phf_map! {
    "январь" => 1,
    "января" => 1,
    "январе" => 1,
    "янв" => 1,
    "янв." => 1,
    "февраль" => 2,
    "февраля" => 2,
    "феврале" => 2,
    "фев" => 2,
    "фев." => 2,
    "март" => 3,
    "марта" => 3,
    "марте" => 3,
    "мар" => 3,
    "мар." => 3,
    "апрель" => 4,
    "апреля" => 4,
    "апреле" => 4,
    "апр" => 4,
    "апр." => 4,
    "май" => 5,
    "мая" => 5,
    "мае" => 5,
    "июнь" => 6,
    "июня" => 6,
    "июне" => 6,
    "июль" => 7,
    "июля" => 7,
    "июле" => 7,
    "август" => 8,
    "августа" => 8,
    "августе" => 8,
    "авг" => 8,
    "авг." => 8,
    "сентябрь" => 9,
    "сентября" => 9,
    "сентябре" => 9,
    "сен" => 9,
    "сен." => 9,
    "октябрь" => 10,
    "октября" => 10,
    "октябре" => 10,
    "окт" => 10,
    "окт." => 10,
    "ноябрь" => 11,
    "ноября" => 11,
    "ноябре" => 11,
    "ноя" => 11,
    "ноя." => 11,
    "декабрь" => 12,
    "декабря" => 12,
    "декабре" => 12,
    "дек" => 12,
    "дек." => 12,
};

/// Integer words dictionary (includes various cases)
pub static INTEGER_WORD_MAP: phf::Map<&'static str, f64> = phf_map! {
    "один" => 1.0,
    "одна" => 1.0,
    "одной" => 1.0,
    "одну" => 1.0,
    "одного" => 1.0,
    "две" => 2.0,
    "два" => 2.0,
    "двух" => 2.0,
    "три" => 3.0,
    "трех" => 3.0,
    "трёх" => 3.0,
    "четыре" => 4.0,
    "четырех" => 4.0,
    "четырёх" => 4.0,
    "пять" => 5.0,
    "пяти" => 5.0,
    "шесть" => 6.0,
    "шести" => 6.0,
    "семь" => 7.0,
    "семи" => 7.0,
    "восемь" => 8.0,
    "восьми" => 8.0,
    "девять" => 9.0,
    "девяти" => 9.0,
    "десять" => 10.0,
    "десяти" => 10.0,
    "одиннадцать" => 11.0,
    "одиннадцати" => 11.0,
    "двенадцать" => 12.0,
    "двенадцати" => 12.0,
};

/// Ordinal words dictionary
pub static ORDINAL_WORD_MAP: phf::Map<&'static str, u32> = phf_map! {
    "первое" => 1,
    "первого" => 1,
    "второе" => 2,
    "второго" => 2,
    "третье" => 3,
    "третьего" => 3,
    "четвертое" => 4,
    "четвертого" => 4,
    "пятое" => 5,
    "пятого" => 5,
    "шестое" => 6,
    "шестого" => 6,
    "седьмое" => 7,
    "седьмого" => 7,
    "восьмое" => 8,
    "восьмого" => 8,
    "девятое" => 9,
    "девятого" => 9,
    "десятое" => 10,
    "десятого" => 10,
};

/// Time unit dictionary (includes diminutives and cases)
pub static TIME_UNIT_MAP: phf::Map<&'static str, TimeUnit> = phf_map! {
    "сек" => TimeUnit::Second,
    "секунда" => TimeUnit::Second,
    "секунд" => TimeUnit::Second,
    "секунды" => TimeUnit::Second,
    "секунду" => TimeUnit::Second,
    "секундочка" => TimeUnit::Second,
    "секундочки" => TimeUnit::Second,
    "мин" => TimeUnit::Minute,
    "минута" => TimeUnit::Minute,
    "минут" => TimeUnit::Minute,
    "минуты" => TimeUnit::Minute,
    "минуту" => TimeUnit::Minute,
    "минутка" => TimeUnit::Minute,
    "минуток" => TimeUnit::Minute,
    "час" => TimeUnit::Hour,
    "часов" => TimeUnit::Hour,
    "часа" => TimeUnit::Hour,
    "часу" => TimeUnit::Hour,
    "часик" => TimeUnit::Hour,
    "часиков" => TimeUnit::Hour,
    "день" => TimeUnit::Day,
    "дня" => TimeUnit::Day,
    "дней" => TimeUnit::Day,
    "суток" => TimeUnit::Day,
    "сутки" => TimeUnit::Day,
    "неделя" => TimeUnit::Week,
    "неделе" => TimeUnit::Week,
    "недели" => TimeUnit::Week,
    "неделю" => TimeUnit::Week,
    "недель" => TimeUnit::Week,
    "месяц" => TimeUnit::Month,
    "месяце" => TimeUnit::Month,
    "месяцев" => TimeUnit::Month,
    "месяца" => TimeUnit::Month,
    "квартал" => TimeUnit::Quarter,
    "квартале" => TimeUnit::Quarter,
    "кварталов" => TimeUnit::Quarter,
    "год" => TimeUnit::Year,
    "года" => TimeUnit::Year,
    "году" => TimeUnit::Year,
    "годов" => TimeUnit::Year,
    "лет" => TimeUnit::Year,
    "годик" => TimeUnit::Year,
    "годика" => TimeUnit::Year,
};

/// Casual date keywords
pub static CASUAL_DATE_MAP: phf::Map<&'static str, CasualDateType> = phf_map! {
    "сейчас" => CasualDateType::Now,
    "сегодня" => CasualDateType::Today,
    "сегодня ночью" => CasualDateType::Tonight,
    "сегодня вечером" => CasualDateType::Tonight,
    "завтра" => CasualDateType::Tomorrow,
    "вчера" => CasualDateType::Yesterday,
    "послезавтра" => CasualDateType::Overmorrow,
    "позавчера" => CasualDateType::DayBeforeYesterday,
};

/// Casual time keywords
pub static CASUAL_TIME_MAP: phf::Map<&'static str, CasualTimeType> = phf_map! {
    "полдень" => CasualTimeType::Noon,
    "полудень" => CasualTimeType::Noon,
    "полночь" => CasualTimeType::Midnight,
    "утро" => CasualTimeType::Morning,
    "утром" => CasualTimeType::Morning,
    "день" => CasualTimeType::Afternoon,
    "днем" => CasualTimeType::Afternoon,
    "днём" => CasualTimeType::Afternoon,
    "вечер" => CasualTimeType::Evening,
    "вечером" => CasualTimeType::Evening,
    "ночь" => CasualTimeType::Night,
    "ночью" => CasualTimeType::Night,
};

/// Relative modifiers
pub static RELATIVE_MODIFIER_MAP: phf::Map<&'static str, RelativeModifier> = phf_map! {
    "этот" => RelativeModifier::This,
    "эта" => RelativeModifier::This,
    "это" => RelativeModifier::This,
    "эту" => RelativeModifier::This,
    "следующий" => RelativeModifier::Next,
    "следующая" => RelativeModifier::Next,
    "следующее" => RelativeModifier::Next,
    "следующую" => RelativeModifier::Next,
    "будущий" => RelativeModifier::Next,
    "будущая" => RelativeModifier::Next,
    "будущее" => RelativeModifier::Next,
    "прошлый" => RelativeModifier::Last,
    "прошлая" => RelativeModifier::Last,
    "прошлое" => RelativeModifier::Last,
    "прошлую" => RelativeModifier::Last,
    "предыдущий" => RelativeModifier::Last,
    "предыдущая" => RelativeModifier::Last,
    "предыдущее" => RelativeModifier::Last,
    "последний" => RelativeModifier::Last,
    "последняя" => RelativeModifier::Last,
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
pub fn get_integer_word(s: &str) -> Option<f64> {
    INTEGER_WORD_MAP.get(s).copied()
}

#[inline]
pub fn get_ordinal_word(s: &str) -> Option<u32> {
    ORDINAL_WORD_MAP.get(s).copied()
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

/// Parse a number pattern
pub fn parse_number_pattern(text: &str) -> f64 {
    let lower = text.to_lowercase();

    if let Some(val) = get_integer_word(&lower) {
        return val;
    }

    match lower.as_str() {
        s if s.contains("несколько") => return 3.0,
        s if s.contains("пол") => return 0.5,
        s if s.contains("пар") => return 2.0,
        s if s.trim().is_empty() => return 1.0,
        _ => {}
    }

    text.parse::<f64>().unwrap_or(0.0)
}

/// Parse an ordinal number pattern
pub fn parse_ordinal_pattern(text: &str) -> Option<u32> {
    let lower = text.to_lowercase();

    if let Some(val) = get_ordinal_word(&lower) {
        return Some(val);
    }

    let cleaned = lower
        .trim_end_matches("го")
        .trim_end_matches("ого")
        .trim_end_matches("е")
        .trim_end_matches("ое");

    cleaned.parse::<u32>().ok()
}
