# Components

Date/time components represent the individual parts of a parsed date.

## Overview

- **Rust**: `FastComponents` struct with `Component` enum
- **Swift/Kotlin/Python**: `ParsedComponents` record/class

## Component Types

| Component | Description | Range |
|-----------|-------------|-------|
| `Year` | Calendar year | e.g., 2024 |
| `Month` | Month of year | 1-12 |
| `Day` | Day of month | 1-31 |
| `Hour` | Hour of day | 0-23 |
| `Minute` | Minute | 0-59 |
| `Second` | Second | 0-59 |
| `Millisecond` | Millisecond | 0-999 |
| `Weekday` | Day of week | 0-6 (Sunday=0) |
| `TimezoneOffset` | UTC offset | Minutes from UTC |

## Rust API

### FastComponents

```rust
use whichtime_sys::{FastComponents, Component};

// Get a component value
if let Some(hour) = components.get(Component::Hour) {
    println!("Hour: {}", hour);
}

// Check if a component is set
if components.is_known(Component::Year) {
    println!("Year is explicitly set");
}

// Check if a component is implied
if components.is_implied(Component::Year) {
    println!("Year is implied (not in original text)");
}
```

### Component Enum

```rust
pub enum Component {
    Year = 0,
    Month = 1,
    Day = 2,
    Hour = 3,
    Minute = 4,
    Second = 5,
    Millisecond = 6,
    Weekday = 7,
    TimezoneOffset = 8,
    Meridiem = 9,
}
```

### Setting Components

```rust
let mut components = FastComponents::default();

// Set a known component (explicitly in text)
components.set(Component::Hour, 15);

// Set an implied component (inferred, not in text)
components.imply(Component::Minute, 0);
```

### Converting to DateTime

```rust
use whichtime_sys::ReferenceWithTimezone;
use chrono::Local;

let ref_tz = ReferenceWithTimezone::new(Local::now(), None);

if let Some(datetime) = components.to_datetime(&ref_tz) {
    println!("DateTime: {}", datetime.to_rfc3339());
}
```

## FFI API

### ParsedComponents

::: code-group

```swift [Swift]
struct ParsedComponents {
    let year: Int32?
    let month: Int32?
    let day: Int32?
    let hour: Int32?
    let minute: Int32?
    let second: Int32?
    let millisecond: Int32?
    let weekday: Int32?
    let timezoneOffset: Int32?
}
```

```kotlin [Kotlin]
data class ParsedComponents(
    val year: Int?,
    val month: Int?,
    val day: Int?,
    val hour: Int?,
    val minute: Int?,
    val second: Int?,
    val millisecond: Int?,
    val weekday: Int?,
    val timezoneOffset: Int?
)
```

```python [Python]
@dataclass
class ParsedComponents:
    year: int | None
    month: int | None
    day: int | None
    hour: int | None
    minute: int | None
    second: int | None
    millisecond: int | None
    weekday: int | None
    timezone_offset: int | None
```

:::

### Accessing Components

::: code-group

```swift [Swift]
let results = try parse(text: "December 25, 2024 at 3pm")
if let result = results.first {
    let c = result.start
    
    if let year = c.year { print("Year: \(year)") }       // 2024
    if let month = c.month { print("Month: \(month)") }   // 12
    if let day = c.day { print("Day: \(day)") }           // 25
    if let hour = c.hour { print("Hour: \(hour)") }       // 15
}
```

```kotlin [Kotlin]
val results = parse("December 25, 2024 at 3pm")
results.firstOrNull()?.let { result ->
    val c = result.start
    
    c.year?.let { println("Year: $it") }     // 2024
    c.month?.let { println("Month: $it") }   // 12
    c.day?.let { println("Day: $it") }       // 25
    c.hour?.let { println("Hour: $it") }     // 15
}
```

```python [Python]
results = parse("December 25, 2024 at 3pm")
if results:
    c = results[0].start
    
    if c.year: print(f"Year: {c.year}")     # 2024
    if c.month: print(f"Month: {c.month}")  # 12
    if c.day: print(f"Day: {c.day}")        # 25
    if c.hour: print(f"Hour: {c.hour}")     # 15
```

:::

## Weekday Values

Weekdays are represented as integers:

| Value | Day |
|-------|-----|
| 0 | Sunday |
| 1 | Monday |
| 2 | Tuesday |
| 3 | Wednesday |
| 4 | Thursday |
| 5 | Friday |
| 6 | Saturday |

```python
results = parse("next Monday")
if results:
    weekday = results[0].start.weekday
    print(f"Weekday: {weekday}")  # 1 (Monday)
```

## Timezone Offset

The `timezone_offset` is in minutes from UTC:

| Offset | Timezone |
|--------|----------|
| 0 | UTC |
| -300 | EST (UTC-5) |
| -480 | PST (UTC-8) |
| 60 | CET (UTC+1) |
| 540 | JST (UTC+9) |

```python
results = parse("3pm EST")
if results:
    offset = results[0].start.timezone_offset
    if offset is not None:
        hours = offset // 60
        print(f"Offset: UTC{hours:+d}")  # UTC-5
```

## Null/None Components

A component is `None`/`null` when:

1. It wasn't specified in the input text
2. It couldn't be inferred from context

```python
results = parse("3pm")  # No date specified
if results:
    c = results[0].start
    
    print(c.hour)   # 15 - specified
    print(c.minute) # 0 - implied (default)
    print(c.year)   # None - not specified
    print(c.month)  # None - not specified
    print(c.day)    # None - not specified
```

When converting to a full datetime, missing components are filled from the reference date.

