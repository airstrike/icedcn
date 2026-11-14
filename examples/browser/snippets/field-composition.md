## Composition

Start with `field(control)`, then add its label and optional description or error. A field group arranges related fields together.

### Field

A single control with label, helper text, and validation.

```text
Field
├── FieldLabel
├── Input / Textarea / Switch / Select
├── FieldDescription
└── FieldError
```

### FieldGroup

Place related fields in `field::group(...)`. Put a `separator()` between sections when needed.

```text
FieldGroup
├── Field
│   ├── FieldLabel
│   ├── Input / Textarea / Switch / Select
│   ├── FieldDescription
│   └── FieldError
├── FieldSeparator
└── Field
    ├── FieldLabel
    └── Input / Textarea / Switch / Select
```

### FieldSet

Semantic grouping with a legend and description, usually containing a `FieldGroup`.

```text
FieldSet
├── FieldLegend
├── FieldDescription
└── FieldGroup
    ├── Field
    │   ├── FieldLabel
    │   ├── Input / Textarea / Switch / Select
    │   ├── FieldDescription
    │   └── FieldError
    └── Field
        ├── FieldLabel
        └── Input / Textarea / Switch / Select
```
