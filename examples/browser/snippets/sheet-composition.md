## Composition

Provide a trigger and content closure to `sheet`. The content can include a header, controls and a footer with a close action.

```text
sheet(trigger, content)
├── s.trigger(button)
└── column!
    ├── sheet::header(title, description)
    ├── body
    └── sheet::footer(actions)
```
