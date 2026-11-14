## Composition

The content closure builds the panel header and body. Add footer actions with the dialog’s `.footer(...)` builder.

```text
dialog(trigger, content)
├── d.trigger(button)
├── dialog::header(title, description)
├── body or .scrollable_body(...)
└── .footer(...)
```
