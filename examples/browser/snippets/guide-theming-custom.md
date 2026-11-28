## Variants of your own

Name a tweaked variant when the same adjustment repeats. `Destructive + radius(0)` has the type `Tweaked<Destructive>`: the variant it starts from, plus its tweaks. The variant stays in the type, not yet resolved for any one component, so the same value converts into every `Spec` that converts from `Destructive`. One `danger()` then styles a button and a badge alike, and `Tweaked<Outline>` works wherever `Outline` does:

```rust
use icedcn::View;
use icedcn::theme::tweak::radius;
use icedcn::theme::{Destructive, Tweaked};
use icedcn::widget::{badge, button, row};

pub fn danger() -> Tweaked<Destructive> {
    Destructive + radius(0)
}

pub fn failed<'a>() -> View<'a> {
    row![
        button("Drop table").style(danger()),
        badge("Failed").style(danger()),
    ]
    .spacing(8)
    .into()
}
```

For a new name, declare a variant with `variant!`, list the components it styles, and convert it into each one's `Spec`. Building it from a built-in variant keeps that variant's focus ring, sizing, and disabled treatment:

```rust
use icedcn::View;
use icedcn::theme::button::Spec;
use icedcn::theme::tweak::{Bg, Fg, Token, radius};
use icedcn::widget::{button, row};

icedcn::core::variant! {
    pub struct Confirm: button;
}

impl From<Confirm> for Spec {
    fn from(_: Confirm) -> Spec {
        Spec::from(button::Default) + Bg(Token::Success) + Fg(Token::SuccessForeground)
    }
}

pub fn approve<'a>() -> View<'a> {
    row![
        button("Approve").style(Confirm),
        button("Approve all").style(Confirm + radius(999)),
    ]
    .spacing(8)
    .into()
}
```

The names a list accepts are `button`, `badge`, `checkbox`, `switch`, `text_input`, `text`, `container`, `accordion`, `progress_bar`, `alert`, `alert_dialog`, `code_block`, and `toggle`. Each lets that component take the variant bare, in `.style(..)` or, for an alert dialog, `.media_style(..)`, and fails at the declaration if the conversion is missing. Builders that take `impl Into<Spec>`, such as alerts, code blocks, toggles, items, and keyboard keys, accept the bare variant without the list. A tweaked variant such as `Confirm + radius(0)` works on any component whose `Spec` converts from the variant, listed or not.

For full control, `Spec::new` takes the function itself. It receives the theme and the status, so it can read any role. The button keeps the function's colors and geometry, then adds the focus and invalid rings and your tweaks. The function owns the disabled look; the built-in variants' sizing and disabled fade do not apply. Buttons, containers, code blocks, and text have `Spec::new`:

```rust
use icedcn::theme::button::{Spec, Status, Style, default};
use icedcn::widget::button;
use icedcn::{View, Theme};

fn notice(theme: &Theme, status: Status) -> Style {
    let alpha = if status == Status::Disabled { 0.5 } else { 1.0 };
    Style {
        background: Some(theme.info.base.alpha(alpha).into()),
        text_color: theme.info.foreground.alpha(alpha),
        ..default(theme, status)
    }
}

pub fn read_more<'a>() -> View<'a> {
    button("Read more").style(Spec::new(notice)).into()
}
```

`.style(..)` also accepts a plain iced style function. Its colors are kept exactly, but the widget cannot tell which variant it represents and adds no focus ring or underline.
