# ParsedResult / WhichTimeResult

Represents a parsed date/time expression.

## Overview

- **Rust**: `ParsedResult` struct in `whichtime_sys`
- **Swift/Kotlin/Python**: `WhichTimeResult` record/class

## Fields

| Field | Type | Description |
|-------|------|-------------|
| `index` | `usize` / `UInt32` | Start byte position in original text |
| `end_index` | `usize` / `UInt32` | End byte position in original text |
| `text` | `String` | The matched text |
| `start` | `FastComponents` / `ParsedComponents` | Start date/time components |
| `end` | `Option<...>` | End components (for ranges) |
| `date_millis` | `Option<i64>` (FFI only) | Unix timestamp in milliseconds |

## Rust API

```rust
pub struct ParsedResult {
    pub index: usize,
    pub end_index: usize,
    pub text: String,
    pub start: FastComponents,
    pub end: Option<FastComponents>,
    pub ref_date: DateTime<Local>,
}
```

### Accessing Fields

```rust
let results = parser.parse("tomorrow at 3pm", None)?;

if let Some(result) = results.first() {
    println!("Text: {}", result.text);
    println!("Position: {} to {}", result.index, result.end_index);
    println!("Start components: {:?}", result.start);
    
    // Check if it's a range
    if let Some(end) = &result.end {
        println!("End components: {:?}", end);
    }
}
```

### Converting to DateTime

```rust
use whichtime_sys::ReferenceWithTimezone;

let result = &results[0];
let ref_tz = ReferenceWithTimezone::new(result.ref_date, None);

if let Some(datetime) = result.start.to_datetime(&ref_tz) {
    println!("DateTime: {}", datetime.to_rfc3339());
}
```

## FFI API

### WhichTimeResult

::: code-group

```swift [Swift]
struct WhichTimeResult {
    let index: UInt32
    let endIndex: UInt32
    let text: String
    let dateMillis: Int64?
    let start: ParsedComponents
    let end: ParsedComponents?
}
```

```kotlin [Kotlin]
data class WhichTimeResult(
    val index: UInt,
    val endIndex: UInt,
    val text: String,
    val dateMillis: Long?,
    val start: ParsedComponents,
    val end: ParsedComponents?
)
```

```python [Python]
@dataclass
class WhichTimeResult:
    index: int
    end_index: int
    text: str
    date_millis: int | None
    start: ParsedComponents
    end: ParsedComponents | None
```

:::

### Accessing Fields

::: code-group

```swift [Swift]
let results = try parse(text: "tomorrow at 3pm")

for result in results {
    print("Text: \(result.text)")
    print("Position: \(result.index) to \(result.endIndex)")
    
    // Get timestamp
    if let millis = result.dateMillis {
        let date = Date(timeIntervalSince1970: Double(millis) / 1000.0)
        print("Date: \(date)")
    }
    
    // Access components
    if let hour = result.start.hour {
        print("Hour: \(hour)")
    }
    
    // Check for range
    if let end = result.end {
        print("This is a range, end: \(end)")
    }
}
```

```kotlin [Kotlin]
val results = parse("tomorrow at 3pm")

results.forEach { result ->
    println("Text: ${result.text}")
    println("Position: ${result.index} to ${result.endIndex}")
    
    // Get timestamp
    result.dateMillis?.let { millis ->
        val date = java.util.Date(millis)
        println("Date: $date")
    }
    
    // Access components
    result.start.hour?.let { hour ->
        println("Hour: $hour")
    }
    
    // Check for range
    result.end?.let { end ->
        println("This is a range, end: $end")
    }
}
```

```python [Python]
from datetime import datetime

results = parse("tomorrow at 3pm")

for result in results:
    print(f"Text: {result.text}")
    print(f"Position: {result.index} to {result.end_index}")
    
    # Get timestamp
    if result.date_millis:
        dt = datetime.fromtimestamp(result.date_millis / 1000.0)
        print(f"Date: {dt}")
    
    # Access components
    if result.start.hour:
        print(f"Hour: {result.start.hour}")
    
    # Check for range
    if result.end:
        print(f"This is a range, end: {result.end}")
```

:::

## Working with Ranges

When parsing range expressions like "Monday to Friday", the result will have both `start` and `end` components:

```python
results = parse("Monday to Friday")

if results:
    result = results[0]
    
    # Single date vs range
    if result.end is None:
        print("Single date")
    else:
        print(f"Range: {result.start.weekday} to {result.end.weekday}")
```

## Index Positions

The `index` and `end_index` fields give byte positions, not character positions. For ASCII text they're the same, but for UTF-8 text with multi-byte characters, be careful:

```python
text = "明日の午後3時"  # Japanese: "tomorrow at 3pm"
results = parse_with_locale(text, WhichTimeLocale.JA)

if results:
    result = results[0]
    # Use byte positions to extract matched text
    matched = text[result.index:result.end_index]
    # Or just use result.text which is already the matched text
    assert matched == result.text
```

