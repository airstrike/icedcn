## Composition

Build the trigger list and panels inside the `tabs` closure. Pair each trigger with its panel using the same value.

```text
tabs(|t| ...)
├── t.list(triggers)
│   └── t.trigger(value, label)
└── t.content(value, panel)
```
