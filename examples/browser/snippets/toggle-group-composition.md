## Composition

Create each choice with `toggle_group::item` and supply the choices to `toggle_group`. Each item has its own value and content.

```text
toggle_group(|| items)
├── toggle_group::item(value, content)
└── toggle_group::item(value, content)
```
