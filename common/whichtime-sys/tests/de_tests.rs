//! German locale tests for whichtime-sys
//!
//! Comprehensive tests ported from the original whichtime crate

use chrono::{Local, TimeZone};
use whichtime_sys::{Component, Locale, Meridiem, ParsedResult, Weekday, WhichTime};

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
    WhichTime::with_locale(Locale::De)
}

// =============================================================================
// Tests from de_casual.test.ts
// =============================================================================

#[test]
fn test_casual_jetzt() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 8, 9, 10);
    test_single_case(&p, "Die Deadline ist jetzt", Some(ref_date), |r| {
        assert_eq!(r.text, "jetzt");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(8));
        assert_component(r, Component::Minute, Some(9));
        assert_component(r, Component::Second, Some(10));
    });
}

#[test]
fn test_casual_heute() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "Die Deadline ist heute", Some(ref_date), |r| {
        assert_eq!(r.text, "heute");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_casual_morgen() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "Die Deadline ist morgen", Some(ref_date), |r| {
        assert_eq!(r.text, "morgen");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(11));
    });
}

#[test]
fn test_casual_morgen_early() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 1, 0, 0);
    test_single_case(&p, "Die Deadline ist morgen", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(11));
    });
}

#[test]
fn test_casual_gestern() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "Die Deadline war gestern", Some(ref_date), |r| {
        assert_eq!(r.text, "gestern");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(9));
    });
}

#[test]
fn test_casual_letzte_nacht() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "Die Deadline war letzte Nacht", Some(ref_date), |r| {
        assert_eq!(r.text, "letzte Nacht");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(9));
        assert_component(r, Component::Hour, Some(0));
    });
}

#[test]
fn test_casual_gestern_nacht() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "Die Deadline war gestern Nacht", Some(ref_date), |r| {
        assert_eq!(r.text, "gestern Nacht");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(9));
        assert_component(r, Component::Hour, Some(22));
    });
}

#[test]
fn test_casual_heute_morgen() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "Die Deadline war heute Morgen", Some(ref_date), |r| {
        assert_eq!(r.text, "heute Morgen");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(6));
    });
}

#[test]
fn test_casual_heute_nachmittag() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "Die Deadline war heute Nachmittag",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "heute Nachmittag");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(8));
            assert_component(r, Component::Day, Some(10));
            assert_component(r, Component::Hour, Some(15));
        },
    );
}

#[test]
fn test_casual_heute_abend() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "Die Deadline war heute Abend", Some(ref_date), |r| {
        assert_eq!(r.text, "heute Abend");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(18));
    });
}

#[test]
fn test_casual_heute_nacht() {
    let p = parser();
    let ref_date = create_ref(2012, 1, 1, 12, 0, 0);
    test_single_case(&p, "heute Nacht", Some(ref_date), |r| {
        assert_eq!(r.text, "heute Nacht");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(1));
        assert_component(r, Component::Day, Some(1));
        assert_component(r, Component::Hour, Some(22));
    });
}

#[test]
fn test_casual_heute_nacht_mit_uhrzeit() {
    let p = parser();
    let ref_date = create_ref(2012, 1, 1, 12, 0, 0);
    test_single_case(&p, "heute Nacht um 20 Uhr", Some(ref_date), |r| {
        assert_eq!(r.text, "heute Nacht um 20 Uhr");
        assert_component(r, Component::Hour, Some(20));
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(1));
        assert_component(r, Component::Day, Some(1));
    });
}

#[test]
fn test_casual_heute_abend_um_8() {
    let p = parser();
    let ref_date = create_ref(2012, 1, 1, 12, 0, 0);
    test_single_case(&p, "heute Abend um 8", Some(ref_date), |r| {
        assert_eq!(r.text, "heute Abend um 8");
        assert_component(r, Component::Hour, Some(20));
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(1));
        assert_component(r, Component::Day, Some(1));
    });
}

#[test]
fn test_casual_8_uhr_abends() {
    let p = parser();
    let ref_date = create_ref(2012, 1, 1, 12, 0, 0);
    test_single_case(&p, "8 Uhr abends", Some(ref_date), |r| {
        assert_eq!(r.text, "8 Uhr abends");
        assert_component(r, Component::Hour, Some(20));
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(1));
        assert_component(r, Component::Day, Some(1));
    });
}

#[test]
fn test_casual_donnerstag_abbrev() {
    let p = parser();
    test_single_case(&p, "Do", None, |r| {
        assert_eq!(r.text, "Do");
        assert_eq!(
            r.start.get(Component::Weekday),
            Some(Weekday::Thursday as i32)
        );
    });
}

