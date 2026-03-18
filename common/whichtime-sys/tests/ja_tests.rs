//! Japanese locale tests for whichtime-sys
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
    WhichTime::with_locale(Locale::Ja)
}

// =============================================================================
// Tests from ja_casual.test.ts
// =============================================================================

#[test]
fn test_casual_kyou_today() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "今日感じたことを忘れずに",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "今日");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(8));
            assert_component(r, Component::Day, Some(10));
        },
    );
}

#[test]
fn test_casual_kyou_hiragana() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "きょう感じたことを忘れずに",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "きょう");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(8));
            assert_component(r, Component::Day, Some(10));
        },
    );
}

#[test]
fn test_casual_honjitsu() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "本日はお日柄もよく", Some(ref_date), |r| {
        assert_eq!(r.text, "本日");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_casual_kinou_yesterday() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "昨日の全国観測値ランキング",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "昨日");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(8));
            assert_component(r, Component::Day, Some(9));
        },
    );
}

#[test]
fn test_casual_kinou_hiragana() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "きのうの全国観測値ランキング",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "きのう");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(8));
            assert_component(r, Component::Day, Some(9));
        },
    );
}

#[test]
fn test_casual_ashita_tomorrow() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "明日の天気は晴れです", Some(ref_date), |r| {
        assert_eq!(r.text, "明日");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(11));
    });
}

#[test]
fn test_casual_ashita_hiragana() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "あしたの天気は晴れです",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "あした");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(8));
            assert_component(r, Component::Day, Some(11));
        },
    );
}

#[test]
fn test_casual_konya_tonight() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "今夜には雨が降るでしょう",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "今夜");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(8));
            assert_component(r, Component::Day, Some(10));
            assert_component(r, Component::Hour, Some(22));
        },
    );
}

#[test]
fn test_casual_konban_tonight() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "今晩には雨が降るでしょう",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "今晩");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(8));
            assert_component(r, Component::Day, Some(10));
            assert_component(r, Component::Hour, Some(22));
        },
    );
}

#[test]
fn test_casual_kesa_morning() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "今朝食べたパンは美味しかった",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "今朝");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(8));
            assert_component(r, Component::Day, Some(10));
            assert_component(r, Component::Hour, Some(6));
        },
    );
}

// =============================================================================
// Tests from ja_slash_date_format.test.ts
// =============================================================================

#[test]
fn test_slash_date_full() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "2012/3/31", Some(ref_date), |r| {
        assert_eq!(r.text, "2012/3/31");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(3));
        assert_component(r, Component::Day, Some(31));
    });
}

#[test]
fn test_slash_date_month_day() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "12/31", Some(ref_date), |r| {
        assert_eq!(r.text, "12/31");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(12));
        assert_component(r, Component::Day, Some(31));
    });
}

#[test]
fn test_slash_date_short() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "8/5", Some(ref_date), |r| {
        assert_eq!(r.text, "8/5");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(5));
    });
}

#[test]
fn test_slash_date_range() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "2013/12/26~2014/1/7", Some(ref_date), |r| {
        assert_eq!(r.text, "2013/12/26~2014/1/7");
        assert_component(r, Component::Year, Some(2013));
        assert_component(r, Component::Month, Some(12));
        assert_component(r, Component::Day, Some(26));
        if let Some(end) = &r.end {
            assert_eq!(end.get(Component::Year), Some(2014));
            assert_eq!(end.get(Component::Month), Some(1));
            assert_eq!(end.get(Component::Day), Some(7));
        }
    });
}

// =============================================================================
// Tests from ja_standard.test.ts
// =============================================================================

#[test]
fn test_standard_full_date() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "主な株主（2012年3月31日現在）",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "2012年3月31日");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(3));
            assert_component(r, Component::Day, Some(31));
        },
    );
}

#[test]
fn test_standard_full_width_month() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "主な株主（2012年９月3日現在）",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "2012年９月3日");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(9));
            assert_component(r, Component::Day, Some(3));
        },
    );
}

#[test]
fn test_standard_leap_year() {
    let p = parser();
    let ref_date = create_ref(2019, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "主な株主（2020年2月29日現在）",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "2020年2月29日");
            assert_component(r, Component::Year, Some(2020));
            assert_component(r, Component::Month, Some(2));
            assert_component(r, Component::Day, Some(29));
        },
    );
}

#[test]
fn test_standard_month_day_only() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "主な株主（９月3日現在）",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "９月3日");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(9));
            assert_component(r, Component::Day, Some(3));
        },
    );
}

#[test]
fn test_standard_heisei_era() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "主な株主（平成26年12月29日）",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "平成26年12月29日");
            assert_component(r, Component::Year, Some(2014));
            assert_component(r, Component::Month, Some(12));
            assert_component(r, Component::Day, Some(29));
        },
    );
}

