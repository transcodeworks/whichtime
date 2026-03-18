# Russian (ru)

Russian locale support in whichtime.

## Casual Dates

| Expression | Meaning |
|------------|---------|
| сегодня | today |
| завтра | tomorrow |
| вчера | yesterday |
| послезавтра | day after tomorrow |
| позавчера | day before yesterday |

## Weekdays

| Expression | Meaning |
|------------|---------|
| понедельник | Monday |
| вторник | Tuesday |
| среда | Wednesday |
| четверг | Thursday |
| пятница | Friday |
| суббота | Saturday |
| воскресенье | Sunday |

## Month Names

| Expression | Month |
|------------|-------|
| январь | 1 |
| февраль | 2 |
| март | 3 |
| апрель | 4 |
| май | 5 |
| июнь | 6 |
| июль | 7 |
| август | 8 |
| сентябрь | 9 |
| октябрь | 10 |
| ноябрь | 11 |
| декабрь | 12 |

## Date Formats

Russian uses DD.MM.YYYY format:

| Format | Example |
|--------|---------|
| DD.MM.YYYY | 25.12.2024 |
| D Month YYYY | 25 декабря 2024 |

## Examples

```python
from whichtime import parse_with_locale, WhichTimeLocale

parse_with_locale("сегодня", WhichTimeLocale.RU)
parse_with_locale("завтра", WhichTimeLocale.RU)
parse_with_locale("завтра в 15:00", WhichTimeLocale.RU)
```

