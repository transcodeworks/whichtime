//! French locale tests for whichtime-sys
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
    WhichTime::with_locale(Locale::Fr)
}

// =============================================================================
// Tests from fr_casual.test.ts
// =============================================================================

#[test]
fn test_casual_maintenant() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 8, 9, 10);
    test_single_case(&p, "La deadline est maintenant", Some(ref_date), |r| {
        assert_eq!(r.text, "maintenant");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(8));
        assert_component(r, Component::Minute, Some(9));
        assert_component(r, Component::Second, Some(10));
    });
}

#[test]
fn test_casual_aujourdhui() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "La deadline est aujourd'hui", Some(ref_date), |r| {
        assert_eq!(r.text, "aujourd'hui");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_casual_demain() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "La deadline est demain", Some(ref_date), |r| {
        assert_eq!(r.text, "demain");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(11));
    });
}

#[test]
fn test_casual_hier() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "La deadline était hier", Some(ref_date), |r| {
        assert_eq!(r.text, "hier");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(9));
    });
}

#[test]
fn test_casual_la_veille() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "La deadline était la veille", Some(ref_date), |r| {
        assert_eq!(r.text, "la veille");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(9));
        assert_component(r, Component::Hour, Some(0));
    });
}

#[test]
fn test_casual_ce_matin() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "La deadline est ce matin", Some(ref_date), |r| {
        assert_eq!(r.text, "ce matin");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(8));
    });
}

#[test]
fn test_casual_cet_apres_midi() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "La deadline est cet après-midi", Some(ref_date), |r| {
        assert_eq!(r.text, "cet après-midi");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(14));
    });
}

#[test]
fn test_casual_cet_aprem() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "La deadline est cet aprem", Some(ref_date), |r| {
        assert_eq!(r.text, "cet aprem");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(14));
    });
}

#[test]
fn test_casual_ce_soir() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "La deadline est ce soir", Some(ref_date), |r| {
        assert_eq!(r.text, "ce soir");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(18));
    });
}

#[test]
fn test_casual_a_midi() {
    let p = parser();
    test_single_case(&p, "a midi", None, |r| {
        assert_eq!(r.text, "a midi");
        assert_component(r, Component::Hour, Some(12));
    });
}

#[test]
fn test_casual_a_minuit() {
    let p = parser();
    test_single_case(&p, "à minuit", None, |r| {
        assert_eq!(r.text, "à minuit");
        assert_component(r, Component::Hour, Some(0));
    });
}

#[test]
fn test_casual_date_range() {
    let p = parser();
    test_single_case(&p, "Du 24 août 2023 au 26 août 2023", None, |r| {
        assert_eq!(r.text, "24 août 2023 au 26 août 2023");
        assert_component(r, Component::Year, Some(2023));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(24));
        if let Some(end) = &r.end {
            assert_eq!(end.get(Component::Year), Some(2023));
            assert_eq!(end.get(Component::Month), Some(8));
            assert_eq!(end.get(Component::Day), Some(26));
        }
    });
}

#[test]
fn test_casual_combined_aujourdhui_17h() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "La deadline est aujourd'hui 17:00",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "aujourd'hui 17:00");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(8));
            assert_component(r, Component::Day, Some(10));
            assert_component(r, Component::Hour, Some(17));
        },
    );
}

#[test]
fn test_casual_combined_demain_17h() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "La deadline est demain 17:00", Some(ref_date), |r| {
        assert_eq!(r.text, "demain 17:00");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(11));
        assert_component(r, Component::Hour, Some(17));
    });
}

#[test]
fn test_casual_demain_matin_11h() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "La deadline est demain matin 11h",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "demain matin 11h");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(8));
            assert_component(r, Component::Day, Some(11));
            assert_component(r, Component::Hour, Some(11));
        },
    );
}

// =============================================================================
// Tests from fr_little_endian.test.ts
// =============================================================================

#[test]
fn test_little_endian_basic() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "10 Août 2012", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_eq!(r.text, "10 Août 2012");
    });
}

