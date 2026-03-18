# Portuguese (pt)

Portuguese locale support in whichtime.

## Casual Dates

| Expression | Meaning |
|------------|---------|
| hoje | today |
| amanhã | tomorrow |
| ontem | yesterday |
| depois de amanhã | day after tomorrow |
| anteontem | day before yesterday |

## Weekdays

| Expression | Meaning |
|------------|---------|
| segunda-feira | Monday |
| terça-feira | Tuesday |
| quarta-feira | Wednesday |
| quinta-feira | Thursday |
| sexta-feira | Friday |
| sábado | Saturday |
| domingo | Sunday |

## Month Names

| Expression | Month |
|------------|-------|
| janeiro | 1 |
| fevereiro | 2 |
| março | 3 |
| abril | 4 |
| maio | 5 |
| junho | 6 |
| julho | 7 |
| agosto | 8 |
| setembro | 9 |
| outubro | 10 |
| novembro | 11 |
| dezembro | 12 |

## Date Formats

Portuguese uses DD/MM/YYYY format:

| Format | Example |
|--------|---------|
| DD/MM/YYYY | 25/12/2024 |
| D de Month de YYYY | 25 de dezembro de 2024 |

## Examples

```python
from whichtime import parse_with_locale, WhichTimeLocale

parse_with_locale("hoje", WhichTimeLocale.PT)
parse_with_locale("amanhã", WhichTimeLocale.PT)
parse_with_locale("amanhã às 15h", WhichTimeLocale.PT)
```

