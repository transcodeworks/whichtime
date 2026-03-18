use std::time::Instant;
use whichtime::WhichTime;
fn main() {
    let inputs = vec![
        // Basic absolute dates / times
        "2014-08-17 10:00",
        "08/17/2014 10:00 AM",
        "17 Aug 2014 10:00",
        "August 17th, 2014 10:00",
        "2025-01-01T00:00:00Z",
        "2025-01-01T23:59:59+02:00",
        // Relative dates (English)
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
        "next decade",
        "next century",
        "next friday at 3pm",
        "first of august last year",
        // Ranges and partials (may or may not be fully supported, good for stress)
        "between 3pm and 5pm tomorrow",
        "end of next month",
        "start of next year",
        // Edge-ish phrases
        "midnight",
        "noon",
        "end of today",
    ];
    let parser = WhichTime::default();

    let now = Instant::now();

    for input in inputs {
        let results = parser.parse_date(input, None).unwrap();
        println!(
            "{} -> {}",
            input,
            results
                .map(|r| r.to_rfc3339())
                .unwrap_or("Invalid date".to_string())
        );
    }

    let elapsed = now.elapsed();
    println!("Time elapsed: {}ms", elapsed.as_millis());
}
