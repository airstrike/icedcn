### Spacing Variable

The upstream Card now uses `--card-spacing` for its section gap, vertical padding and part insets.

When adapting an older Rust composition, keep one spacing value and pass it to:

- the outer column's `.spacing(spacing)` and top padding;
- `card::header(...).inset(spacing)`;
- `card::content(...).padding([0.0, spacing])`;
- `card::footer_with_spacing(..., spacing)`.

Use 16px by default and 12px for the small size. A card without a footer needs the matching bottom padding; an image-first card leaves top padding at zero. This preserves the source spacing relationship without changing other cards in the application.
