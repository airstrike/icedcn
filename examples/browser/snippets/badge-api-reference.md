### Badge

Constructor for a `Badge`. The single-arg constructor takes any `impl Into<Slot>`; the `badge!` macro wraps multiple children in a row with 4px spacing for the icon + label idiom.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.style(style)` | `impl badge::IntoStyle` | `badge::Default` | The variant: `badge::Secondary`, `badge::Destructive`, `badge::Outline`, `badge::Ghost`, `badge::Link`, or a tweaked `Spec`. |
| `.state_style(callback)` | `Fn(&Theme, badge::Status) -> badge::Style` | None | Replace all paint with a state-aware callback. |
| `.invalid(invalid)` | `bool` | `false` | Destructive border and keyboard focus ring. |
| `.disabled(disabled)` | `bool` | `false` | Suppress input, focus and hover; disabled ancestry also applies. |
| `.on_press(message)` | `Message` | `None` | Make the badge clickable (pointer, keyboard focus and activation). |

Use `.icon_start()` or `.icon_end()` to reduce the padding on the icon side to 6px, plus the 1px border.

**`badge!`**

Wraps multiple children in a `Row` with 4px spacing. The variant is set after the macro.

| Macro | Description |
| --- | --- |
| `badge!["text", icon::foo()].icon_end()` | Icon on the right. |
| `badge![icon::foo(), "text"].icon_start().style(badge::Secondary)` | Icon on the left, secondary variant. |

**Variants**

| Style | Description |
| --- | --- |
| `badge::Default` | Primary-foreground pill. |
| `badge::Secondary` | Muted background. |
| `badge::Destructive` | Destructive foreground on a tinted background. |
| `badge::Outline` | Border-only, transparent background. |
| `badge::Ghost` | Transparent pill with a muted surface on hover, including passive badges. |
| `badge::Link` | Primary-colored text on a transparent surface, underlined on hover. |

Variants retain their hover and focus rules. A `Spec` applies its tweaks after those rules, in the order supplied; `hover`, `focus`, `active` and `disabled` conditions are supported. Focus rings require an interactive badge with keyboard-visible focus. Pointer activation retains focus without showing the keyboard ring.

Existing one-argument `container::StyleFn` callbacks remain accepted by `.style()`. They supply static paint in every state, without automatic variant hover, underline or focus effects. Use `.state_style()` for a two-argument callback. The last `.style()` or `.state_style()` call replaces all earlier styling. A callback can extend a variant by resolving a `Spec` for the supplied `Status`.

```rust
use iced::widget::container;
use icedcn::Theme;
use icedcn::core::theme::badge::Spec;
use icedcn::widget::{badge, row};

fn example() -> iced::Element<'static, (), Theme> {
    let legacy: container::StyleFn<'static, Theme> = Box::new(|theme| {
        container::Style::default().color(iced::Color::from(theme.neutral.foreground))
    });
    let spec: Spec = badge::Link.into();
    row![
        badge("Static").style(legacy),
        badge("Open Link")
            .on_press(())
            .state_style(move |theme, status| spec.resolve_state(theme, status)),
    ]
    .spacing(8)
    .into()
}
```
