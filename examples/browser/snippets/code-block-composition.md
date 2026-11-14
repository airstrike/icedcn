## Composition

```text
CodeBlock
├── .title(...) — header title, optionally an icon and code_block::title(...)
│   └── .actions(...) — header controls, including the application's copy button
├── .actions(...) without a title — floating controls
├── code_block(source) or CodeBlock::lines(...) — selectable content
│   └── (LineStyle, Slot) — styled lines; gutter and diff markers are separate
└── .footer(...) — controls below the content
```

Pass a string to `code_block`. Use `CodeBlock::lines` when you need line variants or highlighted tokens. The slots accept the same `Compose` children used elsewhere in icedxdy.
