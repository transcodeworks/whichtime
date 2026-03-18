//! Chinese locale tests for whichtime-sys
//!
//! Comprehensive tests ported from the original whichtime crate (combines hans and hant variants)

use chrono::{Local, TimeZone};
use whichtime_sys::{Component, Locale, ParsedResult, Weekday, WhichTime};

// =============================================================================
// Test utilities
// =============================================================================

fn create_ref(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
) -> chrono::DateTime<Local> {
    Local
        .with_ymd_and_hms(year, month, day, hour, minute, second)
        .single()
        .expect("Invalid date")
}

fn assert_component(result: &ParsedResult, component: Component, expected: Option<i32>) {
    let actual = result.start.get(component);
    assert_eq!(
        actual, expected,
        "Component {:?} mismatch in '{}': expected {:?}, got {:?}",
        component, result.text, expected, actual
    );
}

fn test_single_case<F>(
    parser: &WhichTime,
    text: &str,
    reference: Option<chrono::DateTime<Local>>,
    check: F,
) where
    F: FnOnce(&ParsedResult),
{
    let results = parser
        .parse(text, reference)
        .expect("Parsing should succeed");
    assert!(
        !results.is_empty(),
        "Expected at least one result from '{}', got none",
        text
    );
    check(&results[0]);
}

fn parser() -> WhichTime {
    WhichTime::with_locale(Locale::Zh)
}

// =============================================================================
// Tests from zh_hans_casual.test.ts (Simplified Chinese)
// =============================================================================

#[test]
fn test_hans_casual_jintian() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "我今天要打游戏", Some(ref_date), |r| {
        assert_eq!(r.text, "今天");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_hans_casual_mingri() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "我明日要打游戏", Some(ref_date), |r| {
        assert_eq!(r.text, "明日");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(11));
    });
}

#[test]
fn test_hans_casual_mingtian() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "我明天要打游戏", Some(ref_date), |r| {
        assert_eq!(r.text, "明天");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(11));
    });
}

#[test]
fn test_hans_casual_qiantian() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "我前天要打游戏", Some(ref_date), |r| {
        assert_eq!(r.text, "前天");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(8));
    });
}

#[test]
fn test_hans_casual_zuori() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "我昨日要打游戏", Some(ref_date), |r| {
        assert_eq!(r.text, "昨日");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(9));
    });
}

#[test]
fn test_hans_casual_zuotian_wanshang() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "我昨天晚上要打游戏", Some(ref_date), |r| {
        assert_eq!(r.text, "昨天晚上");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(9));
        assert_component(r, Component::Hour, Some(22));
    });
}

#[test]
fn test_hans_casual_jintian_zaoshang() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "我今天早上要打游戏", Some(ref_date), |r| {
        assert_eq!(r.text, "今天早上");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(6));
    });
}

#[test]
fn test_hans_casual_xiawu() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "我下午要打游戏", Some(ref_date), |r| {
        assert_eq!(r.text, "下午");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(15));
    });
}

#[test]
fn test_hans_casual_jinwan() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "我今晚要打游戏", Some(ref_date), |r| {
        assert_eq!(r.text, "今晚");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(22));
    });
}

#[test]
fn test_hans_casual_combined_jintian_xiawu_5dian() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "我今天下午5点要打游戏", Some(ref_date), |r| {
        assert_eq!(r.text, "今天下午5点");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(17));
    });
}

// =============================================================================
// Tests from zh_hans_date.test.ts (Simplified Chinese)
// =============================================================================

#[test]
fn test_hans_date_2016_9_3() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "我2016年9月3号要打游戏", Some(ref_date), |r| {
        assert_eq!(r.text, "2016年9月3号");
        assert_component(r, Component::Year, Some(2016));
        assert_component(r, Component::Month, Some(9));
        assert_component(r, Component::Day, Some(3));
    });
}

