```rust
use crate::Message;
use iced::{Element, Widget};
use iced::widget::svg;
use iced::widget::component::{Component, component};
use icedcn::{Slot, Theme};
use icedcn::theme::button::{IntoStyle, state};
use icedcn::theme::text_input;
use icedcn::widget::{button, button_group, input, input_group, tooltip};
use icedcn::widget::input_group::Align;

struct Voice {
    narrow: bool,
}
impl<'a> Component<'a, Message, Theme> for Voice {
    type State = bool;
    type Event = bool;
    fn update(&self, state: &mut bool, enabled: bool, _: &iced::Renderer) -> Option<Message> {
        *state = enabled;
        None
    }
    fn view(&self, enabled: &bool) -> impl Widget<bool, Theme> + 'a {
        let enabled = *enabled;
        let audio = icon::audio_lines().style(move |theme: &Theme, _| svg::Style {
            color: Some(if enabled {
                voice_foreground(theme)
            } else {
                theme.neutral.foreground.into()
            }),
        });
        let toggle = input_group::button(audio.boxed())
            .square()
            .on_press(!enabled);
        let toggle = if enabled {
            toggle.style(VoiceActive)
        } else {
            toggle
        };
        Element::from(button_group![
            button_group![
                button(icon::plus().boxed())
                    .rounded()
                    .square()
                    .style(button::Outline)
            ],
            button_group![
                input_group(
                    input(if enabled {
                        "Record and send audio..."
                    } else {
                        "Send a message..."
                    })
                    .text_size(if self.narrow { 16. } else { 14. })
                    .disabled(enabled)
                )
                .style(Box::new(|theme: &Theme, status| {
                    let mut style = text_input::default(theme, status);
                    style.border.radius = 9999.0.into();
                    style
                }) as text_input::StyleFn<'static, Theme>)
                .addon(input_group::addon(tooltip(toggle, "Voice Mode")).align(Align::InlineEnd))
            ]
            .width(if self.narrow { 220. } else { 215.203 })
        ])
    }
}

/// Keep the Ghost variant's geometry and keyboard decoration in both themes.
struct VoiceActive;
impl<'a> IntoStyle<'a> for VoiceActive {
    fn into_style(self) -> state::StyleFn<'a> {
        let ghost = button::Ghost.into_style();
        Box::new(move |theme, status| {
            let mut style = ghost(theme, status);
            style.paint.background = Some(
                if theme.neutral.is_dark {
                    iced::Color::from_rgb8(154, 52, 18)
                } else {
                    iced::Color::from_rgb8(255, 237, 213)
                }
                .into(),
            );
            style.paint.text_color = voice_foreground(theme);
            style
        })
    }
}

fn voice_foreground(theme: &Theme) -> iced::Color {
    if theme.neutral.is_dark {
        iced::Color::from_rgb8(255, 237, 213)
    } else {
        iced::Color::from_rgb8(194, 65, 12)
    }
}

pub fn view(window_width: f32) -> Slot<'static, Message> {
    let narrow = window_width < 768.;
    Slot::from(component(Voice { narrow }).boxed())
}

mod icon {
    use iced::widget::svg;
    use icedcn::Theme;
    pub fn audio_lines<'a>() -> svg::Svg<'a, Theme> {
        svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 10v3" /> <path d="M6 6v11" /> <path d="M10 3v18" /> <path d="M14 8v7" /> <path d="M18 5v13" /> <path d="M22 10v3" /> </svg>"#.as_bytes())).width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
    }
    pub fn plus<'a>() -> svg::Svg<'a, Theme> {
        svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 12h14" /> <path d="M12 5v14" /> </svg>"#.as_bytes())).width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
    }
}
```
