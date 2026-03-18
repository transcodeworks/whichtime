# WhichTime / WhichTimeParser

The main parser type for whichtime.

## Overview

- **Rust**: `WhichTime` struct in `whichtime_sys`
- **Swift/Kotlin/Python**: `WhichTimeParser` class

## Rust API

### Creating a Parser

```rust
use whichtime_sys::{WhichTime, Locale};

// Default (English)
let parser = WhichTime::new();

// With specific locale
let parser = WhichTime::with_locale(Locale::De);

// With custom configuration
let parser = WhichTime::with_configuration(config);
```

### Methods

#### `parse`

Parse text and return all date/time expressions found.

```rust
pub fn parse(
    &self,
    text: &str,
    reference: Option<DateTime<Local>>,
) -> Result<Vec<ParsedResult>>
```

**Parameters:**
- `text` - The text to parse
- `reference` - Optional reference date (defaults to now)

**Returns:** `Result<Vec<ParsedResult>>` - All parsed results

**Example:**
```rust
let results = parser.parse("Meet me tomorrow at 3pm", None)?;
for result in results {
    println!("{}: {:?}", result.text, result.start);
}
```

#### `parse_date`

Parse text and return the first date found.

```rust
pub fn parse_date(
    &self,
    text: &str,
    reference: Option<DateTime<Local>>,
) -> Result<Option<DateTime<Local>>>
```

**Parameters:**
- `text` - The text to parse
- `reference` - Optional reference date (defaults to now)

**Returns:** `Result<Option<DateTime<Local>>>` - First resolved date, or None

**Example:**
```rust
if let Some(date) = parser.parse_date("tomorrow", None)? {
    println!("Date: {}", date.to_rfc3339());
}
```

#### `locale`

Get the current locale.

```rust
pub fn locale(&self) -> Locale
```

## FFI API (Swift/Kotlin/Python)

### Creating a Parser

::: code-group

```swift [Swift]
let parser = WhichTimeParser()
```

```kotlin [Kotlin]
val parser = WhichTimeParser()
```

```python [Python]
parser = WhichTimeParser()
```

:::

### Methods

#### `parse`

Parse text and return all results.

::: code-group

```swift [Swift]
func parse(text: String, locale: WhichTimeLocale) throws -> [WhichTimeResult]
```

```kotlin [Kotlin]
fun parse(text: String, locale: WhichTimeLocale): List<WhichTimeResult>
```

```python [Python]
def parse(text: str, locale: WhichTimeLocale) -> list[WhichTimeResult]
```

:::

#### `parseDate` / `parse_date`

Parse text and return the first date as Unix timestamp.

::: code-group

```swift [Swift]
func parseDate(text: String, locale: WhichTimeLocale) throws -> Int64?
```

```kotlin [Kotlin]
fun parseDate(text: String, locale: WhichTimeLocale): Long?
```

```python [Python]
def parse_date(text: str, locale: WhichTimeLocale) -> int | None
```

:::

#### `parseEn` / `parse_en`

Convenience method to parse with English locale.

::: code-group

```swift [Swift]
func parseEn(text: String) throws -> [WhichTimeResult]
```

```kotlin [Kotlin]
fun parseEn(text: String): List<WhichTimeResult>
```

```python [Python]
def parse_en(text: str) -> list[WhichTimeResult]
```

:::

## Top-Level Functions

For convenience, top-level functions are also available:

::: code-group

```swift [Swift]
// Parse with English locale
let results = try parse(text: "tomorrow")

// Parse and get first date
let timestamp = try parseDate(text: "tomorrow")

// Parse with specific locale
let results = try parseWithLocale(text: "demain", locale: .fr)
```

```kotlin [Kotlin]
// Parse with English locale
val results = parse("tomorrow")

// Parse and get first date
val timestamp = parseDate("tomorrow")

// Parse with specific locale
val results = parseWithLocale("demain", WhichTimeLocale.FR)
```

```python [Python]
from whichtime import parse, parse_date, parse_with_locale

# Parse with English locale
results = parse("tomorrow")

# Parse and get first date
timestamp = parse_date("tomorrow")

# Parse with specific locale
results = parse_with_locale("demain", WhichTimeLocale.FR)
```

:::

## Locales

### Locale Enum

::: code-group

```rust [Rust]
use whichtime_sys::Locale;

Locale::En  // English
Locale::De  // German
Locale::Es  // Spanish
Locale::Fr  // French
Locale::It  // Italian
Locale::Ja  // Japanese
Locale::Nl  // Dutch
Locale::Pt  // Portuguese
Locale::Ru  // Russian
Locale::Sv  // Swedish
Locale::Uk  // Ukrainian
Locale::Zh  // Chinese
```

```swift [Swift]
WhichTimeLocale.en  // English
WhichTimeLocale.de  // German
WhichTimeLocale.es  // Spanish
WhichTimeLocale.fr  // French
WhichTimeLocale.it  // Italian
WhichTimeLocale.ja  // Japanese
WhichTimeLocale.nl  // Dutch
WhichTimeLocale.pt  // Portuguese
WhichTimeLocale.ru  // Russian
WhichTimeLocale.sv  // Swedish
WhichTimeLocale.uk  // Ukrainian
WhichTimeLocale.zh  // Chinese
```

```kotlin [Kotlin]
WhichTimeLocale.EN  // English
WhichTimeLocale.DE  // German
WhichTimeLocale.ES  // Spanish
WhichTimeLocale.FR  // French
WhichTimeLocale.IT  // Italian
WhichTimeLocale.JA  // Japanese
WhichTimeLocale.NL  // Dutch
WhichTimeLocale.PT  // Portuguese
WhichTimeLocale.RU  // Russian
WhichTimeLocale.SV  // Swedish
WhichTimeLocale.UK  // Ukrainian
WhichTimeLocale.ZH  // Chinese
```

```python [Python]
WhichTimeLocale.EN  # English
WhichTimeLocale.DE  # German
WhichTimeLocale.ES  # Spanish
WhichTimeLocale.FR  # French
WhichTimeLocale.IT  # Italian
WhichTimeLocale.JA  # Japanese
WhichTimeLocale.NL  # Dutch
WhichTimeLocale.PT  # Portuguese
WhichTimeLocale.RU  # Russian
WhichTimeLocale.SV  # Swedish
WhichTimeLocale.UK  # Ukrainian
WhichTimeLocale.ZH  # Chinese
```

:::

## Thread Safety

Both `WhichTime` (Rust) and `WhichTimeParser` (FFI) are thread-safe:

- Can be shared across threads
- Multiple threads can call parse methods concurrently
- No global mutable state

