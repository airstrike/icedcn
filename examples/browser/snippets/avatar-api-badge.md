### AvatarBadge

| Rust API | Type | Default | Behavior |
| --- | --- | --- | --- |
| `avatar::badge(avatar, content)` | `Avatar`, `impl Into<Slot>` | no badge | Returns the Avatar with a retained badge, usable in a group. |
| `.badge_style(style)` | `Fn(&Theme) -> container::Style` | primary | Styles the badge while retaining its 2px outer background ring. |
| `avatar::badge::presence` | style function | green | Uses a green status fill that adapts to the theme. |
| `avatar::dot()` | empty `Slot` | — | Presence-dot content. |

The badge disk is 8/10/12px for small/default/large. Small avatars hide the inner content. Compose-aware content inherits 8px; use `.size(8).line_height(1.0)` for a Lucide glyph.

```rust
use icedcn::widget::avatar::badge;
use crate::icon;
use icedcn::widget::avatar;


fn view<'a>() -> avatar::group::Group<'a, ()> {
    avatar::group![
        avatar::badge(avatar(None).fallback("HR"), avatar::dot()).badge_style(badge::presence),
        avatar::badge(
            avatar(None).fallback("IC"),
            icon::plus().size(8).line_height(1.0),
        ),
    ]
}
```
