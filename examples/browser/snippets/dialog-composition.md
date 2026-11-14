## Composition

Build the opening control in the trigger closure. Place the header, body and footer in the content closure and use the handle to wire up dismissal.

```text
dialog(trigger, content)
├── d.trigger(button)
└── column!
    ├── dialog::header(title, description)
    ├── body
    └── dialog::footer(actions)
```
