Every tweak is a function in `icedcn::theme`:

- `bg` and `fg` for colors, `border::color`, `border::width`, and `radius` for the border; an `icedcn::core::Shadow` adds the same way
- `ring(color, width, offset)` for the focus ring; only buttons draw the offset
- `alpha` to fade the fill (on text, the text color) and `opaque()` to flatten a translucent fill onto the page background
- `size`, `weight`, `family`, `line_height`, `italic()`, and `align` for text
- `hover`, `active`, `focus`, and `disabled`, which apply the tweaks they wrap only in that state

Colors are `Token`s: theme roles such as `Token::Primary`, `Token::Muted`, and `Token::Border`, so a tweaked style still follows the theme. `bg` and `fg` also take a literal color, and `Token::themed(light, dark)` a pair that follows the mode. For a role, `Bg::Muted` and `Fg::MutedForeground` are short for `bg(Token::Muted)` and `fg(Token::MutedForeground)`.

Each component decides what a tweak means. On a switch, `bg` sets the track and `fg` the thumb. A component ignores tweaks it has no use for, as CSS ignores properties that do not apply.

The right side of `+` is always one tweak. Two variants do not combine: `button::Outline + button::Ghost` does not compile, because each sets every property. Start from the closer variant and tweak the rest. Tweaks added together without a variant apply to the default one, and a function reuses the same tweaks on several variants:

```rust
use icedcn::View;
use icedcn::theme::button::Spec;
use icedcn::theme::{Bg, Fg, hover};
use icedcn::widget::{button, row};

fn quiet(spec: impl Into<Spec>) -> Spec {
    spec.into() + Fg::MutedForeground + hover(Fg::Foreground)
}

pub fn reused<'a>() -> View<'a> {
    row![
        button("Accent").style(Bg::Accent + Fg::AccentForeground),
        button("Ghost").style(quiet(button::Ghost)),
        button("Outline").style(quiet(button::Outline)),
    ]
    .spacing(8)
    .into()
}
```
