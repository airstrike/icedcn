## Composition

Start with `input_group(control)` and add the adjacent content with `.addon(...)`. Set alignment on each addon.

```text
input_group(control)
├── input(placeholder) or textarea(placeholder)
└── .addon(input_group::addon(content))
    └── input_group::text(label) or input_group::button(content)
```
