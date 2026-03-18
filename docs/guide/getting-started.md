# Getting Started

whichtime is a high-performance natural language date parsing library. It understands expressions like "tomorrow at 3pm", "in 2 weeks", or "last Friday" and converts them to proper date/time values.

## What is whichtime?

whichtime parses human-readable date and time expressions into structured data. It's:

- **Fast** — Built in Rust with performance-focused data structures
- **Multi-language** — Rust is the primary published target, with Swift, Kotlin, and Python bindings available from source
- **Multi-locale** — Supports 12 languages for parsing
- **Flexible** — Handles casual, relative, and explicit date formats

## Installation

Choose your platform:

::: code-group

```toml [Rust]
[dependencies]
whichtime = "0.1"
```

```bash [Swift (preview)]
./common/build-ios.sh
```

```bash [Kotlin (preview)]
./common/build-kotlin.sh
```

```bash [Python (preview)]
./common/build-python.sh
```

:::

See the [Installation Guide](/guide/installation) for detailed instructions.

## Basic Usage

### Parse a Date Expression

::: code-group

```rust [Rust]
use whichtime::WhichTime;

fn main() {
    let parser = WhichTime::new();
    
    match parser.parse_date("tomorrow at 3pm", None) {
        Ok(Some(date)) => println!("Parsed: {}", date.to_rfc3339()),
        Ok(None) => println!("No date found"),
        Err(e) => println!("Error: {}", e),
    }
}
```

```swift [Swift]
import WhichtimeCore

do {
    let results = try parse(text: "tomorrow at 3pm")
    for result in results {
        if let millis = result.dateMillis {
            let date = Date(timeIntervalSince1970: Double(millis) / 1000.0)
            print("Parsed: \(date)")
        }
    }
} catch {
    print("Error: \(error)")
}
```

```kotlin [Kotlin]
import works.transcode.whichtime.*

fun main() {
    val results = parse("tomorrow at 3pm")
    results.forEach { result ->
        result.dateMillis?.let { millis ->
            val date = java.util.Date(millis)
            println("Parsed: $date")
        }
    }
}
```

```python [Python]
from whichtime import parse
from datetime import datetime

results = parse("tomorrow at 3pm")
for result in results:
    if result.date_millis:
        dt = datetime.fromtimestamp(result.date_millis / 1000.0)
        print(f"Parsed: {dt}")
```

:::

### Parse with Different Locales

whichtime supports 12 locales. Here's how to parse German expressions:

::: code-group

```rust [Rust]
use whichtime::{Locale, WhichTime};

let parser = WhichTime::with_locale(Locale::De);
let date = parser.parse_date("morgen um 15 Uhr", None)?;
```

```swift [Swift]
let parser = WhichTimeParser()
let results = try parser.parse(text: "morgen um 15 Uhr", locale: .de)
```

```kotlin [Kotlin]
val parser = WhichTimeParser()
val results = parser.parse("morgen um 15 Uhr", WhichTimeLocale.DE)
```

```python [Python]
from whichtime import WhichTimeParser, WhichTimeLocale

parser = WhichTimeParser()
results = parser.parse("morgen um 15 Uhr", WhichTimeLocale.DE)
```

:::

## Notes On Package Layers

- Use `whichtime` for the recommended Rust API.
- Use `whichtime-sys` only when you need the lower-level parser engine directly.
- Swift, Kotlin, and Python bindings are still source-build integrations in this release.

## Next Steps

- [Installation](/guide/installation) — Detailed setup for each platform
- [Basic Usage](/guide/basic-usage) — More parsing examples
- [Working with Locales](/guide/locales) — Using different languages
- [API Reference](/api/) — Complete API documentation