#[test]
fn test_standard_showa_era() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "主な株主（昭和６４年１月７日）",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "昭和６４年１月７日");
            assert_component(r, Component::Year, Some(1989));
            assert_component(r, Component::Month, Some(1));
            assert_component(r, Component::Day, Some(7));
        },
    );
}

#[test]
fn test_standard_reiwa_gannen() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "主な株主（令和元年5月1日）",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "令和元年5月1日");
            assert_component(r, Component::Year, Some(2019));
            assert_component(r, Component::Month, Some(5));
            assert_component(r, Component::Day, Some(1));
        },
    );
}

#[test]
fn test_standard_reiwa_2() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "主な株主（令和2年5月1日）",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "令和2年5月1日");
            assert_component(r, Component::Year, Some(2020));
            assert_component(r, Component::Month, Some(5));
            assert_component(r, Component::Day, Some(1));
        },
    );
}

#[test]
fn test_standard_dounen_same_year() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "主な株主（同年7月27日）",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "同年7月27日");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(7));
            assert_component(r, Component::Day, Some(27));
        },
    );
}

#[test]
fn test_standard_honnen_this_year() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "主な株主（本年7月27日）",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "本年7月27日");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(7));
            assert_component(r, Component::Day, Some(27));
        },
    );
}

#[test]
fn test_standard_kotoshi_this_year() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "主な株主（今年7月27日）",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "今年7月27日");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(7));
            assert_component(r, Component::Day, Some(27));
        },
    );
}

#[test]
fn test_standard_month_day_without_year() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "7月27日", Some(ref_date), |r| {
        assert_eq!(r.text, "7月27日");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(7));
        assert_component(r, Component::Day, Some(27));
    });
}

#[test]
fn test_standard_date_range() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "2013年12月26日-2014年1月7日",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "2013年12月26日-2014年1月7日");
            assert_component(r, Component::Year, Some(2013));
            assert_component(r, Component::Month, Some(12));
            assert_component(r, Component::Day, Some(26));
            if let Some(end) = &r.end {
                assert_eq!(end.get(Component::Year), Some(2014));
                assert_eq!(end.get(Component::Month), Some(1));
                assert_eq!(end.get(Component::Day), Some(7));
            }
        },
    );
}

// =============================================================================
// Tests from ja_time_exp.test.ts
// =============================================================================

#[test]
fn test_time_gozen_am() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "私は午前6時13分に起きた",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "午前6時13分");
            assert_component(r, Component::Hour, Some(6));
            assert_component(r, Component::Minute, Some(13));
        },
    );
}

#[test]
fn test_time_gozen_8_ji() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "私は午前8時に起きる", Some(ref_date), |r| {
        assert_eq!(r.text, "午前8時");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(8));
    });
}

#[test]
fn test_time_gogo_pm() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "午後8時", Some(ref_date), |r| {
        assert_eq!(r.text, "午後8時");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(20));
    });
}

#[test]
fn test_time_range() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "私は本日午前八時十分から午後11時32分までゲームをした",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "本日午前八時十分から午後11時32分");
            assert_component(r, Component::Hour, Some(8));
            assert_component(r, Component::Minute, Some(10));
            if let Some(end) = &r.end {
                assert_eq!(end.get(Component::Hour), Some(23));
                assert_eq!(end.get(Component::Minute), Some(32));
            }
        },
    );
}

#[test]
fn test_time_range_pm() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "6時30分PM-11時PM", Some(ref_date), |r| {
        assert_eq!(r.text, "6時30分PM-11時PM");
        assert_component(r, Component::Hour, Some(18));
        assert_component(r, Component::Minute, Some(30));
        assert_eq!(r.start.get(Component::Meridiem), Some(Meridiem::PM as i32));
        if let Some(end) = &r.end {
            assert_eq!(end.get(Component::Hour), Some(23));
            assert_eq!(end.get(Component::Minute), Some(0));
            assert_eq!(end.get(Component::Meridiem), Some(Meridiem::PM as i32));
        }
    });
}

#[test]
fn test_time_date_time_combined() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "僕は2018年11月26日午後三時半五十九秒にゲームを始めた",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "2018年11月26日午後三時半五十九秒");
            assert_component(r, Component::Year, Some(2018));
            assert_component(r, Component::Month, Some(11));
            assert_component(r, Component::Day, Some(26));
            assert_component(r, Component::Hour, Some(15));
            assert_component(r, Component::Minute, Some(30));
            assert_component(r, Component::Second, Some(59));
        },
    );
}

