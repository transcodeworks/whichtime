//! Ukrainian (Українська) locale dictionaries

use super::{CasualDateType, CasualTimeType, RelativeModifier, TimeUnit, Weekday};
use phf::phf_map;

/// Weekday dictionary (includes various cases)
pub static WEEKDAY_MAP: phf::Map<&'static str, Weekday> = phf_map! {
    "неділя" => Weekday::Sunday,
    "неділю" => Weekday::Sunday,
    "неділі" => Weekday::Sunday,
    "нд" => Weekday::Sunday,
    "нд." => Weekday::Sunday,
    "понеділок" => Weekday::Monday,
    "понеділка" => Weekday::Monday,
    "пн" => Weekday::Monday,
    "пн." => Weekday::Monday,
    "вівторок" => Weekday::Tuesday,
    "вівторка" => Weekday::Tuesday,
    "вт" => Weekday::Tuesday,
    "вт." => Weekday::Tuesday,
    "середа" => Weekday::Wednesday,
    "середу" => Weekday::Wednesday,
    "середи" => Weekday::Wednesday,
    "ср" => Weekday::Wednesday,
    "ср." => Weekday::Wednesday,
    "четвер" => Weekday::Thursday,
    "четверга" => Weekday::Thursday,
    "чт" => Weekday::Thursday,
    "чт." => Weekday::Thursday,
    "п'ятниця" => Weekday::Friday,
    "п'ятницю" => Weekday::Friday,
    "п'ятниці" => Weekday::Friday,
    "пятниця" => Weekday::Friday,
    "пт" => Weekday::Friday,
    "пт." => Weekday::Friday,
    "субота" => Weekday::Saturday,
    "суботу" => Weekday::Saturday,
    "суботи" => Weekday::Saturday,
    "сб" => Weekday::Saturday,
    "сб." => Weekday::Saturday,
};

/// Month dictionary (includes various cases)
pub static MONTH_MAP: phf::Map<&'static str, u32> = phf_map! {
    "січень" => 1,
    "січня" => 1,
    "січні" => 1,
    "січ" => 1,
    "січ." => 1,
    "лютий" => 2,
    "лютого" => 2,
    "лютому" => 2,
    "лют" => 2,
    "лют." => 2,
    "березень" => 3,
    "березня" => 3,
    "березні" => 3,
    "бер" => 3,
    "бер." => 3,
    "квітень" => 4,
    "квітня" => 4,
    "квітні" => 4,
    "квіт" => 4,
    "квіт." => 4,
    "травень" => 5,
    "травня" => 5,
    "травні" => 5,
    "трав" => 5,
    "трав." => 5,
    "червень" => 6,
    "червня" => 6,
    "червні" => 6,
    "черв" => 6,
    "черв." => 6,
    "липень" => 7,
    "липня" => 7,
    "липні" => 7,
    "лип" => 7,
    "лип." => 7,
    "серпень" => 8,
    "серпня" => 8,
    "серпні" => 8,
    "серп" => 8,
    "серп." => 8,
    "вересень" => 9,
    "вересня" => 9,
    "вересні" => 9,
    "вер" => 9,
    "вер." => 9,
    "жовтень" => 10,
    "жовтня" => 10,
    "жовтні" => 10,
    "жовт" => 10,
    "жовт." => 10,
    "листопад" => 11,
    "листопада" => 11,
    "листопаді" => 11,
    "лист" => 11,
    "лист." => 11,
    "грудень" => 12,
    "грудня" => 12,
    "грудні" => 12,
    "груд" => 12,
    "груд." => 12,
};

/// Integer words dictionary (includes various cases)
pub static INTEGER_WORD_MAP: phf::Map<&'static str, f64> = phf_map! {
    "один" => 1.0,
    "одна" => 1.0,
    "одну" => 1.0,
    "одного" => 1.0,
    "два" => 2.0,
    "дві" => 2.0,
    "двох" => 2.0,
    "три" => 3.0,
    "трьох" => 3.0,
    "чотири" => 4.0,
    "чотирьох" => 4.0,
    "п'ять" => 5.0,
    "пять" => 5.0,
    "п'яти" => 5.0,
    "пяти" => 5.0,
    "шість" => 6.0,
    "шести" => 6.0,
    "сім" => 7.0,
    "семи" => 7.0,
    "вісім" => 8.0,
    "восьми" => 8.0,
    "дев'ять" => 9.0,
    "девять" => 9.0,
    "дев'яти" => 9.0,
    "девяти" => 9.0,
    "десять" => 10.0,
    "десяти" => 10.0,
    "одинадцять" => 11.0,
    "одинадцяти" => 11.0,
    "дванадцять" => 12.0,
    "дванадцяти" => 12.0,
};

