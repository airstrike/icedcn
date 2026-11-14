## Composition

Start with `field(control)`, then add its label and optional description or error. A field group arranges related fields together.

### Field

A single control with label, helper text, and validation.

```text
field(control)
├── .label(text)
├── .description(text)
└── .error(text)
```

### FieldGroup

Place related fields in `field::group(...)`. Put a `separator()` between sections when needed.

```text
field::group(items)
├── field(control).label(text)
├── separator()
└── field(control).label(text)
```

### FieldSet

Group controls under a legend and description.

```text
field::set(legend, description, controls)
└── field::group(items)
    ├── field(control).label(text)
    └── field(control).label(text)
```
