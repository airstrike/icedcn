These fixed Rust recipes retain token ranges and colors generated with the pinned upstream Shiki 3.23.0 JavaScript regex engine and GitHub Light/Dark themes. Streaming fixtures also retain the grammar result for unfinished final lines. `CodeBlock::lines` accepts these rich-text slots without depending on a syntax engine.

For arbitrary input, tokenize in `update` or an asynchronous task and pass the stored line slots to `CodeBlock::lines`. The public widget does not include a general-purpose Shiki runtime.
