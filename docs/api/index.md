# API Reference

This section provides detailed API documentation for whichtime.

## Quick Reference

### Main Types

| Type | Description |
|------|-------------|
| [`WhichTime`](/api/whichtime) | Main parser (Rust) |
| [`WhichTimeParser`](/api/whichtime) | Main parser (FFI) |
| [`ParsedResult`](/api/parsed-result) | Parsing result |
| [`FastComponents`](/api/components) | Date/time components |
| [`Locale`](/api/whichtime#locales) | Supported locales |

### Key Methods

| Method | Description |
|--------|-------------|
| `parse()` | Parse text, return all matches |
| `parse_date()` | Parse text, return first date |
| `parse_with_locale()` | Parse with specific locale |

## By Language

### Rust

```rust
use whichtime::{Locale, ParsedResult, WhichTime};

let parser = WhichTime::new();
let results: Vec<ParsedResult> = parser.parse("tomorrow", None)?;
```

See [Rust API details](/languages/rust).

### Swift

```swift
import WhichtimeCore

let parser = WhichTimeParser()
let results: [WhichTimeResult] = try parser.parse(text: "tomorrow", locale: .en)
```

See [Swift API details](/languages/swift).

### Kotlin

```kotlin
import works.transcode.whichtime.*

val parser = WhichTimeParser()
val results: List<WhichTimeResult> = parser.parse("tomorrow", WhichTimeLocale.EN)
```

See [Kotlin API details](/languages/kotlin).

### Python

```python
from whichtime import WhichTimeParser, WhichTimeLocale

parser = WhichTimeParser()
results: list[WhichTimeResult] = parser.parse("tomorrow", WhichTimeLocale.EN)
```

See [Python API details](/languages/python).

## Rust Docs

For complete Rust API documentation, see [docs.rs/whichtime](https://docs.rs/whichtime). If you need the lower-level engine crate, see [docs.rs/whichtime-sys](https://docs.rs/whichtime-sys).