#[test]
fn test_casual_donnerstag_full() {
    let p = parser();
    test_single_case(&p, "Donnerstag", None, |r| {
        assert_eq!(r.text, "Donnerstag");
        assert_eq!(
            r.start.get(Component::Weekday),
            Some(Weekday::Thursday as i32)
        );
    });
}

#[test]
fn test_casual_gestern_nachmittag() {
    let p = parser();
    let ref_date = create_ref(2016, 10, 1, 0, 0, 0);
    test_single_case(&p, "gestern Nachmittag", Some(ref_date), |r| {
        assert_eq!(r.text, "gestern Nachmittag");
        assert_component(r, Component::Year, Some(2016));
        assert_component(r, Component::Month, Some(9));
        assert_component(r, Component::Day, Some(30));
        assert_component(r, Component::Hour, Some(15));
    });
}

#[test]
fn test_casual_morgen_morgen() {
    let p = parser();
    let ref_date = create_ref(2016, 10, 1, 8, 0, 0);
    test_single_case(&p, "morgen Morgen", Some(ref_date), |r| {
        assert_eq!(r.text, "morgen Morgen");
        assert_component(r, Component::Year, Some(2016));
        assert_component(r, Component::Month, Some(10));
        assert_component(r, Component::Day, Some(2));
        assert_component(r, Component::Hour, Some(6));
    });
}

#[test]
fn test_casual_uebermorgen() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "übermorgen", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(12));
    });
}

#[test]
fn test_casual_uebermorgen_abend() {
    let p = parser();
    let ref_date = create_ref(2016, 10, 1, 8, 0, 0);
    test_single_case(&p, "uebermorgen Abend", Some(ref_date), |r| {
        assert_eq!(r.text, "uebermorgen Abend");
        assert_component(r, Component::Year, Some(2016));
        assert_component(r, Component::Month, Some(10));
        assert_component(r, Component::Day, Some(3));
        assert_component(r, Component::Hour, Some(18));
    });
}

#[test]
fn test_casual_vorgestern() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "vorgestern", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(8));
    });
}

#[test]
fn test_casual_vorgestern_vormittag() {
    let p = parser();
    let ref_date = create_ref(2016, 10, 1, 0, 0, 0);
    test_single_case(&p, "vorgestern Vormittag", Some(ref_date), |r| {
        assert_eq!(r.text, "vorgestern Vormittag");
        assert_component(r, Component::Year, Some(2016));
        assert_component(r, Component::Month, Some(9));
        assert_component(r, Component::Day, Some(29));
        assert_component(r, Component::Hour, Some(9));
    });
}

#[test]
fn test_casual_negative_cases() {
    let p = parser();
    test_no_result(&p, "nicheute", None);
    test_no_result(&p, "heutenicht", None);
    test_no_result(&p, "angestern", None);
    test_no_result(&p, "jetztig", None);
    test_no_result(&p, "ljetztlich", None);
}

// =============================================================================
// Tests from de_time_exp.test.ts
// =============================================================================

#[test]
fn test_time_expression_simple_colon() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "18:10", Some(ref_date), |r| {
        assert_eq!(r.text, "18:10");
        assert_component(r, Component::Hour, Some(18));
        assert_component(r, Component::Minute, Some(10));
    });
}

#[test]
fn test_time_expression_um_uhr() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "um 14 Uhr", Some(ref_date), |r| {
        assert_component(r, Component::Hour, Some(14));
        assert_component(r, Component::Minute, Some(0));
    });
}

#[test]
fn test_time_expression_um_h() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "um 16h", Some(ref_date), |r| {
        assert_component(r, Component::Hour, Some(16));
        assert_component(r, Component::Minute, Some(0));
    });
}

#[test]
fn test_time_expression_um_morgens() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "um 7 morgens", Some(ref_date), |r| {
        assert_component(r, Component::Hour, Some(7));
        assert_component(r, Component::Minute, Some(0));
        assert_eq!(r.start.get(Component::Meridiem), Some(Meridiem::AM as i32));
    });
}

#[test]
fn test_time_expression_vormittags() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "11:00 Uhr vormittags", Some(ref_date), |r| {
        assert_eq!(r.text, "11:00 Uhr vormittags");
        assert_component(r, Component::Hour, Some(11));
        assert_component(r, Component::Minute, Some(0));
        assert_eq!(r.start.get(Component::Meridiem), Some(Meridiem::AM as i32));
    });
}

