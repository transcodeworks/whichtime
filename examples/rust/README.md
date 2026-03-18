# WhichTime Rust Example

This example shows how to use `whichtime-sys` directly from Rust.

## Run the Example

From the repository root:

```bash
cargo run -p whichtime-example-rust
```

From the example directory:

```bash
cargo run
```

## Dependency

Add the core crate to your `Cargo.toml`:

```toml
[dependencies]
whichtime-sys = "0.1"
```

In this workspace, the example uses the local workspace dependency defined in the top-level `Cargo.toml`.

## Common Use Cases

### Parse the First Date

```rust
use whichtime_sys::WhichTime;

let parser = WhichTime::new();
let date = parser.parse_date("tomorrow at 3pm", None)?;
```

### Parse with a Specific Locale

```rust
use whichtime_sys::{Locale, WhichTime};

let parser = WhichTime::with_locale(Locale::De);
let date = parser.parse_date("morgen um 15 Uhr", None)?;
```

### Use a Custom Configuration

```rust
use whichtime_sys::{Configuration, Locale, WhichTime};

let config = Configuration {
    parsers: vec![],
    refiners: vec![],
    locale: Locale::En,
};

let parser = WhichTime::with_configuration(config);
```

## What the Example Covers

`src/main.rs` exercises a mix of:

- absolute date formats
- casual expressions such as `tomorrow at noon`
- relative expressions such as `in 2 hours`
- ranges and edge cases

## Learn More

- [Root README](../../README.md)
- [Getting started guide](../../docs/guide/getting-started.md)
- [Architecture guide](../../docs/guide/architecture.md)
- [API reference overview](../../docs/api/index.md)
