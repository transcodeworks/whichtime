# Italian (it)

Italian locale support in whichtime.

## Casual Dates

| Expression | Meaning |
|------------|---------|
| oggi | today |
| domani | tomorrow |
| ieri | yesterday |
| dopodomani | day after tomorrow |
| l'altro ieri | day before yesterday |

## Weekdays

| Expression | Meaning |
|------------|---------|
| lunedì | Monday |
| martedì | Tuesday |
| mercoledì | Wednesday |
| giovedì | Thursday |
| venerdì | Friday |
| sabato | Saturday |
| domenica | Sunday |

## Month Names

| Expression | Month |
|------------|-------|
| gennaio | 1 |
| febbraio | 2 |
| marzo | 3 |
| aprile | 4 |
| maggio | 5 |
| giugno | 6 |
| luglio | 7 |
| agosto | 8 |
| settembre | 9 |
| ottobre | 10 |
| novembre | 11 |
| dicembre | 12 |

## Date Formats

Italian uses DD/MM/YYYY format:

| Format | Example |
|--------|---------|
| DD/MM/YYYY | 25/12/2024 |
| D Month YYYY | 25 dicembre 2024 |

## Examples

```python
from whichtime import parse_with_locale, WhichTimeLocale

parse_with_locale("oggi", WhichTimeLocale.IT)
parse_with_locale("domani", WhichTimeLocale.IT)
parse_with_locale("domani alle 15", WhichTimeLocale.IT)
```

