//! Spanish locale tests for whichtime-sys
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
    WhichTime::with_locale(Locale::Es)
}

// =============================================================================
// Tests from es_casual.test.ts
// =============================================================================

#[test]
fn test_casual_ahora() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 8, 9, 10);
    test_single_case(&p, "La fecha límite es ahora", Some(ref_date), |r| {
        assert_eq!(r.text, "ahora");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(8));
        assert_component(r, Component::Minute, Some(9));
        assert_component(r, Component::Second, Some(10));
    });
}

#[test]
fn test_casual_hoy() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "La fecha límite es hoy", Some(ref_date), |r| {
        assert_eq!(r.text, "hoy");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_casual_manana() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "La fecha límite es Mañana", Some(ref_date), |r| {
        assert_eq!(r.text, "Mañana");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(11));
    });
}

#[test]
fn test_casual_ayer() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "La fecha límite fue ayer", Some(ref_date), |r| {
        assert_eq!(r.text, "ayer");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(9));
    });
}

#[test]
fn test_casual_ayer_de_noche() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "La fecha límite fue ayer de noche",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "ayer de noche");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(8));
            assert_component(r, Component::Day, Some(9));
            assert_component(r, Component::Hour, Some(22));
        },
    );
}

#[test]
fn test_casual_esta_manana() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "La fecha límite fue esta mañana",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "esta mañana");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(8));
            assert_component(r, Component::Day, Some(10));
            assert_component(r, Component::Hour, Some(6));
        },
    );
}

#[test]
fn test_casual_esta_tarde() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "La fecha límite fue esta tarde", Some(ref_date), |r| {
        assert_eq!(r.text, "esta tarde");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(15));
    });
}

#[test]
fn test_casual_combined_hoy_5pm() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "La fecha límite es hoy a las 5PM",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "hoy a las 5PM");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(8));
            assert_component(r, Component::Day, Some(10));
            assert_component(r, Component::Hour, Some(17));
        },
    );
}

#[test]
fn test_casual_esta_noche() {
    let p = parser();
    let ref_date = create_ref(2012, 1, 1, 12, 0, 0);
    test_single_case(&p, "esta noche", Some(ref_date), |r| {
        assert_eq!(r.text, "esta noche");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(1));
        assert_component(r, Component::Day, Some(1));
        assert_component(r, Component::Hour, Some(22));
        assert_eq!(r.start.get(Component::Meridiem), Some(Meridiem::PM as i32));
    });
}

#[test]
fn test_casual_esta_noche_8pm() {
    let p = parser();
    let ref_date = create_ref(2012, 1, 1, 12, 0, 0);
    test_single_case(&p, "esta noche 8pm", Some(ref_date), |r| {
        assert_eq!(r.text, "esta noche 8pm");
        assert_component(r, Component::Hour, Some(20));
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(1));
        assert_component(r, Component::Day, Some(1));
        assert_eq!(r.start.get(Component::Meridiem), Some(Meridiem::PM as i32));
    });
}

#[test]
fn test_casual_esta_noche_a_las_8() {
    let p = parser();
    let ref_date = create_ref(2012, 1, 1, 12, 0, 0);
    test_single_case(&p, "esta noche a las 8", Some(ref_date), |r| {
        assert_eq!(r.text, "esta noche a las 8");
        assert_component(r, Component::Hour, Some(20));
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(1));
        assert_component(r, Component::Day, Some(1));
        assert_eq!(r.start.get(Component::Meridiem), Some(Meridiem::PM as i32));
    });
}

#[test]
fn test_casual_jueves() {
    let p = parser();
    test_single_case(&p, "jueves", None, |r| {
        assert_eq!(r.text, "jueves");
        assert_eq!(
            r.start.get(Component::Weekday),
            Some(Weekday::Thursday as i32)
        );
    });
}

#[test]
fn test_casual_viernes() {
    let p = parser();
    test_single_case(&p, "viernes", None, |r| {
        assert_eq!(r.text, "viernes");
        assert_eq!(
            r.start.get(Component::Weekday),
            Some(Weekday::Friday as i32)
        );
    });
}

#[test]
fn test_casual_el_mediodia() {
    let p = parser();
    let ref_date = create_ref(2020, 9, 1, 11, 0, 0);
    test_single_case(&p, "el mediodía", Some(ref_date), |r| {
        assert_component(r, Component::Hour, Some(12));
        assert_component(r, Component::Year, Some(2020));
        assert_component(r, Component::Month, Some(9));
        assert_component(r, Component::Day, Some(1));
    });
}

