# Spanish (es)

Spanish locale support in whichtime.

## Casual Dates

| Expression | Meaning |
|------------|---------|
| hoy | today |
| mañana | tomorrow |
| ayer | yesterday |
| pasado mañana | day after tomorrow |
| anteayer | day before yesterday |

## Relative Dates

### Past

| Expression | Meaning |
|------------|---------|
| hace X días | X days ago |
| hace X semanas | X weeks ago |
| la semana pasada | last week |
| el mes pasado | last month |
| el año pasado | last year |

### Future

| Expression | Meaning |
|------------|---------|
| en X días | in X days |
| dentro de X días | in X days |
| la próxima semana | next week |
| el próximo mes | next month |
| el próximo año | next year |

## Weekdays

| Expression | Meaning |
|------------|---------|
| lunes | Monday |
| martes | Tuesday |
| miércoles | Wednesday |
| jueves | Thursday |
| viernes | Friday |
| sábado | Saturday |
| domingo | Sunday |
| el próximo lunes | next Monday |
| el viernes pasado | last Friday |

## Month Names

| Expression | Month |
|------------|-------|
| enero | 1 |
| febrero | 2 |
| marzo | 3 |
| abril | 4 |
| mayo | 5 |
| junio | 6 |
| julio | 7 |
| agosto | 8 |
| septiembre | 9 |
| octubre | 10 |
| noviembre | 11 |
| diciembre | 12 |

## Time Expressions

| Expression | Time |
|------------|------|
| 3pm | 15:00 |
| las 15:30 | 15:30 |
| a las 3pm | at 15:00 |
| mediodía | noon |
| medianoche | midnight |
| por la mañana | in the morning |
| por la tarde | in the afternoon |
| por la noche | at night |

## Date Formats

Spanish uses DD/MM/YYYY format:

| Format | Example |
|--------|---------|
| DD/MM/YYYY | 25/12/2024 |
| D de Month de YYYY | 25 de diciembre de 2024 |

## Examples

```python
from whichtime import parse_with_locale, WhichTimeLocale

# Casual
parse_with_locale("hoy", WhichTimeLocale.ES)
parse_with_locale("mañana", WhichTimeLocale.ES)

# Relative
parse_with_locale("en 2 días", WhichTimeLocale.ES)
parse_with_locale("hace 3 semanas", WhichTimeLocale.ES)

# Combined
parse_with_locale("mañana a las 3pm", WhichTimeLocale.ES)
parse_with_locale("el próximo lunes", WhichTimeLocale.ES)
```

