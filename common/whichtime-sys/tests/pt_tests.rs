//! Portuguese locale tests for whichtime-sys
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
    WhichTime::with_locale(Locale::Pt)
}

// =============================================================================
// Tests from pt_casual.test.ts
// =============================================================================

#[test]
fn test_casual_agora() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 8, 9, 10);
    test_single_case(&p, "O prazo é agora", Some(ref_date), |r| {
        assert_eq!(r.text, "agora");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(8));
        assert_component(r, Component::Minute, Some(9));
        assert_component(r, Component::Second, Some(10));
    });
}

#[test]
fn test_casual_hoje() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "O prazo é hoje", Some(ref_date), |r| {
        assert_eq!(r.text, "hoje");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_casual_amanha() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "O prazo é Amanhã", Some(ref_date), |r| {
        assert_eq!(r.text, "Amanhã");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(11));
    });
}

#[test]
fn test_casual_ontem() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "O prazo foi ontem", Some(ref_date), |r| {
        assert_eq!(r.text, "ontem");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(9));
    });
}

#[test]
fn test_casual_ontem_a_noite() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "O prazo foi ontem à noite ", Some(ref_date), |r| {
        assert_eq!(r.text, "ontem à noite");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(9));
        assert_component(r, Component::Hour, Some(22));
    });
}

#[test]
fn test_casual_esta_manha() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "O prazo foi esta manhã ", Some(ref_date), |r| {
        assert_eq!(r.text, "esta manhã");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(6));
    });
}

#[test]
fn test_casual_esta_tarde() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "O prazo foi esta tarde ", Some(ref_date), |r| {
        assert_eq!(r.text, "esta tarde");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(15));
    });
}

#[test]
fn test_casual_combined_hoje_5pm() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "O prazo é hoje às 5PM", Some(ref_date), |r| {
        assert_eq!(r.text, "hoje às 5PM");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(17));
    });
}

#[test]
fn test_casual_esta_noite() {
    let p = parser();
    let ref_date = create_ref(2012, 1, 1, 12, 0, 0);
    test_single_case(&p, "esta noite", Some(ref_date), |r| {
        assert_eq!(r.text, "esta noite");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(1));
        assert_component(r, Component::Day, Some(1));
        assert_component(r, Component::Hour, Some(22));
        assert_eq!(r.start.get(Component::Meridiem), Some(Meridiem::PM as i32));
    });
}

#[test]
fn test_casual_esta_noite_8pm() {
    let p = parser();
    let ref_date = create_ref(2012, 1, 1, 12, 0, 0);
    test_single_case(&p, "esta noite 8pm", Some(ref_date), |r| {
        assert_eq!(r.text, "esta noite 8pm");
        assert_component(r, Component::Hour, Some(20));
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(1));
        assert_component(r, Component::Day, Some(1));
        assert_eq!(r.start.get(Component::Meridiem), Some(Meridiem::PM as i32));
    });
}

#[test]
fn test_casual_esta_noite_as_8() {
    let p = parser();
    let ref_date = create_ref(2012, 1, 1, 12, 0, 0);
    test_single_case(&p, "esta noite às 8", Some(ref_date), |r| {
        assert_eq!(r.text, "esta noite às 8");
        assert_component(r, Component::Hour, Some(20));
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(1));
        assert_component(r, Component::Day, Some(1));
        assert_eq!(r.start.get(Component::Meridiem), Some(Meridiem::PM as i32));
    });
}

#[test]
fn test_casual_quinta() {
    let p = parser();
    test_single_case(&p, "quinta", None, |r| {
        assert_eq!(r.text, "quinta");
        assert_eq!(
            r.start.get(Component::Weekday),
            Some(Weekday::Thursday as i32)
        );
    });
}

#[test]
fn test_casual_sexta() {
    let p = parser();
    test_single_case(&p, "sexta", None, |r| {
        assert_eq!(r.text, "sexta");
        assert_eq!(
            r.start.get(Component::Weekday),
            Some(Weekday::Friday as i32)
        );
    });
}

