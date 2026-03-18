//! Italian locale tests for whichtime-sys
//!
//! Comprehensive tests ported from the original whichtime crate

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
    WhichTime::with_locale(Locale::It)
}

// =============================================================================
// Tests from it_casual.test.ts
// =============================================================================

#[test]
fn test_casual_ora() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 8, 9, 10);
    test_single_case(&p, "La scadenza è ora", Some(ref_date), |r| {
        assert_eq!(r.text, "ora");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(8));
        assert_component(r, Component::Minute, Some(9));
        assert_component(r, Component::Second, Some(10));
    });
}

#[test]
fn test_casual_oggi() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 14, 12, 0);
    test_single_case(&p, "La scadenza è oggi", Some(ref_date), |r| {
        assert_eq!(r.text, "oggi");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_casual_domani() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 14, 12, 0);
    test_single_case(&p, "La scadenza è domani", Some(ref_date), |r| {
        assert_eq!(r.text, "domani");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(11));
    });
}

#[test]
fn test_casual_ieri() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 14, 12, 0);
    test_single_case(&p, "La scadenza era ieri", Some(ref_date), |r| {
        assert_eq!(r.text, "ieri");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(9));
    });
}

#[test]
fn test_casual_dopodomani() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 14, 12, 0);
    test_single_case(&p, "La scadenza è dopodomani", Some(ref_date), |r| {
        assert_eq!(r.text, "dopodomani");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(12));
    });
}

#[test]
fn test_casual_stasera() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 14, 12, 0);
    test_single_case(&p, "Ci vediamo stasera", Some(ref_date), |r| {
        assert_eq!(r.text, "stasera");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(22));
    });
}

#[test]
fn test_casual_stamattina() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 14, 12, 0);
    test_single_case(&p, "Ci vediamo stamattina", Some(ref_date), |r| {
        assert_eq!(r.text, "stamattina");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(6));
    });
}

// =============================================================================
// Tests from it_month_name_little_endian.test.ts
// =============================================================================

#[test]
fn test_month_name_little_endian_basic() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "10 Agosto 2012", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_eq!(r.text, "10 Agosto 2012");
    });
}

#[test]
fn test_month_name_little_endian_il() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "La scadenza è il 10 Agosto", Some(ref_date), |r| {
        assert_eq!(r.text, "10 Agosto");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_month_name_little_endian_range() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "10 - 22 Agosto 2012", Some(ref_date), |r| {
        assert_eq!(r.text, "10 - 22 Agosto 2012");
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

#[test]
fn test_month_name_little_endian_combined_time() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "12 Luglio alle 19:00", Some(ref_date), |r| {
        assert_eq!(r.text, "12 Luglio alle 19:00");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(7));
        assert_component(r, Component::Day, Some(12));
        assert_component(r, Component::Hour, Some(19));
        assert_component(r, Component::Minute, Some(0));
    });
}

// =============================================================================
// Tests from it_month_name_middle_endian.test.ts
// =============================================================================

#[test]
fn test_month_name_middle_endian_basic() {
    let p = parser();
    test_single_case(&p, "Ci vediamo ad Agosto 2017.", None, |r| {
        assert_component(r, Component::Year, Some(2017));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(1));
        assert_eq!(r.text, "Agosto 2017");
    });
}

#[test]
fn test_month_name_middle_endian_comma() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "Agosto 10, 2012", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_eq!(r.text, "Agosto 10, 2012");
    });
}

#[test]
fn test_month_name_middle_endian_scadenza() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "La scadenza è Agosto 10", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_eq!(r.text, "Agosto 10");
    });
}

// =============================================================================
// Tests from it_weekday.test.ts
// =============================================================================

#[test]
fn test_weekday_lunedi() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 9, 12, 0, 0);
    test_single_case(&p, "Lunedì", Some(ref_date), |r| {
        assert_eq!(r.text, "Lunedì");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(6));
        assert_eq!(
            r.start.get(Component::Weekday),
            Some(Weekday::Monday as i32)
        );
    });
}

#[test]
fn test_weekday_giovedi() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 9, 12, 0, 0);
    test_single_case(&p, "Giovedì", Some(ref_date), |r| {
        assert_eq!(r.text, "Giovedì");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(9));
        assert_eq!(
            r.start.get(Component::Weekday),
            Some(Weekday::Thursday as i32)
        );
    });
}

