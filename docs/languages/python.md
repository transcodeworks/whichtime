# Python

Using whichtime in Python applications.

## Installation

Python bindings are currently a preview/source-build integration. A published package will be provided in the future. For now, you can build the bindings from source.

### Build from Source

```bash
# Build bindings
./common/build-python.sh

# Install in development mode
cd python
pip install -e .
```

## Basic Usage

```python
from whichtime import parse, parse_date
from datetime import datetime

# Parse and get all results
results = parse("tomorrow at 3pm")
for result in results:
    print(f"Found: {result.text}")
    if result.date_millis:
        dt = datetime.fromtimestamp(result.date_millis / 1000.0)
        print(f"Date: {dt}")

# Get just the first date
timestamp = parse_date("tomorrow at 3pm")
if timestamp:
    dt = datetime.fromtimestamp(timestamp / 1000.0)
    print(f"Date: {dt}")
```

## Using the Parser Object

```python
from whichtime import WhichTimeParser, WhichTimeLocale

parser = WhichTimeParser()

# Parse with specific locale
results = parser.parse("demain matin", WhichTimeLocale.FR)

# Parse with English locale
english_results = parser.parse_en("tomorrow morning")

# Get first date with locale
timestamp = parser.parse_date("morgen", WhichTimeLocale.DE)
if timestamp:
    print(f"Timestamp: {timestamp}ms")
```

## Working with Results

```python
from whichtime import parse
from datetime import datetime

results = parse("December 25, 2024 at 3pm")

if results:
    result = results[0]
    
    # Access matched text and position
    print(f"Text: {result.text}")
    print(f"Position: {result.index}...{result.end_index}")
    
    # Access components
    c = result.start
    if c.year: print(f"Year: {c.year}")
    if c.month: print(f"Month: {c.month}")
    if c.day: print(f"Day: {c.day}")
    if c.hour: print(f"Hour: {c.hour}")
    
    # Get as datetime
    if result.date_millis:
        dt = datetime.fromtimestamp(result.date_millis / 1000.0)
        print(f"Date: {dt}")
    
    # Check for range
    if result.end:
        print("This is a range")
        if result.end.day:
            print(f"End day: {result.end.day}")
```

## Error Handling

```python
from whichtime import parse, WhichTimeError

try:
    results = parse("some text")
    if not results:
        print("No date found")
except WhichTimeError.LocaleNotFound as e:
    print(f"Unknown locale: {e.locale}")
except WhichTimeError.InvalidDateTime as e:
    print(f"Invalid date/time: {e.message}")
except WhichTimeError.ParseError as e:
    print(f"Parse error: {e.message}")
except Exception as e:
    print(f"Error: {e}")
```

## Using Multiple Locales

```python
from whichtime import WhichTimeParser, WhichTimeLocale
from datetime import datetime

parser = WhichTimeParser()

phrases = [
    ("tomorrow", WhichTimeLocale.EN),
    ("demain", WhichTimeLocale.FR),
    ("morgen", WhichTimeLocale.DE),
    ("mañana", WhichTimeLocale.ES),
    ("明日", WhichTimeLocale.JA),
]

for phrase, locale in phrases:
    try:
        results = parser.parse(phrase, locale)
        if results and results[0].date_millis:
            dt = datetime.fromtimestamp(results[0].date_millis / 1000.0)
            print(f"{phrase} ({locale.name}): {dt}")
    except Exception as e:
        print(f"{phrase}: Error - {e}")
```

## Helper Functions

```python
from whichtime import parse, parse_date
from datetime import datetime
from typing import Optional

def parse_to_datetime(text: str) -> Optional[datetime]:
    """Parse text and return as datetime, or None."""
    timestamp = parse_date(text)
    if timestamp:
        return datetime.fromtimestamp(timestamp / 1000.0)
    return None

def parse_safe(text: str, default: datetime = None) -> datetime:
    """Parse text with a fallback default."""
    try:
        result = parse_to_datetime(text)
        return result if result else default
    except Exception:
        return default

# Usage
dt = parse_to_datetime("tomorrow at 3pm")
safe_dt = parse_safe("invalid", default=datetime.now())
```

