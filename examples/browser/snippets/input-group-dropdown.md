```rust
use crate::icon;
use iced::Fill;
use icedcn::Slot;
use icedcn::core::radius;
use icedcn::theme::text_input;
use icedcn::widget::input_group::{Align, addon};
use icedcn::widget::{column, dropdown_menu, input, input_group, menu, row};

fn dropdowns<'a, M: Clone + 'static>(size: f32) -> Slot<'a, M> {
    column![
        input_group(input("Enter file name").text_size(size)).addon(
            addon(
                dropdown_menu(
                    |d| {
                        d.trigger(
                            input_group::button(icon::ellipsis().size(16).line_height(1.0))
                                .square(),
                        )
                    },
                    |_| {
                        menu::group(
                            ["Settings", "Copy path", "Open location"]
                                .map(|item| menu::item(item).into()),
                        )
                    },
                )
                .align(dropdown_menu::Align::End),
            )
            .align(Align::InlineEnd),
        ),
        input_group(input("Enter search query").text_size(size))
            .style(text_input::Default + radius(16))
            .addon(
                addon(
                    dropdown_menu(
                        |d| {
                            d.trigger(input_group::button(
                                row![
                                    "Search In...",
                                    icon::chevron_down().size(16).line_height(1.0)
                                ]
                                .spacing(4),
                            ))
                        },
                        |_| {
                            menu::group(
                                ["Documentation", "Blog Posts", "Changelog"]
                                    .map(|item| menu::item(item).into()),
                            )
                        },
                    )
                    .align(dropdown_menu::Align::End),
                )
                .align(Align::InlineEnd),
            ),
    ]
    .spacing(16)
    .width(Fill)
    .max_width(384.)
    .into()
}
```
