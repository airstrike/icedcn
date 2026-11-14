## Composition

Pass the options to `native_select`. With `.on_value_change(Message::Selected)`, choosing an option publishes its string value in a `Message::Selected` message.

### Simple

Pass individual `native_select::item(value, label)` entries directly to `native_select`.

```text
native_select(items)
├── native_select::item(value, label)
└── native_select::item(value, label)
```

### With groups

Use `native_select::group(label, items)` to organize options into categories.

```text
native_select(groups)
├── native_select::group(label, items)
│   └── native_select::item(value, label)
└── native_select::group(label, items)
    └── native_select::item(value, label)
```
