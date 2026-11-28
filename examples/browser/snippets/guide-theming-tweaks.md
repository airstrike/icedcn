## Tweaks

A tweak changes one property of a variant. Add tweaks with `+`. They apply in order, so a later tweak overrides an earlier one:

```rust
use icedcn::Slot;
use icedcn::theme::tweak::{Bg, Fg, Token, hover, radius};
use icedcn::widget::{button, row};

pub fn tweaked<'a>() -> Slot<'a, ()> {
    let quiet = button::Ghost + Fg(Token::MutedForeground) + hover(Fg(Token::Foreground));
    row![
        button("Muted").style(button::Destructive + Bg(Token::Muted)),
        button("Square").style(button::Outline + radius(0)),
        button("Quiet").style(quiet),
    ]
    .spacing(8)
    .into()
}
```

`icedcn::theme::tweak` provides:

- `Bg`, `Fg`, and `border::Color` for colors, and `border::Width` and `radius` for the border; an `icedcn::core::Shadow` adds the same way
- `Ring` for the focus ring's color, width, and, on buttons, offset
- `Alpha` to fade the fill and `Opaque` to flatten a translucent fill onto the page background
- `Size`, `Weight`, `Family`, `LineHeight`, `Italic`, and `Align` for text
- `hover`, `active`, `focus`, and `disabled`, which apply the tweaks they wrap only in that state

Colors are `Token`s: theme roles such as `Token::Primary`, `Token::Muted`, and `Token::Border`, so a tweaked style still follows the theme. `Token::Custom(color)` takes a literal color, and `Token::themed(light, dark)` a pair that follows the mode.

Each component decides what a tweak means. On a switch, `Bg` sets the track and `Fg` the thumb. A component ignores tweaks it has no use for, as CSS ignores properties that do not apply.

The right side of `+` is always one tweak. Two variants do not combine: `button::Outline + button::Ghost` does not compile, because each sets every property. Start from the closer variant and tweak the rest. Tweaks added together without a variant apply to the default one, and a function reuses the same tweaks on several variants:

```rust
use icedcn::Slot;
use icedcn::theme::button::Spec;
use icedcn::theme::tweak::{Bg, Fg, Token, hover};
use icedcn::widget::{button, row};

fn quiet(spec: impl Into<Spec>) -> Spec {
    spec.into() + Fg(Token::MutedForeground) + hover(Fg(Token::Foreground))
}

pub fn reused<'a>() -> Slot<'a, ()> {
    row![
        button("Accent").style(Bg(Token::Accent) + Fg(Token::AccentForeground)),
        button("Ghost").style(quiet(button::Ghost)),
        button("Outline").style(quiet(button::Outline)),
    ]
    .spacing(8)
    .into()
}
```
