## Composition

Build individual choices with the handle’s `item` within `radio_group`. Give each item a distinct value and an associated field label.

```text
radio_group(|g| ...)
├── field(g.item(value)).label(text)
└── field(g.item(value)).label(text)
```
