### Show More

```rust
code_block::show_more(height, || code_block![..])
```

Clips the content to `height` with a Show more button in the footer. It returns a `Collapsible`, so `.open(..)` and `.on_open_change(..)` control it.