#[test]
fn test_little_endian_forward_date() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "8 Février", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2013));
        assert_component(r, Component::Month, Some(2));
        assert_component(r, Component::Day, Some(8));
        assert_eq!(r.text, "8 Février");
    });
}

#[test]
fn test_little_endian_1er() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 1, 12, 0, 0);
    test_single_case(&p, "1er Août 2012", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(1));
        assert_eq!(r.text, "1er Août 2012");
    });
}

#[test]
fn test_little_endian_ac() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "10 Août 234 AC", Some(ref_date), |r| {
        assert_eq!(r.text, "10 Août 234 AC");
        assert_component(r, Component::Year, Some(-234));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_little_endian_pc() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "10 Août 88 p. Chr. n.", Some(ref_date), |r| {
        assert_eq!(r.text, "10 Août 88 p. Chr. n.");
        assert_component(r, Component::Year, Some(88));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_little_endian_abbrev() {
    let p = parser();
    let ref_date = create_ref(2013, 8, 10, 12, 0, 0);
    test_single_case(&p, "Dim 15 Sept", Some(ref_date), |r| {
        assert_eq!(r.text, "Dim 15 Sept");
        assert_component(r, Component::Year, Some(2013));
        assert_component(r, Component::Month, Some(9));
        assert_component(r, Component::Day, Some(15));
    });
}

#[test]
fn test_little_endian_abbrev_caps() {
    let p = parser();
    let ref_date = create_ref(2013, 8, 10, 12, 0, 0);
    test_single_case(&p, "DIM 15SEPT", Some(ref_date), |r| {
        assert_eq!(r.text, "DIM 15SEPT");
        assert_component(r, Component::Year, Some(2013));
        assert_component(r, Component::Month, Some(9));
        assert_component(r, Component::Day, Some(15));
    });
}

#[test]
fn test_little_endian_le_10_aout() {
    let p = parser();
    let ref_date = create_ref(2013, 8, 10, 12, 0, 0);
    test_single_case(&p, "La date limite est le 10 Août", Some(ref_date), |r| {
        assert_eq!(r.text, "10 Août");
        assert_component(r, Component::Year, Some(2013));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_little_endian_mardi_10_janvier() {
    let p = parser();
    let ref_date = create_ref(2013, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "La date limite est le Mardi 10 janvier",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "Mardi 10 janvier");
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
fn test_little_endian_mar_10_jan() {
    let p = parser();
    let ref_date = create_ref(2013, 8, 10, 12, 0, 0);
    test_single_case(&p, "La date limite est Mar 10 Jan", Some(ref_date), |r| {
        assert_eq!(r.text, "Mar 10 Jan");
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
fn test_little_endian_31_mars_2016() {
    let p = parser();
    let ref_date = create_ref(2013, 8, 10, 12, 0, 0);
    test_single_case(&p, "31 mars 2016", Some(ref_date), |r| {
        assert_eq!(r.text, "31 mars 2016");
        assert_component(r, Component::Year, Some(2016));
        assert_component(r, Component::Month, Some(3));
        assert_component(r, Component::Day, Some(31));
    });
}

#[test]
fn test_little_endian_range() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "10 - 22 août 2012", Some(ref_date), |r| {
        assert_eq!(r.text, "10 - 22 août 2012");
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
fn test_little_endian_range_au() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "10 au 22 août 2012", Some(ref_date), |r| {
        assert_eq!(r.text, "10 au 22 août 2012");
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
fn test_little_endian_range_cross_month() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "10 août - 12 septembre", Some(ref_date), |r| {
        assert_eq!(r.text, "10 août - 12 septembre");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        if let Some(end) = &r.end {
            assert_eq!(end.get(Component::Year), Some(2012));
            assert_eq!(end.get(Component::Month), Some(9));
            assert_eq!(end.get(Component::Day), Some(12));
        }
    });
}

#[test]
fn test_little_endian_range_cross_month_year() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "10 août - 12 septembre 2013", Some(ref_date), |r| {
        assert_eq!(r.text, "10 août - 12 septembre 2013");
        assert_component(r, Component::Year, Some(2013));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        if let Some(end) = &r.end {
            assert_eq!(end.get(Component::Year), Some(2013));
            assert_eq!(end.get(Component::Month), Some(9));
            assert_eq!(end.get(Component::Day), Some(12));
        }
    });
}

#[test]
fn test_little_endian_combined_time() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "12 juillet à 19:00", Some(ref_date), |r| {
        assert_eq!(r.text, "12 juillet à 19:00");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(7));
        assert_component(r, Component::Day, Some(12));
        assert_component(r, Component::Hour, Some(19));
        assert_component(r, Component::Minute, Some(0));
    });
}

#[test]
fn test_little_endian_combined_time_mai() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "5 mai 12:00", Some(ref_date), |r| {
        assert_eq!(r.text, "5 mai 12:00");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(5));
        assert_component(r, Component::Day, Some(5));
        assert_component(r, Component::Hour, Some(12));
        assert_component(r, Component::Minute, Some(0));
    });
}

#[test]
fn test_little_endian_impossible_dates() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_no_result(&p, "32 Août 2014", Some(ref_date));
    test_no_result(&p, "29 Février 2014", Some(ref_date));
    test_no_result(&p, "32 Aout", Some(ref_date));
    let ref_date = create_ref(2013, 8, 10, 12, 0, 0);
    test_no_result(&p, "29 Fevrier", Some(ref_date));
}

