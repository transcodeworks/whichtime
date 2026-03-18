# Swedish (sv)

Swedish locale support in whichtime.

## Casual Dates

| Expression | Meaning |
|------------|---------|
| idag | today |
| imorgon | tomorrow |
| igår | yesterday |
| i övermorgon | day after tomorrow |
| i förrgår | day before yesterday |

## Weekdays

| Expression | Meaning |
|------------|---------|
| måndag | Monday |
| tisdag | Tuesday |
| onsdag | Wednesday |
| torsdag | Thursday |
| fredag | Friday |
| lördag | Saturday |
| söndag | Sunday |

## Month Names

| Expression | Month |
|------------|-------|
| januari | 1 |
| februari | 2 |
| mars | 3 |
| april | 4 |
| maj | 5 |
| juni | 6 |
| juli | 7 |
| augusti | 8 |
| september | 9 |
| oktober | 10 |
| november | 11 |
| december | 12 |

## Date Formats

Swedish uses DD/MM/YYYY format:

| Format | Example |
|--------|---------|
| YYYY-MM-DD | 2024-12-25 |
| D Month YYYY | 25 december 2024 |

## Examples

```python
from whichtime import parse_with_locale, WhichTimeLocale

parse_with_locale("idag", WhichTimeLocale.SV)
parse_with_locale("imorgon", WhichTimeLocale.SV)
parse_with_locale("imorgon kl 15", WhichTimeLocale.SV)
```

