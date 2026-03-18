//! Dutch locale tests for whichtime-sys
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
    WhichTime::with_locale(Locale::Nl)
}

// =============================================================================
// Tests from nl_casual.test.ts
// =============================================================================

#[test]
fn test_casual_nu() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 8, 9, 10);
    test_single_case(&p, "De deadline is nu", Some(ref_date), |r| {
        assert_eq!(r.text, "nu");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(8));
        assert_component(r, Component::Minute, Some(9));
        assert_component(r, Component::Second, Some(10));
    });
}

#[test]
fn test_casual_vandaag() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 14, 12, 0);
    test_single_case(&p, "De deadline is vandaag", Some(ref_date), |r| {
        assert_eq!(r.text, "vandaag");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_casual_morgen() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 14, 12, 0);
    test_single_case(&p, "De deadline is morgen", Some(ref_date), |r| {
        assert_eq!(r.text, "morgen");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(11));
    });
}

#[test]
fn test_casual_gisteren() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "De deadline was gisteren", Some(ref_date), |r| {
        assert_eq!(r.text, "gisteren");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(9));
    });
}

#[test]
fn test_casual_deze_ochtend() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "De Deadline was deze ochtend", Some(ref_date), |r| {
        assert_eq!(r.text, "deze ochtend");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(6));
    });
}

#[test]
fn test_casual_deze_namiddag() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "De Deadline was deze namiddag ", Some(ref_date), |r| {
        assert_eq!(r.text, "deze namiddag");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(15));
    });
}

#[test]
fn test_casual_deze_avond() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "De Deadline was deze avond ", Some(ref_date), |r| {
        assert_eq!(r.text, "deze avond");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(20));
    });
}

#[test]
fn test_casual_vanavond() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "De deadline is vanavond", Some(ref_date), |r| {
        assert_eq!(r.text, "vanavond");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(20));
    });
}

#[test]
fn test_casual_middernacht() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 1, 0, 0);
    test_single_case(&p, "The Deadline is om middernacht ", Some(ref_date), |r| {
        assert_eq!(r.text, "middernacht");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(11));
        assert_component(r, Component::Hour, Some(0));
    });
}

#[test]
fn test_casual_combined_vandaag_17h() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "De deadline is vandaag om 17:00", Some(ref_date), |r| {
        assert_eq!(r.text, "vandaag om 17:00");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(17));
    });
}

#[test]
fn test_casual_gisterenochtend() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 14, 0, 0);
    test_single_case(&p, "gisterenochtend", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(9));
        assert_component(r, Component::Hour, Some(6));
    });
}

#[test]
fn test_casual_gisterenmiddag() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 14, 0, 0);
    test_single_case(&p, "gisterenmiddag", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(9));
        assert_component(r, Component::Hour, Some(12));
    });
}

#[test]
fn test_casual_gisterenavond() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 14, 0, 0);
    test_single_case(&p, "gisterenavond", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(9));
        assert_component(r, Component::Hour, Some(20));
    });
}

#[test]
fn test_casual_vanochtend() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 14, 0, 0);
    test_single_case(&p, "vanochtend", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(6));
    });
}

#[test]
fn test_casual_vanmiddag() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 14, 0, 0);
    test_single_case(&p, "vanmiddag", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(12));
    });
}

#[test]
fn test_casual_morgenochtend() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 14, 0, 0);
    test_single_case(&p, "morgenochtend", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(11));
        assert_component(r, Component::Hour, Some(6));
    });
}

#[test]
fn test_casual_morgenmiddag() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 14, 0, 0);
    test_single_case(&p, "morgenmiddag", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(11));
        assert_component(r, Component::Hour, Some(12));
    });
}

#[test]
fn test_casual_morgenavond() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 14, 0, 0);
    test_single_case(&p, "morgenavond", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(11));
        assert_component(r, Component::Hour, Some(20));
    });
}

#[test]
fn test_casual_date_range() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 4, 12, 0, 0);
    test_single_case(
        &p,
        "Het evenement is vandaag - volgende vrijdag",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "vandaag - volgende vrijdag");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(8));
            assert_component(r, Component::Day, Some(4));
            assert_component(r, Component::Hour, Some(12));
            if let Some(end) = &r.end {
                assert_eq!(end.get(Component::Year), Some(2012));
                assert_eq!(end.get(Component::Month), Some(8));
                assert_eq!(end.get(Component::Day), Some(10));
                assert_eq!(end.get(Component::Hour), Some(12));
            }
        },
    );
}

#[test]
fn test_casual_random_vanavond() {
    let p = parser();
    let ref_date = create_ref(2012, 1, 1, 12, 0, 0);
    test_single_case(&p, "vanavond", Some(ref_date), |r| {
        assert_eq!(r.text, "vanavond");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(1));
        assert_component(r, Component::Day, Some(1));
        assert_component(r, Component::Hour, Some(20));
        assert_eq!(r.start.get(Component::Meridiem), Some(Meridiem::PM as i32));
    });
}

#[test]
fn test_casual_middag() {
    let p = parser();
    let ref_date = create_ref(2012, 1, 1, 12, 0, 0);
    test_single_case(&p, "middag", Some(ref_date), |r| {
        assert_eq!(r.text, "middag");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(1));
        assert_component(r, Component::Day, Some(1));
        assert_component(r, Component::Hour, Some(12));
        assert_eq!(r.start.get(Component::Meridiem), Some(Meridiem::AM as i32));
    });
}

#[test]
fn test_casual_negative_cases() {
    let p = parser();
    test_no_result(&p, "notoday", None);
    test_no_result(&p, "tdtmr", None);
    test_no_result(&p, "xyesterday", None);
    test_no_result(&p, "nowhere", None);
    test_no_result(&p, "noway", None);
    test_no_result(&p, "knowledge", None);
}

