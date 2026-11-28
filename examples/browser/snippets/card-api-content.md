### CardContent

`card::content(child)` accepts `impl Into<View>` and returns a `Content` builder implementing `Compose`. It preserves inherited state and uses the root spacing for horizontal padding (16px outside a Card), full width and no vertical padding. `.padding(...)`, `.width(...)` and `.height(...)` are explicit overrides. Inner columns retain their own gaps.
