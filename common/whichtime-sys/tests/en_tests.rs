//! English locale tests for whichtime-sys

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
    WhichTime::with_locale(Locale::En)
}

// =============================================================================
// Casual Date Tests
// =============================================================================

#[test]
fn test_casual_now() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 8, 9, 10);
    test_single_case(&p, "The Deadline is now", Some(ref_date), |r| {
        assert_eq!(r.text, "now");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(8));
        assert_component(r, Component::Minute, Some(9));
        assert_component(r, Component::Second, Some(10));
    });
}

#[test]
fn test_casual_today() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 14, 12, 0);
    test_single_case(&p, "The Deadline is today", Some(ref_date), |r| {
        assert_eq!(r.text, "today");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_casual_tonight() {
    let p = parser();
    let ref_date = create_ref(2012, 1, 1, 12, 0, 0);
    test_single_case(&p, "tonight", Some(ref_date), |r| {
        assert_eq!(r.text, "tonight");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(1));
        assert_component(r, Component::Day, Some(1));
        assert_component(r, Component::Hour, Some(22));
    });
}

#[test]
fn test_casual_tomorrow() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 17, 10, 0);
    test_single_case(&p, "The Deadline is Tomorrow", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(11));
    });
}

#[test]
fn test_casual_yesterday() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "The Deadline was yesterday", Some(ref_date), |r| {
        assert_eq!(r.text, "yesterday");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(9));
    });
}

#[test]
fn test_casual_negative_cases() {
    let p = parser();
    test_no_result(&p, "notoday", None);
    test_no_result(&p, "tdtmr", None);
    test_no_result(&p, "xyesterday", None);
    test_no_result(&p, "knowledge", None);
}

// =============================================================================
// ISO Format Tests
// =============================================================================

#[test]
fn test_iso_format_basic() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 8, 12, 0, 0);
    test_single_case(
        &p,
        "Let's finish this before this 2013-2-7.",
        Some(ref_date),
        |r| {
            assert_component(r, Component::Year, Some(2013));
            assert_component(r, Component::Month, Some(2));
            assert_component(r, Component::Day, Some(7));
        },
    );
}

#[test]
fn test_iso_format_with_time() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "2014-04-18 13:00", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2014));
        assert_component(r, Component::Month, Some(4));
        assert_component(r, Component::Day, Some(18));
        assert_component(r, Component::Hour, Some(13));
        assert_component(r, Component::Minute, Some(0));
    });
}

#[test]
fn test_iso_format_with_timezone() {
    let p = parser();
    test_single_case(&p, "1994-11-05T08:15:30-05:30", None, |r| {
        assert_component(r, Component::Year, Some(1994));
        assert_component(r, Component::Month, Some(11));
        assert_component(r, Component::Day, Some(5));
        assert_component(r, Component::Hour, Some(8));
        assert_component(r, Component::Minute, Some(15));
        assert_component(r, Component::Second, Some(30));
    });
}

#[test]
fn test_iso_format_utc() {
    let p = parser();
    test_single_case(&p, "1994-11-05T13:15:30Z", None, |r| {
        assert_component(r, Component::Year, Some(1994));
        assert_component(r, Component::Month, Some(11));
        assert_component(r, Component::Day, Some(5));
        assert_component(r, Component::Hour, Some(13));
        assert_component(r, Component::Minute, Some(15));
        assert_component(r, Component::Second, Some(30));
    });
}

// =============================================================================
// Slash Format Tests
// =============================================================================

#[test]
fn test_slash_format() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "8/10/2012", Some(ref_date), |r| {
        assert_eq!(r.text, "8/10/2012");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_slash_format_short_year() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "8/10/12", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

// =============================================================================
// Month Name Tests
// =============================================================================

#[test]
fn test_month_name_with_year() {
    let p = parser();
    test_single_case(&p, "September 2012", None, |r| {
        assert_eq!(r.text, "September 2012");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(9));
    });
}

#[test]
fn test_month_name_abbreviated() {
    let p = parser();
    test_single_case(&p, "Sept 2012", None, |r| {
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(9));
    });
}

