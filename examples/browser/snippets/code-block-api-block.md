### CodeBlock

The surface that holds the header, the content and the footer.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `code_block(part)` | `impl Into<Part>` | - | A header, actions, content or footer. A string is the content. |
| `.push(part)` | `impl Into<Part>` | - | Another part. |
| `.style(style)` | `impl Into<code_block::Spec>` | `code_block::Default` | The tone of the block: `Destructive`, or either variant with tweaks such as `Destructive + Bg(Token::Muted)`. |
| `.size(size)` | `impl Into<Size>` | `Size::Default` | `Size::Sm` has tighter padding and 12px code, for tool output. |
