# whichtime

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org/)

High-performance natural language date parsing in Rust.

`whichtime` ships three intentional public Rust packages:

- `whichtime`: the recommended high-level Rust API
- `whichtime-sys`: the lower-level parsing engine crate
- `whichtime-cli`: a small CLI for quick parsing and smoke tests

Swift, Kotlin, and Python bindings are available as preview/source-build integrations until further testing is complete.

## Documentation

- Guide: <https://transcodeworks.github.io/whichtime/>
- Rust docs (`whichtime`): <https://docs.rs/whichtime>
- Rust docs (`whichtime-sys`): <https://docs.rs/whichtime-sys>

## Quick Start

Add whichtime to your Rust project:

```toml
[dependencies]
whichtime = "0.1"
```

```rust
use whichtime::WhichTime;

fn main() -> whichtime::Result<()> {
    let parser = WhichTime::new();

    if let Some(date) = parser.parse_date("tomorrow at 3pm", None)? {
        println!("{}", date.to_rfc3339());
    }

    if let Some(date) = parser.fr().parse_date("demain matin", None)? {
        println!("{}", date.to_rfc3339());
    }

    // Using a reference date
    let reference = chrono::Utc::now();
    if let Some(date) = parser.parse_date("tomorrow at 3pm", Some(reference))? {
        println!("{}", date.to_rfc3339());
    }

    Ok(())
}
```

Use `whichtime-sys` when you want direct access to the lower-level engine, or when you need to build a custom parser pipeline.

Install the CLI with:

```bash
cargo install whichtime-cli
```

## Supported Expressions

`whichtime` can parse casual dates, relative dates, weekdays, explicit dates, time expressions, and date ranges such as:

- `tomorrow at 3pm`
- `in 2 hours`
- `next Friday`
- `December 25th, 2026`
- `Monday to Friday`

## Supported Locales

The Rust crates currently support these locales:

- `en` English
- `de` German
- `es` Spanish
- `fr` French
- `it` Italian
- `ja` Japanese
- `nl` Dutch
- `pt` Portuguese
- `ru` Russian
- `sv` Swedish
- `uk` Ukrainian
- `zh` Chinese

## Package Lineup

- `whichtime`: recommended high-level Rust API for application code
- `whichtime-sys`: lower-level parsing engine for direct parser access and custom integration work
- `whichtime-cli`: command-line interface that prints the first parsed timestamp as RFC 3339 in UTC
- `whichtime-ffi`: internal crate used for UniFFI bindings and shared cross-language transport types

## Preview Bindings

The repository also contains Swift, Kotlin, and Python bindings. 

- Swift: build the XCFramework locally with `./common/build-ios.sh`
- Kotlin: generate bindings locally with `./common/build-kotlin.sh`
- Python: build and install locally with `./common/build-python.sh`

## Development

```bash
cargo test --workspace --all-features
cargo fmt --all -- --check
cargo clippy --workspace --all-features -- -D warnings
```

To validate publishable crates locally:

```bash
cargo package -p whichtime-sys
cargo package --list -p whichtime
cargo package --list -p whichtime-cli
```

## License

MIT. See `LICENSE`.

## Acknowledgments

- [chrono-node](https://github.com/wanasit/chrono) — The JavaScript library that inspired this project
- [UniFFI](https://mozilla.github.io/uniffi-rs/) — Mozilla's FFI bindings generator
- [Ferrostar](https://github.com/stadiamaps/ferrostar) — Inspiration for project structure and build system