#[test]
fn test_demain_a_midi() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "demain à midi", Some(ref_date), |r| {
        assert_eq!(r.text, "demain à midi");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(11));
        assert_component(r, Component::Hour, Some(12));
        assert_component(r, Component::Minute, Some(0));
    });
}

#[test]
fn test_hier_a_18h() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "hier à 18h", Some(ref_date), |r| {
        assert_eq!(r.text, "hier à 18h");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(9));
        assert_component(r, Component::Hour, Some(18));
        assert_component(r, Component::Minute, Some(0));
    });
}

#[test]
fn test_vendredi_prochain_a_15h() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "vendredi prochain à 15h", Some(ref_date), |r| {
        assert_eq!(r.text, "vendredi prochain à 15h");
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

// =============================================================================
// Tests from fr_slash.test.ts
// =============================================================================

#[test]
fn test_slash_basic() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "8/2/2016", Some(ref_date), |r| {
        assert_eq!(r.text, "8/2/2016");
        assert_component(r, Component::Year, Some(2016));
        assert_component(r, Component::Month, Some(2));
        assert_component(r, Component::Day, Some(8));
    });
}

#[test]
fn test_slash_le() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "le 8/2/2016", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2016));
        assert_component(r, Component::Month, Some(2));
        assert_component(r, Component::Day, Some(8));
    });
}

#[test]
fn test_slash_short() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "le 8/2", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2013));
        assert_component(r, Component::Month, Some(2));
        assert_component(r, Component::Day, Some(8));
    });
}

#[test]
fn test_slash_with_weekday_lundi() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "lundi 8/2/2016", Some(ref_date), |r| {
        assert_eq!(r.text, "lundi 8/2/2016");
        assert_component(r, Component::Year, Some(2016));
        assert_component(r, Component::Month, Some(2));
        assert_component(r, Component::Day, Some(8));
    });
}

#[test]
fn test_slash_with_weekday_samedi() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "samedi 9/2/20 ", Some(ref_date), |r| {
        assert_eq!(r.text, "samedi 9/2/20");
        assert_component(r, Component::Year, Some(2020));
        assert_component(r, Component::Month, Some(2));
        assert_component(r, Component::Day, Some(9));
    });
}

// =============================================================================
// Tests from fr_time_exp.test.ts
// =============================================================================

#[test]
fn test_time_expression_8h10() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "8h10", Some(ref_date), |r| {
        assert_eq!(r.text, "8h10");
        assert_component(r, Component::Hour, Some(8));
        assert_component(r, Component::Minute, Some(10));
    });
}

