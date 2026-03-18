//! Basic tests for whichtime-sys

use chrono::{Datelike, Local, TimeZone};
use whichtime_sys::{Component, WhichTime};

fn create_date(
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

fn create_ref(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
) -> chrono::DateTime<Local> {
    create_date(year, month, day, hour, minute, second)
}

#[test]
fn test_iso_format() {
    let parser = WhichTime::new();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);

    let results = parser.parse("2014-04-18 13:00", Some(ref_date)).unwrap();
    assert_eq!(results.len(), 1);

    let result = &results[0];
    assert_eq!(result.start.get(Component::Year), Some(2014));
    assert_eq!(result.start.get(Component::Month), Some(4));
    assert_eq!(result.start.get(Component::Day), Some(18));
    assert_eq!(result.start.get(Component::Hour), Some(13));
    assert_eq!(result.start.get(Component::Minute), Some(0));
}

#[test]
fn test_casual_today() {
    let parser = WhichTime::new();
    let ref_date = create_ref(2012, 8, 10, 14, 12, 0);

    let results = parser
        .parse("The Deadline is today", Some(ref_date))
        .unwrap();
    assert_eq!(results.len(), 1);

    let result = &results[0];
    assert_eq!(result.text, "today");
    assert_eq!(result.start.get(Component::Year), Some(2012));
    assert_eq!(result.start.get(Component::Month), Some(8));
    assert_eq!(result.start.get(Component::Day), Some(10));
}

#[test]
fn test_casual_tomorrow() {
    let parser = WhichTime::new();
    let ref_date = create_ref(2012, 8, 10, 17, 10, 0);

    let results = parser
        .parse("The Deadline is Tomorrow", Some(ref_date))
        .unwrap();
    assert_eq!(results.len(), 1);

    let result = &results[0];
    assert_eq!(result.start.get(Component::Year), Some(2012));
    assert_eq!(result.start.get(Component::Month), Some(8));
    assert_eq!(result.start.get(Component::Day), Some(11));
}

#[test]
fn test_casual_yesterday() {
    let parser = WhichTime::new();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);

    let results = parser
        .parse("The Deadline was yesterday", Some(ref_date))
        .unwrap();
    assert_eq!(results.len(), 1);

    let result = &results[0];
    assert_eq!(result.start.get(Component::Year), Some(2012));
    assert_eq!(result.start.get(Component::Month), Some(8));
    assert_eq!(result.start.get(Component::Day), Some(9));
}

#[test]
fn test_time_expression() {
    let parser = WhichTime::new();
    let ref_date = create_ref(2020, 7, 6, 12, 0, 0);

    let results = parser.parse("meeting at 3:30 PM", Some(ref_date)).unwrap();
    assert!(results.len() >= 1);

    let result = &results[0];
    assert_eq!(result.start.get(Component::Hour), Some(15));
    assert_eq!(result.start.get(Component::Minute), Some(30));
}

#[test]
fn test_slash_date() {
    let parser = WhichTime::new();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);

    let results = parser.parse("8/10/2012", Some(ref_date)).unwrap();
    assert_eq!(results.len(), 1);

    let result = &results[0];
    assert_eq!(result.start.get(Component::Year), Some(2012));
    assert_eq!(result.start.get(Component::Month), Some(8));
    assert_eq!(result.start.get(Component::Day), Some(10));
}

#[test]
fn test_month_name() {
    let parser = WhichTime::new();

    let results = parser.parse("September 2012", None).unwrap();
    assert_eq!(results.len(), 1);

    let result = &results[0];
    assert_eq!(result.start.get(Component::Year), Some(2012));
    assert_eq!(result.start.get(Component::Month), Some(9));
}

#[test]
fn test_weekday() {
    let parser = WhichTime::new();
    let ref_date = create_ref(2012, 8, 9, 12, 0, 0); // Thursday

    let results = parser.parse("last Friday", Some(ref_date)).unwrap();
    assert_eq!(results.len(), 1);

    let result = &results[0];
    assert_eq!(result.start.get(Component::Year), Some(2012));
    assert_eq!(result.start.get(Component::Month), Some(8));
    assert_eq!(result.start.get(Component::Day), Some(3));
}

#[test]
fn test_time_unit_ago() {
    let parser = WhichTime::new();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);

    let results = parser.parse("5 days ago", Some(ref_date)).unwrap();
    assert_eq!(results.len(), 1);

    let result = &results[0];
    assert_eq!(result.start.get(Component::Year), Some(2012));
    assert_eq!(result.start.get(Component::Month), Some(8));
    assert_eq!(result.start.get(Component::Day), Some(5));
}

#[test]
fn test_time_unit_later() {
    let parser = WhichTime::new();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);

    let results = parser.parse("2 days later", Some(ref_date)).unwrap();
    assert_eq!(results.len(), 1);

    let result = &results[0];
    assert_eq!(result.start.get(Component::Year), Some(2012));
    assert_eq!(result.start.get(Component::Month), Some(8));
    assert_eq!(result.start.get(Component::Day), Some(12));
}

#[test]
fn test_within() {
    let parser = WhichTime::new();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);

    let results = parser.parse("in 5 days", Some(ref_date)).unwrap();
    assert_eq!(results.len(), 1);

    let result = &results[0];
    assert_eq!(result.start.get(Component::Year), Some(2012));
    assert_eq!(result.start.get(Component::Month), Some(8));
    assert_eq!(result.start.get(Component::Day), Some(15));
}

#[test]
fn test_relative_this_week() {
    let parser = WhichTime::new();
    let ref_date = create_ref(2017, 11, 19, 12, 0, 0);

    let results = parser.parse("this week", Some(ref_date)).unwrap();
    assert_eq!(results.len(), 1);

    let result = &results[0];
    assert_eq!(result.start.get(Component::Year), Some(2017));
    assert_eq!(result.start.get(Component::Month), Some(11));
}

#[test]
fn test_relative_next_month() {
    let parser = WhichTime::new();
    let ref_date = create_ref(2016, 10, 1, 12, 0, 0);

    let results = parser.parse("next month", Some(ref_date)).unwrap();
    assert_eq!(results.len(), 1);

    let result = &results[0];
    assert_eq!(result.start.get(Component::Year), Some(2016));
    assert_eq!(result.start.get(Component::Month), Some(11));
}

#[test]
fn test_no_false_positives() {
    let parser = WhichTime::new();

    // These should not match
    let results = parser.parse("notoday", None).unwrap();
    assert_eq!(results.len(), 0);

    let results = parser.parse("knowledge", None).unwrap();
    assert_eq!(results.len(), 0);
}

#[test]
fn test_parse_date_shortcut() {
    let parser = WhichTime::new();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);

    let date = parser.parse_date("tomorrow", Some(ref_date)).unwrap();
    assert!(date.is_some());

    let date = date.unwrap();
    assert_eq!(date.year(), 2012);
    assert_eq!(date.month(), 8);
    assert_eq!(date.day(), 11);
}