#[test]
fn test_weekday_domenica() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 9, 12, 0, 0);
    test_single_case(&p, "Domenica", Some(ref_date), |r| {
        assert_eq!(r.text, "Domenica");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(12));
        assert_eq!(
            r.start.get(Component::Weekday),
            Some(Weekday::Sunday as i32)
        );
    });
}

#[test]
fn test_weekday_venerdi_scorso() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 9, 12, 0, 0);
    test_single_case(
        &p,
        "La scadenza era venerdì scorso...",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "venerdì scorso");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(8));
            assert_component(r, Component::Day, Some(3));
            assert_eq!(
                r.start.get(Component::Weekday),
                Some(Weekday::Friday as i32)
            );
        },
    );
}

#[test]
fn test_weekday_venerdi_prossimo() {
    let p = parser();
    let ref_date = create_ref(2015, 4, 18, 12, 0, 0);
    test_single_case(
        &p,
        "Fissiamo una riunione venerdì prossimo",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "venerdì prossimo");
            assert_component(r, Component::Year, Some(2015));
            assert_component(r, Component::Month, Some(4));
            assert_component(r, Component::Day, Some(24));
            assert_eq!(
                r.start.get(Component::Weekday),
                Some(Weekday::Friday as i32)
            );
        },
    );
}

// =============================================================================
// Tests from it_year_month_day.test.ts
// =============================================================================

#[test]
fn test_year_month_day_dots() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "2012.08.10", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_eq!(r.text, "2012.08.10");
    });
}

#[test]
fn test_year_month_day_la_scadenza() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "La scadenza è 2012.08.10", Some(ref_date), |r| {
        assert_eq!(r.text, "2012.08.10");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_year_month_day_impossible_dates() {
    let p = parser();
    test_no_result(&p, "2014.08.32", None);
    test_no_result(&p, "2014.02.30", None);
    test_no_result(&p, "2012.80.10", None);
}

// =============================================================================
// Additional Italian tests
// =============================================================================

#[test]
fn test_time_expression_alle() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "alle 15:30", Some(ref_date), |r| {
        assert_component(r, Component::Hour, Some(15));
        assert_component(r, Component::Minute, Some(30));
    });
}

#[test]
fn test_time_expression_range() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "dalle 8:00 alle 17:00", Some(ref_date), |r| {
        assert_component(r, Component::Hour, Some(8));
        assert_component(r, Component::Minute, Some(0));
        if let Some(end) = &r.end {
            assert_eq!(end.get(Component::Hour), Some(17));
            assert_eq!(end.get(Component::Minute), Some(0));
        }
    });
}

#[test]
fn test_time_units_fra_5_minuti() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 14, 0);
    test_single_case(&p, "fra 5 minuti", Some(ref_date), |r| {
        assert_eq!(r.text, "fra 5 minuti");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(12));
        assert_component(r, Component::Minute, Some(19));
    });
}

#[test]
fn test_time_units_tra_2_ore() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 14, 0);
    test_single_case(&p, "tra 2 ore", Some(ref_date), |r| {
        assert_eq!(r.text, "tra 2 ore");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(14));
        assert_component(r, Component::Minute, Some(14));
    });
}

#[test]
fn test_time_units_in_una_settimana() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 14, 0);
    test_single_case(&p, "in una settimana", Some(ref_date), |r| {
        assert_eq!(r.text, "in una settimana");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(17));
    });
}

#[test]
fn test_time_units_ago_5_giorni_fa() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "5 giorni fa", Some(ref_date), |r| {
        assert_eq!(r.text, "5 giorni fa");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(5));
    });
}

#[test]
fn test_time_units_ago_2_settimane_fa() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "2 settimane fa", Some(ref_date), |r| {
        assert_eq!(r.text, "2 settimane fa");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(7));
        assert_component(r, Component::Day, Some(27));
    });
}

#[test]
fn test_domani_alle_15() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "domani alle 15:00", Some(ref_date), |r| {
        assert_eq!(r.text, "domani alle 15:00");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(11));
        assert_component(r, Component::Hour, Some(15));
        assert_component(r, Component::Minute, Some(0));
    });
}

#[test]
fn test_casual_negative_cases() {
    let p = parser();
    test_no_result(&p, "nooggi", None);
    test_no_result(&p, "oggino", None);
    test_no_result(&p, "xdomani", None);
}
