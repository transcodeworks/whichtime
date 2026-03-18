# Supported Locales

whichtime supports parsing natural language dates in 12 languages.

## Overview

| Code | Language | Native Name |
|------|----------|-------------|
| `en` | [English](/locales/en) | English |
| `de` | [German](/locales/de) | Deutsch |
| `es` | [Spanish](/locales/es) | Español |
| `fr` | [French](/locales/fr) | Français |
| `it` | [Italian](/locales/it) | Italiano |
| `ja` | [Japanese](/locales/ja) | 日本語 |
| `nl` | [Dutch](/locales/nl) | Nederlands |
| `pt` | [Portuguese](/locales/pt) | Português |
| `ru` | [Russian](/locales/ru) | Русский |
| `sv` | [Swedish](/locales/sv) | Svenska |
| `uk` | [Ukrainian](/locales/uk) | Українська |
| `zh` | [Chinese](/locales/zh) | 中文 |

## Feature Coverage

Not all locales have the same level of feature coverage. Here's an overview:

| Feature | en | de | es | fr | it | ja | nl | pt | ru | sv | uk | zh |
|---------|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| Casual dates | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Weekdays | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Month names | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Time expressions | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Relative (ago) | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Relative (in X) | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| ISO dates | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Slash dates | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |

## Date Format Conventions

Different locales use different date format conventions:

| Locale | Format | Example |
|--------|--------|---------|
| en (US) | MM/DD/YYYY | 12/25/2024 |
| de, es, fr, it, nl, pt, ru, sv, uk | DD/MM/YYYY | 25/12/2024 |
| ja, zh | YYYY/MM/DD | 2024/12/25 |

## Quick Examples

::: code-group

```rust [Rust]
use whichtime_sys::{WhichTime, Locale};

// English
let en = WhichTime::with_locale(Locale::En);
en.parse_date("tomorrow at 3pm", None)?;

// German
let de = WhichTime::with_locale(Locale::De);
de.parse_date("morgen um 15 Uhr", None)?;

// Japanese
let ja = WhichTime::with_locale(Locale::Ja);
ja.parse_date("明日の午後3時", None)?;
```

```python [Python]
from whichtime import parse_with_locale, WhichTimeLocale

# English
parse_with_locale("tomorrow at 3pm", WhichTimeLocale.EN)

# German
parse_with_locale("morgen um 15 Uhr", WhichTimeLocale.DE)

# Japanese
parse_with_locale("明日の午後3時", WhichTimeLocale.JA)
```

:::

## Adding New Locales

Want to add support for a new locale? See the [Architecture guide](/guide/architecture) for details on:

1. Creating a dictionary file
2. Implementing locale-specific parsers
3. Adding tests

Contributions are welcome!

