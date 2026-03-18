# Errors

Error types and handling in whichtime.

## Error Types

| Error | Description |
|-------|-------------|
| `LocaleNotFound` | The specified locale is not supported |
| `RegexError` | A regex compilation or matching error |
| `InvalidDateTime` | The parsed values don't form a valid date/time |
| `ParseError` | A general parsing error |

## Rust API

### Error Enum

```rust
use whichtime_sys::Error;

pub enum Error {
    LocaleNotFound(String),
    Regex(regex::Error),
    InvalidDateTime(String),
    Parse(String),
}
```

### Handling Errors

```rust
use whichtime_sys::{WhichTime, Error, Result};

fn parse_date(text: &str) -> Result<String> {
    let parser = WhichTime::new();
    
    match parser.parse_date(text, None) {
        Ok(Some(date)) => Ok(date.to_rfc3339()),
        Ok(None) => Err(Error::Parse("No date found".into())),
        Err(e) => Err(e),
    }
}

// Using ? operator
fn parse_date_short(text: &str) -> Result<Option<String>> {
    let parser = WhichTime::new();
    let date = parser.parse_date(text, None)?;
    Ok(date.map(|d| d.to_rfc3339()))
}
```

### Pattern Matching

```rust
match parser.parse("text", None) {
    Ok(results) => {
        // Handle results
    }
    Err(Error::LocaleNotFound(locale)) => {
        println!("Unknown locale: {}", locale);
    }
    Err(Error::Regex(e)) => {
        println!("Regex error: {}", e);
    }
    Err(Error::InvalidDateTime(msg)) => {
        println!("Invalid date/time: {}", msg);
    }
    Err(Error::Parse(msg)) => {
        println!("Parse error: {}", msg);
    }
}
```

## FFI API

### WhichTimeError

::: code-group

```swift [Swift]
enum WhichTimeError: Error {
    case localeNotFound(locale: String)
    case regexError(message: String)
    case invalidDateTime(message: String)
    case parseError(message: String)
}
```

```kotlin [Kotlin]
sealed class WhichTimeException : Exception() {
    data class LocaleNotFound(val locale: String) : WhichTimeException()
    data class RegexError(override val message: String) : WhichTimeException()
    data class InvalidDateTime(override val message: String) : WhichTimeException()
    data class ParseError(override val message: String) : WhichTimeException()
}
```

```python [Python]
class WhichTimeError(Exception):
    class LocaleNotFound(WhichTimeError):
        locale: str
    
    class RegexError(WhichTimeError):
        message: str
    
    class InvalidDateTime(WhichTimeError):
        message: str
    
    class ParseError(WhichTimeError):
        message: str
```

:::

### Handling Errors

::: code-group

```swift [Swift]
do {
    let results = try parse(text: "some text")
    // Handle results
} catch WhichTimeError.localeNotFound(let locale) {
    print("Unknown locale: \(locale)")
} catch WhichTimeError.invalidDateTime(let message) {
    print("Invalid date/time: \(message)")
} catch WhichTimeError.parseError(let message) {
    print("Parse error: \(message)")
} catch {
    print("Unexpected error: \(error)")
}
```

```kotlin [Kotlin]
try {
    val results = parse("some text")
    // Handle results
} catch (e: WhichTimeException.LocaleNotFound) {
    println("Unknown locale: ${e.locale}")
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
    # Handle results
except WhichTimeError.LocaleNotFound as e:
    print(f"Unknown locale: {e.locale}")
except WhichTimeError.InvalidDateTime as e:
    print(f"Invalid date/time: {e.message}")
except WhichTimeError.ParseError as e:
    print(f"Parse error: {e.message}")
except Exception as e:
    print(f"Unexpected error: {e}")
```

:::

## Common Error Scenarios

### No Date Found (Not an Error)

An empty result is not an error—it just means no date was found:

```python
results = parse("hello world")
if not results:
    print("No date found")  # This is expected, not an error
```

### Invalid Input

Most invalid input won't cause errors—it just won't match:

```python
results = parse("asdfghjkl")
assert len(results) == 0  # No error, just no matches
```

### Truly Invalid Dates

Some invalid date values can cause errors:

```rust
// This might cause an InvalidDateTime error
// if internal validation catches it
parser.parse("February 30th", None)?;
```

## Best Practices

### 1. Always Handle Empty Results

```python
def get_date(text: str) -> datetime | None:
    results = parse(text)
    if results and results[0].date_millis:
        return datetime.fromtimestamp(results[0].date_millis / 1000.0)
    return None
```

### 2. Provide Fallbacks

```python
def parse_with_fallback(text: str, fallback: datetime) -> datetime:
    try:
        results = parse(text)
        if results and results[0].date_millis:
            return datetime.fromtimestamp(results[0].date_millis / 1000.0)
    except WhichTimeError:
        pass
    return fallback
```

### 3. Log Errors for Debugging

```python
import logging

logger = logging.getLogger(__name__)

def safe_parse(text: str) -> list:
    try:
        return parse(text)
    except WhichTimeError as e:
        logger.warning(f"Parse failed for '{text}': {e}")
        return []
```

