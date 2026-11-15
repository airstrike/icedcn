## Composition

The sheet owns its open state. Use `sheet::content` for a fixed header and footer with an expanding body. Wrap a footer action in `s.close(..)` to dismiss the sheet.

```text
sheet(trigger, content)
├── s.trigger(button)
└── sheet::content
    ├── sheet::header(title, description)
    ├── body
    └── actions
```