#[test]
fn test_time_expression_8h10m() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "8h10m", Some(ref_date), |r| {
        assert_eq!(r.text, "8h10m");
        assert_component(r, Component::Hour, Some(8));
        assert_component(r, Component::Minute, Some(10));
    });
}

#[test]
fn test_time_expression_pm() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "8:10 PM", Some(ref_date), |r| {
        assert_eq!(r.text, "8:10 PM");
        assert_component(r, Component::Hour, Some(20));
        assert_component(r, Component::Minute, Some(10));
    });
}

#[test]
fn test_time_expression_range() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "8:10 - 12.32", Some(ref_date), |r| {
        assert_eq!(r.text, "8:10 - 12.32");
        assert_component(r, Component::Hour, Some(8));
        assert_component(r, Component::Minute, Some(10));
        if let Some(end) = &r.end {
            assert_eq!(end.get(Component::Hour), Some(12));
            assert_eq!(end.get(Component::Minute), Some(32));
        }
    });
}

#[test]
fn test_time_expression_range_pm() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, " de 6:30pm à 11:00pm ", Some(ref_date), |r| {
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
fn test_time_expression_impossible() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_no_result(&p, "8:62", Some(ref_date));
    test_no_result(&p, "25:12", Some(ref_date));
    test_no_result(&p, "12h12:99s", Some(ref_date));
    test_no_result(&p, "13.12 PM", Some(ref_date));
}

// =============================================================================
// Tests from fr_time_units_ago.test.ts
// =============================================================================

#[test]
fn test_time_units_ago_5_jours() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "il y a 5 jours, on a fait quelque chose",
        Some(ref_date),
        |r| {
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(8));
            assert_component(r, Component::Day, Some(5));
            assert_eq!(r.text, "il y a 5 jours");
        },
    );
}

#[test]
fn test_time_units_ago_10_jours() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 13, 30, 0);
    test_single_case(
        &p,
        "il y a 10 jours, on a fait quelque chose",
        Some(ref_date),
        |r| {
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(7));
            assert_component(r, Component::Day, Some(31));
            assert_eq!(r.text, "il y a 10 jours");
        },
    );
}

#[test]
fn test_time_units_ago_15_minutes() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 14, 0);
    test_single_case(&p, "il y a 15 minutes", Some(ref_date), |r| {
        assert_eq!(r.text, "il y a 15 minutes");
        assert_component(r, Component::Hour, Some(11));
        assert_component(r, Component::Minute, Some(59));
    });
}

#[test]
fn test_time_units_ago_12_heures() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 14, 0);
    test_single_case(&p, "   il y a    12 heures", Some(ref_date), |r| {
        assert_eq!(r.text, "il y a    12 heures");
        assert_component(r, Component::Hour, Some(0));
        assert_component(r, Component::Minute, Some(14));
    });
}

// =============================================================================
// Tests from fr_time_units_casual_relative.test.ts
// =============================================================================

#[test]
fn test_time_units_casual_relative_semaine_prochaine() {
    let p = parser();
    let ref_date = create_ref(2017, 5, 12, 0, 0, 0);
    test_single_case(&p, "la semaine prochaine", Some(ref_date), |r| {
        assert_eq!(r.text, "la semaine prochaine");
        assert_component(r, Component::Year, Some(2017));
        assert_component(r, Component::Month, Some(5));
        assert_component(r, Component::Day, Some(19));
    });
}

#[test]
fn test_time_units_casual_relative_2_prochaines_semaines() {
    let p = parser();
    let ref_date = create_ref(2017, 5, 12, 18, 11, 0);
    test_single_case(&p, "les 2 prochaines semaines", Some(ref_date), |r| {
        assert_eq!(r.text, "les 2 prochaines semaines");
        assert_component(r, Component::Year, Some(2017));
        assert_component(r, Component::Month, Some(5));
        assert_component(r, Component::Day, Some(26));
        assert_component(r, Component::Hour, Some(18));
        assert_component(r, Component::Minute, Some(11));
    });
}

