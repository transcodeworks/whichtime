//! Swedish locale tests for whichtime-sys
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
    WhichTime::with_locale(Locale::Sv)
}

// =============================================================================
// Tests from sv_casual.test.ts
// =============================================================================

#[test]
fn test_casual_idag() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "idag", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_casual_imorgon() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "imorgon", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(11));
    });
}

#[test]
fn test_casual_igar() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "igår", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(9));
    });
}

#[test]
fn test_casual_forrgar() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "förrgår", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(8));
    });
}

#[test]
fn test_casual_combined_idag_pa_morgonen() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "idag på morgonen", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(6));
    });
}

#[test]
fn test_casual_combined_idag_pa_formiddagen() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "idag på förmiddagen", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(9));
    });
}

#[test]
fn test_casual_combined_idag_pa_middagen() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "idag på middagen", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(12));
    });
}

#[test]
fn test_casual_combined_idag_pa_eftermiddagen() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "idag på eftermiddagen", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(15));
    });
}

#[test]
fn test_casual_combined_idag_pa_kvallen() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "idag på kvällen", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(20));
    });
}

#[test]
fn test_casual_combined_idag_pa_natten() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "idag på natten", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(2));
    });
}

#[test]
fn test_casual_combined_idag_vid_midnatt() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "idag vid midnatt", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(0));
    });
}

// =============================================================================
// Tests from sv_month_name_little_endian.test.ts
// =============================================================================

#[test]
fn test_month_name_little_endian_den_15_augusti() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "den 15 augusti", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(15));
    });
}

#[test]
fn test_month_name_little_endian_15_augusti_2012() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "15 augusti 2012", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(15));
    });
}

#[test]
fn test_month_name_little_endian_15_aug_2012() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "15 aug 2012", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(15));
    });
}

#[test]
fn test_month_name_little_endian_range() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "15-16 augusti", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(15));
        if let Some(end) = &r.end {
            assert_eq!(end.get(Component::Year), Some(2012));
            assert_eq!(end.get(Component::Month), Some(8));
            assert_eq!(end.get(Component::Day), Some(16));
        }
    });
}

#[test]
fn test_month_name_little_endian_range_till() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "15 till 16 augusti", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(15));
        if let Some(end) = &r.end {
            assert_eq!(end.get(Component::Year), Some(2012));
            assert_eq!(end.get(Component::Month), Some(8));
            assert_eq!(end.get(Component::Day), Some(16));
        }
    });
}

#[test]
fn test_month_name_little_endian_impossible_dates() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_no_result(&p, "32 augusti", Some(ref_date));
}

// =============================================================================
// Tests from sv_time_units_casual_relative.test.ts
// =============================================================================

#[test]
fn test_time_units_casual_relative_nasta_2_veckor() {
    let p = parser();
    let ref_date = create_ref(2016, 10, 1, 12, 0, 0);
    test_single_case(&p, "nästa 2 veckor", Some(ref_date), |r| {
        assert_eq!(r.text, "nästa 2 veckor");
        assert_component(r, Component::Year, Some(2016));
        assert_component(r, Component::Month, Some(10));
        assert_component(r, Component::Day, Some(15));
    });
}

#[test]
fn test_time_units_casual_relative_nasta_2_dagar() {
    let p = parser();
    let ref_date = create_ref(2016, 10, 1, 12, 0, 0);
    test_single_case(&p, "nästa 2 dagar", Some(ref_date), |r| {
        assert_eq!(r.text, "nästa 2 dagar");
        assert_component(r, Component::Year, Some(2016));
        assert_component(r, Component::Month, Some(10));
        assert_component(r, Component::Day, Some(3));
        assert_component(r, Component::Hour, Some(12));
    });
}

#[test]
fn test_time_units_casual_relative_nasta_tva_ar() {
    let p = parser();
    let ref_date = create_ref(2016, 10, 1, 12, 0, 0);
    test_single_case(&p, "nästa två år", Some(ref_date), |r| {
        assert_eq!(r.text, "nästa två år");
        assert_component(r, Component::Year, Some(2018));
        assert_component(r, Component::Month, Some(10));
        assert_component(r, Component::Day, Some(1));
        assert_component(r, Component::Hour, Some(12));
    });
}

#[test]
fn test_time_units_casual_relative_efter_ett_ar() {
    let p = parser();
    let ref_date = create_ref(2016, 10, 1, 12, 0, 0);
    test_single_case(&p, "efter ett år", Some(ref_date), |r| {
        assert_eq!(r.text, "efter ett år");
        assert_component(r, Component::Year, Some(2017));
        assert_component(r, Component::Month, Some(10));
        assert_component(r, Component::Day, Some(1));
        assert_component(r, Component::Hour, Some(12));
    });
}

