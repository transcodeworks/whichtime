//! Russian locale tests for whichtime-sys
//!
//! Comprehensive tests ported from the original whichtime crate

use chrono::{Local, TimeZone};
use whichtime_sys::{Component, Locale, ParsedResult, WhichTime};

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

fn test_no_result(parser: &WhichTime, text: &str, reference: Option<chrono::DateTime<Local>>) {
    let results = parser
        .parse(text, reference)
        .expect("Parsing should succeed");
    assert!(
        results.is_empty(),
        "Expected no results from '{}', but got {} results",
        text,
        results.len()
    );
}

fn parser() -> WhichTime {
    WhichTime::with_locale(Locale::Ru)
}

// =============================================================================
// Tests from ru_casual.test.ts
// =============================================================================

#[test]
fn test_casual_segodnya() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 17, 10, 0);
    test_single_case(&p, "Дедлайн сегодня", Some(ref_date), |r| {
        assert_eq!(r.text, "сегодня");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_casual_zavtra() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 17, 10, 0);
    test_single_case(&p, "Дедлайн завтра", Some(ref_date), |r| {
        assert_eq!(r.text, "завтра");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(11));
    });
}

#[test]
fn test_casual_poslezavtra() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 17, 10, 0);
    test_single_case(
        &p,
        "Дедлайн послезавтра",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "послезавтра");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(8));
            assert_component(r, Component::Day, Some(12));
        },
    );
}

#[test]
fn test_casual_vchera() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 17, 10, 0);
    test_single_case(&p, "Дедлайн вчера", Some(ref_date), |r| {
        assert_eq!(r.text, "вчера");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(9));
    });
}

#[test]
fn test_casual_pozavchera() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 17, 10, 0);
    test_single_case(
        &p,
        "Дедлайн позавчера",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "позавчера");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(8));
            assert_component(r, Component::Day, Some(8));
        },
    );
}

#[test]
fn test_casual_seychas() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 8, 9, 10);
    test_single_case(&p, "Дедлайн сейчас", Some(ref_date), |r| {
        assert_eq!(r.text, "сейчас");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(8));
        assert_component(r, Component::Minute, Some(9));
        assert_component(r, Component::Second, Some(10));
    });
}

#[test]
fn test_casual_utrom() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 8, 9, 10);
    test_single_case(&p, "Дедлайн утром", Some(ref_date), |r| {
        assert_eq!(r.text, "утром");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(6));
    });
}

#[test]
fn test_casual_v_polden() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 8, 9, 10);
    test_single_case(
        &p,
        "Дедлайн в полдень",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "в полдень");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(8));
            assert_component(r, Component::Day, Some(10));
            assert_component(r, Component::Hour, Some(12));
        },
    );
}

#[test]
fn test_casual_vecherom() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 8, 9, 10);
    test_single_case(&p, "Дедлайн вечером", Some(ref_date), |r| {
        assert_eq!(r.text, "вечером");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(20));
    });
}

#[test]
fn test_casual_combined_vchera_vecherom() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "Дедлайн вчера вечером",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "вчера вечером");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(8));
            assert_component(r, Component::Day, Some(9));
            assert_component(r, Component::Hour, Some(20));
        },
    );
}

#[test]
fn test_casual_combined_zavtra_utrom() {
    let p = parser();
    let ref_date = create_ref(2012, 9, 10, 14, 0, 0);
    test_single_case(
        &p,
        "Дедлайн завтра утром",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "завтра утром");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(9));
            assert_component(r, Component::Day, Some(11));
            assert_component(r, Component::Hour, Some(6));
        },
    );
}

#[test]
fn test_casual_negative_cases() {
    let p = parser();
    test_no_result(&p, "несегодня", None);
    test_no_result(&p, "зявтра", None);
    test_no_result(&p, "вчеера", None);
    test_no_result(&p, "январ", None);
}

// =============================================================================
// Tests from ru_month_name_little_endian.test.ts
// =============================================================================

#[test]
fn test_month_name_little_endian_dot() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "10.08.2012", Some(ref_date), |r| {
        assert_eq!(r.text, "10.08.2012");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_month_name_little_endian_avgusta() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "10 августа 2012", Some(ref_date), |r| {
        assert_eq!(r.text, "10 августа 2012");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_month_name_little_endian_tretye_fev() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "третье фев 82", Some(ref_date), |r| {
        assert_eq!(r.text, "третье фев 82");
        assert_component(r, Component::Year, Some(1982));
        assert_component(r, Component::Month, Some(2));
        assert_component(r, Component::Day, Some(3));
    });
}

#[test]
fn test_month_name_little_endian_deadline_10_avgusta() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "Дедлайн 10 августа",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "10 августа");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(8));
            assert_component(r, Component::Day, Some(10));
        },
    );
}