## Django Integration Example

```python
# models.py
from django.db import models

class Event(models.Model):
    title = models.CharField(max_length=200)
    date_expression = models.CharField(max_length=200)
    resolved_date = models.DateTimeField(null=True, blank=True)
    
    def save(self, *args, **kwargs):
        from whichtime import parse_date
        from datetime import datetime
        
        if self.date_expression:
            timestamp = parse_date(self.date_expression)
            if timestamp:
                self.resolved_date = datetime.fromtimestamp(timestamp / 1000.0)
        
        super().save(*args, **kwargs)

# views.py
from whichtime import parse_date
from datetime import datetime
from django.http import JsonResponse

def parse_date_view(request):
    text = request.GET.get('text', '')
    timestamp = parse_date(text)
    
    if timestamp:
        dt = datetime.fromtimestamp(timestamp / 1000.0)
        return JsonResponse({
            'success': True,
            'datetime': dt.isoformat(),
            'timestamp_ms': timestamp,
        })
    
    return JsonResponse({
        'success': False,
        'error': 'No date found',
    })
```

## FastAPI Integration Example

```python
from fastapi import FastAPI, Query
from datetime import datetime
from whichtime import parse_date, WhichTimeError

app = FastAPI()

@app.get("/parse")
async def parse_date_endpoint(
    text: str = Query(..., description="Date expression to parse")
):
    try:
        timestamp = parse_date(text)
        if timestamp:
            dt = datetime.fromtimestamp(timestamp / 1000.0)
            return {
                "success": True,
                "datetime": dt.isoformat(),
                "timestamp_ms": timestamp,
            }
        return {"success": False, "error": "No date found"}
    except WhichTimeError as e:
        return {"success": False, "error": str(e)}
```

## API Reference

### Top-Level Functions

```python
def parse(text: str) -> list[WhichTimeResult]:
    """Parse with English locale, return all results."""

def parse_date(text: str) -> int | None:
    """Parse with English locale, return first date as timestamp (ms)."""

def parse_with_locale(text: str, locale: WhichTimeLocale) -> list[WhichTimeResult]:
    """Parse with specific locale, return all results."""

def parse_date_with_locale(text: str, locale: WhichTimeLocale) -> int | None:
    """Parse with specific locale, return first date as timestamp (ms)."""
```

### WhichTimeParser

```python
class WhichTimeParser:
    def __init__(self) -> None: ...
    
    def parse(self, text: str, locale: WhichTimeLocale) -> list[WhichTimeResult]: ...
    def parse_date(self, text: str, locale: WhichTimeLocale) -> int | None: ...
    def parse_en(self, text: str) -> list[WhichTimeResult]: ...
    def parse_date_en(self, text: str) -> int | None: ...
```

### WhichTimeResult

```python
@dataclass
class WhichTimeResult:
    index: int
    end_index: int
    text: str
    date_millis: int | None
    start: ParsedComponents
    end: ParsedComponents | None
```

### ParsedComponents

```python
@dataclass
class ParsedComponents:
    year: int | None
    month: int | None
    day: int | None
    hour: int | None
    minute: int | None
    second: int | None
    millisecond: int | None
    weekday: int | None
    timezone_offset: int | None
```

### WhichTimeLocale

```python
class WhichTimeLocale(Enum):
    EN = ...  # English
    DE = ...  # German
    ES = ...  # Spanish
    FR = ...  # French
    IT = ...  # Italian
    JA = ...  # Japanese
    NL = ...  # Dutch
    PT = ...  # Portuguese
    RU = ...  # Russian
    SV = ...  # Swedish
    UK = ...  # Ukrainian
    ZH = ...  # Chinese
```

