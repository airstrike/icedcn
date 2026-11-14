### Card spacing

Set `.spacing(spacing)` on the Card root, or `.size(Size::Sm)` for 12px defaults. Remove the repeated outer column spacing/padding, header inset, content padding and `footer_with_spacing` argument when those values should follow the root. Keep explicit overrides for custom layouts.

Card roots and Content now retain Slots through Compose. Replace stock `.boxed()` conversions with `.into()` at the application's Element boundary. Existing surface-only compositions can preserve their explicit layout with `.gap(0).padding(0)`. A card without a footer keeps bottom padding automatically; an image-first composition uses `.padding(0)`.
