# English (en)

English is the default and most complete locale in whichtime.

## Casual Dates

| Expression | Meaning |
|------------|---------|
| today | Current day |
| tomorrow | Next day |
| yesterday | Previous day |
| now | Current moment |

## Relative Dates

### Past

| Expression | Meaning |
|------------|---------|
| X days ago | X days in the past |
| X weeks ago | X weeks in the past |
| X months ago | X months in the past |
| X years ago | X years in the past |
| last week | Previous week |
| last month | Previous month |
| last year | Previous year |

### Future

| Expression | Meaning |
|------------|---------|
| in X days | X days from now |
| in X weeks | X weeks from now |
| in X months | X months from now |
| in X years | X years from now |
| next week | Coming week |
| next month | Coming month |
| next year | Coming year |

## Weekdays

| Expression | Meaning |
|------------|---------|
| Monday, Mon | Weekday 1 |
| Tuesday, Tue, Tues | Weekday 2 |
| Wednesday, Wed | Weekday 3 |
| Thursday, Thu, Thurs | Weekday 4 |
| Friday, Fri | Weekday 5 |
| Saturday, Sat | Weekday 6 |
| Sunday, Sun | Weekday 0 |
| next Monday | Coming Monday |
| last Friday | Previous Friday |
| this Saturday | Saturday of current week |

## Month Names

| Expression | Month |
|------------|-------|
| January, Jan | 1 |
| February, Feb | 2 |
| March, Mar | 3 |
| April, Apr | 4 |
| May | 5 |
| June, Jun | 6 |
| July, Jul | 7 |
| August, Aug | 8 |
| September, Sep, Sept | 9 |
| October, Oct | 10 |
| November, Nov | 11 |
| December, Dec | 12 |

## Date Formats

| Format | Example |
|--------|---------|
| MM/DD/YYYY | 12/25/2024 |
| MM-DD-YYYY | 12-25-2024 |
| Month DD, YYYY | December 25, 2024 |
| Month DD | December 25 |
| DDth of Month | 25th of December |
| YYYY-MM-DD (ISO) | 2024-12-25 |

## Time Expressions

| Expression | Time |
|------------|------|
| 3pm | 15:00 |
| 3:30pm | 15:30 |
| 3:30 PM | 15:30 |
| 15:30 | 15:30 |
| 3:30:45 | 03:30:45 |
| noon | 12:00 |
| midnight | 00:00 |
| morning | ~08:00 (implied) |
| afternoon | ~14:00 (implied) |
| evening | ~18:00 (implied) |
| night | ~20:00 (implied) |

## Combined Expressions

| Expression | Result |
|------------|--------|
| tomorrow at 3pm | Next day at 15:00 |
| next Monday at 9:30am | Coming Monday at 09:30 |
| December 25th at noon | Dec 25 at 12:00 |
| in 2 days at 5pm | 2 days from now at 17:00 |

## Date Ranges

| Expression | Result |
|------------|--------|
| Monday to Friday | Weekday range |
| from Monday to Friday | Weekday range |
| January 1 to January 15 | Date range |
| from 9am to 5pm | Time range |

## Examples

```python
from whichtime import parse

# Casual
parse("today")
parse("tomorrow")
parse("yesterday")

# Relative
parse("in 2 hours")
parse("3 days ago")
parse("next week")
parse("last month")

# Weekdays
parse("Monday")
parse("next Friday")
parse("last Tuesday at 9:30")

# Explicit dates
parse("December 25, 2024")
parse("12/25/2024")
parse("2024-12-25")

# Times
parse("3pm")
parse("15:30")
parse("noon")

# Combined
parse("tomorrow at 3pm")
parse("next Monday morning")
```

