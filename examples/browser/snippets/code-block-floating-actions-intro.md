## Floating Actions

Without a header, put `CodeBlockActions` directly inside `CodeBlock`. The actions float at the top end and stay hidden until the block is hovered or focused. They always show on touch screens.

The native composition is `code_block(source).actions(controls)`. This desktop recipe supports pointer and keyboard focus; it does not implement the touch-specific always-visible mode.
