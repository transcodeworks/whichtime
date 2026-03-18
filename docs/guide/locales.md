# Working with Locales

whichtime supports parsing date expressions in 12 different languages. This guide covers how to use locales effectively.

## Supported Locales

| Code | Language | Example Expression |
|------|----------|-------------------|
| `en` | English | tomorrow at 3pm |
| `de` | German | morgen um 15 Uhr |
| `es` | Spanish | mañana a las 3pm |
| `fr` | French | demain à 15h |
| `it` | Italian | domani alle 15 |
| `ja` | Japanese | 明日の午後3時 |
| `nl` | Dutch | morgen om 15:00 |
| `pt` | Portuguese | amanhã às 15h |
| `ru` | Russian | завтра в 15:00 |
| `sv` | Swedish | imorgon kl 15 |
| `uk` | Ukrainian | завтра о 15:00 |
| `zh` | Chinese | 明天下午3点 |

## Using Locales

### Default Locale (English)

By default, whichtime uses English:

::: code-group

```rust [Rust]
use whichtime_sys::WhichTime;

let parser = WhichTime::new(); // English by default
let date = parser.parse_date("tomorrow at 3pm", None)?;
```

```swift [Swift]
// parse() uses English by default
let results = try parse(text: "tomorrow at 3pm")
```

```kotlin [Kotlin]
// parse() uses English by default
val results = parse("tomorrow at 3pm")
```

```python [Python]
# parse() uses English by default
results = parse("tomorrow at 3pm")
```

:::

### Specifying a Locale

To parse expressions in other languages:

::: code-group

```rust [Rust]
use whichtime_sys::{WhichTime, Locale};

// Create a parser for German
let parser = WhichTime::with_locale(Locale::De);
let date = parser.parse_date("morgen um 15 Uhr", None)?;

// Create a parser for French
let parser_fr = WhichTime::with_locale(Locale::Fr);
let date_fr = parser_fr.parse_date("demain matin", None)?;
```

```swift [Swift]
let parser = WhichTimeParser()

// Parse German
let german = try parser.parse(text: "morgen um 15 Uhr", locale: .de)

// Parse French
let french = try parser.parse(text: "demain matin", locale: .fr)

// Parse Japanese
let japanese = try parser.parse(text: "明日", locale: .ja)
```

```kotlin [Kotlin]
val parser = WhichTimeParser()

// Parse German
val german = parser.parse("morgen um 15 Uhr", WhichTimeLocale.DE)

// Parse French
val french = parser.parse("demain matin", WhichTimeLocale.FR)

// Parse Japanese
val japanese = parser.parse("明日", WhichTimeLocale.JA)
```

```python [Python]
from whichtime import WhichTimeParser, WhichTimeLocale

parser = WhichTimeParser()

# Parse German
german = parser.parse("morgen um 15 Uhr", WhichTimeLocale.DE)

# Parse French
french = parser.parse("demain matin", WhichTimeLocale.FR)

# Parse Japanese
japanese = parser.parse("明日", WhichTimeLocale.JA)
```

:::

### Convenience Functions

For quick parsing with a specific locale:

::: code-group

```rust [Rust]
// Rust uses the parser directly
use whichtime_sys::{WhichTime, Locale};

let parser = WhichTime::with_locale(Locale::Es);
let date = parser.parse_date("mañana", None)?;
```

```swift [Swift]
// Use parseWithLocale for one-off parsing
let results = try parseWithLocale(text: "mañana", locale: .es)
```

```kotlin [Kotlin]
// Use parseWithLocale for one-off parsing
val results = parseWithLocale("mañana", WhichTimeLocale.ES)
```

```python [Python]
from whichtime import parse_with_locale, WhichTimeLocale

# Use parse_with_locale for one-off parsing
results = parse_with_locale("mañana", WhichTimeLocale.ES)
```

:::

## Locale Examples

### German (de)

```
heute           → today
morgen          → tomorrow
gestern         → yesterday
nächsten Montag → next Monday
vor 2 Tagen     → 2 days ago
in einer Woche  → in a week
```

### French (fr)

```
aujourd'hui     → today
demain          → tomorrow
hier            → yesterday
lundi prochain  → next Monday
il y a 2 jours  → 2 days ago
dans une semaine → in a week
demain matin    → tomorrow morning
```

### Spanish (es)

```
hoy             → today
mañana          → tomorrow
ayer            → yesterday
el próximo lunes → next Monday
hace 2 días     → 2 days ago
```

### Japanese (ja)

```
今日            → today
明日            → tomorrow
昨日            → yesterday
来週月曜日      → next Monday
2日前           → 2 days ago
```

### Chinese (zh)

```
今天            → today
明天            → tomorrow
昨天            → yesterday
下周一          → next Monday
2天前           → 2 days ago
```

## Best Practices

### 1. Reuse Parser Instances

Creating a parser has some initialization cost. Reuse the same parser for multiple parses:

```rust
// Good: Create once, use many times
let parser = WhichTime::with_locale(Locale::De);
for text in inputs {
    let result = parser.parse_date(text, None)?;
}

// Avoid: Creating a new parser each time
for text in inputs {
    let parser = WhichTime::with_locale(Locale::De); // Wasteful
    let result = parser.parse_date(text, None)?;
}
```

### 2. Match Locale to Input

Ensure the locale matches the language of the input text:

```rust
// Good: German text with German locale
let parser = WhichTime::with_locale(Locale::De);
parser.parse_date("morgen", None)?; // Works

// Bad: German text with English locale
let parser = WhichTime::new(); // English
parser.parse_date("morgen", None)?; // Won't recognize
```

### 3. Handle Unknown Locales

If your application supports user-selected locales, validate them:

```rust
fn get_parser(locale_code: &str) -> Option<WhichTime> {
    match locale_code {
        "en" => Some(WhichTime::with_locale(Locale::En)),
        "de" => Some(WhichTime::with_locale(Locale::De)),
        "fr" => Some(WhichTime::with_locale(Locale::Fr)),
        // ... other locales
        _ => None, // Unknown locale
    }
}
```

## Locale-Specific Documentation

For detailed information about each locale's supported expressions:

- [English](/locales/en)
- [German](/locales/de)
- [Spanish](/locales/es)
- [French](/locales/fr)
- [Italian](/locales/it)
- [Japanese](/locales/ja)
- [Dutch](/locales/nl)
- [Portuguese](/locales/pt)
- [Russian](/locales/ru)
- [Swedish](/locales/sv)
- [Ukrainian](/locales/uk)
- [Chinese](/locales/zh)