#[test]
fn test_time_expression_nachmittags() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "um 8 Uhr nachmittags", Some(ref_date), |r| {
        assert_eq!(r.text, "um 8 Uhr nachmittags");
        assert_component(r, Component::Hour, Some(20));
        assert_component(r, Component::Minute, Some(0));
        assert_eq!(r.start.get(Component::Meridiem), Some(Meridiem::PM as i32));
    });
}

#[test]
fn test_time_expression_in_der_nacht_pm() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "um 8 Uhr in der Nacht", Some(ref_date), |r| {
        assert_eq!(r.text, "um 8 Uhr in der Nacht");
        assert_component(r, Component::Hour, Some(20));
        assert_component(r, Component::Minute, Some(0));
        assert_eq!(r.start.get(Component::Meridiem), Some(Meridiem::PM as i32));
    });
}

#[test]
fn test_time_expression_in_der_nacht_am() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "um 5 Uhr in der Nacht", Some(ref_date), |r| {
        assert_eq!(r.text, "um 5 Uhr in der Nacht");
        assert_component(r, Component::Hour, Some(5));
        assert_component(r, Component::Minute, Some(0));
        assert_eq!(r.start.get(Component::Meridiem), Some(Meridiem::AM as i32));
    });
}

#[test]
fn test_time_expression_mittags() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 8, 9, 10);
    test_single_case(&p, "Die Deadline ist mittags", Some(ref_date), |r| {
        assert_eq!(r.text, "mittags");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(12));
        assert_component(r, Component::Minute, Some(0));
        assert_component(r, Component::Second, Some(0));
    });
}

#[test]
fn test_time_expression_mitternacht_pm() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "um Mitternacht", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(11));
        assert_component(r, Component::Hour, Some(0));
        assert_component(r, Component::Minute, Some(0));
        assert_component(r, Component::Second, Some(0));
    });
}

#[test]
fn test_time_expression_mitternacht_am() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 1, 0, 0);
    test_single_case(&p, "um Mitternacht", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(0));
        assert_component(r, Component::Minute, Some(0));
        assert_component(r, Component::Second, Some(0));
    });
}

#[test]
fn test_time_expression_range() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "18:10 - 22.32", Some(ref_date), |r| {
        assert_eq!(r.text, "18:10 - 22.32");
        assert_component(r, Component::Hour, Some(18));
        assert_component(r, Component::Minute, Some(10));
        if let Some(end) = &r.end {
            assert_eq!(end.get(Component::Hour), Some(22));
            assert_eq!(end.get(Component::Minute), Some(32));
        }
    });
}

#[test]
fn test_time_expression_von_bis() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "von 6:30 bis 23:00", Some(ref_date), |r| {
        assert_eq!(r.text, "von 6:30 bis 23:00");
        assert_component(r, Component::Hour, Some(6));
        assert_component(r, Component::Minute, Some(30));
        if let Some(end) = &r.end {
            assert_eq!(end.get(Component::Hour), Some(23));
            assert_eq!(end.get(Component::Minute), Some(0));
        }
    });
}

#[test]
fn test_time_expression_von_bis_h() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "von 6h30 bis 23h00", Some(ref_date), |r| {
        assert_eq!(r.text, "von 6h30 bis 23h00");
        assert_component(r, Component::Hour, Some(6));
        assert_component(r, Component::Minute, Some(30));
        if let Some(end) = &r.end {
            assert_eq!(end.get(Component::Hour), Some(23));
            assert_eq!(end.get(Component::Minute), Some(0));
        }
    });
}

#[test]
fn test_time_expression_mittag() {
    let p = parser();
    test_single_case(&p, "am Mittag", None, |r| {
        assert_eq!(r.text, "Mittag");
        assert_component(r, Component::Hour, Some(12));
    });
}

// =============================================================================
// Tests from de_dash.test.ts
// =============================================================================

#[test]
fn test_dash_date_expression() {
    let p = parser();
    test_single_case(&p, "30-12-16", None, |r| {
        assert_component(r, Component::Year, Some(2016));
        assert_component(r, Component::Month, Some(12));
        assert_component(r, Component::Day, Some(30));
    });
}

#[test]
fn test_dash_date_with_weekday() {
    let p = parser();
    test_single_case(&p, "Freitag 30-12-16", None, |r| {
        assert_eq!(r.text, "Freitag 30-12-16");
        assert_component(r, Component::Year, Some(2016));
        assert_component(r, Component::Month, Some(12));
        assert_component(r, Component::Day, Some(30));
    });
}

