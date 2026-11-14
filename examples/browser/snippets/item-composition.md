## Composition

Compose each item from media, title, description and actions, with optional header and footer content. Group related items in reading order.

```text
column(items)
└── item::item(children)
    ├── item::header(children)
    ├── item::media(content)
    ├── item::content(children)
    │   ├── item::title(text)
    │   └── item::description(text)
    ├── item::actions(children)
    └── item::footer(children)
```
