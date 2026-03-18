# Chinese (zh)

Chinese locale support in whichtime.

## Casual Dates

| Expression | Meaning |
|------------|---------|
| 今天 | today |
| 明天 | tomorrow |
| 昨天 | yesterday |
| 后天 | day after tomorrow |
| 前天 | day before yesterday |

## Relative Dates

### Past

| Expression | Meaning |
|------------|---------|
| X天前 | X days ago |
| X周前 | X weeks ago |
| X个月前 | X months ago |
| 上周 | last week |
| 上个月 | last month |
| 去年 | last year |

### Future

| Expression | Meaning |
|------------|---------|
| X天后 | in X days |
| X周后 | in X weeks |
| 下周 | next week |
| 下个月 | next month |
| 明年 | next year |

## Weekdays

| Expression | Meaning |
|------------|---------|
| 星期一 / 周一 | Monday |
| 星期二 / 周二 | Tuesday |
| 星期三 / 周三 | Wednesday |
| 星期四 / 周四 | Thursday |
| 星期五 / 周五 | Friday |
| 星期六 / 周六 | Saturday |
| 星期日 / 周日 | Sunday |
| 下周一 | next Monday |

## Date Formats

Chinese uses YYYY/MM/DD format:

| Format | Example |
|--------|---------|
| YYYY年MM月DD日 | 2024年12月25日 |
| YYYY/MM/DD | 2024/12/25 |
| MM月DD日 | 12月25日 |

## Time Expressions

| Expression | Time |
|------------|------|
| 15点 | 15:00 |
| 15点30分 | 15:30 |
| 下午3点 | 15:00 (3pm) |
| 上午9点 | 09:00 (9am) |
| 中午 | noon |
| 午夜 | midnight |
| 早上 | morning |
| 下午 | afternoon |
| 晚上 | evening |

## Examples

```python
from whichtime import parse_with_locale, WhichTimeLocale

# Casual
parse_with_locale("今天", WhichTimeLocale.ZH)
parse_with_locale("明天", WhichTimeLocale.ZH)

# Relative
parse_with_locale("2天后", WhichTimeLocale.ZH)
parse_with_locale("3周前", WhichTimeLocale.ZH)

# Combined
parse_with_locale("明天下午3点", WhichTimeLocale.ZH)
parse_with_locale("下周一", WhichTimeLocale.ZH)

# Date formats
parse_with_locale("2024年12月25日", WhichTimeLocale.ZH)
parse_with_locale("12月25日", WhichTimeLocale.ZH)
```

