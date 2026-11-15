//! Recovery navigation for an unrecognized site path.

use iced::{Element, Fill};
use icedcn::Theme;
use icedcn::widget::{button, column, container, row, text};

use super::Message;

pub(super) fn view(path: &str) -> Element<'_, Message, Theme> {
    container(
        column![
            text::muted("404"),
            text::h1("Page not found"),
            text::p("We couldn't find a page at this address."),
            text::muted(path)
                .width(Fill)
                .height(24)
                .wrapping(text::Wrapping::None)
                .ellipsis(text::Ellipsis::End),
            row![
                button("Home").anchor().on_press(Message::GoTo("/")),
                button("Components")
                    .anchor()
                    .style(button::Outline)
                    .on_press(Message::GoTo("/docs/components")),
                button("Examples")
                    .anchor()
                    .style(button::Outline)
                    .on_press(Message::GoTo("/examples")),
            ]
            .spacing(8),
        ]
        .spacing(16)
        .width(Fill)
        .max_width(600.),
    )
    .padding(32)
    .center_x(Fill)
    .center_y(Fill)
    .into()
}
