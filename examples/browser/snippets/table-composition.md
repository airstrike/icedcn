## Composition

Group rows into a header, body and optional footer. Use head cells for labels and data cells for the body.

```text
table::table(children)
├── table::header(rows)
│   └── table::row(cells) → table::head(content)
├── table::body(rows)
│   └── table::row(cells) → table::cell(content)
├── table::footer(rows)
└── table::caption(content)
```
