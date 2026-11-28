## Semantic roles

The theme has a palette for each role: neutral, primary, secondary, muted, accent, destructive, info, success, and warning. Card, popover, code, and the sidebar have their own surface palettes. `input` holds the input border color and `chart` the five chart colors.

A palette holds a fill (`base`), the text drawn on it (`foreground`), a `border`, a focus `ring`, and `tint_text`, the text color for transparent variants such as outline and ghost. `Theme` dereferences to its `Design`, so `theme.primary.base` reads a role directly.

Variants and tweaks refer to roles, never to fixed colors. That is what lets one style work in every theme.
