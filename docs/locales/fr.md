# French (fr)

French locale support in whichtime.

## Casual Dates

| Expression | Meaning |
|------------|---------|
| aujourd'hui | today |
| demain | tomorrow |
| hier | yesterday |
| après-demain | day after tomorrow |
| avant-hier | day before yesterday |

## Relative Dates

### Past

| Expression | Meaning |
|------------|---------|
| il y a X jours | X days ago |
| il y a X semaines | X weeks ago |
| la semaine dernière | last week |
| le mois dernier | last month |
| l'année dernière | last year |

### Future

| Expression | Meaning |
|------------|---------|
| dans X jours | in X days |
| dans X semaines | in X weeks |
| dans une semaine | in a week |
| la semaine prochaine | next week |
| le mois prochain | next month |
| l'année prochaine | next year |

## Weekdays

| Expression | Meaning |
|------------|---------|
| lundi | Monday |
| mardi | Tuesday |
| mercredi | Wednesday |
| jeudi | Thursday |
| vendredi | Friday |
| samedi | Saturday |
| dimanche | Sunday |
| lundi prochain | next Monday |
| vendredi dernier | last Friday |

## Month Names

| Expression | Month |
|------------|-------|
| janvier | 1 |
| février | 2 |
| mars | 3 |
| avril | 4 |
| mai | 5 |
| juin | 6 |
| juillet | 7 |
| août | 8 |
| septembre | 9 |
| octobre | 10 |
| novembre | 11 |
| décembre | 12 |

## Time Expressions

| Expression | Time |
|------------|------|
| 15h | 15:00 |
| 15h30 | 15:30 |
| à 15h | at 15:00 |
| midi | noon |
| minuit | midnight |
| le matin | morning |
| l'après-midi | afternoon |
| le soir | evening |

## Date Formats

French uses DD/MM/YYYY format:

| Format | Example |
|--------|---------|
| DD/MM/YYYY | 25/12/2024 |
| D Month YYYY | 25 décembre 2024 |
| le D Month | le 25 décembre |

## Examples

```python
from whichtime import parse_with_locale, WhichTimeLocale

# Casual
parse_with_locale("aujourd'hui", WhichTimeLocale.FR)
parse_with_locale("demain", WhichTimeLocale.FR)

# Relative
parse_with_locale("dans 2 jours", WhichTimeLocale.FR)
parse_with_locale("il y a 3 semaines", WhichTimeLocale.FR)

# Combined
parse_with_locale("demain à 15h", WhichTimeLocale.FR)
parse_with_locale("lundi prochain", WhichTimeLocale.FR)
parse_with_locale("demain matin", WhichTimeLocale.FR)
```

