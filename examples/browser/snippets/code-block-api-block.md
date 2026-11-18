### CodeBlock

The surface that holds the header, the content and the footer.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `code_block(content)` | `impl Into<Content>` | - | A string, a list of `code_block::line`s, or `code_block::content(..)`. |
| `.header(header)` | `impl Into<Header>` | - | The row above the content. |
| `.actions(actions)` | `impl Into<Actions>` | - | Actions that float over the content. A single button works too. |
| `.footer(footer)` | `impl Into<Footer>` | - | The row below the content. |
| `.style(style)` | `impl Into<code_block::Spec>` | `code_block::Default` | The tone of the block: `Destructive`, or either variant with tweaks such as `Destructive + Bg(Token::Muted)`. |
| `.size(size)` | `impl Into<Size>` | `Size::Default` | `Size::Sm` has tighter padding and 12px code, for tool output. |
