## Widget styles

Use the variant exported by the widget module. Style tweaks can refer to a theme token, so the result follows the active palette:

```rust
use icedcn::Slot;
use icedcn::theme::tweak::{Bg, Token};
use icedcn::widget::button;

pub fn action<'a>() -> Slot<'a, ()> {
    button("Continue")
        .style(button::Outline + Bg(Token::Accent))
        .into()
}
```

Token-based styling keeps the color tied to its role when switching modes. For more specialized rendering, consult the component's style API before introducing a style function: some components carry focus or decoration information in addition to their fill and border.

Theme choice and font loading are separate. Register font bytes with iced and choose a font for the application; changing a palette does not download fonts. The [Typography page](icedcn://component/typography) covers the available text treatments.
