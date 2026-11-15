## Semantic roles

The theme exposes palettes for neutral, primary, secondary, muted, accent, destructive, info, success, and warning roles. A palette contains its fill, foreground, border, ring, and text color for transparent variants. Card, popover, code, and sidebar surfaces have their own roles.

Choose a semantic variant before overriding an individual color. For example, `button::Destructive` keeps a destructive action's interaction styles together, while `button::Outline` supplies the outlined treatment. `Theme` wraps a `Design`; inspect `theme.design()` when building an application-wide palette editor.

A custom `Design` contains related fields, including derived tint colors and the seed used by stock iced widgets. Keep those values consistent when changing a whole palette. The repository's palette example provides a working application for inspecting theme roles:

```sh
cargo run --locked -p palette
```
