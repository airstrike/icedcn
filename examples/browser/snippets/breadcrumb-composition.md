## Composition

Supply links, custom items and the current page to `breadcrumb` in reading order. The list inserts its own separators.

```text
breadcrumb(items)
├── breadcrumb::link(label, message)
├── breadcrumb::link(label, message)
└── breadcrumb::page(label)
```