#[test]
fn test_month_name_little_endian_chetverg_10_yanvarya() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "Дедлайн Четверг, 10 января",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "Четверг, 10 января");
            assert_component(r, Component::Year, Some(2013));
            assert_component(r, Component::Month, Some(1));
            assert_component(r, Component::Day, Some(10));
        },
    );
}

#[test]
fn test_month_name_little_endian_range() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "10 - 22 августа 2012", Some(ref_date), |r| {
        assert_eq!(r.text, "10 - 22 августа 2012");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        if let Some(end) = &r.end {
            assert_eq!(end.get(Component::Year), Some(2012));
            assert_eq!(end.get(Component::Month), Some(8));
            assert_eq!(end.get(Component::Day), Some(22));
        }
    });
}

// =============================================================================
// Tests from ru_month.test.ts
// =============================================================================

#[test]
fn test_month_year_sentyabr() {
    let p = parser();
    test_single_case(&p, "Сентябрь 2012", None, |r| {
        assert_eq!(r.text, "Сентябрь 2012");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(9));
        assert_component(r, Component::Day, Some(1));
    });
}

// =============================================================================
// Tests from ru_relative.test.ts
// =============================================================================

#[test]
fn test_relative_na_etoy_nedele() {
    let p = parser();
    let ref_date = create_ref(2017, 11, 19, 12, 0, 0);
    test_single_case(&p, "на этой неделе", Some(ref_date), |r| {
        assert_eq!(r.text, "на этой неделе");
        assert_component(r, Component::Year, Some(2017));
        assert_component(r, Component::Month, Some(11));
        assert_component(r, Component::Day, Some(19));
        assert_component(r, Component::Hour, Some(12));
    });
}

// =============================================================================
// Tests from ru_time_exp.test.ts
// =============================================================================

#[test]
fn test_time_expression_20_32_13() {
    let p = parser();
    let ref_date = create_ref(2016, 10, 1, 8, 0, 0);
    test_single_case(&p, "20:32:13", Some(ref_date), |r| {
        assert_eq!(r.text, "20:32:13");
        assert_component(r, Component::Year, Some(2016));
        assert_component(r, Component::Month, Some(10));
        assert_component(r, Component::Day, Some(1));
        assert_component(r, Component::Hour, Some(20));
        assert_component(r, Component::Minute, Some(32));
        assert_component(r, Component::Second, Some(13));
    });
}

// =============================================================================
// Tests from ru_time_units_ago.test.ts
// =============================================================================

#[test]
fn test_time_units_ago_5_dney_nazad() {
    let p = parser();
    let ref_date = create_ref(2012, 7, 10, 0, 0, 0);
    test_single_case(
        &p,
        "5 дней назад что-то было",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "5 дней назад");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(7));
            assert_component(r, Component::Day, Some(5));
        },
    );
}

// =============================================================================
// Tests from ru_time_units_casual_relative.test.ts
// =============================================================================

#[test]
fn test_time_units_casual_relative_sleduyuschie_2_nedeli() {
    let p = parser();
    let ref_date = create_ref(2016, 10, 1, 12, 0, 0);
    test_single_case(
        &p,
        "следующие 2 недели",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "следующие 2 недели");
            assert_component(r, Component::Year, Some(2016));
            assert_component(r, Component::Month, Some(10));
            assert_component(r, Component::Day, Some(15));
            assert_component(r, Component::Hour, Some(12));
        },
    );
}

// =============================================================================
// Tests from ru_time_units_within.test.ts
// =============================================================================

#[test]
fn test_time_units_within_v_techenie_minuty() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 0, 0, 0);
    test_single_case(
        &p,
        "будет сделано в течение минуты",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "в течение минуты");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(8));
            assert_component(r, Component::Day, Some(10));
            assert_component(r, Component::Hour, Some(0));
            assert_component(r, Component::Minute, Some(1));
        },
    );
}

// =============================================================================
// Tests from ru_weekday.test.ts
// =============================================================================

#[test]
fn test_weekday_ponedelnik() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 9, 12, 0, 0);
    test_single_case(&p, "понедельник", Some(ref_date), |r| {
        assert_eq!(r.text, "понедельник");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(6));
    });
}

#[test]
fn test_weekday_v_pyatnitsu() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 9, 12, 0, 0);
    test_single_case(
        &p,
        "Дедлайн в пятницу...",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "в пятницу");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(8));
            assert_component(r, Component::Day, Some(10));
        },
    );
}

#[test]
fn test_zavtra_v_polden() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "завтра в полдень", Some(ref_date), |r| {
        assert_eq!(r.text, "завтра в полдень");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(11));
        assert_component(r, Component::Hour, Some(12));
        assert_component(r, Component::Minute, Some(0));
    });
}
