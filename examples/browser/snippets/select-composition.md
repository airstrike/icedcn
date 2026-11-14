## Composition

Build the options from `select::group`, `select::label` and `select::item`. The select manages its trigger, list and selected value.

```text
select(entries)
├── select::group(entries)
│   ├── select::label(label)
│   └── select::item(value, label)
├── select::separator()
└── select::group(entries)
```