#[test]
fn test_casual_la_medianoche() {
    let p = parser();
    let ref_date = create_ref(2020, 9, 1, 11, 0, 0);
    test_single_case(&p, "la medianoche", Some(ref_date), |r| {
        assert_component(r, Component::Hour, Some(0));
        assert_component(r, Component::Year, Some(2020));
        assert_component(r, Component::Month, Some(9));
        assert_component(r, Component::Day, Some(2));
    });
}

#[test]
fn test_casual_negative_cases() {
    let p = parser();
    test_no_result(&p, "nohoy", None);
    test_no_result(&p, "hymañana", None);
    test_no_result(&p, "xayer", None);
    test_no_result(&p, "porhora", None);
    test_no_result(&p, "ahoraxsd", None);
}

// =============================================================================
// Tests from es_month_name_little_endian.test.ts
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
    test_single_case(&p, "Dom 15Sep", Some(ref_date), |r| {
        assert_eq!(r.text, "Dom 15Sep");
        assert_component(r, Component::Year, Some(2013));
        assert_component(r, Component::Month, Some(9));
        assert_component(r, Component::Day, Some(15));
    });
}

#[test]
fn test_month_name_little_endian_abbrev_caps() {
    let p = parser();
    let ref_date = create_ref(2013, 8, 10, 12, 0, 0);
    test_single_case(&p, "DOM 15SEP", Some(ref_date), |r| {
        assert_eq!(r.text, "DOM 15SEP");
        assert_component(r, Component::Year, Some(2013));
        assert_component(r, Component::Month, Some(9));
        assert_component(r, Component::Day, Some(15));
    });
}

#[test]
fn test_month_name_little_endian_es() {
    let p = parser();
    let ref_date = create_ref(2013, 8, 10, 12, 0, 0);
    test_single_case(&p, "La fecha límite es 10 Agosto", Some(ref_date), |r| {
        assert_eq!(r.text, "10 Agosto");
        assert_component(r, Component::Year, Some(2013));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_month_name_little_endian_weekday_martes() {
    let p = parser();
    let ref_date = create_ref(2013, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "La fecha límite es el martes, 10 de enero",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "martes, 10 de enero");
            assert_component(r, Component::Year, Some(2013));
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
fn test_month_name_little_endian_weekday_miercoles() {
    let p = parser();
    let ref_date = create_ref(2013, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "La fecha límite es el miércoles, 10 de enero",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "miércoles, 10 de enero");
            assert_component(r, Component::Year, Some(2013));
            assert_component(r, Component::Month, Some(1));
            assert_component(r, Component::Day, Some(10));
            assert_eq!(
                r.start.get(Component::Weekday),
                Some(Weekday::Wednesday as i32)
            );
        },
    );
}

#[test]
fn test_month_name_little_endian_de() {
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
    test_single_case(&p, "10 Agosto - 12 Septiembre", Some(ref_date), |r| {
        assert_eq!(r.text, "10 Agosto - 12 Septiembre");
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
    test_single_case(&p, "10 Agosto - 12 Septiembre 2013", Some(ref_date), |r| {
        assert_eq!(r.text, "10 Agosto - 12 Septiembre 2013");
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
    test_single_case(&p, "12 de julio a las 19:00", Some(ref_date), |r| {
        assert_eq!(r.text, "12 de julio a las 19:00");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(7));
        assert_component(r, Component::Day, Some(12));
        assert_component(r, Component::Hour, Some(19));
        assert_component(r, Component::Minute, Some(0));
    });
}

// =============================================================================
// Tests from es_slash.test.ts
// =============================================================================

#[test]
fn test_slash_lunes() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "lunes 8/2/2016", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2016));
        assert_component(r, Component::Month, Some(2));
        assert_component(r, Component::Day, Some(8));
        assert_eq!(r.text, "lunes 8/2/2016");
    });
}

#[test]
fn test_slash_martes() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "Martes 9/2/2016", Some(ref_date), |r| {
        assert_component(r, Component::Year, Some(2016));
        assert_component(r, Component::Month, Some(2));
        assert_component(r, Component::Day, Some(9));
        assert_eq!(r.text, "Martes 9/2/2016");
    });
}

// =============================================================================
// Tests from es_time_exp.test.ts
// =============================================================================