#[test]
fn test_dot_date_expression() {
    let p = parser();
    test_single_case(&p, "30.12.16", None, |r| {
        assert_component(r, Component::Year, Some(2016));
        assert_component(r, Component::Month, Some(12));
        assert_component(r, Component::Day, Some(30));
    });
}

#[test]
fn test_dot_date_with_weekday() {
    let p = parser();
    test_single_case(&p, "Freitag 30.12.16", None, |r| {
        assert_eq!(r.text, "Freitag 30.12.16");
        assert_component(r, Component::Year, Some(2016));
        assert_component(r, Component::Month, Some(12));
        assert_component(r, Component::Day, Some(30));
    });
}

// =============================================================================
// Tests from de_month_name_little_endian.test.ts
// =============================================================================

#[test]
fn test_month_name_little_endian_basic() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "10. August 2012", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_eq!(r.text, "10. August 2012");
    });
}

#[test]
fn test_month_name_little_endian_bc() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "10. August 113 v. Chr.", Some(ref_date), |r| {
        assert_eq!(r.text, "10. August 113 v. Chr.");
        assert_component(r, Component::Year, Some(-113));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_month_name_little_endian_ad() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "10. August 85 n. Chr.", Some(ref_date), |r| {
        assert_eq!(r.text, "10. August 85 n. Chr.");
        assert_component(r, Component::Year, Some(85));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_month_name_little_endian_abbrev() {
    let p = parser();
    let ref_date = create_ref(2013, 8, 10, 12, 0, 0);
    test_single_case(&p, "So 15.Sep", Some(ref_date), |r| {
        assert_eq!(r.text, "So 15.Sep");
        assert_component(r, Component::Year, Some(2013));
        assert_component(r, Component::Month, Some(9));
        assert_component(r, Component::Day, Some(15));
    });
}

#[test]
fn test_month_name_little_endian_abbrev_caps() {
    let p = parser();
    let ref_date = create_ref(2013, 8, 10, 12, 0, 0);
    test_single_case(&p, "SO 15.SEPT", Some(ref_date), |r| {
        assert_eq!(r.text, "SO 15.SEPT");
        assert_component(r, Component::Year, Some(2013));
        assert_component(r, Component::Month, Some(9));
        assert_component(r, Component::Day, Some(15));
    });
}

#[test]
fn test_month_name_little_endian_am() {
    let p = parser();
    let ref_date = create_ref(2013, 8, 10, 12, 0, 0);
    test_single_case(&p, "Die Deadline ist am 10. August", Some(ref_date), |r| {
        assert_eq!(r.text, "am 10. August");
        assert_component(r, Component::Year, Some(2013));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_month_name_little_endian_weekday() {
    let p = parser();
    let ref_date = create_ref(2013, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "Die Deadline ist am Dienstag, den 10. Januar",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "am Dienstag, den 10. Januar");
            assert_component(r, Component::Year, Some(2014));
            assert_component(r, Component::Month, Some(1));
            assert_component(r, Component::Day, Some(10));
            assert_eq!(
                r.start.get(Component::Weekday),
                Some(Weekday::Tuesday as i32)
            );
        },
    );
}

#[test]
fn test_month_name_little_endian_weekday_abbrev() {
    let p = parser();
    let ref_date = create_ref(2013, 8, 10, 12, 0, 0);
    test_single_case(&p, "Die Deadline ist Di, 10. Januar", Some(ref_date), |r| {
        assert_eq!(r.text, "Di, 10. Januar");
        assert_component(r, Component::Year, Some(2014));
        assert_component(r, Component::Month, Some(1));
        assert_component(r, Component::Day, Some(10));
        assert_eq!(
            r.start.get(Component::Weekday),
            Some(Weekday::Tuesday as i32)
        );
    });
}

#[test]
fn test_month_name_little_endian_maerz() {
    let p = parser();
    let ref_date = create_ref(2013, 8, 10, 12, 0, 0);
    test_single_case(&p, "31. März 2016", Some(ref_date), |r| {
        assert_eq!(r.text, "31. März 2016");
        assert_component(r, Component::Year, Some(2016));
        assert_component(r, Component::Month, Some(3));
        assert_component(r, Component::Day, Some(31));
    });
}

#[test]
fn test_month_name_little_endian_maerz_ae() {
    let p = parser();
    let ref_date = create_ref(2013, 8, 10, 12, 0, 0);
    test_single_case(&p, "31.Maerz 2016", Some(ref_date), |r| {
        assert_eq!(r.text, "31.Maerz 2016");
        assert_component(r, Component::Year, Some(2016));
        assert_component(r, Component::Month, Some(3));
        assert_component(r, Component::Day, Some(31));
    });
}

#[test]
fn test_month_name_little_endian_range() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "10. - 22. August 2012", Some(ref_date), |r| {
        assert_eq!(r.text, "10. - 22. August 2012");
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
fn test_month_name_little_endian_range_bis() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "10. bis 22. Oktober 2012", Some(ref_date), |r| {
        assert_eq!(r.text, "10. bis 22. Oktober 2012");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(10));
        assert_component(r, Component::Day, Some(10));
        if let Some(end) = &r.end {
            assert_eq!(end.get(Component::Year), Some(2012));
            assert_eq!(end.get(Component::Month), Some(10));
            assert_eq!(end.get(Component::Day), Some(22));
        }
    });
}

#[test]
fn test_month_name_little_endian_range_cross_month() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "10. Oktober - 12. Dezember", Some(ref_date), |r| {
        assert_eq!(r.text, "10. Oktober - 12. Dezember");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(10));
        assert_component(r, Component::Day, Some(10));
        if let Some(end) = &r.end {
            assert_eq!(end.get(Component::Year), Some(2012));
            assert_eq!(end.get(Component::Month), Some(12));
            assert_eq!(end.get(Component::Day), Some(12));
        }
    });
}

