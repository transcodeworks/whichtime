#!/usr/bin/env python3
"""
WhichTime Python Example

This example demonstrates how to use the WhichTime library from Python.

To run this example:
    1. Build the UniFFI bindings: ./common/build-python.sh
    2. Run: python examples/python/example.py

Or install the package first:
    cd python && pip install -e .
    python examples/python/example.py
"""

import sys
from datetime import datetime
from pathlib import Path

# Add the python bindings to the path
python_bindings = Path(__file__).parent.parent.parent / "python" / "whichtime"
sys.path.insert(0, str(python_bindings.parent))

try:
    from whichtime import (
        parse,
        parse_date,
        parse_with_locale,
        parse_date_with_locale,
        WhichTimeParser,
        WhichTimeLocale,
        WhichTimeResult,
        ParsedComponents,
    )
except ImportError as e:
    print(f"Error importing whichtime: {e}")
    print("\nTo fix this, first build the Python bindings:")
    print("  ./common/build-python.sh")
    print("\nOr install the package:")
    print("  cd python && pip install -e .")
    sys.exit(1)


def millis_to_datetime(millis: int) -> datetime:
    """Convert Unix milliseconds to datetime."""
    return datetime.fromtimestamp(millis / 1000.0)


def main():
    print("=== WhichTime Python Example ===\n")

    # Example 1: Simple parsing with the convenience function
    print("1. Parsing 'tomorrow at 3pm':")
    try:
        results = parse("tomorrow at 3pm")
        for result in results:
            print(f"   Text: '{result.text}'")
            print(f"   Index: {result.index} - {result.end_index}")
            if result.date_millis is not None:
                dt = millis_to_datetime(result.date_millis)
                print(f"   Date: {dt}")
            print(
                f"   Components: year={result.start.year}, month={result.start.month}, day={result.start.day}"
            )
            print()
    except Exception as e:
        print(f"   Error: {e}")

    # Example 2: Using the parser object with different locales
    print("2. Parsing with different locales:")
    parser = WhichTimeParser()

    test_phrases = [
        ("next Monday", WhichTimeLocale.EN),
        ("demain matin", WhichTimeLocale.FR),  # tomorrow morning (French)
        ("nächsten Montag", WhichTimeLocale.DE),  # next Monday (German)
        ("mañana", WhichTimeLocale.ES),  # tomorrow (Spanish)
        ("明日", WhichTimeLocale.JA),  # tomorrow (Japanese)
        ("завтра", WhichTimeLocale.RU),  # tomorrow (Russian)
    ]

    for phrase, locale in test_phrases:
        print(f"   '{phrase}' ({locale.name}):")
        try:
            results = parser.parse(phrase, locale)
            if results:
                first = results[0]
                if first.date_millis is not None:
                    dt = millis_to_datetime(first.date_millis)
                    print(f"      -> {dt}")
                else:
                    print(f"      -> Components: {first.start}")
            else:
                print("      -> No date found")
        except Exception as e:
            print(f"      -> Error: {e}")

    # Example 3: Getting just the date as a timestamp
    print("\n3. Getting date as Unix timestamp:")
    try:
        timestamp = parse_date("December 25, 2024")
        if timestamp is not None:
            dt = millis_to_datetime(timestamp)
            print(f"   'December 25, 2024' -> {timestamp}ms ({dt})")
    except Exception as e:
        print(f"   Error: {e}")

    # Example 4: Parsing various date formats
    print("\n4. Parsing various date formats:")
    test_inputs = [
        "in 2 hours",
        "last Friday",
        "3 days ago",
        "next week",
        "January 15th, 2025",
        "2024-12-25",
        "5:30pm",
        "noon tomorrow",
    ]

    for text in test_inputs:
        try:
            results = parse(text)
            if results:
                first = results[0]
                if first.date_millis is not None:
                    dt = millis_to_datetime(first.date_millis)
                    print(f"   '{text}' -> {dt}")
                else:
                    components = first.start
                    parts = []
                    if components.year:
                        parts.append(f"year={components.year}")
                    if components.month:
                        parts.append(f"month={components.month}")
                    if components.day:
                        parts.append(f"day={components.day}")
                    if components.hour:
                        parts.append(f"hour={components.hour}")
                    if components.minute:
                        parts.append(f"minute={components.minute}")
                    print(f"   '{text}' -> {', '.join(parts) or 'parsed'}")
            else:
                print(f"   '{text}' -> No date found")
        except Exception as e:
            print(f"   '{text}' -> Error: {e}")

    # Example 5: Parsing date ranges
    print("\n5. Parsing date ranges:")
    try:
        results = parse("from Monday to Friday")
        for result in results:
            print(f"   Text: '{result.text}'")
            print(f"   Start: day={result.start.day}, weekday={result.start.weekday}")
            if result.end:
                print(f"   End: day={result.end.day}, weekday={result.end.weekday}")
    except Exception as e:
        print(f"   Error: {e}")

    print("\n=== Done ===")


if __name__ == "__main__":
    main()
