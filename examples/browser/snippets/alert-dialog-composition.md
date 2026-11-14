## Composition

```text
alert_dialog(trigger, content)
├── a.trigger(button)
└── column!
    ├── a.header(title, description).media(icon)
    └── a.footer(buttons)
        ├── a.cancel(button)
        └── a.action(button)
```