#[test]
fn test_month_name_little_endian_range_cross_month_year() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "10. August - 12. Oktober 2013", Some(ref_date), |r| {
        assert_eq!(r.text, "10. August - 12. Oktober 2013");
        assert_component(r, Component::Year, Some(2013));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        if let Some(end) = &r.end {
            assert_eq!(end.get(Component::Year), Some(2013));
            assert_eq!(end.get(Component::Month), Some(10));
            assert_eq!(end.get(Component::Day), Some(12));
        }
    });
}

#[test]
fn test_month_name_little_endian_austrian_jaenner() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "10. jänner 2012", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(1));
        assert_component(r, Component::Day, Some(10));
        assert_eq!(r.text, "10. jänner 2012");
    });
}

#[test]
fn test_month_name_little_endian_combined_time() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "12. Juli um 19:00", Some(ref_date), |r| {
        assert_eq!(r.text, "12. Juli um 19:00");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(7));
        assert_component(r, Component::Day, Some(12));
        assert_component(r, Component::Hour, Some(19));
        assert_component(r, Component::Minute, Some(0));
    });
}

#[test]
fn test_month_name_little_endian_combined_time_uhr() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "12. Juli um 19 Uhr", Some(ref_date), |r| {
        assert_eq!(r.text, "12. Juli um 19 Uhr");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(7));
        assert_component(r, Component::Day, Some(12));
        assert_component(r, Component::Hour, Some(19));
        assert_component(r, Component::Minute, Some(0));
    });
}

#[test]
fn test_month_name_little_endian_combined_time_full() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "12. Juli um 19:53 Uhr", Some(ref_date), |r| {
        assert_eq!(r.text, "12. Juli um 19:53 Uhr");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(7));
        assert_component(r, Component::Day, Some(12));
        assert_component(r, Component::Hour, Some(19));
        assert_component(r, Component::Minute, Some(53));
    });
}

#[test]
fn test_month_name_little_endian_combined_time_noon() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "5. Juni 12:00", Some(ref_date), |r| {
        assert_eq!(r.text, "5. Juni 12:00");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(6));
        assert_component(r, Component::Day, Some(5));
        assert_component(r, Component::Hour, Some(12));
        assert_component(r, Component::Minute, Some(0));
    });
}

#[test]
fn test_month_name_little_endian_impossible_dates() {
    let p = parser();
    test_no_result(&p, "32. Oktober 2015", None);
}

// =============================================================================
// Tests from de_weekday.test.ts
// =============================================================================

