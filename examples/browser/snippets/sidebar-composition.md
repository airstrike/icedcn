## Composition

Place the sidebar and main panel inside the provider. Use header, content and footer slots to keep persistent controls around the navigation groups.

```text
sidebar::provider(...)
├── sidebar::fixed(content)
│   ├── sidebar::header(content)
│   ├── sidebar::content(groups)
│   │   └── sidebar::group(content)
│   │       ├── sidebar::group_label(label)
│   │       └── sidebar::menu(items)
│   │           └── sidebar::menu_item(content)
│   │               ├── sidebar::menu_button(content)
│   │               └── sidebar::menu_badge(content)
│   └── sidebar::footer(content)
└── sidebar::main(content)
```
