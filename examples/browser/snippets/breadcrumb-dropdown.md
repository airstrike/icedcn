```rust
use iced::Element;
use icedxdy::widget::button::widget;
use icedxdy::widget::{breadcrumb, dropdown_menu, menu, row};
use icedxdy::{Slot, Theme};

breadcrumb([
    breadcrumb::link("Home", Message::GoHome),
    breadcrumb::item(
        dropdown_menu(
            |m| {
                let content = row!["Components", icon::chevron_down().size(14)].spacing(4);
                let style: widget::StyleFn<'_, Theme> = Box::new(|theme, _| widget::Style {
                    text_color: theme.muted.foreground.into(),
                    ..widget::Style::default()
                });
                let trigger = widget::Button::new(Slot::from(content).compose())
                    .padding(0)
                    .translate(false)
                    .style(style);
                m.trigger(Element::from(trigger))
            },
            |_| {
                menu::group([
                    menu::item("Documentation").into(),
                    menu::item("Themes").into(),
                    menu::item("GitHub").into(),
                ])
            },
        )
        .align(dropdown_menu::Align::Start),
    ),
    breadcrumb::page("Breadcrumb"),
])
.separator(|| "\u{2022}".into())
```