#[test]
fn test_weekday_montag() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 9, 12, 0, 0);
    test_single_case(&p, "Montag", Some(ref_date), |r| {
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
fn test_weekday_am_donnerstag() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 9, 12, 0, 0);
    test_single_case(&p, "am Donnerstag", Some(ref_date), |r| {
        assert_eq!(r.text, "am Donnerstag");
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
fn test_weekday_sonntag() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 9, 12, 0, 0);
    test_single_case(&p, "Sonntag", Some(ref_date), |r| {
        assert_eq!(r.text, "Sonntag");
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
fn test_weekday_letzten_freitag() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 9, 12, 0, 0);
    test_single_case(
        &p,
        "Die Deadline war letzten Freitag...",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "letzten Freitag");
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
fn test_weekday_naechste_woche() {
    let p = parser();
    let ref_date = create_ref(2015, 4, 18, 12, 0, 0);
    test_single_case(
        &p,
        "Treffen wir uns am Freitag nächste Woche",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "am Freitag nächste Woche");
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

#[test]
fn test_weekday_dienstag_naechste_woche() {
    let p = parser();
    let ref_date = create_ref(2015, 4, 18, 12, 0, 0);
    test_single_case(
        &p,
        "Ich habe vor, am Dienstag nächste Woche freizunehmen",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "am Dienstag nächste Woche");
            assert_component(r, Component::Year, Some(2015));
            assert_component(r, Component::Month, Some(4));
            assert_component(r, Component::Day, Some(21));
            assert_eq!(
                r.start.get(Component::Weekday),
                Some(Weekday::Tuesday as i32)
            );
        },
    );
}

#[test]
fn test_weekday_overlap_den() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 9, 12, 0, 0);
    test_single_case(&p, "Sonntag, den 7. Dezember 2014", Some(ref_date), |r| {
        assert_eq!(r.text, "Sonntag, den 7. Dezember 2014");
        assert_component(r, Component::Year, Some(2014));
        assert_component(r, Component::Month, Some(12));
        assert_component(r, Component::Day, Some(7));
        assert_eq!(
            r.start.get(Component::Weekday),
            Some(Weekday::Sunday as i32)
        );
    });
}

#[test]
fn test_weekday_overlap_dot() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 9, 12, 0, 0);
    test_single_case(&p, "Sonntag 7.12.2014", Some(ref_date), |r| {
        assert_eq!(r.text, "Sonntag 7.12.2014");
        assert_component(r, Component::Year, Some(2014));
        assert_component(r, Component::Month, Some(12));
        assert_component(r, Component::Day, Some(7));
        assert_eq!(
            r.start.get(Component::Weekday),
            Some(Weekday::Sunday as i32)
        );
    });
}

// =============================================================================
// Tests from de_time_units_within.test.ts
// =============================================================================

#[test]
fn test_time_units_within_tagen() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "Wir müssen etwas in 5 Tagen erledigen.",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "in 5 Tagen");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(8));
            assert_component(r, Component::Day, Some(15));
        },
    );
}

#[test]
fn test_time_units_within_fuenf_tagen() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 11, 12, 0);
    test_single_case(
        &p,
        "Wir müssen etwas in fünf Tagen erledigen.",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "in fünf Tagen");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(8));
            assert_component(r, Component::Day, Some(15));
            assert_component(r, Component::Hour, Some(11));
            assert_component(r, Component::Minute, Some(12));
        },
    );
}

#[test]
fn test_time_units_within_minuten() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 14, 0);
    test_single_case(&p, "in 5 Minuten", Some(ref_date), |r| {
        assert_eq!(r.text, "in 5 Minuten");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(12));
        assert_component(r, Component::Minute, Some(19));
    });
}

#[test]
fn test_time_units_within_fuer_minuten() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 14, 0);
    test_single_case(&p, "für 5 minuten", Some(ref_date), |r| {
        assert_eq!(r.text, "für 5 minuten");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(12));
        assert_component(r, Component::Minute, Some(19));
    });
}

#[test]
fn test_time_units_within_einer_stunde() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 14, 0);
    test_single_case(&p, "in einer Stunde", Some(ref_date), |r| {
        assert_eq!(r.text, "in einer Stunde");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(13));
        assert_component(r, Component::Minute, Some(14));
    });
}

#[test]
fn test_time_units_within_timer() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 14, 0);
    test_single_case(
        &p,
        "starte einen Timer für 5 Minuten",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "für 5 Minuten");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(8));
            assert_component(r, Component::Day, Some(10));
            assert_component(r, Component::Hour, Some(12));
            assert_component(r, Component::Minute, Some(19));
        },
    );
}

#[test]
fn test_time_units_within_sekunden() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 14, 0);
    test_single_case(
        &p,
        "In 5 Sekunden wird ein Auto fahren",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "In 5 Sekunden");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(8));
            assert_component(r, Component::Day, Some(10));
            assert_component(r, Component::Hour, Some(12));
            assert_component(r, Component::Minute, Some(14));
            assert_component(r, Component::Second, Some(5));
        },
    );
}

