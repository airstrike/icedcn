### Alert

Constructor for an `Alert` builder with the default style.

The optional parts default to `None`:

- `.icon(icon)` takes `impl Into<Slot>` for the leading 16px icon.
- `.title(title)` takes `impl IntoFragment` for the 14px, medium-weight title.
- `.description(text)` takes `impl IntoFragment` for the 14px body in the style's description color.
- `.action(action)` takes `impl Into<Slot>` for a top-right action, typically a button.

`.style(style)` takes `impl Into<alert::Spec>` and defaults to `alert::Default`. The default style uses a card background and muted description. `alert::Destructive` uses a destructive foreground and a description at 90% opacity. Both variants retain a neutral border. A tweaked `Spec` is also accepted.

`.max_width(width)` takes `impl Into<Pixels>` and defaults to `f32::INFINITY`. The examples use 448px.

`.description_wrapping(mode)` takes `text::Wrapping` and defaults to `Word`. The examples choose `Balanced` below a 768px window width.