#[test]
fn test_hans_date_chinese_numerals() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "我二零一六年，九月三号要打游戏",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "二零一六年，九月三号");
            assert_component(r, Component::Year, Some(2016));
            assert_component(r, Component::Month, Some(9));
            assert_component(r, Component::Day, Some(3));
        },
    );
}

// =============================================================================
// Tests from zh_hant_casual.test.ts (Traditional Chinese / Cantonese)
// =============================================================================

#[test]
fn test_hant_casual_yiga_now() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 8, 9, 10);
    test_single_case(&p, "雞而家全部都係雞", Some(ref_date), |r| {
        assert_eq!(r.text, "而家");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(8));
        assert_component(r, Component::Minute, Some(9));
        assert_component(r, Component::Second, Some(10));
    });
}

#[test]
fn test_hant_casual_gamyat_today() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 8, 9, 10);
    test_single_case(&p, "雞今日全部都係雞", Some(ref_date), |r| {
        assert_eq!(r.text, "今日");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_hant_casual_tingyat_tomorrow() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 8, 9, 10);
    test_single_case(&p, "雞聽日全部都係雞", Some(ref_date), |r| {
        assert_eq!(r.text, "聽日");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(11));
    });
}

// =============================================================================
// Tests from zh_hant_date.test.ts (Traditional Chinese)
// =============================================================================

#[test]
fn test_hant_date_2016_9_3() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "雞2016年9月3號全部都係雞",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "2016年9月3號");
            assert_component(r, Component::Year, Some(2016));
            assert_component(r, Component::Month, Some(9));
            assert_component(r, Component::Day, Some(3));
        },
    );
}

// =============================================================================
// Tests from zh_hans_time_exp.test.ts (Simplified Chinese)
// =============================================================================

#[test]
fn test_hans_time_shangwu_6_13() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "我上午6点13分打游戏", Some(ref_date), |r| {
        assert_eq!(r.text, "上午6点13分");
        assert_component(r, Component::Hour, Some(6));
        assert_component(r, Component::Minute, Some(13));
    });
}

// =============================================================================
// Tests from zh_hans_deadline.test.ts (Simplified Chinese)
// =============================================================================

#[test]
fn test_hans_deadline_wuri_nei() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "五日内我要通关游戏", Some(ref_date), |r| {
        assert_eq!(r.text, "五日内");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(15));
    });
}

// =============================================================================
// Tests from zh_hans_weekday.test.ts (Simplified Chinese)
// =============================================================================

#[test]
fn test_hans_weekday_xingqisi() {
    let p = parser();
    let ref_date = create_ref(2016, 9, 2, 12, 0, 0);
    test_single_case(&p, "星期四", Some(ref_date), |r| {
        assert_eq!(r.text, "星期四");
        assert_component(r, Component::Year, Some(2016));
        assert_component(r, Component::Month, Some(9));
        assert_component(r, Component::Day, Some(1));
        assert_eq!(
            r.start.get(Component::Weekday),
            Some(Weekday::Thursday as i32)
        );
    });
}

// =============================================================================
// Tests from zh.test.ts (Combined zh configuration)
// =============================================================================

#[test]
fn test_zh_combined_mingtian_zaoshang_8dian_simplified() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 8, 12, 0, 0);
    test_single_case(&p, "明天早上8点", Some(ref_date), |r| {
        assert_eq!(r.text, "明天早上8点");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(9));
        assert_component(r, Component::Hour, Some(8));
    });
}

#[test]
fn test_zh_combined_mingtian_zaoshang_8dian_traditional() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 8, 12, 0, 0);
    test_single_case(&p, "明天早上8點", Some(ref_date), |r| {
        assert_eq!(r.text, "明天早上8點");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(9));
        assert_component(r, Component::Hour, Some(8));
    });
}

#[test]
fn test_mingtian_zhongwu() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "明天中午", Some(ref_date), |r| {
        assert_eq!(r.text, "明天中午");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(11));
        assert_component(r, Component::Hour, Some(12));
        assert_component(r, Component::Minute, Some(0));
    });
}
