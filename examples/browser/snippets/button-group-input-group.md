```rust
use crate::icon;
use iced::Fill;
use icedcn::theme::text_input;
use icedcn::theme::tweak::{Bg, Fg, Token, hover};
use icedcn::widget::input_group::Align;
use icedcn::widget::{button, button_group, input, input_group, tooltip};
use icedcn::{Slot, radius};

pub fn view<'a>(enabled: bool) -> Slot<'a, bool> {
    let toggle = input_group::button(icon::audio_lines().size(16).line_height(1.0))
        .square()
        .on_press(!enabled);
    let toggle = if enabled {
        let background = Token::themed(iced::color!(0xffedd5), iced::color!(0x9a3412));
        let foreground = Token::themed(iced::color!(0xc2410c), iced::color!(0xffedd5));
        toggle.style(button::Ghost + Bg(background) + Fg(foreground))
    } else {
        toggle.style(button::Ghost + Fg(Token::MutedForeground) + hover(Fg(Token::Foreground)))
    };
    let placeholder = if enabled {
        "Record and send audio..."
    } else {
        "Send a message..."
    };
    let editor = input_group(input(placeholder).disabled(enabled))
        .style(text_input::Default + radius(9999.))
        .addon(input_group::addon(tooltip(toggle, "Voice Mode")).align(Align::InlineEnd));
    button_group![
        button_group![
            button(icon::plus().size(16).line_height(1.0))
                .rounded()
                .square()
                .style(button::Outline)
        ],
        button_group![editor].width(Fill),
    ]
    .width(Fill.max(256.))
    .into()
}
```

Start with `enabled = false` and store the emitted boolean in `update`. The input keeps the draft between modes. `Token::themed(light, dark)` selects the matching color while preserving the Ghost button's focus styling.