#[test]
fn test_month_name_middle_endian() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "August 10, 2012", Some(ref_date), |r| {
        assert_eq!(r.text, "August 10, 2012");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_month_name_little_endian() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "10 August 2012", Some(ref_date), |r| {
        assert_eq!(r.text, "10 August 2012");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

// =============================================================================
// Weekday Tests
// =============================================================================

#[test]
fn test_weekday_monday() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 9, 12, 0, 0); // Thursday
    test_single_case(&p, "Monday", Some(ref_date), |r| {
        assert_eq!(r.text, "Monday");
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
fn test_weekday_last_friday() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 9, 12, 0, 0); // Thursday
    test_single_case(&p, "last Friday", Some(ref_date), |r| {
        assert!(r.text.contains("Friday"));
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(3));
    });
}

#[test]
fn test_weekday_abbreviated() {
    let p = parser();
    let ref_date = create_ref(2012, 1, 1, 12, 0, 0);
    test_single_case(&p, "thurs", Some(ref_date), |r| {
        assert_eq!(
            r.start.get(Component::Weekday),
            Some(Weekday::Thursday as i32)
        );
    });
}

// =============================================================================
// Time Expression Tests
// =============================================================================

#[test]
fn test_time_am_pm() {
    let p = parser();
    let ref_date = create_ref(2016, 10, 1, 8, 0, 0);
    test_single_case(&p, "11 AM", Some(ref_date), |r| {
        assert!(r.text.contains("11"));
        assert_component(r, Component::Hour, Some(11));
    });
}

#[test]
fn test_time_24h() {
    let p = parser();
    let ref_date = create_ref(2016, 10, 1, 8, 0, 0);
    test_single_case(&p, "20:32:13", Some(ref_date), |r| {
        assert_eq!(r.text, "20:32:13");
        assert_component(r, Component::Hour, Some(20));
        assert_component(r, Component::Minute, Some(32));
        assert_component(r, Component::Second, Some(13));
    });
}

// =============================================================================
// Time Unit Ago Tests
// =============================================================================

#[test]
fn test_days_ago() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "5 days ago", Some(ref_date), |r| {
        assert_eq!(r.text, "5 days ago");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(5));
    });
}

#[test]
fn test_minutes_ago() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 14, 0);
    test_single_case(&p, "15 minute ago", Some(ref_date), |r| {
        assert_component(r, Component::Hour, Some(11));
        assert_component(r, Component::Minute, Some(59));
    });
}

#[test]
fn test_hours_ago() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 14, 0);
    test_single_case(&p, "12 hours ago", Some(ref_date), |r| {
        assert_component(r, Component::Hour, Some(0));
        assert_component(r, Component::Minute, Some(14));
    });
}

// =============================================================================
// Time Unit Later/Within Tests
// =============================================================================

#[test]
fn test_days_later() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "2 days later", Some(ref_date), |r| {
        assert_eq!(r.text, "2 days later");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(12));
    });
}

#[test]
fn test_in_days() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "in 5 days", Some(ref_date), |r| {
        assert_eq!(r.text, "in 5 days");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(15));
    });
}

// =============================================================================
// Relative Date Tests
// =============================================================================

#[test]
fn test_this_week() {
    let p = parser();
    let ref_date = create_ref(2017, 11, 19, 12, 0, 0);
    test_single_case(&p, "this week", Some(ref_date), |r| {
        assert_eq!(r.text, "this week");
        assert_component(r, Component::Year, Some(2017));
        assert_component(r, Component::Month, Some(11));
    });
}

#[test]
fn test_next_month() {
    let p = parser();
    let ref_date = create_ref(2016, 10, 1, 12, 0, 0);
    test_single_case(&p, "next month", Some(ref_date), |r| {
        assert_eq!(r.text, "next month");
        assert_component(r, Component::Year, Some(2016));
        assert_component(r, Component::Month, Some(11));
    });
}

#[test]
fn test_last_week() {
    let p = parser();
    let ref_date = create_ref(2016, 10, 1, 12, 0, 0);
    test_single_case(&p, "last week", Some(ref_date), |r| {
        assert_eq!(r.text, "last week");
        assert_component(r, Component::Year, Some(2016));
        assert_component(r, Component::Month, Some(9));
        assert_component(r, Component::Day, Some(24));
    });
}

#[test]
fn test_in_n_hours() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "in 5 hours", Some(ref_date), |r| {
        assert_eq!(r.text, "in 5 hours");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(17));
    });
    test_single_case(&p, "in 1h", Some(ref_date), |r| {
        assert_eq!(r.text, "in 1h");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(13));
    });
}
