# whichtime-sys

`whichtime-sys` is the lower-level Rust parsing engine that powers the higher-level [`whichtime`](https://docs.rs/whichtime) crate.

Use this crate when you want direct access to the engine itself, its locale-specific parser construction, or its lower-level types without the higher-level convenience wrapper.

## Installation

```toml
[dependencies]
whichtime-sys = "0.1"
```

## When To Use `whichtime-sys`

- You want direct control over the underlying parser engine.
- You are building custom parser/refiner pipelines.
- You want the lower-level crate that `whichtime` builds on top of.

If you just want the recommended Rust API for application code, use [`whichtime`](https://docs.rs/whichtime) instead.

## Example

```rust
use whichtime_sys::{Locale, WhichTime};

fn main() -> whichtime_sys::Result<()> {
    let parser = WhichTime::with_locale(Locale::En);

    if let Some(date) = parser.parse_date("next Friday at noon", None)? {
        println!("{}", date.to_rfc3339());
    }

    Ok(())
}
```

## Performance Notes

The engine uses a parser pipeline with fast token scanning, compile-time dictionaries, and compact component storage to keep parsing efficient. Benchmark and implementation details live in the repository guide.

## Development Commands

```bash
cargo test -p whichtime-sys
cargo bench -p whichtime-sys
```

## Documentation

- Guide: <https://transcodeworks.github.io/whichtime/>
- API docs: <https://docs.rs/whichtime-sys>
- Recommended high-level crate: <https://docs.rs/whichtime>
