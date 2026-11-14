Pass the window width to switch layouts at 640px.

```rust
use icedcn::Size::*;
use icedcn::widget::{Row, button, column, row};
use icedcn::Slot;
use crate::Message;
mod icons {
use iced::Widget;
use iced::widget::svg;
use icedcn::{Slot, Theme};

pub fn arrow_up<'a, M: 'a>(size: f32) -> Slot<'a, M> {
    const SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m5 12 7-7 7 7"></path><path d="M12 19V5"></path></svg>"#;
    svg(svg::Handle::from_memory(SVG.as_bytes()))
        .width(size).height(size)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
        .boxed().into()
}

pub fn arrow_up_right<'a, M: 'a>(size: f32) -> Slot<'a, M> {
    const SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M7 7h10v10"></path><path d="M7 17 17 7"></path></svg>"#;
    svg(svg::Handle::from_memory(SVG.as_bytes()))
        .width(size).height(size)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
        .boxed().into()
}

pub fn circle_fading_arrow_up<'a, M: 'a>(size: f32) -> Slot<'a, M> {
    const SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 2a10 10 0 0 1 7.38 16.75"></path><path d="m16 12-4-4-4 4"></path><path d="M12 16V8"></path><path d="M2.5 8.875a10 10 0 0 0-.5 3"></path><path d="M2.83 16a10 10 0 0 0 2.43 3.4"></path><path d="M4.636 5.235a10 10 0 0 1 .891-.857"></path><path d="M8.644 21.42a10 10 0 0 0 7.631-.38"></path></svg>"#;
    svg(svg::Handle::from_memory(SVG.as_bytes()))
        .width(size).height(size)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
        .boxed().into()
}

pub fn git_branch<'a, M: 'a>(size: f32) -> Slot<'a, M> {
    const SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M7 18m-2 0a2 2 0 1 0 4 0a2 2 0 1 0 -4 0"></path><path d="M7 6m-2 0a2 2 0 1 0 4 0a2 2 0 1 0 -4 0"></path><path d="M17 6m-2 0a2 2 0 1 0 4 0a2 2 0 1 0 -4 0"></path><path d="M7 8l0 8"></path><path d="M9 18h6a2 2 0 0 0 2 -2v-5"></path><path d="M14 14l3 -3l3 3"></path></svg>"#;
    svg(svg::Handle::from_memory(SVG.as_bytes()))
        .width(size).height(size)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
        .boxed().into()
}


}

pub fn view<'a>(window_width: f32) -> Slot<'a, Message> {
        let pairs: [Slot<'a, Message>; 4] = [
            row![
                button("Extra Small").style(button::Outline).size(Xs),
                button(icons::arrow_up_right(12.0))
                    .square()
                    .style(button::Outline)
                    .size(Xs),
            ]
            .spacing(8)
            .into(),
            row![
                button("Small").style(button::Outline).size(Sm),
                button(icons::arrow_up_right(16.0))
                    .square()
                    .style(button::Outline)
                    .size(Sm),
            ]
            .spacing(8)
            .into(),
            row![
                button("Default").style(button::Outline),
                button(icons::arrow_up_right(16.0))
                    .square()
                    .style(button::Outline),
            ]
            .spacing(8)
            .into(),
            row![
                button("Large").style(button::Outline).size(Lg),
                button(icons::arrow_up_right(16.0))
                    .square()
                    .style(button::Outline)
                    .size(Lg),
            ]
            .spacing(8)
            .into(),
        ];
    if window_width >= 640.0 {
        Row::with_children(pairs).spacing(32).align_y(iced::Top).into()
    } else {
        column(pairs).spacing(32).into()
    }
}
```
