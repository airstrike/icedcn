## Header

`CodeBlockHeader` holds a `CodeBlockTitle` and `CodeBlockActions`. `CodeBlockCopyButton` copies its `value`.

The Rust equivalents are `.title(...)`, `code_block::title(...)` and `.actions(...)`. The copy control publishes a typed message; `update` writes its exact source value to the clipboard.
