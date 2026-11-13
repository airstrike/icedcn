## Composition

Build the opening control in the trigger closure. Place the header, body and footer in the content closure and use the handle to wire up dismissal.

```text
Dialog
├── DialogTrigger
└── DialogContent
    ├── DialogHeader
    │   ├── DialogTitle
    │   └── DialogDescription
    └── DialogFooter
```
