## API Reference

`button_group![...]` joins buttons and inputs in their visual order. Use `icedcn::widget::button::group` to build a group from an iterator of `group::Item` values.

| Builder | Default | Behavior |
| --- | --- | --- |
| `.orientation(axis)` | `Horizontal` | Arrange items in a row or stack them with `Vertical`. |
| `.width(width)` | `Length::Shrink` | Set the group’s outer width. |

The macro accepts buttons, inputs and `group::separator()` directly. When using `group::group(items)`, convert each button, input or separator to `group::Item` with `.into()`. Use `group::decorated(button, decorate)` to add a tooltip or popup around a joined button.

Buttons retain rounded corners on the outside edges of the group. Interior corners are square, and separators count when determining the first and last items. A separator runs perpendicular to the group. Apply `.style(...)` to individual buttons before placing them in the group.
