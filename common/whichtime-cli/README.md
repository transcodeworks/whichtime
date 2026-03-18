# whichtime-cli

`whichtime-cli` is a small command-line interface for quickly parsing natural language date expressions from a terminal.

It is useful for smoke testing expressions, trying different phrases during development, and scripting simple parsing workflows.

## Installation

```bash
cargo install whichtime-cli
```

## Example

```bash
whichtime tomorrow at noon
whichtime "next Friday 6pm"
```

## Output

The CLI prints the first parsed date as an RFC 3339 timestamp in UTC.
