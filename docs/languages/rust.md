# Rust

For most use cases, the `whichtime` crate is the recommended high-level API.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
whichtime = "0.1"
```

For direct use of the parser engine itself, custom pipelines, or lower-level integration work, use `whichtime-sys` instead.

## Basic Usage

```rust
use whichtime::WhichTime;

fn main() -> whichtime::Result<()> {
    let parser = WhichTime::new();
    
    // Parse and get first date (default: English)
    if let Some(date) = parser.parse_date("tomorrow at 3pm", None)? {
        println!("Date: {}", date.to_rfc3339());
    }
    
    // Parse and get all results
    let results = parser.parse("Meet me Monday or Tuesday", None)?;
    for result in results {
        println!("Found: {} at {}", result.text, result.index);
    }
    
    Ok(())
}
```

## Using Locales

Use `get_locale_parser` to get a parser for a specific locale, then call `parse` or `parse_date` on it:

```rust
use whichtime::{WhichTime, Locale};

let parser = WhichTime::new();
let de_parser = parser.get_locale_parser(Locale::De);
let date = de_parser.parse_date("morgen um 15 Uhr", None)?;
```

## Working with Results

The crate re-exports the same result and component types as the core engine:

```rust
use whichtime::{WhichTime, Component, ReferenceWithTimezone};

let parser = WhichTime::new();
let results = parser.parse("December 25, 2024 at 3pm", None)?;

if let Some(result) = results.first() {
    // Access matched text and position
    println!("Text: {}", result.text);
    println!("Position: {}..{}", result.index, result.end_index);
    
    // Access components
    let components = &result.start;
    if let Some(year) = components.get(Component::Year) {
        println!("Year: {}", year);
    }
    if let Some(month) = components.get(Component::Month) {
        println!("Month: {}", month);
    }
    if let Some(day) = components.get(Component::Day) {
        println!("Day: {}", day);
    }
    if let Some(hour) = components.get(Component::Hour) {
        println!("Hour: {}", hour);
    }
    
    // Convert to DateTime
    let ref_tz = ReferenceWithTimezone::new(result.ref_date, None);
    if let Some(datetime) = result.start.to_datetime(&ref_tz) {
        println!("DateTime: {}", datetime.to_rfc3339());
    }
    
    // Check for range
    if let Some(end) = &result.end {
        println!("This is a range, end: {:?}", end);
    }
}
```

## Custom Reference Date

```rust
use chrono::{Local, TimeZone};
use whichtime::WhichTime;

let parser = WhichTime::new();

// Use a specific reference date
let reference = Local.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap();
let date = parser.parse_date("next Monday", Some(reference))?;
```

## Error Handling

```rust
use whichtime::{WhichTime, Error};

let parser = WhichTime::new();

match parser.parse_date("some text", None) {
    Ok(Some(date)) => {
        println!("Parsed: {}", date.to_rfc3339());
    }
    Ok(None) => {
        println!("No date found");
    }
    Err(Error::InvalidDateTime(msg)) => {
        eprintln!("Invalid date: {}", msg);
    }
    Err(e) => {
        eprintln!("Error: {}", e);
    }
}
```

## Performance Tips

### Reuse Parsers

```rust
// Good: Create once, reuse
let parser = WhichTime::new();
for text in inputs {
    let _ = parser.parse_date(text, None);
}

// Bad: Create each time
for text in inputs {
    let parser = WhichTime::new(); // Wasteful
    let _ = parser.parse_date(text, None);
}
```

### Use parse_date for Single Results

```rust
// More efficient for single result
let date = parser.parse_date("tomorrow", None)?;

// Less efficient when you only need one date
use whichtime::ReferenceWithTimezone;
let results = parser.parse("tomorrow", None)?;
let date = results.first().and_then(|r| {
    let ref_tz = ReferenceWithTimezone::new(r.ref_date, None);
    r.date(&ref_tz)
});
```

## Features

The **whichtime** crate supports optional features such as `wasm_js` for JavaScript/WASM. See the [crate documentation](https://docs.rs/whichtime) for the full list.

## API Reference

For complete API documentation, see [docs.rs/whichtime](https://docs.rs/whichtime). The underlying engine types are the same as [whichtime-sys](https://docs.rs/whichtime-sys); use that crate if you need the core API directly.

