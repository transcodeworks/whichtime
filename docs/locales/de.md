# German (de)

German locale support in whichtime.

## Casual Dates

| Expression | Meaning |
|------------|---------|
| heute | today |
| morgen | tomorrow |
| gestern | yesterday |
| übermorgen | day after tomorrow |
| vorgestern | day before yesterday |

## Relative Dates

### Past

| Expression | Meaning |
|------------|---------|
| vor X Tagen | X days ago |
| vor X Wochen | X weeks ago |
| vor X Monaten | X months ago |
| letzte Woche | last week |
| letzten Monat | last month |
| letztes Jahr | last year |

### Future

| Expression | Meaning |
|------------|---------|
| in X Tagen | in X days |
| in X Wochen | in X weeks |
| in einer Woche | in a week |
| nächste Woche | next week |
| nächsten Monat | next month |
| nächstes Jahr | next year |

## Weekdays

| Expression | Meaning |
|------------|---------|
| Montag | Monday |
| Dienstag | Tuesday |
| Mittwoch | Wednesday |
| Donnerstag | Thursday |
| Freitag | Friday |
| Samstag | Saturday |
| Sonntag | Sunday |
| nächsten Montag | next Monday |
| letzten Freitag | last Friday |

## Month Names

| Expression | Month |
|------------|-------|
| Januar | 1 |
| Februar | 2 |
| März | 3 |
| April | 4 |
| Mai | 5 |
| Juni | 6 |
| Juli | 7 |
| August | 8 |
| September | 9 |
| Oktober | 10 |
| November | 11 |
| Dezember | 12 |

## Time Expressions

| Expression | Time |
|------------|------|
| 15 Uhr | 15:00 |
| 15:30 Uhr | 15:30 |
| um 15 Uhr | at 15:00 |
| mittags | noon |
| mitternacht | midnight |
| morgens | morning |
| nachmittags | afternoon |
| abends | evening |

## Date Formats

German uses DD/MM/YYYY format:

| Format | Example |
|--------|---------|
| DD.MM.YYYY | 25.12.2024 |
| DD/MM/YYYY | 25/12/2024 |
| D. Month YYYY | 25. Dezember 2024 |

## Examples

```python
from whichtime import parse_with_locale, WhichTimeLocale

# Casual
parse_with_locale("heute", WhichTimeLocale.DE)
parse_with_locale("morgen", WhichTimeLocale.DE)

# Relative
parse_with_locale("in 2 Tagen", WhichTimeLocale.DE)
parse_with_locale("vor 3 Wochen", WhichTimeLocale.DE)

# Combined
parse_with_locale("morgen um 15 Uhr", WhichTimeLocale.DE)
parse_with_locale("nächsten Montag", WhichTimeLocale.DE)
```

