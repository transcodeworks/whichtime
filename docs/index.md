---
layout: home

hero:
  name: "whichtime"
  text: "Natural Language Date Parsing"
  tagline: High-performance parsing for Rust, Swift, Kotlin, and Python
  actions:
    - theme: brand
      text: Get Started
      link: /guide/getting-started
    - theme: alt
      text: View on GitHub
      link: https://github.com/transcodeworks/whichtime

features:
  - icon: 🚀
    title: High Performance
    details: Built with Aho-Corasick scanning, PHF dictionaries, and zero-allocation component storage for blazing fast parsing.
  - icon: 🌍
    title: 12 Locales
    details: Parse dates in English, German, Spanish, French, Italian, Japanese, Dutch, Portuguese, Russian, Swedish, Ukrainian, and Chinese.
  - icon: 📱
    title: Cross-Platform
    details: Native bindings for Swift (iOS/macOS), Kotlin (Android/JVM), and Python via UniFFI.
  - icon: 🦀
    title: Pure Rust
    details: No runtime dependencies, minimal binary size, and memory-safe by design.
---

## Quick Example

::: code-group

```rust [Rust]
use whichtime_sys::WhichTime;

let parser = WhichTime::new();
let date = parser.parse_date("tomorrow at 3pm", None)?;
println!("{}", date.unwrap().to_rfc3339());
```

```swift [Swift]
import WhichtimeCore

let results = try parse(text: "tomorrow at 3pm")
if let millis = results.first?.dateMillis {
    let date = Date(timeIntervalSince1970: Double(millis) / 1000.0)
    print(date)
}
```

```kotlin [Kotlin]
import works.transcode.whichtime.*

val results = parse("tomorrow at 3pm")
results.firstOrNull()?.dateMillis?.let { millis ->
    println(java.util.Date(millis))
}
```

```python [Python]
from whichtime import parse
from datetime import datetime

results = parse("tomorrow at 3pm")
if results and results[0].date_millis:
    dt = datetime.fromtimestamp(results[0].date_millis / 1000.0)
    print(dt)
```

:::

## Supported Expressions

| Category | Examples |
|----------|----------|
| Casual dates | today, tomorrow, yesterday |
| Relative dates | in 2 hours, 3 days ago, next week |
| Weekdays | Monday, next Friday, last Tuesday |
| Explicit dates | 2024-12-25, December 25th |
| Time expressions | 3pm, 15:30, noon, midnight |
| Combined | tomorrow at 3pm, next Monday morning |

