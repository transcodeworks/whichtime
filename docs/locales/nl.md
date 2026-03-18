# Dutch (nl)

Dutch locale support in whichtime.

## Casual Dates

| Expression | Meaning |
|------------|---------|
| vandaag | today |
| morgen | tomorrow |
| gisteren | yesterday |
| overmorgen | day after tomorrow |
| eergisteren | day before yesterday |

## Weekdays

| Expression | Meaning |
|------------|---------|
| maandag | Monday |
| dinsdag | Tuesday |
| woensdag | Wednesday |
| donderdag | Thursday |
| vrijdag | Friday |
| zaterdag | Saturday |
| zondag | Sunday |

## Month Names

| Expression | Month |
|------------|-------|
| januari | 1 |
| februari | 2 |
| maart | 3 |
| april | 4 |
| mei | 5 |
| juni | 6 |
| juli | 7 |
| augustus | 8 |
| september | 9 |
| oktober | 10 |
| november | 11 |
| december | 12 |

## Date Formats

Dutch uses DD/MM/YYYY format:

| Format | Example |
|--------|---------|
| DD-MM-YYYY | 25-12-2024 |
| D Month YYYY | 25 december 2024 |

## Examples

```python
from whichtime import parse_with_locale, WhichTimeLocale

parse_with_locale("vandaag", WhichTimeLocale.NL)
parse_with_locale("morgen", WhichTimeLocale.NL)
parse_with_locale("morgen om 15:00", WhichTimeLocale.NL)
```

