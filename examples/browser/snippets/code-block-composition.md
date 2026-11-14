## Composition

```text
CodeBlock
├── .title(...) — header title, optionally an icon and code_block::title(...)
│   └── .actions(...) — header controls, including code_block::copy_button(text)
├── .actions(...) without a title — floating controls
├── code_block(source) or CodeBlock::lines(...) — selectable content
│   └── (LineStyle, Slot) — styled lines; gutter and diff markers are separate
└── .footer(...) — controls below the content
```

Pass a string to `code_block`. Use `.language(...)` for syntax colors and `.line_style(...)` to decorate source lines. `CodeBlock::lines(...)` accepts custom widgets in each row. The slots accept the same `Compose` children used elsewhere in icedcn.