#[test]
fn test_time_units_casual_relative_trois_prochaines_semaines() {
    let p = parser();
    let ref_date = create_ref(2017, 5, 12, 18, 11, 0);
    test_single_case(&p, "les trois prochaines semaines", Some(ref_date), |r| {
        assert_eq!(r.text, "les trois prochaines semaines");
        assert_component(r, Component::Year, Some(2017));
        assert_component(r, Component::Month, Some(6));
        assert_component(r, Component::Day, Some(2));
    });
}

#[test]
fn test_time_units_casual_relative_mois_dernier() {
    let p = parser();
    let ref_date = create_ref(2017, 5, 12, 0, 0, 0);
    test_single_case(&p, "le mois dernier", Some(ref_date), |r| {
        assert_eq!(r.text, "le mois dernier");
        assert_component(r, Component::Year, Some(2017));
        assert_component(r, Component::Month, Some(4));
        assert_component(r, Component::Day, Some(12));
    });
}

// =============================================================================
// Tests from fr_time_units_within.test.ts
// =============================================================================

#[test]
fn test_time_units_within_dans_5_jours() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "On doit faire quelque chose dans 5 jours.",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "dans 5 jours");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(8));
            assert_component(r, Component::Day, Some(15));
        },
    );
}

#[test]
fn test_time_units_within_dans_5_minutes() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 14, 0);
    test_single_case(&p, "dans 5 minutes", Some(ref_date), |r| {
        assert_eq!(r.text, "dans 5 minutes");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(12));
        assert_component(r, Component::Minute, Some(19));
    });
}

#[test]
fn test_time_units_within_pour_5_minutes() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 14, 0);
    test_single_case(&p, "pour 5 minutes", Some(ref_date), |r| {
        assert_eq!(r.text, "pour 5 minutes");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(12));
        assert_component(r, Component::Minute, Some(19));
    });
}

#[test]
fn test_time_units_within_en_1_heure() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 14, 0);
    test_single_case(&p, "en 1 heure", Some(ref_date), |r| {
        assert_eq!(r.text, "en 1 heure");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(13));
        assert_component(r, Component::Minute, Some(14));
    });
}

// =============================================================================
// Tests from fr_weekday.test.ts
// =============================================================================

#[test]
fn test_weekday_lundi() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 9, 12, 0, 0);
    test_single_case(&p, "Lundi", Some(ref_date), |r| {
        assert_eq!(r.text, "Lundi");
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
fn test_weekday_jeudi() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 9, 12, 0, 0);
    test_single_case(&p, "Jeudi", Some(ref_date), |r| {
        assert_eq!(r.text, "Jeudi");
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
fn test_weekday_dimanche() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 9, 12, 0, 0);
    test_single_case(&p, "Dimanche", Some(ref_date), |r| {
        assert_eq!(r.text, "Dimanche");
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
fn test_weekday_vendredi_dernier() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 9, 12, 0, 0);
    test_single_case(
        &p,
        "la deadline était vendredi dernier...",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "vendredi dernier");
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
fn test_weekday_vendredi_prochain() {
    let p = parser();
    let ref_date = create_ref(2015, 4, 18, 12, 0, 0);
    test_single_case(
        &p,
        "Planifions une réunion vendredi prochain",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "vendredi prochain");
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
fn test_weekday_overlap_dimanche_7_decembre() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 9, 12, 0, 0);
    test_single_case(&p, "Dimanche 7 décembre 2014", Some(ref_date), |r| {
        assert_eq!(r.text, "Dimanche 7 décembre 2014");
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
fn test_weekday_overlap_dimanche_slash() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 9, 12, 0, 0);
    test_single_case(&p, "Dimanche 7/12/2014", Some(ref_date), |r| {
        assert_eq!(r.text, "Dimanche 7/12/2014");
        assert_component(r, Component::Year, Some(2014));
        assert_component(r, Component::Month, Some(12));
        assert_component(r, Component::Day, Some(7));
        assert_eq!(
            r.start.get(Component::Weekday),
            Some(Weekday::Sunday as i32)
        );
    });
}