#[test]
fn test_time_units_within_zwei_wochen() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 14, 0);
    test_single_case(&p, "in zwei Wochen", Some(ref_date), |r| {
        assert_eq!(r.text, "in zwei Wochen");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(24));
    });
}

#[test]
fn test_time_units_within_einem_monat() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 14, 0);
    test_single_case(&p, "in einem Monat", Some(ref_date), |r| {
        assert_eq!(r.text, "in einem Monat");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(9));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_time_units_within_einigen_monaten() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 14, 0);
    test_single_case(&p, "in einigen Monaten", Some(ref_date), |r| {
        assert_eq!(r.text, "in einigen Monaten");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(11));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_time_units_within_einem_jahr() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 14, 0);
    test_single_case(&p, "in einem Jahr", Some(ref_date), |r| {
        assert_eq!(r.text, "in einem Jahr");
        assert_component(r, Component::Year, Some(2013));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_time_units_within_20_jahren() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 14, 0);
    test_single_case(&p, "in 20 Jahren", Some(ref_date), |r| {
        assert_eq!(r.text, "in 20 Jahren");
        assert_component(r, Component::Year, Some(2032));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_time_units_within_min_abbrev() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 14, 0);
    test_single_case(&p, "In 5 Min wird ein Auto fahren", Some(ref_date), |r| {
        assert_eq!(r.text, "In 5 Min");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(12));
        assert_component(r, Component::Minute, Some(19));
    });
}

// =============================================================================
// Tests from de_time_units_casual_relative.test.ts
// =============================================================================

#[test]
fn test_time_units_casual_relative_kommende_woche() {
    let p = parser();
    let ref_date = create_ref(2017, 5, 12, 0, 0, 0);
    test_single_case(&p, "kommende Woche", Some(ref_date), |r| {
        assert_eq!(r.text, "kommende Woche");
        assert_component(r, Component::Year, Some(2017));
        assert_component(r, Component::Month, Some(5));
        assert_component(r, Component::Day, Some(19));
    });
}

#[test]
fn test_time_units_casual_relative_in_2_wochen() {
    let p = parser();
    let ref_date = create_ref(2017, 5, 12, 18, 11, 0);
    test_single_case(&p, "in 2 Wochen", Some(ref_date), |r| {
        assert_eq!(r.text, "in 2 Wochen");
        assert_component(r, Component::Year, Some(2017));
        assert_component(r, Component::Month, Some(5));
        assert_component(r, Component::Day, Some(26));
        assert_component(r, Component::Hour, Some(18));
        assert_component(r, Component::Minute, Some(11));
    });
}

#[test]
fn test_time_units_casual_relative_in_drei_wochen() {
    let p = parser();
    let ref_date = create_ref(2017, 5, 12, 18, 11, 0);
    test_single_case(&p, "in drei Wochen", Some(ref_date), |r| {
        assert_eq!(r.text, "in drei Wochen");
        assert_component(r, Component::Year, Some(2017));
        assert_component(r, Component::Month, Some(6));
        assert_component(r, Component::Day, Some(2));
    });
}

#[test]
fn test_time_units_casual_relative_letzten_monat() {
    let p = parser();
    let ref_date = create_ref(2017, 5, 12, 0, 0, 0);
    test_single_case(&p, "letzten Monat", Some(ref_date), |r| {
        assert_eq!(r.text, "letzten Monat");
        assert_component(r, Component::Year, Some(2017));
        assert_component(r, Component::Month, Some(4));
        assert_component(r, Component::Day, Some(12));
    });
}

#[test]
fn test_time_units_casual_relative_letztes_quartal() {
    let p = parser();
    let ref_date = create_ref(2017, 5, 12, 11, 27, 0);
    test_single_case(&p, "letztes Quartal", Some(ref_date), |r| {
        assert_eq!(r.text, "letztes Quartal");
        assert_component(r, Component::Year, Some(2017));
        assert_component(r, Component::Month, Some(2));
        assert_component(r, Component::Day, Some(12));
        assert_component(r, Component::Hour, Some(11));
        assert_component(r, Component::Minute, Some(27));
        assert_component(r, Component::Second, Some(0));
    });
}

