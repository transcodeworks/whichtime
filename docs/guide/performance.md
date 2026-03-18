# Performance

whichtime is designed for high-performance parsing. This guide explains the optimizations and how to get the best performance.

## Performance Characteristics

whichtime achieves high performance through three main optimizations:

### 1. Aho-Corasick Scanner

Before running individual parsers, whichtime pre-scans the input text using an [Aho-Corasick](https://en.wikipedia.org/wiki/Aho%E2%80%93Corasick_algorithm) automaton:

- **Single pass** over the input text
- **SIMD-optimized** via the `aho-corasick` crate
- Identifies all potential date-related tokens at once
- Enables fast-path filtering with `should_apply()`

This means parsers can quickly determine if they should even attempt to parse the text.

### 2. PHF Dictionaries

Keywords (months, weekdays, time units, etc.) are stored in compile-time [perfect hash functions](https://en.wikipedia.org/wiki/Perfect_hash_function):

- **O(1) guaranteed** lookup time
- **No runtime hashing** - computed at compile time
- **Zero heap allocations** for lookups
- Maps stored in read-only `.rodata` section

```rust
// Example: Month name lookup is O(1)
pub static MONTH_MAP: phf::Map<&'static str, u32> = phf_map! {
    "january" => 1, "jan" => 1,
    "february" => 2, "feb" => 2,
    // ...
};
```

### 3. FastComponents

Date/time components use an optimized storage format:

- **Fixed-size array** `[i32; 10]` instead of `HashMap`
- **Bitflags** for tracking known vs. implied components
- **Copy semantics** - no heap allocation, no cloning cost
- Fits in ~44 bytes (cache-line friendly)

```rust
#[derive(Clone, Copy, Default)]
pub struct FastComponents {
    values: [i32; 10],      // Component values
    known: ComponentFlags,   // Known (certain) components
    implied: ComponentFlags, // Implied components
}
```

## Benchmarking

Run the included benchmarks:

```bash
cargo bench -p whichtime-sys
```

This uses [Criterion](https://github.com/bheisler/criterion.rs) and produces HTML reports in `target/criterion/`.

### Benchmark Categories

- **Simple inputs** - Single expressions like "tomorrow"
- **Long text** - Paragraphs with embedded dates
- **Pathological** - Edge cases and complex expressions
- **Locales** - Per-locale parsing performance

## Performance Tips

### 1. Reuse Parser Instances

Creating a parser involves initializing regex patterns and other state. Reuse parsers:

```rust
// Good: Create once
let parser = WhichTime::new();
for text in inputs {
    parser.parse_date(text, None)?;
}

// Bad: Create each time
for text in inputs {
    let parser = WhichTime::new(); // Wasteful
    parser.parse_date(text, None)?;
}
```

### 2. Use the Right Method

If you only need the first date, use `parse_date()` instead of `parse()`:

```rust
// More efficient if you only need the first date
let date = parser.parse_date("text with date", None)?;

// Less efficient - parses all dates, then you take the first
let results = parser.parse("text with date", None)?;
let date = results.first();
```

### 3. Consider Text Length

For very long texts, parsing time scales roughly linearly. If you have extremely long documents, consider:

- Chunking the text into smaller pieces
- Pre-filtering to identify relevant sections
- Using the scanner to check if dates exist before full parsing

### 4. Batch Processing

When processing many texts, batch them to amortize any overhead:

```rust
let parser = WhichTime::new();
let results: Vec<_> = inputs
    .iter()
    .map(|text| parser.parse_date(text, None))
    .collect();
```

## Memory Usage

whichtime is designed for minimal memory allocation:

- **Parser state** - Fixed size, allocated once
- **Results** - Allocated per parse, proportional to matches found
- **Components** - Stack-allocated (Copy type)
- **Dictionaries** - Static, read-only memory

For typical inputs, memory usage is minimal. Very long texts with many matches will use more memory for results.

## Comparison with Alternatives

While we don't publish formal benchmarks against other libraries, whichtime is designed to be competitive with or faster than JavaScript-based solutions, especially for:

- Batch processing of many inputs
- Server-side applications
- Mobile applications where startup time matters

## Profiling

To profile whichtime in your application:

```bash
# Using flamegraph (requires cargo-flamegraph)
cargo flamegraph --bench comparison -p whichtime-sys

# Using perf (Linux)
perf record --call-graph dwarf cargo bench -p whichtime-sys
perf report
```

## Reporting Performance Issues

If you encounter performance problems:

1. Identify the specific input causing issues
2. Run benchmarks to quantify the problem
3. Open an issue with reproduction steps and benchmark results

