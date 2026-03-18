# Japanese (ja)

Japanese locale support in whichtime.

## Casual Dates

| Expression | Meaning |
|------------|---------|
| 今日 | today |
| 明日 | tomorrow |
| 昨日 | yesterday |
| 明後日 | day after tomorrow |
| 一昨日 | day before yesterday |

## Relative Dates

### Past

| Expression | Meaning |
|------------|---------|
| X日前 | X days ago |
| X週間前 | X weeks ago |
| X ヶ月前 | X months ago |
| 先週 | last week |
| 先月 | last month |
| 去年 | last year |

### Future

| Expression | Meaning |
|------------|---------|
| X日後 | in X days |
| X週間後 | in X weeks |
| 来週 | next week |
| 来月 | next month |
| 来年 | next year |

## Weekdays

| Expression | Meaning |
|------------|---------|
| 月曜日 | Monday |
| 火曜日 | Tuesday |
| 水曜日 | Wednesday |
| 木曜日 | Thursday |
| 金曜日 | Friday |
| 土曜日 | Saturday |
| 日曜日 | Sunday |
| 来週月曜日 | next Monday |

## Date Formats

Japanese uses YYYY/MM/DD format:

| Format | Example |
|--------|---------|
| YYYY年MM月DD日 | 2024年12月25日 |
| YYYY/MM/DD | 2024/12/25 |
| MM月DD日 | 12月25日 |

## Time Expressions

| Expression | Time |
|------------|------|
| 15時 | 15:00 |
| 15時30分 | 15:30 |
| 午後3時 | 15:00 (3pm) |
| 午前9時 | 09:00 (9am) |
| 正午 | noon |
| 深夜 | midnight |
| 朝 | morning |
| 午後 | afternoon |
| 夜 | night |

## Examples

```python
from whichtime import parse_with_locale, WhichTimeLocale

# Casual
parse_with_locale("今日", WhichTimeLocale.JA)
parse_with_locale("明日", WhichTimeLocale.JA)

# Relative
parse_with_locale("2日後", WhichTimeLocale.JA)
parse_with_locale("3週間前", WhichTimeLocale.JA)

# Combined
parse_with_locale("明日の午後3時", WhichTimeLocale.JA)
parse_with_locale("来週月曜日", WhichTimeLocale.JA)

# Date formats
parse_with_locale("2024年12月25日", WhichTimeLocale.JA)
parse_with_locale("12月25日", WhichTimeLocale.JA)
```