#[test]
fn test_time_meridiem_imply() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "午後1時30分から3時10分", Some(ref_date), |r| {
        assert_component(r, Component::Hour, Some(13));
        assert_component(r, Component::Minute, Some(30));
        assert_eq!(r.start.get(Component::Meridiem), Some(Meridiem::PM as i32));
        if let Some(end) = &r.end {
            assert_eq!(end.get(Component::Hour), Some(15));
            assert_eq!(end.get(Component::Minute), Some(10));
            assert_eq!(end.get(Component::Meridiem), Some(Meridiem::PM as i32));
        }
    });
}

#[test]
fn test_time_incorrect() {
    let p = parser();
    test_no_result(&p, "午後１3時", None);
    test_no_result(&p, "25時", None);
    test_no_result(&p, "5時70分", None);
    test_no_result(&p, "5時30分65秒", None);
    test_no_result(&p, "23時-25時", None);
}

#[test]
fn test_non_time() {
    let p = parser();
    test_no_result(&p, "1", None);
    test_no_result(&p, "12", None);
    test_no_result(&p, "12a", None);
    test_no_result(&p, "1時間", None);
    test_no_result(&p, "25時間", None);
}

// =============================================================================
// Tests from ja_weekday.test.ts
// =============================================================================

#[test]
fn test_weekday_mokuyoubi_thursday() {
    let p = parser();
    let ref_date = create_ref(2016, 9, 2, 12, 0, 0);
    test_single_case(&p, "木曜日", Some(ref_date), |r| {
        assert_eq!(r.text, "木曜日");
        assert_component(r, Component::Year, Some(2016));
        assert_component(r, Component::Month, Some(9));
        assert_component(r, Component::Day, Some(1));
        assert_eq!(
            r.start.get(Component::Weekday),
            Some(Weekday::Thursday as i32)
        );
    });
}

#[test]
fn test_weekday_mae_no_suiyoubi() {
    let p = parser();
    let ref_date = create_ref(2016, 9, 2, 12, 0, 0);
    test_single_case(&p, "前の水曜日", Some(ref_date), |r| {
        assert_eq!(r.text, "前の水曜日");
        assert_component(r, Component::Year, Some(2016));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(31));
        assert_eq!(
            r.start.get(Component::Weekday),
            Some(Weekday::Wednesday as i32)
        );
    });
}

#[test]
fn test_weekday_parentheses() {
    let p = parser();
    let ref_date = create_ref(2016, 9, 2, 12, 0, 0);
    test_single_case(&p, "(木)", Some(ref_date), |r| {
        assert_eq!(r.text, "(木)");
        assert_component(r, Component::Year, Some(2016));
        assert_component(r, Component::Month, Some(9));
        assert_component(r, Component::Day, Some(1));
        assert_eq!(
            r.start.get(Component::Weekday),
            Some(Weekday::Thursday as i32)
        );
    });
}

#[test]
fn test_weekday_fullwidth_parentheses() {
    let p = parser();
    let ref_date = create_ref(2016, 9, 2, 12, 0, 0);
    test_single_case(&p, "（木）", Some(ref_date), |r| {
        assert_eq!(r.text, "（木）");
        assert_component(r, Component::Year, Some(2016));
        assert_component(r, Component::Month, Some(9));
        assert_component(r, Component::Day, Some(1));
        assert_eq!(
            r.start.get(Component::Weekday),
            Some(Weekday::Thursday as i32)
        );
    });
}

#[test]
fn test_weekday_merge_date() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "8月27日水曜日", Some(ref_date), |r| {
        assert_eq!(r.text, "8月27日水曜日");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(27));
        assert_eq!(
            r.start.get(Component::Weekday),
            Some(Weekday::Wednesday as i32)
        );
    });
}

#[test]
fn test_weekday_merge_date_parentheses() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "8月27日（水）", Some(ref_date), |r| {
        assert_eq!(r.text, "8月27日（水）");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(27));
        assert_eq!(
            r.start.get(Component::Weekday),
            Some(Weekday::Wednesday as i32)
        );
    });
}

#[test]
fn test_weekday_slash_format() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "2012/8/27（水）", Some(ref_date), |r| {
        assert_eq!(r.text, "2012/8/27（水）");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(27));
        assert_eq!(
            r.start.get(Component::Weekday),
            Some(Weekday::Wednesday as i32)
        );
    });
}

#[test]
fn test_ashita_no_shogo() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "明日の正午", Some(ref_date), |r| {
        assert_eq!(r.text, "明日の正午");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(11));
        assert_component(r, Component::Hour, Some(12));
        assert_component(r, Component::Minute, Some(0));
    });
}

#[test]
fn test_raishuu_kinyoubi_gogo3ji() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "来週の金曜日の午後3時", Some(ref_date), |r| {
        assert_eq!(r.text, "来週の金曜日の午後3時");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(17));
        assert_component(r, Component::Hour, Some(15));
        assert_component(r, Component::Minute, Some(0));
        assert_eq!(
            r.start.get(Component::Weekday),
            Some(Weekday::Friday as i32)
        );
    });
}
