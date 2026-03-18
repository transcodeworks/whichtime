# WhichTime Python Example

This example demonstrates the Python bindings generated from the Rust core library.

## Install and Run

From the repository root:

```bash
./common/build-python.sh
cd python
pip install -e .
cd ../examples/python
python example.py
```

If you do not want to install the package, `example.py` can also run directly because it adds the local bindings directory to `sys.path`.

## Generated Files

The build step generates:

- `python/whichtime/whichtime.py`
- `python/whichtime/libuniffi_whichtime.dylib` on macOS, or the matching shared library on other platforms

## API at a Glance

### Convenience functions

- `parse(text)` parses with the default English configuration
- `parse_date(text)` returns the first parsed date as Unix milliseconds
- `parse_with_locale(text, locale)` parses with an explicit locale
- `parse_date_with_locale(text, locale)` returns the first parsed date for a locale

### Main types

- `WhichTimeParser` for repeated parsing
- `WhichTimeLocale` for locale selection
- `WhichTimeResult` for matched text, indexes, timestamps, and parsed components

## Example Snippet

```python
from datetime import datetime
from whichtime import parse, WhichTimeLocale, WhichTimeParser

results = parse("tomorrow at 3pm")
parser = WhichTimeParser()
french_results = parser.parse("demain matin", WhichTimeLocale.FR)

for result in results:
    if result.date_millis is not None:
        print(datetime.fromtimestamp(result.date_millis / 1000.0))
```

## Error Handling

Parsing normally returns an empty result set when nothing matches, but binding or parsing issues can still raise errors:

```python
from whichtime import WhichTimeError, parse

try:
    results = parse("tomorrow at 3pm")
except WhichTimeError as error:
    print(f"whichtime failed: {error}")
```

If imports fail, rebuild the bindings with `./common/build-python.sh` and confirm the shared library exists under `python/whichtime/`.

## Learn More

- [Installation guide](../../docs/guide/installation.md)
- [Getting started guide](../../docs/guide/getting-started.md)
- [Python language docs](../../docs/languages/python.md)