#[test]
fn test_time_expression_single() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "Estaremos a las 6.13 AM", Some(ref_date), |r| {
        assert_eq!(r.text, "las 6.13 AM");
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
    test_single_case(&p, "de 6:30pm a 11:00pm", Some(ref_date), |r| {
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
        "Algo pasó el 10 de Agosto de 2012 10:12:59 pm",
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

// =============================================================================
// Tests from es_time_units_within.test.ts
// =============================================================================

#[test]
fn test_time_units_within_en_5_dias() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(
        &p,
        "Tenemos que hacer algo en 5 días.",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "en 5 días");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(8));
            assert_component(r, Component::Day, Some(15));
        },
    );
}

#[test]
fn test_time_units_within_en_cinco_dias() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 11, 12, 0);
    test_single_case(
        &p,
        "Tenemos que hacer algo en cinco días.",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "en cinco días");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(8));
            assert_component(r, Component::Day, Some(15));
            assert_component(r, Component::Hour, Some(11));
            assert_component(r, Component::Minute, Some(12));
        },
    );
}

#[test]
fn test_time_units_within_en_5_minutos() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 14, 0);
    test_single_case(&p, "en 5 minutos", Some(ref_date), |r| {
        assert_eq!(r.text, "en 5 minutos");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(12));
        assert_component(r, Component::Minute, Some(19));
    });
}

#[test]
fn test_time_units_within_por_5_minutos() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 14, 0);
    test_single_case(&p, "por 5 minutos", Some(ref_date), |r| {
        assert_eq!(r.text, "por 5 minutos");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(12));
        assert_component(r, Component::Minute, Some(19));
    });
}

#[test]
fn test_time_units_within_en_1_hora() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 14, 0);
    test_single_case(&p, "en 1 hora", Some(ref_date), |r| {
        assert_eq!(r.text, "en 1 hora");
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
        "establecer un temporizador de 5 minutos",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "de 5 minutos");
            assert_component(r, Component::Year, Some(2012));
            assert_component(r, Component::Month, Some(8));
            assert_component(r, Component::Day, Some(10));
            assert_component(r, Component::Hour, Some(12));
            assert_component(r, Component::Minute, Some(19));
        },
    );
}

#[test]
fn test_time_units_within_en_5_minutos_sentence() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 14, 0);
    test_single_case(&p, "En 5 minutos me voy a casa", Some(ref_date), |r| {
        assert_eq!(r.text, "En 5 minutos");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
        assert_component(r, Component::Hour, Some(12));
        assert_component(r, Component::Minute, Some(19));
    });
}

#[test]
fn test_time_units_within_en_5_segundos() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 14, 0);
    test_single_case(
        &p,
        "En 5 segundos un auto se moverá",
        Some(ref_date),
        |r| {
            assert_eq!(r.text, "En 5 segundos");
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
fn test_time_units_within_en_dos_semanas() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 14, 0);
    test_single_case(&p, "en dos semanas", Some(ref_date), |r| {
        assert_eq!(r.text, "en dos semanas");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(24));
    });
}

#[test]
fn test_time_units_within_dentro_de_un_mes() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 14, 0);
    test_single_case(&p, "dentro de un mes", Some(ref_date), |r| {
        assert_eq!(r.text, "dentro de un mes");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(9));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_time_units_within_en_algunos_meses() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 14, 0);
    test_single_case(&p, "en algunos meses", Some(ref_date), |r| {
        assert_eq!(r.text, "en algunos meses");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(11));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_time_units_within_en_un_ano() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 14, 0);
    test_single_case(&p, "en un año", Some(ref_date), |r| {
        assert_eq!(r.text, "en un año");
        assert_component(r, Component::Year, Some(2013));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_time_units_within_dentro_de_un_ano() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 14, 0);
    test_single_case(&p, "dentro de un año", Some(ref_date), |r| {
        assert_eq!(r.text, "dentro de un año");
        assert_component(r, Component::Year, Some(2013));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(10));
    });
}

#[test]
fn test_manana_a_mediodia() {
    let p = parser();
    let ref_date = create_ref(2012, 8, 10, 12, 0, 0);
    test_single_case(&p, "mañana a mediodía", Some(ref_date), |r| {
        assert_eq!(r.text, "mañana a mediodía");
        assert_component(r, Component::Year, Some(2012));
        assert_component(r, Component::Month, Some(8));
        assert_component(r, Component::Day, Some(11));
        assert_component(r, Component::Hour, Some(12));
        assert_component(r, Component::Minute, Some(0));
    });
}