#[test]
fn test_time_units_casual_relative_kommendes_jahr() {
    let p = parser();
    let ref_date = create_ref(2017, 5, 12, 11, 27, 0);
    test_single_case(&p, "kommendes Jahr", Some(ref_date), |r| {
        assert_eq!(r.text, "kommendes Jahr");
        assert_component(r, Component::Year, Some(2018));
        assert_component(r, Component::Month, Some(5));
        assert_component(r, Component::Day, Some(12));
        assert_component(r, Component::Hour, Some(11));
        assert_component(r, Component::Minute, Some(27));
        assert_component(r, Component::Second, Some(0));
    });
}

// =============================================================================
// Tests from de_year.test.ts
// =============================================================================

#[test]
fn test_year_vuz() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "10. August 234 v.u.Z.", Some(ref_date), |r| {
        assert_eq!(r.text, "10. August 234 v.u.Z.");
        assert_component(r, Component::Year, Some(-234));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_year_nuz() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "10. August 88 nuZ", Some(ref_date), |r| {
        assert_eq!(r.text, "10. August 88 nuZ");
        assert_component(r, Component::Year, Some(88));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_year_uz() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "10. August 88 uZ", Some(ref_date), |r| {
        assert_eq!(r.text, "10. August 88 uZ");
        assert_component(r, Component::Year, Some(88));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_year_dgz() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "10. August 88 d.g.Z.", Some(ref_date), |r| {
        assert_eq!(r.text, "10. August 88 d.g.Z.");
        assert_component(r, Component::Year, Some(88));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_year_vchr() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "10. August 234 v.Chr.", Some(ref_date), |r| {
        assert_eq!(r.text, "10. August 234 v.Chr.");
        assert_component(r, Component::Year, Some(-234));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_year_nc() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "10. August 88 nC", Some(ref_date), |r| {
        assert_eq!(r.text, "10. August 88 nC");
        assert_component(r, Component::Year, Some(88));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_year_vdz() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "10. August 234 v.d.Z.", Some(ref_date), |r| {
        assert_eq!(r.text, "10. August 234 v.d.Z.");
        assert_component(r, Component::Year, Some(-234));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_year_ndz() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "10. August 88 ndZ", Some(ref_date), |r| {
        assert_eq!(r.text, "10. August 88 ndZ");
        assert_component(r, Component::Year, Some(88));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_year_vdgz() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "10. August 234 v.d.g.Z.", Some(ref_date), |r| {
        assert_eq!(r.text, "10. August 234 v.d.g.Z.");
        assert_component(r, Component::Year, Some(-234));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_year_ndgz() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "10. August 88 ndgZ", Some(ref_date), |r| {
        assert_eq!(r.text, "10. August 88 ndgZ");
        assert_component(r, Component::Year, Some(88));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

// =============================================================================
// Additional tests from de_casual.test.ts
// =============================================================================

#[test]
fn test_casual_combined_heute_17_uhr() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "Die Deadline ist heute 17 Uhr", Some(ref_date), |r| {
        assert_eq!(r.text, "heute 17 Uhr");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(17));
    });
}

#[test]
fn test_casual_combined_heute_um_17_uhr() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "Die Deadline ist heute um 17 Uhr",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "heute um 17 Uhr");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(8));
            assert_component(r, Component::Day, Some(10));
            assert_component(r, Component::Hour, Some(17));
        },
    );
}

#[test]
fn test_casual_date_range_heute_naechsten_freitag() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 4, 12, 0, 0);
    test_single_case(
        &p,
        "Der Event ist heute - nächsten Freitag",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "heute - nächsten Freitag");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(8));
            assert_component(r, Component::Day, Some(4));
            if let Some(end) = &r.end {
                assert_eq!(end.get(Component::Year), Some(2012));
                assert_eq!(end.get(Component::Month), Some(8));
                assert_eq!(end.get(Component::Day), Some(10));
            }
        },
    );
}

#[test]
fn test_morgen_um_12_uhr() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "morgen um 12 Uhr", Some(ref_date), |r| {
        assert_eq!(r.text, "morgen um 12 Uhr");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(11));
        assert_component(r, Component::Hour, Some(12));
        assert_component(r, Component::Minute, Some(0));
    });
}

#[test]
fn test_dot_date_with_time() {
    let p = parser();
    let ref_date = create_ref(2014, 8, 17, 12, 0, 0);
    test_single_case(&p, "17.08.2014 um 10:00", Some(ref_date), |r| {
        assert_eq!(r.text, "17.08.2014 um 10:00");
        assert_component(r, Component::Year, Some(2014));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(17));
        assert_component(r, Component::Hour, Some(10));
        assert_component(r, Component::Minute, Some(0));
    });
}
