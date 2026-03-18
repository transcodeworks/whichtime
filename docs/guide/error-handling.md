# Error Handling

whichtime can encounter errors during parsing. This guide covers how to handle them gracefully.

## Error Types

whichtime defines several error types:

| Error | Description |
|-------|-------------|
| `LocaleNotFound` | The specified locale is not supported |
| `RegexError` | A regex compilation or matching error |
| `InvalidDateTime` | The parsed values don't form a valid date/time |
| `ParseError` | A general parsing error |

## Handling Errors

::: code-group

```rust [Rust]
use whichtime_sys::{WhichTime, Error};

let parser = WhichTime::new();

match parser.parse_date("some text", None) {
    Ok(Some(date)) => {
        println!("Parsed: {}", date.to_rfc3339());
    }
    Ok(None) => {
        // No date found in the text - this is not an error
        println!("No date found");
    }
    Err(Error::InvalidDateTime(msg)) => {
        println!("Invalid date/time: {}", msg);
    }
    Err(Error::Regex(e)) => {
        println!("Regex error: {}", e);
    }
    Err(e) => {
        println!("Other error: {}", e);
    }
}
```

```swift [Swift]
import WhichtimeCore

do {
    let results = try parse(text: "some text")
    if results.isEmpty {
        print("No date found")
    } else {
        for result in results {
            print("Found: \(result.text)")
        }
    }
} catch WhichTimeError.LocaleNotFound(let locale) {
    print("Locale not found: \(locale)")
} catch WhichTimeError.InvalidDateTime(let message) {
    print("Invalid date/time: \(message)")
} catch WhichTimeError.ParseError(let message) {
    print("Parse error: \(message)")
} catch {
    print("Unexpected error: \(error)")
}
```

```kotlin [Kotlin]
import works.transcode.whichtime.*

try {
    val results = parse("some text")
    if (results.isEmpty()) {
        println("No date found")
    } else {
        results.forEach { println("Found: ${it.text}") }
    }
} catch (e: WhichTimeException.LocaleNotFound) {
    println("Locale not found: ${e.locale}")
} catch (e: WhichTimeException.InvalidDateTime) {
    println("Invalid date/time: ${e.message}")
} catch (e: WhichTimeException.ParseError) {
    println("Parse error: ${e.message}")
} catch (e: Exception) {
    println("Unexpected error: $e")
}
```

```python [Python]
from whichtime import parse, WhichTimeError

try:
    results = parse("some text")
    if not results:
        print("No date found")
    else:
        for result in results:
            print(f"Found: {result.text}")
except WhichTimeError.LocaleNotFound as e:
    print(f"Locale not found: {e.locale}")
except WhichTimeError.InvalidDateTime as e:
    print(f"Invalid date/time: {e.message}")
except WhichTimeError.ParseError as e:
    print(f"Parse error: {e.message}")
except Exception as e:
    print(f"Unexpected error: {e}")
```

:::

## No Date Found vs. Error

It's important to distinguish between "no date found" and an actual error:

- **No date found**: The text was parsed successfully, but didn't contain any recognizable date expressions. This returns an empty list or `None`.
- **Error**: Something went wrong during parsing (invalid regex, internal error, etc.).

::: code-group

```rust [Rust]
let parser = WhichTime::new();

// No date found - not an error
let result = parser.parse_date("hello world", None)?;
assert!(result.is_none()); // Ok(None)

// Text with a date
let result = parser.parse_date("tomorrow", None)?;
assert!(result.is_some()); // Ok(Some(date))
```

```swift [Swift]
// No date found - empty array, not an error
let results = try parse(text: "hello world")
assert(results.isEmpty) // []

// Text with a date
let results2 = try parse(text: "tomorrow")
assert(!results2.isEmpty) // [result]
```

```kotlin [Kotlin]
// No date found - empty list, not an error
val results = parse("hello world")
assert(results.isEmpty()) // []

// Text with a date
val results2 = parse("tomorrow")
assert(results2.isNotEmpty()) // [result]
```

```python [Python]
# No date found - empty list, not an error
results = parse("hello world")
assert len(results) == 0  # []

# Text with a date
results2 = parse("tomorrow")
assert len(results2) > 0  # [result]
```

:::

## Best Practices

### 1. Always Handle the Empty Case

```python
results = parse(user_input)
if not results:
    # Handle gracefully - maybe prompt user for a different format
    print("I couldn't find a date in that text. Try something like 'tomorrow at 3pm'")
else:
    process_date(results[0])
```

### 2. Validate Resolved Dates

Even when parsing succeeds, the resolved date might not be what you expect:

```python
results = parse("February 30th")  # Invalid date
if results:
    if results[0].date_millis is None:
        print("Parsed but couldn't resolve to a valid date")
```

### 3. Provide Fallbacks

```python
def parse_user_date(text: str, fallback=None):
    try:
        results = parse(text)
        if results and results[0].date_millis:
            return datetime.fromtimestamp(results[0].date_millis / 1000.0)
    except Exception:
        pass
    return fallback
```

### 4. Log Errors for Debugging

```python
import logging

logger = logging.getLogger(__name__)

try:
    results = parse(user_input)
except WhichTimeError as e:
    logger.warning(f"Failed to parse date from '{user_input}': {e}")
    results = []
```

## Common Issues

### Ambiguous Dates

Some expressions are ambiguous and might not parse as expected:

```python
# "5/6/2024" - Is this May 6 or June 5?
# Depends on locale settings
results = parse("5/6/2024")  # Parses as May 6 (US format)
```

### Partial Dates

Expressions with missing components use defaults:

```python
# "March 15" - No year specified
results = parse("March 15")
# Uses current year (or next year if date has passed)
```

### Timezone Issues

Parsed dates use the local timezone by default:

```python
results = parse("tomorrow at noon")
# date_millis is in local time
# Convert to UTC if needed
```

