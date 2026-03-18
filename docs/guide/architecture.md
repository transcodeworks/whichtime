# Architecture

This guide explains the internal architecture of whichtime for developers who want to understand how it works or contribute to the project.

## Overview

whichtime uses a pipeline architecture:

```
Input Text → Scanner → Parsers → Refiners → Results
```

1. **Scanner** - Pre-scans text to identify potential date tokens
2. **Parsers** - Extract date/time components from text
3. **Refiners** - Clean up, merge, and enhance parsed results
4. **Results** - Final structured output

## Core Components

### WhichTime

The main entry point that orchestrates parsing:

```rust
pub struct WhichTime {
    parsers: Vec<Box<dyn Parser>>,
    refiners: Vec<Box<dyn Refiner>>,
    locale: Locale,
}
```

### ParsingContext

Holds the input text and reference date:

```rust
pub struct ParsingContext<'a> {
    pub text: &'a str,
    pub reference: &'a ReferenceWithTimezone,
    pub locale: Locale,
}
```

### FastComponents

Stores parsed date/time components efficiently:

```rust
#[derive(Clone, Copy, Default)]
pub struct FastComponents {
    values: [i32; 10],
    known: ComponentFlags,
    implied: ComponentFlags,
}
```

### ParsedResult

The output of parsing:

```rust
pub struct ParsedResult {
    pub index: usize,      // Start position in text
    pub end_index: usize,  // End position in text
    pub text: String,      // Matched text
    pub start: FastComponents,
    pub end: Option<FastComponents>,
    pub ref_date: DateTime<Local>,
}
```

## Parser Pipeline

### 1. Scanner (Aho-Corasick)

The scanner pre-identifies potential date tokens using Aho-Corasick:

```rust
pub struct Scanner {
    automaton: AhoCorasick,
    patterns: Vec<&'static str>,
}

impl Scanner {
    pub fn find_matches(&self, text: &str) -> Vec<Match> {
        // Returns all potential date-related tokens
    }
}
```

Parsers use this to implement `should_apply()`:

```rust
impl Parser for WeekdayParser {
    fn should_apply(&self, context: &ParsingContext) -> bool {
        // Quick check: does the text contain any weekday names?
        context.scanner.contains_any(&["monday", "tuesday", ...])
    }
}
```

### 2. Parsers

Each parser extracts specific date patterns:

```rust
pub trait Parser: Send + Sync {
    /// Quick check if this parser should run
    fn should_apply(&self, context: &ParsingContext) -> bool;
    
    /// Extract date/time expressions
    fn parse(&self, context: &ParsingContext) -> Result<Vec<ParsedResult>>;
}
```

Parser types include:

| Parser | Purpose |
|--------|---------|
| `ISOFormatParser` | ISO 8601 dates (2024-12-25) |
| `SlashDateParser` | Slash dates (12/25/2024) |
| `MonthNameParser` | Month names (December 25th) |
| `WeekdayParser` | Weekdays (next Monday) |
| `TimeExpressionParser` | Times (3pm, 15:30) |
| `CasualDateParser` | Casual (today, tomorrow) |
| `CasualTimeParser` | Casual times (noon, midnight) |
| `TimeUnitAgoParser` | Relative past (3 days ago) |
| `TimeUnitWithinParser` | Relative future (in 2 weeks) |

### 3. Refiners

Refiners post-process parsed results:

```rust
pub trait Refiner: Send + Sync {
    fn refine(
        &self,
        context: &ParsingContext,
        results: Vec<ParsedResult>,
    ) -> Vec<ParsedResult>;
}
```

Refiner types include:

| Refiner | Purpose |
|---------|---------|
| `OverlapRemovalRefiner` | Remove overlapping matches |
| `MergeDateTimeRefiner` | Merge date + time into one result |
| `MergeWeekdayDateRefiner` | Merge weekday + date |
| `MergeDateRangeRefiner` | Create ranges from adjacent dates |
| `ForwardDateRefiner` | Adjust ambiguous dates forward |

## Data Flow Example

For input "Meet me next Monday at 3pm":

```
1. Scanner identifies: "next", "Monday", "3pm"

2. Parsers run:
   - WeekdayParser finds "next Monday" → weekday=1
   - TimeExpressionParser finds "3pm" → hour=15

3. Results after parsing:
   [
     { text: "next Monday", start: {weekday: 1} },
     { text: "3pm", start: {hour: 15} }
   ]

4. Refiners run:
   - MergeDateTimeRefiner combines them

5. Final result:
   [
     { text: "next Monday at 3pm", start: {weekday: 1, hour: 15} }
   ]
```

## Locale Support

Each locale has its own set of parsers:

```rust
pub fn create_configuration_for_locale(locale: Locale) -> Configuration {
    match locale {
        Locale::En => Configuration {
            parsers: vec![
                Box::new(ISOFormatParser),
                Box::new(SlashDateParser::new(false)), // MM/DD/YYYY
                Box::new(ENMonthNameParser::new()),
                Box::new(ENWeekdayParser::new()),
                // ...
            ],
            refiners: vec![...],
            locale,
        },
        Locale::De => Configuration {
            parsers: vec![
                Box::new(ISOFormatParser),
                Box::new(SlashDateParser::new(true)), // DD/MM/YYYY
                Box::new(DEMonthNameParser::new()),
                Box::new(DEWeekdayParser::new()),
                // ...
            ],
            refiners: vec![...],
            locale,
        },
        // ... other locales
    }
}
```

## Dictionaries (PHF)

Keywords are stored in compile-time perfect hash maps:

```rust
pub static WEEKDAY_MAP: phf::Map<&'static str, Weekday> = phf_map! {
    "sunday" => Weekday::Sunday,
    "sun" => Weekday::Sunday,
    "monday" => Weekday::Monday,
    "mon" => Weekday::Monday,
    // ...
};
```

Each locale has its own dictionary:

```
src/dictionaries/
├── mod.rs      # Shared types
├── en.rs       # English keywords
├── de.rs       # German keywords
├── fr.rs       # French keywords
└── ...
```

## FFI Layer

The internal `whichtime-ffi` crate wraps `whichtime` for UniFFI exports:

```rust
// whichtime-ffi/src/lib.rs

#[derive(uniffi::Object)]
pub struct WhichTimeParser {
    inner: whichtime::WhichTime,
}

#[uniffi::export]
impl WhichTimeParser {
    #[uniffi::constructor]
    pub fn new() -> Arc<Self> { ... }
    
    pub fn parse(&self, text: String, locale: WhichTimeLocale) 
        -> Result<Vec<WhichTimeResult>, WhichTimeError> { ... }
}
```

UniFFI generates the Swift, Kotlin, and Python bindings from this crate.

## Thread Safety

- `WhichTime` is `Send + Sync`
- Parsers and refiners are `Send + Sync`
- No global mutable state
- FFI bindings use `Arc<WhichTimeParser>`

## Extension Points

### Adding a Parser

1. Implement the `Parser` trait
2. Add to the locale's configuration
3. Add tests

### Adding a Refiner

1. Implement the `Refiner` trait
2. Add to the locale's configuration (order matters)
3. Add tests

### Adding a Locale

1. Create dictionary in `src/dictionaries/<locale>.rs`
2. Create parsers in `src/parsers/<locale>/`
3. Add configuration in `src/lib.rs`
4. Add tests in `tests/<locale>_tests.rs`

