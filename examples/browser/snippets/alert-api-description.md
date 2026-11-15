### AlertDescription

Set the description with `.description(text)` on `alert()`. It accepts `impl IntoFragment` and defaults to `None`.

The body uses 14px text and a 20px line height. Its color is muted by default, or destructive at 90% opacity in the destructive variant.

Use `.description_wrapping(mode)` with `text::Wrapping` to choose line breaking. It defaults to `Word`. Text wraps within the available width automatically; choose `Balanced` explicitly when you want balanced lines. Title wrapping is unchanged.