// =============================================================================
// Tests from nl_month_name_little_endian.test.ts
// =============================================================================

#[test]
fn test_month_name_little_endian_basic() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "10 augustus 2012", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_eq!(r.text, "10 augustus 2012");
    });
}

#[test]
fn test_month_name_little_endian_short_year() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "3 februari 82", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(1982));
        assert_component(r, Component::Month, Some(2));
        assert_component(r, Component::Day, Some(3));
        assert_eq!(r.text, "3 februari 82");
    });
}

#[test]
fn test_month_name_little_endian_bc() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "10 augustus 234 voor Christus", Some(ref_date), |r| {
        assert_eq!(r.text, "10 augustus 234 voor Christus");
        assert_component(r, Component::Year, Some(-234));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_month_name_little_endian_ad() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "10 augustus 88 na Christus", Some(ref_date), |r| {
        assert_eq!(r.text, "10 augustus 88 na Christus");
        assert_component(r, Component::Year, Some(88));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_month_name_little_endian_weekday_abbrev() {
    let p = parser();
    let ref_date = create_ref(2013, 8, 10, 12, 0, 0);
    test_single_case(&p, "Zon 15 Sept", Some(ref_date), |r| {
        assert_eq!(r.text, "Zon 15 Sept");
        assert_component(r, Component::Year, Some(2013));
        assert_component(r, Component::Month, Some(9));
        assert_component(r, Component::Day, Some(15));
    });
}

#[test]
fn test_month_name_little_endian_weekday_caps() {
    let p = parser();
    let ref_date = create_ref(2013, 8, 10, 12, 0, 0);
    test_single_case(&p, "ZON 15 SEPT", Some(ref_date), |r| {
        assert_eq!(r.text, "ZON 15 SEPT");
        assert_component(r, Component::Year, Some(2013));
        assert_component(r, Component::Month, Some(9));
        assert_component(r, Component::Day, Some(15));
    });
}

#[test]
fn test_month_name_little_endian_deadline() {
    let p = parser();
    let ref_date = create_ref(2013, 8, 10, 12, 0, 0);
    test_single_case(&p, "De deadline is 10 augustus", Some(ref_date), |r| {
        assert_eq!(r.text, "10 augustus");
        assert_component(r, Component::Year, Some(2013));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_month_name_little_endian_dinsdag() {
    let p = parser();
    let ref_date = create_ref(2013, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "De deadline is dinsdag, 10 januari",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "dinsdag, 10 januari");
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
fn test_month_name_little_endian_di_abbrev() {
    let p = parser();
    let ref_date = create_ref(2013, 8, 10, 12, 0, 0);
    test_single_case(&p, "De deadline is di, 10 januari", Some(ref_date), |r| {
        assert_eq!(r.text, "di, 10 januari");
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
fn test_month_name_little_endian_31ste() {
    let p = parser();
    let ref_date = create_ref(2013, 8, 10, 12, 0, 0);
    test_single_case(&p, "31ste maart 2016", Some(ref_date), |r| {
        assert_eq!(r.text, "31ste maart 2016");
        assert_component(r, Component::Year, Some(2016));
        assert_component(r, Component::Month, Some(3));
        assert_component(r, Component::Day, Some(31));
    });
}

#[test]
fn test_month_name_little_endian_23ste() {
    let p = parser();
    let ref_date = create_ref(2013, 8, 10, 12, 0, 0);
    test_single_case(&p, "23ste februari 2016", Some(ref_date), |r| {
        assert_eq!(r.text, "23ste februari 2016");
        assert_component(r, Component::Year, Some(2016));
        assert_component(r, Component::Month, Some(2));
        assert_component(r, Component::Day, Some(23));
    });
}

#[test]
fn test_month_name_little_endian_range() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "10 - 22 augustus 2012", Some(ref_date), |r| {
        assert_eq!(r.text, "10 - 22 augustus 2012");
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
fn test_month_name_little_endian_range_tot() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "10 tot 22 augustus 2012", Some(ref_date), |r| {
        assert_eq!(r.text, "10 tot 22 augustus 2012");
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
// Tests from nl_weekday.test.ts
// =============================================================================

#[test]
fn test_weekday_maandag() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 9, 12, 0, 0);
    test_single_case(&p, "Maandag", Some(ref_date), |r| {
        assert_eq!(r.text, "Maandag");
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
fn test_weekday_donderdag() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 9, 12, 0, 0);
    test_single_case(&p, "Donderdag", Some(ref_date), |r| {
        assert_eq!(r.text, "Donderdag");
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
fn test_weekday_zondag() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 9, 12, 0, 0);
    test_single_case(&p, "Zondag", Some(ref_date), |r| {
        assert_eq!(r.text, "Zondag");
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
fn test_weekday_afgelopen_vrijdag() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 9, 12, 0, 0);
    test_single_case(
        &p,
        "De deadline was afgelopen vrijdag...",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "afgelopen vrijdag");
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
fn test_weekday_volgende_vrijdag() {
    let p = parser();
    let ref_date = create_ref(2015, 4, 18, 12, 0, 0);
    test_single_case(
        &p,
        "Laten we vrijdag volgende week afspreken",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "vrijdag volgende week");
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
fn test_morgen_om_12_uur() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "morgen om 12 uur", Some(ref_date), |r| {
        assert_eq!(r.text, "morgen om 12 uur");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(11));
        assert_component(r, Component::Hour, Some(12));
        assert_component(r, Component::Minute, Some(0));
    });
}