/// Time unit dictionary (includes various cases)
pub static TIME_UNIT_MAP: phf::Map<&'static str, TimeUnit> = phf_map! {
    "сек" => TimeUnit::Second,
    "секунда" => TimeUnit::Second,
    "секунд" => TimeUnit::Second,
    "секунди" => TimeUnit::Second,
    "секунду" => TimeUnit::Second,
    "хв" => TimeUnit::Minute,
    "хвилина" => TimeUnit::Minute,
    "хвилин" => TimeUnit::Minute,
    "хвилини" => TimeUnit::Minute,
    "хвилину" => TimeUnit::Minute,
    "год" => TimeUnit::Hour,
    "година" => TimeUnit::Hour,
    "годин" => TimeUnit::Hour,
    "години" => TimeUnit::Hour,
    "годину" => TimeUnit::Hour,
    "день" => TimeUnit::Day,
    "дня" => TimeUnit::Day,
    "днів" => TimeUnit::Day,
    "дні" => TimeUnit::Day,
    "тиждень" => TimeUnit::Week,
    "тижня" => TimeUnit::Week,
    "тижнів" => TimeUnit::Week,
    "тижні" => TimeUnit::Week,
    "місяць" => TimeUnit::Month,
    "місяця" => TimeUnit::Month,
    "місяців" => TimeUnit::Month,
    "місяці" => TimeUnit::Month,
    "квартал" => TimeUnit::Quarter,
    "кварталу" => TimeUnit::Quarter,
    "кварталів" => TimeUnit::Quarter,
    "рік" => TimeUnit::Year,
    "року" => TimeUnit::Year,
    "років" => TimeUnit::Year,
    "роки" => TimeUnit::Year,
};

/// Casual date keywords
pub static CASUAL_DATE_MAP: phf::Map<&'static str, CasualDateType> = phf_map! {
    "зараз" => CasualDateType::Now,
    "сьогодні" => CasualDateType::Today,
    "сьогодни" => CasualDateType::Today,
    "сьогодні ввечері" => CasualDateType::Tonight,
    "завтра" => CasualDateType::Tomorrow,
    "вчора" => CasualDateType::Yesterday,
    "післязавтра" => CasualDateType::Overmorrow,
    "позавчора" => CasualDateType::DayBeforeYesterday,
};

/// Casual time keywords
pub static CASUAL_TIME_MAP: phf::Map<&'static str, CasualTimeType> = phf_map! {
    "полудень" => CasualTimeType::Noon,
    "опівдні" => CasualTimeType::Noon,
    "північ" => CasualTimeType::Midnight,
    "опівночі" => CasualTimeType::Midnight,
    "ранок" => CasualTimeType::Morning,
    "вранці" => CasualTimeType::Morning,
    "зранку" => CasualTimeType::Morning,
    "день" => CasualTimeType::Afternoon,
    "вдень" => CasualTimeType::Afternoon,
    "вечір" => CasualTimeType::Evening,
    "ввечері" => CasualTimeType::Evening,
    "увечері" => CasualTimeType::Evening,
    "ніч" => CasualTimeType::Night,
    "вночі" => CasualTimeType::Night,
};

/// Relative modifiers
pub static RELATIVE_MODIFIER_MAP: phf::Map<&'static str, RelativeModifier> = phf_map! {
    "цей" => RelativeModifier::This,
    "ця" => RelativeModifier::This,
    "це" => RelativeModifier::This,
    "цю" => RelativeModifier::This,
    "наступний" => RelativeModifier::Next,
    "наступна" => RelativeModifier::Next,
    "наступне" => RelativeModifier::Next,
    "наступну" => RelativeModifier::Next,
    "майбутній" => RelativeModifier::Next,
    "майбутня" => RelativeModifier::Next,
    "минулий" => RelativeModifier::Last,
    "минула" => RelativeModifier::Last,
    "минуле" => RelativeModifier::Last,
    "минулу" => RelativeModifier::Last,
    "попередній" => RelativeModifier::Last,
    "попередня" => RelativeModifier::Last,
    "останній" => RelativeModifier::Last,
    "остання" => RelativeModifier::Last,
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
        s if s.contains("кілька") || s.contains("декілька") => return 3.0,
        s if s.contains("пів") => return 0.5,
        s if s.contains("пар") => return 2.0,
        s if s.trim().is_empty() => return 1.0,
        _ => {}
    }

    text.parse::<f64>().unwrap_or(0.0)
}