#[test]
fn test_casual_ao_meio_dia() {
    let p = parser();
    let ref_date = create_ref(2020, 9, 1, 11, 0, 0);
    test_single_case(&p, "ao meio-dia", Some(ref_date), |r| {
        assert_component(r, Component::Hour, Some(12));
        assert_component(r, Component::Year, Some(2020));
        assert_component(r, Component::Month, Some(9));
        assert_component(r, Component::Day, Some(1));
    });
}

#[test]
fn test_casual_a_meia_noite() {
    let p = parser();
    let ref_date = create_ref(2020, 9, 1, 11, 0, 0);
    test_single_case(&p, "a meia-noite", Some(ref_date), |r| {
        assert_component(r, Component::Hour, Some(0));
        assert_component(r, Component::Year, Some(2020));
        assert_component(r, Component::Month, Some(9));
        assert_component(r, Component::Day, Some(2));
    });
}

#[test]
fn test_casual_negative_cases() {
    let p = parser();
    test_no_result(&p, "naohoje", None);
    test_no_result(&p, "hyamanhã", None);
    test_no_result(&p, "xontem", None);
    test_no_result(&p, "porhora", None);
    test_no_result(&p, "agoraxsd", None);
}

// =============================================================================
// Tests from pt_month_name_little_endian.test.ts
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
fn test_month_name_little_endian_ac() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "10 Agosto 234 AC", Some(ref_date), |r| {
        assert_eq!(r.text, "10 Agosto 234 AC");
        assert_component(r, Component::Year, Some(-234));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_month_name_little_endian_dc() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "10 Agosto 88 d. C.", Some(ref_date), |r| {
        assert_eq!(r.text, "10 Agosto 88 d. C.");
        assert_component(r, Component::Year, Some(88));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_month_name_little_endian_abbrev() {
    let p = parser();
    let ref_date = create_ref(2013, 8, 10, 12, 0, 0);
    test_single_case(&p, "Dom 15Set", Some(ref_date), |r| {
        assert_eq!(r.text, "Dom 15Set");
        assert_component(r, Component::Year, Some(2013));
        assert_component(r, Component::Month, Some(9));
        assert_component(r, Component::Day, Some(15));
    });
}

#[test]
fn test_month_name_little_endian_abbrev_caps() {
    let p = parser();
    let ref_date = create_ref(2013, 8, 10, 12, 0, 0);
    test_single_case(&p, "DOM 15SET", Some(ref_date), |r| {
        assert_eq!(r.text, "DOM 15SET");
        assert_component(r, Component::Year, Some(2013));
        assert_component(r, Component::Month, Some(9));
        assert_component(r, Component::Day, Some(15));
    });
}

#[test]
fn test_month_name_little_endian_prazo() {
    let p = parser();
    let ref_date = create_ref(2013, 8, 10, 12, 0, 0);
    test_single_case(&p, "O prazo é 10 Agosto", Some(ref_date), |r| {
        assert_eq!(r.text, "10 Agosto");
        assert_component(r, Component::Year, Some(2013));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_month_name_little_endian_terça_feira() {
    let p = parser();
    let ref_date = create_ref(2013, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "O prazo é terça-feira, 10 de janeiro",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "terça-feira, 10 de janeiro");
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
fn test_month_name_little_endian_qua() {
    let p = parser();
    let ref_date = create_ref(2013, 8, 10, 12, 0, 0);
    test_single_case(&p, "O prazo é Qua, 10 Janeiro", Some(ref_date), |r| {
        assert_eq!(r.text, "Qua, 10 Janeiro");
        assert_component(r, Component::Year, Some(2014));
        assert_component(r, Component::Month, Some(1));
        assert_component(r, Component::Day, Some(10));
        assert_eq!(
            r.start.get(Component::Weekday),
            Some(Weekday::Wednesday as i32)
        );
    });
}

#[test]
fn test_month_name_little_endian_de_de() {
    let p = parser();
    let ref_date = create_ref(2010, 2, 1, 12, 0, 0);
    test_single_case(&p, "10 de Agosto de 2012", Some(ref_date), |r| {
        assert_eq!(r.text, "10 de Agosto de 2012");
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
fn test_month_name_little_endian_range_a() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "10 a 22 Agosto 2012", Some(ref_date), |r| {
        assert_eq!(r.text, "10 a 22 Agosto 2012");
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
fn test_month_name_little_endian_range_cross_month() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "10 Agosto - 12 Setembro", Some(ref_date), |r| {
        assert_eq!(r.text, "10 Agosto - 12 Setembro");
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
fn test_month_name_little_endian_range_cross_month_year() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "10 Agosto - 12 Setembro 2013", Some(ref_date), |r| {
        assert_eq!(r.text, "10 Agosto - 12 Setembro 2013");
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
fn test_month_name_little_endian_combined_time() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "12 de Julho às 19:00", Some(ref_date), |r| {
        assert_eq!(r.text, "12 de Julho às 19:00");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(7));
        assert_component(r, Component::Day, Some(12));
        assert_component(r, Component::Hour, Some(19));
        assert_component(r, Component::Minute, Some(0));
    });
}

// =============================================================================
// Tests from pt_slash.test.ts
// =============================================================================

#[test]
fn test_slash_segunda() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "segunda 8/2/2016", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2016));
        assert_component(r, Component::Month, Some(2));
        assert_component(r, Component::Day, Some(8));
        assert_eq!(r.text, "segunda 8/2/2016");
    });
}

#[test]
fn test_slash_terça_feira() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "Terça-feira 9/2/2016", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2016));
        assert_component(r, Component::Month, Some(2));
        assert_component(r, Component::Day, Some(9));
        assert_eq!(r.text, "Terça-feira 9/2/2016");
    });
}

