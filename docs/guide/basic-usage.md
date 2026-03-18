# Basic Usage

This guide covers the fundamentals of parsing dates with whichtime.

## Parsing Text

The simplest way to parse text is with the main parser:

::: code-group

```rust [Rust]
use whichtime::WhichTime;

let parser = WhichTime::new();

// Parse and get the first date found
let date = parser.parse_date("Meet me tomorrow at 3pm", None)?;
if let Some(dt) = date {
    println!("Date: {}", dt.to_rfc3339());
}

// Parse and get all date expressions found
let results = parser.parse("From Monday to Friday at 9am", None)?;
for result in results {
    println!("Found: {} at position {}", result.text, result.index);
}
```

```swift [Swift]
import WhichtimeCore

// Parse and get all results
let results = try parse(text: "Meet me tomorrow at 3pm")
for result in results {
    print("Found: \(result.text)")
    if let millis = result.dateMillis {
        let date = Date(timeIntervalSince1970: Double(millis) / 1000.0)
        print("Date: \(date)")
    }
}

// Parse and get just the first date
if let timestamp = try parseDate(text: "tomorrow at 3pm") {
    let date = Date(timeIntervalSince1970: Double(timestamp) / 1000.0)
    print("First date: \(date)")
}
```

```kotlin [Kotlin]
import works.transcode.whichtime.*

// Parse and get all results
val results = parse("Meet me tomorrow at 3pm")
results.forEach { result ->
    println("Found: ${result.text}")
    result.dateMillis?.let { millis ->
        println("Date: ${java.util.Date(millis)}")
    }
}

// Parse and get just the first date
parseDate("tomorrow at 3pm")?.let { timestamp ->
    println("First date: ${java.util.Date(timestamp)}")
}
```

```python [Python]
from whichtime import parse, parse_date
from datetime import datetime

# Parse and get all results
results = parse("Meet me tomorrow at 3pm")
for result in results:
    print(f"Found: {result.text}")
    if result.date_millis:
        dt = datetime.fromtimestamp(result.date_millis / 1000.0)
        print(f"Date: {dt}")

# Parse and get just the first date
timestamp = parse_date("tomorrow at 3pm")
if timestamp:
    dt = datetime.fromtimestamp(timestamp / 1000.0)
    print(f"First date: {dt}")
```

:::

## Understanding Results

When you parse text, you get back a list of `ParsedResult` objects (or equivalent for your language). Each result contains:

| Field | Description |
|-------|-------------|
| `text` | The matched text from the input |
| `index` | Start position in the original string |
| `end_index` | End position in the original string |
| `date_millis` | Unix timestamp in milliseconds (if resolvable) |
| `start` | Start date/time components |
| `end` | End date/time components (for ranges) |

### Accessing Components

Components give you the individual parts of the parsed date:

::: code-group

```rust [Rust]
use whichtime::{Component, WhichTime};

let parser = WhichTime::new();
let results = parser.parse("December 25, 2024 at 3pm", None)?;

if let Some(result) = results.first() {
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
}
```

```swift [Swift]
let results = try parse(text: "December 25, 2024 at 3pm")
if let result = results.first {
    let components = result.start
    
    if let year = components.year { print("Year: \(year)") }
    if let month = components.month { print("Month: \(month)") }
    if let day = components.day { print("Day: \(day)") }
    if let hour = components.hour { print("Hour: \(hour)") }
}
```

```kotlin [Kotlin]
val results = parse("December 25, 2024 at 3pm")
results.firstOrNull()?.let { result ->
    val components = result.start
    
    components.year?.let { println("Year: $it") }
    components.month?.let { println("Month: $it") }
    components.day?.let { println("Day: $it") }
    components.hour?.let { println("Hour: $it") }
}
```

```python [Python]
results = parse("December 25, 2024 at 3pm")
if results:
    components = results[0].start
    
    if components.year: print(f"Year: {components.year}")
    if components.month: print(f"Month: {components.month}")
    if components.day: print(f"Day: {components.day}")
    if components.hour: print(f"Hour: {components.hour}")
```

:::

## Supported Expression Types

whichtime can parse many types of date expressions:

### Casual Dates

```
today
tomorrow
yesterday
```

### Relative Dates

```
in 2 hours
in 3 days
2 weeks from now
3 months ago
last week
next month
```

### Weekdays

```
Monday
next Friday
last Tuesday
this Saturday
```

### Explicit Dates

```
2024-12-25
12/25/2024
December 25, 2024
25th of December
Aug 17th, 2024
```

### Time Expressions

```
3pm
15:30
3:30 PM
noon
midnight
```

### Combined Expressions

```
tomorrow at 3pm
next Monday at 9:30am
December 25th at noon
in 2 days at 5pm
```

## Reference Date

By default, whichtime uses the current date/time as the reference for relative expressions. You can provide a custom reference:

::: code-group

```rust [Rust]
use chrono::Local;
use whichtime_sys::WhichTime;

let parser = WhichTime::new();

// Use a specific reference date
let reference = Local::now();
let date = parser.parse_date("tomorrow", Some(reference))?;
```

```swift [Swift]
// FFI bindings currently use the current time as reference
// Custom reference dates coming soon
let results = try parse(text: "tomorrow")
```

```kotlin [Kotlin]
// FFI bindings currently use the current time as reference
// Custom reference dates coming soon
val results = parse("tomorrow")
```

```python [Python]
# FFI bindings currently use the current time as reference
# Custom reference dates coming soon
results = parse("tomorrow")
```

:::

## Next Steps

- [Working with Locales](/guide/locales) — Parse dates in different languages
- [Date Ranges](/guide/date-ranges) — Parse expressions like "Monday to Friday"
- [Error Handling](/guide/error-handling) — Handle parsing errors gracefully

