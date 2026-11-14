## Composition

Pass the items to `accordion`. Each `accordion::item` pairs its trigger label with a content slot.

```text
accordion(items)
├── accordion::item(id, trigger).content(body)
└── accordion::item(id, trigger).content(body)
```
