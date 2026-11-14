## Composition

The table contains a grid and an optional caption. Its scrollable viewport keeps all rows, the caption, and their shared columns aligned when space is tight.

```text
table(children)
├── table::grid(tracks)
│   ├── .header(table::header(rows)) → table::row(cells) → table::head(content)
│   ├── .body(table::body(rows)) → table::row(cells) → table::cell(content)
│   └── .footer(table::footer(rows)) → table::row(cells) → table::cell(content)
└── table::caption(content)
```
