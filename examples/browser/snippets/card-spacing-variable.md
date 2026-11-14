### Card spacing

Use one spacing value for the section gap, vertical padding, and horizontal insets. Apply it to:

- the outer column's `.spacing(spacing)` and top padding;
- `card::header(...).inset(spacing)`;
- `card::content(...).padding([0.0, spacing])`;
- `card::footer_with_spacing(..., spacing)`.

Use 16px by default and 12px for the small size. A card without a footer needs the matching bottom padding; an image-first card leaves top padding at zero.
