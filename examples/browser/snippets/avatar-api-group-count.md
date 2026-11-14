### AvatarGroupCount

The `AvatarGroupCount` component displays a count indicator in an avatar group, typically showing the number of additional avatars.

| Rust API | Type | Default | Behavior |
| --- | --- | --- | --- |
| `.count(content)` | `impl Into<Slot>` | none | Appends a muted circular count such as `"+3"` or an icon. |
| `.count_geometry()` | `avatar::Geometry` |32px | Reports the geometry derived from group members. |

The chip is 24px if any member is small, otherwise 40px if any is large, otherwise 32px. This follows the source selector order even for mixed groups. Text remains 14px; stock iced icons should be 12/16/20px for small/default/large groups. The ring is outside the chip’s layout bounds.

For the corresponding web parts, see the [Radix UI Avatar documentation](https://www.radix-ui.com/primitives/docs/components/avatar#api-reference).
