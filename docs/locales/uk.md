# Ukrainian (uk)

Ukrainian locale support in whichtime.

## Casual Dates

| Expression | Meaning |
|------------|---------|
| сьогодні | today |
| завтра | tomorrow |
| вчора | yesterday |
| післязавтра | day after tomorrow |
| позавчора | day before yesterday |

## Weekdays

| Expression | Meaning |
|------------|---------|
| понеділок | Monday |
| вівторок | Tuesday |
| середа | Wednesday |
| четвер | Thursday |
| п'ятниця | Friday |
| субота | Saturday |
| неділя | Sunday |

## Month Names

| Expression | Month |
|------------|-------|
| січень | 1 |
| лютий | 2 |
| березень | 3 |
| квітень | 4 |
| травень | 5 |
| червень | 6 |
| липень | 7 |
| серпень | 8 |
| вересень | 9 |
| жовтень | 10 |
| листопад | 11 |
| грудень | 12 |

## Date Formats

Ukrainian uses DD.MM.YYYY format:

| Format | Example |
|--------|---------|
| DD.MM.YYYY | 25.12.2024 |
| D Month YYYY | 25 грудня 2024 |

## Examples

```python
from whichtime import parse_with_locale, WhichTimeLocale

parse_with_locale("сьогодні", WhichTimeLocale.UK)
parse_with_locale("завтра", WhichTimeLocale.UK)
parse_with_locale("завтра о 15:00", WhichTimeLocale.UK)
```

