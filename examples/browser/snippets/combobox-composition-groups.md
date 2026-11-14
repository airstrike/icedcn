### With groups and collection

Use `.group(name)` on each option to display related items under the same label.

```text
combobox::options(items)
├── combobox::item(value, label).group(group_name)
└── combobox::item(value, label).group(group_name)
```
