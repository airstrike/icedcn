These Rust examples use iced's existing `Parser` and cache token ranges once for each fixed sample. Tokens are styled for GitHub Light and GitHub Dark, matching the upstream theme choices. `CodeBlock::lines` accepts the resulting rich text without depending on a particular syntax engine.

The upstream `useHighlightedCode` hook uses Shiki's JavaScript regex engine. The native recipes use the installed Rust parser, so JavaScript and WASM loading do not apply.
