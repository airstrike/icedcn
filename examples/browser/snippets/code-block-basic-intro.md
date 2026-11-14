## Basic

Pass a string to `CodeBlockContent`. It renders as one text node, so it stays cheap while output streams.

In Rust, pass the string to `code_block(source)`. Numbered or styled content uses one slot per line.