// =============================================================================
// Tests from pt_time_exp.test.ts
// =============================================================================

#[test]
fn test_time_expression_as_6_13_am() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "Ficaremos às 6.13 AM", Some(ref_date), |r| {
        assert_eq!(r.text, "às 6.13 AM");
        assert_component(r, Component::Hour, Some(6));
        assert_component(r, Component::Minute, Some(13));
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
    test_single_case(&p, " de 6:30pm a 11:00pm ", Some(ref_date), |r| {
        assert_eq!(r.text, "de 6:30pm a 11:00pm");
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
fn test_time_expression_date_time() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "Algo passou em 10 de Agosto de 2012 10:12:59 pm",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "10 de Agosto de 2012 10:12:59 pm");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(8));
            assert_component(r, Component::Day, Some(10));
            assert_component(r, Component::Hour, Some(22));
            assert_component(r, Component::Minute, Some(12));
            assert_component(r, Component::Second, Some(59));
        },
    );
}

#[test]
fn test_time_expression_meridiem_imply() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "de 1pm a 3", Some(ref_date), |r| {
        assert_eq!(r.text, "de 1pm a 3");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(13));
        assert_component(r, Component::Minute, Some(0));
        assert_eq!(r.start.get(Component::Meridiem), Some(Meridiem::PM as i32));
        if let Some(end) = &r.end {
            assert_eq!(end.get(Component::Year), Some(2012));
            assert_eq!(end.get(Component::Month), Some(8));
            assert_eq!(end.get(Component::Day), Some(10));
            assert_eq!(end.get(Component::Hour), Some(15));
            assert_eq!(end.get(Component::Minute), Some(0));
            assert_eq!(end.get(Component::Meridiem), Some(Meridiem::PM as i32));
        }
    });
}

#[test]
fn test_amanha_ao_meio_dia() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "amanhã ao meio-dia", Some(ref_date), |r| {
        assert_eq!(r.text, "amanhã ao meio-dia");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(11));
        assert_component(r, Component::Hour, Some(12));
        assert_component(r, Component::Minute, Some(0));
    });
}
