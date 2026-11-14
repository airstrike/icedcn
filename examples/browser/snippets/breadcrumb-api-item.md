### BreadcrumbItem

`breadcrumb::item(content)` embeds any `impl Into<Slot>` in the trail, preserving its own interaction. Use this for a dropdown menu. No extra item wrapper is needed for links, current-page labels or ellipses.

All four part constructors return `breadcrumb::Item`, a `Compose`/`Slot` builder. Parts can also be used outside a trail, including `button(breadcrumb::ellipsis(None))`. Typography and disabled ancestors continue through the part. Construct parts with the lowercase helpers; the former public enum variants have been replaced by this builder.
