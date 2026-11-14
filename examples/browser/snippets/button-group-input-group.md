```rust
use crate::icon;
use icedcn::{Slot, radius};
use icedcn::theme::{button::{IntoStyle, state}, text_input};
use icedcn::theme::tweak::{Fg, Token, hover};
use icedcn::widget::{button, button_group, input, input_group, tooltip};
use icedcn::widget::input_group::Align;

pub fn view(window_width: f32, enabled: bool) -> Slot<'static, bool> {
    let narrow = window_width < 768.;
    let toggle = input_group::button(icon::audio_lines().size(16).line_height(1.0))
        .square().on_press(!enabled);
    let toggle = if enabled {
        toggle.style(VoiceActive)
    } else {
        toggle.style(button::Ghost + Fg(Token::MutedForeground) + hover(Fg(Token::Foreground)))
    };
    let placeholder = if enabled { "Record and send audio..." } else { "Send a message..." };
    let editor = input_group(input(placeholder).text_size(if narrow { 16. } else { 14. }).disabled(enabled))
        .style(text_input::Default + radius(9999.))
        .addon(input_group::addon(tooltip(toggle, "Voice Mode")).align(Align::InlineEnd));
    button_group![
        button_group![button(icon::plus().size(16).line_height(1.0)).rounded().square().style(button::Outline)],
        button_group![editor].width(if narrow { 220. } else { 215.2 }),
    ].into()
}

struct VoiceActive;
impl<'a> IntoStyle<'a> for VoiceActive {
    fn into_style(self) -> state::StyleFn<'a> {
        let ghost = button::Ghost.into_style();
        Box::new(move |theme, status| {
            let mut style = ghost(theme, status);
            let orange_100 = iced::Color::from_rgb8(255, 237, 213);
            let (background, foreground) = if theme.neutral.is_dark {
                (iced::Color::from_rgb8(154, 52, 18), orange_100)
            } else {
                (orange_100, iced::Color::from_rgb8(194, 65, 12))
            };
            style.paint.background = Some(background.into());
            style.paint.text_color = foreground;
            style
        })
    }
}
```

Start with `enabled = false` and store the emitted boolean in `update`. The input keeps the draft between modes.