#[test]
fn test_time_units_casual_relative_efter_en_timme() {
    let p = parser();
    let ref_date = create_ref(2016, 10, 1, 15, 0, 0);
    test_single_case(&p, "efter en timme", Some(ref_date), |r| {
        assert_eq!(r.text, "efter en timme");
        assert_component(r, Component::Year, Some(2016));
        assert_component(r, Component::Month, Some(10));
        assert_component(r, Component::Day, Some(1));
        assert_component(r, Component::Hour, Some(16));
    });
}

#[test]
fn test_time_units_casual_relative_forra_2_veckor() {
    let p = parser();
    let ref_date = create_ref(2016, 10, 1, 12, 0, 0);
    test_single_case(&p, "förra 2 veckor", Some(ref_date), |r| {
        assert_eq!(r.text, "förra 2 veckor");
        assert_component(r, Component::Year, Some(2016));
        assert_component(r, Component::Month, Some(9));
        assert_component(r, Component::Day, Some(17));
        assert_component(r, Component::Hour, Some(12));
    });
}

#[test]
fn test_time_units_casual_relative_plus_15_minuter() {
    let p = parser();
    let ref_date = create_ref(2012, 7, 10, 12, 14, 0);
    test_single_case(&p, "+15 minuter", Some(ref_date), |r| {
        assert_eq!(r.text, "+15 minuter");
        assert_component(r, Component::Hour, Some(12));
        assert_component(r, Component::Minute, Some(29));
    });
}

#[test]
fn test_time_units_casual_relative_minus_3ar() {
    let p = parser();
    let ref_date = create_ref(2015, 7, 10, 12, 14, 0);
    test_single_case(&p, "-3år", Some(ref_date), |r| {
        assert_eq!(r.text, "-3år");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(7));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(12));
        assert_component(r, Component::Minute, Some(14));
    });
}

// =============================================================================
// Tests from sv_weekday.test.ts
// =============================================================================

#[test]
fn test_weekday_mandag() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 9, 12, 0, 0);
    test_single_case(&p, "måndag", Some(ref_date), |r| {
        assert_eq!(r.text, "måndag");
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
fn test_weekday_pa_mandag() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 9, 12, 0, 0);
    test_single_case(&p, "på måndag", Some(ref_date), |r| {
        assert_eq!(r.text, "på måndag");
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
fn test_weekday_nasta_mandag() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 9, 12, 0, 0);
    test_single_case(&p, "nästa måndag", Some(ref_date), |r| {
        assert_eq!(r.text, "nästa måndag");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(13));
        assert_eq!(
            r.start.get(Component::Weekday),
            Some(Weekday::Monday as i32)
        );
    });
}

#[test]
fn test_weekday_forra_mandag() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 9, 12, 0, 0);
    test_single_case(&p, "förra måndag", Some(ref_date), |r| {
        assert_eq!(r.text, "förra måndag");
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
fn test_weekday_sondag() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 9, 12, 0, 0);
    test_single_case(&p, "söndag", Some(ref_date), |r| {
        assert_eq!(
            r.start.get(Component::Weekday),
            Some(Weekday::Sunday as i32)
        );
    });
}

#[test]
fn test_weekday_tisdag() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 9, 12, 0, 0);
    test_single_case(&p, "tisdag", Some(ref_date), |r| {
        assert_eq!(
            r.start.get(Component::Weekday),
            Some(Weekday::Tuesday as i32)
        );
    });
}

#[test]
fn test_weekday_fredag() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 9, 12, 0, 0);
    test_single_case(&p, "fredag", Some(ref_date), |r| {
        assert_eq!(
            r.start.get(Component::Weekday),
            Some(Weekday::Friday as i32)
        );
    });
}

#[test]
fn test_weekday_lordag() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 9, 12, 0, 0);
    test_single_case(&p, "lördag", Some(ref_date), |r| {
        assert_eq!(
            r.start.get(Component::Weekday),
            Some(Weekday::Saturday as i32)
        );
    });
}

#[test]
fn test_imorgon_vid_middag() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "imorgon vid middag", Some(ref_date), |r| {
        assert_eq!(r.text, "imorgon vid middag");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(11));
        assert_component(r, Component::Hour, Some(12));
        assert_component(r, Component::Minute, Some(0));
    });
}
