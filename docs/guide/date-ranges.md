# Date Ranges

whichtime can parse expressions that represent a range of dates, like "Monday to Friday" or "from January to March".

## Parsing Ranges

When a range is detected, the result includes both `start` and `end` components:

::: code-group

```rust [Rust]
use whichtime_sys::WhichTime;

let parser = WhichTime::new();
let results = parser.parse("Monday to Friday", None)?;

if let Some(result) = results.first() {
    println!("Text: {}", result.text);
    println!("Start: {:?}", result.start);
    
    if let Some(end) = &result.end {
        println!("End: {:?}", end);
    }
}
```

```swift [Swift]
let results = try parse(text: "Monday to Friday")
if let result = results.first {
    print("Text: \(result.text)")
    print("Start weekday: \(result.start.weekday ?? -1)")
    
    if let end = result.end {
        print("End weekday: \(end.weekday ?? -1)")
    }
}
```

```kotlin [Kotlin]
val results = parse("Monday to Friday")
results.firstOrNull()?.let { result ->
    println("Text: ${result.text}")
    println("Start weekday: ${result.start.weekday}")
    
    result.end?.let { end ->
        println("End weekday: ${end.weekday}")
    }
}
```

```python [Python]
results = parse("Monday to Friday")
if results:
    result = results[0]
    print(f"Text: {result.text}")
    print(f"Start weekday: {result.start.weekday}")
    
    if result.end:
        print(f"End weekday: {result.end.weekday}")
```

:::

## Supported Range Formats

### Weekday Ranges

```
Monday to Friday
from Monday to Friday
Tuesday through Thursday
```

### Date Ranges

```
January 1 to January 15
from Dec 20 to Dec 31
March 1st through March 15th
```

### Combined with Time

```
Monday to Friday at 9am
from 9am to 5pm
between 3pm and 5pm
```

## Checking for Ranges

To determine if a result is a range, check if the `end` component exists:

::: code-group

```rust [Rust]
let results = parser.parse("Monday to Friday", None)?;

for result in results {
    if result.end.is_some() {
        println!("Range: {} to {}", 
            result.start.get(Component::Weekday).unwrap_or(-1),
            result.end.as_ref().and_then(|e| e.get(Component::Weekday)).unwrap_or(-1)
        );
    } else {
        println!("Single date: {}", result.text);
    }
}
```

```swift [Swift]
let results = try parse(text: "Monday to Friday")

for result in results {
    if result.end != nil {
        print("This is a date range")
    } else {
        print("This is a single date")
    }
}
```

```kotlin [Kotlin]
val results = parse("Monday to Friday")

results.forEach { result ->
    if (result.end != null) {
        println("This is a date range")
    } else {
        println("This is a single date")
    }
}
```

```python [Python]
results = parse("Monday to Friday")

for result in results:
    if result.end:
        print("This is a date range")
    else:
        print("This is a single date")
```

:::

## Working with Range Components

Both `start` and `end` contain the same component types:

| Component | Description |
|-----------|-------------|
| `year` | Year (e.g., 2024) |
| `month` | Month (1-12) |
| `day` | Day of month (1-31) |
| `weekday` | Day of week (0=Sunday, 6=Saturday) |
| `hour` | Hour (0-23) |
| `minute` | Minute (0-59) |
| `second` | Second (0-59) |

## Examples

### Weekday Range

```python
results = parse("Monday to Friday")
# result.start.weekday = 1 (Monday)
# result.end.weekday = 5 (Friday)
```

### Date Range

```python
results = parse("January 1 to January 15")
# result.start: month=1, day=1
# result.end: month=1, day=15
```

### Time Range

```python
results = parse("from 9am to 5pm")
# result.start.hour = 9
# result.end.hour = 17
```

## Limitations

- Complex nested ranges may not parse correctly
- Some range formats are locale-specific
- Time zones in ranges use the same timezone for start and end

