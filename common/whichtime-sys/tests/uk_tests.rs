//! Ukrainian locale tests for whichtime-sys
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
    WhichTime::with_locale(Locale::Uk)
}

// =============================================================================
// Tests from uk_casual.test.ts
// =============================================================================

#[test]
fn test_casual_sogodni() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 17, 10, 0);
    test_single_case(&p, "Дедлайн сьогодні", Some(ref_date), |r| {
        assert_eq!(r.text, "сьогодні");
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
fn test_casual_pislyazavtra() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 17, 10, 0);
    test_single_case(
        &p,
        "Дедлайн післязавтра",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "післязавтра");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(8));
            assert_component(r, Component::Day, Some(12));
        },
    );
}

#[test]
fn test_casual_vchora() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 17, 10, 0);
    test_single_case(&p, "Дедлайн вчора", Some(ref_date), |r| {
        assert_eq!(r.text, "вчора");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(9));
    });
}

#[test]
fn test_casual_pozavchora() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 17, 10, 0);
    test_single_case(
        &p,
        "Дедлайн позавчора",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "позавчора");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(8));
            assert_component(r, Component::Day, Some(8));
        },
    );
}

#[test]
fn test_casual_zaraz() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 8, 9, 10);
    test_single_case(&p, "Дедлайн зараз", Some(ref_date), |r| {
        assert_eq!(r.text, "зараз");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(8));
        assert_component(r, Component::Minute, Some(9));
        assert_component(r, Component::Second, Some(10));
    });
}

#[test]
fn test_casual_vrantsi() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 8, 9, 10);
    test_single_case(&p, "Дедлайн вранці", Some(ref_date), |r| {
        assert_eq!(r.text, "вранці");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(6));
    });
}

#[test]
fn test_casual_opivdni() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 8, 9, 10);
    test_single_case(&p, "Дедлайн опівдні", Some(ref_date), |r| {
        assert_eq!(r.text, "опівдні");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(12));
    });
}

#[test]
fn test_casual_vvecheri() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 8, 9, 10);
    test_single_case(&p, "Дедлайн ввечері", Some(ref_date), |r| {
        assert_eq!(r.text, "ввечері");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(20));
    });
}

#[test]
fn test_casual_combined_vchora_vvecheri() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "Дедлайн вчора ввечері",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "вчора ввечері");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(8));
            assert_component(r, Component::Day, Some(9));
            assert_component(r, Component::Hour, Some(20));
        },
    );
}

#[test]
fn test_casual_combined_zavtra_vrantsi() {
    let p = parser();
    let ref_date = create_ref(2012, 9, 10, 14, 0, 0);
    test_single_case(
        &p,
        "Дедлайн завтра вранці",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "завтра вранці");
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
    test_no_result(&p, "несьогодні", None);
    test_no_result(&p, "звтра", None);
    test_no_result(&p, "ввчора", None);
    test_no_result(&p, "січен", None);
}

// =============================================================================
// Tests from uk_month_name_little_endian.test.ts
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
fn test_month_name_little_endian_serpnya() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "10 серпня 2012", Some(ref_date), |r| {
        assert_eq!(r.text, "10 серпня 2012");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_month_name_little_endian_3_lyut_82() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "3 лют 82", Some(ref_date), |r| {
        assert_eq!(r.text, "3 лют 82");
        assert_component(r, Component::Year, Some(1982));
        assert_component(r, Component::Month, Some(2));
        assert_component(r, Component::Day, Some(3));
    });
}

#[test]
fn test_month_name_little_endian_deadline_10_serpnya() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "Дедлайн 10 серпня", Some(ref_date), |r| {
        assert_eq!(r.text, "10 серпня");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_month_name_little_endian_chetverg_10_sichnya() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "Дедлайн Четвер, 10 січня",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "Четвер, 10 січня");
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
    test_single_case(&p, "10 - 22 серпня 2012", Some(ref_date), |r| {
        assert_eq!(r.text, "10 - 22 серпня 2012");
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
// Tests from uk_weekday.test.ts
// =============================================================================

#[test]
fn test_weekday_ponedilok() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 9, 12, 0, 0);
    test_single_case(&p, "понеділок", Some(ref_date), |r| {
        assert_eq!(r.text, "понеділок");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(6));
    });
}

#[test]
fn test_weekday_u_pyatnytsyu() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 9, 12, 0, 0);
    test_single_case(
        &p,
        "Дедлайн у п'ятницю...",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "у п'ятницю");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(8));
            assert_component(r, Component::Day, Some(10));
        },
    );
}

#[test]
fn test_weekday_v_mynulyy_chetverg() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 9, 12, 0, 0);
    test_single_case(
        &p,
        "Дедлайн в минулий четвер!",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "в минулий четвер");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(8));
            assert_component(r, Component::Day, Some(2));
        },
    );
}

#[test]
fn test_weekday_v_nastupnyy_vivtorok() {
    let p = parser();
    let ref_date = create_ref(2015, 4, 18, 12, 0, 0);
    test_single_case(
        &p,
        "Дедлайн в наступний вівторок!",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "в наступний вівторок");
            assert_component(r, Component::Year, Some(2015));
            assert_component(r, Component::Month, Some(4));
            assert_component(r, Component::Day, Some(21));
        },
    );
}

#[test]
fn test_weekday_with_casual_time_v_seredu_vrantsi() {
    let p = parser();
    let ref_date = create_ref(2015, 4, 18, 12, 0, 0);
    test_single_case(
        &p,
        "Подзвони в середу вранці",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "в середу вранці");
            assert_component(r, Component::Year, Some(2015));
            assert_component(r, Component::Month, Some(4));
            assert_component(r, Component::Day, Some(15));
            assert_component(r, Component::Hour, Some(6));
        },
    );
}

#[test]
fn test_weekday_overlap_nedilya_7_grudnya() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 9, 12, 0, 0);
    test_single_case(
        &p,
        "неділя, 7 грудня 2014",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "неділя, 7 грудня 2014");
            assert_component(r, Component::Year, Some(2014));
            assert_component(r, Component::Month, Some(12));
            assert_component(r, Component::Day, Some(7));
        },
    );
}

#[test]
fn test_zavtra_opivdni() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "завтра опівдні", Some(ref_date), |r| {
        assert_eq!(r.text, "завтра опівдні");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(11));
        assert_component(r, Component::Hour, Some(12));
        assert_component(r, Component::Minute, Some(0));
    });
}
