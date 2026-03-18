//! Benchmark for whichtime-sys

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use std::time::Duration;

// Test inputs - same as the original benchmark
const TEST_INPUTS: &[&str] = &[
    "2014-08-17 10:00",
    "08/17/2014 10:00 AM",
    "17 Aug 2014 10:00",
    "August 17th, 2014 10:00",
    "2025-01-01T00:00:00Z",
    "2025-01-01T23:59:59+02:00",
    "tomorrow at noon",
    "yesterday 5pm",
    "in 2 hours",
    "in 3 days",
    "2 weeks from now",
    "last Monday at 9:30",
    "this evening at 7",
    "next week",
    "last week",
    "next month",
    "next year",
    "next friday at 3pm",
    "first of august last year",
    "midnight",
    "noon",
];

// Additional stress test inputs
const LONG_TEXT: &str = "Meeting notes from Q3 planning session. We discussed the roadmap for the \
    next quarter and decided to schedule the launch for next Tuesday at 3pm. The follow-up \
    meeting will be on December 15, 2024 at 10:00 AM EST. Please review the documents by \
    tomorrow evening and submit feedback within 2 weeks.";

fn benchmark_simple(c: &mut Criterion) {
    let parser = whichtime_sys::WhichTime::new();

    c.bench_function("core_simple_inputs", |b| {
        b.iter(|| {
            for input in TEST_INPUTS {
                let _ = black_box(parser.parse_date(black_box(input), None));
            }
        });
    });
}

fn benchmark_long_text(c: &mut Criterion) {
    let parser = whichtime_sys::WhichTime::new();

    c.bench_function("core_long_text", |b| {
        b.iter(|| {
            let _ = black_box(parser.parse(black_box(LONG_TEXT), None));
        });
    });
}

fn benchmark_pathological(c: &mut Criterion) {
    let parser = whichtime_sys::WhichTime::new();

    // Pathological input that could cause regex backtracking
    let pathological = "111111111111111111111111111111 PM";

    let mut group = c.benchmark_group("pathological");
    group.measurement_time(Duration::from_secs(3));

    group.bench_function("core", |b| {
        b.iter(|| black_box(parser.parse(black_box(pathological), None)));
    });

    group.finish();
}

fn benchmark_locales(c: &mut Criterion) {
    use whichtime_sys::Locale;

    let mut group = c.benchmark_group("locales");
    group.measurement_time(Duration::from_secs(3));

    // English
    let en_parser = whichtime_sys::WhichTime::with_locale(Locale::En);
    group.bench_function("en_tomorrow", |b| {
        b.iter(|| black_box(en_parser.parse_date(black_box("tomorrow at 3pm"), None)));
    });

    // German
    let de_parser = whichtime_sys::WhichTime::with_locale(Locale::De);
    group.bench_function("de_morgen", |b| {
        b.iter(|| black_box(de_parser.parse_date(black_box("morgen um 15 Uhr"), None)));
    });

    // Spanish
    let es_parser = whichtime_sys::WhichTime::with_locale(Locale::Es);
    group.bench_function("es_mañana", |b| {
        b.iter(|| black_box(es_parser.parse_date(black_box("mañana a las 3"), None)));
    });

    // French
    let fr_parser = whichtime_sys::WhichTime::with_locale(Locale::Fr);
    group.bench_function("fr_demain", |b| {
        b.iter(|| black_box(fr_parser.parse_date(black_box("demain à 15h"), None)));
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_simple,
    benchmark_long_text,
    benchmark_pathological,
    benchmark_locales,
);
criterion_main!(benches);
