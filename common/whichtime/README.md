# whichtime

`whichtime` is a natural language date parsing library for Rust.

It provides the ergonomic, high-level API for parsing phrases like "tomorrow at 3pm", "next Friday", or "in two weeks" across the locales supported by the project.

## Installation

```toml
[dependencies]
whichtime = "0.1"
```

## Example

```rust
use whichtime::WhichTime;

fn main() -> whichtime::Result<()> {
    let parser = WhichTime::new();

    if let Some(date) = parser.parse_date("tomorrow at 3pm", None)? {
        println!("{}", date.to_rfc3339());
    }

    Ok(())
}
```

## Documentation

- Guide: <https://transcodeworks.github.io/whichtime/>
- API docs: <https://docs.rs/whichtime>
- Lower-level engine: <https://docs.rs/whichtime-sys>
