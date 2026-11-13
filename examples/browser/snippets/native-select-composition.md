## Composition

Pass the listed options to `native_select` and use its value-change callback when the selection matters to your application.

### Simple

Options placed directly under `NativeSelect` (no `NativeSelectOptGroup`).

```text
NativeSelect
├── NativeSelectOption
├── NativeSelectOption
├── NativeSelectOption
└── NativeSelectOption
```

### With groups

Use `NativeSelectOptGroup` to organize options into categories.

```text
NativeSelect
├── NativeSelectOptGroup
│   ├── NativeSelectOption
│   └── NativeSelectOption
└── NativeSelectOptGroup
    ├── NativeSelectOption
    └── NativeSelectOption
```
