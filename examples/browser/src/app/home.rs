//! A compact starting point for the component library and its tools.

use iced::{Element, Fill, Shrink};
use icedcn::widget::{button, column, container, row, scrollable, text};
use icedcn::{Theme, theme};

use super::Message;

pub(super) fn view<'a>(width: f32) -> Element<'a, Message, Theme> {
    let components = destination(
        "Components",
        "Explore the library, try each interaction, and copy Rust examples.",
        "/docs/components",
    );
    let examples = destination(
        "Examples",
        "See components come together in complete application screens.",
        "/examples",
    );
    let create = destination(
        "Create",
        "Choose your colors and see your theme in a live dashboard.",
        "/create",
    );
    let installation = destination(
        "Installation",
        "Add icedcn to your Rust application and build your first view.",
        "/docs/installation",
    );
    let destinations: Element<'_, Message, Theme> = if width >= 700. {
        column![
            row![components, examples].spacing(16).width(Fill),
            row![create, installation].spacing(16).width(Fill),
        ]
        .spacing(16)
        .width(Fill)
        .into()
    } else {
        column![components, examples, create, installation]
            .spacing(12)
            .width(Fill)
            .into()
    };
    scrollable(
        container(
            column![
                text::small("icedcn").style(theme::text::Muted),
                text::h1("Build interfaces with icedcn.")
                    .size(if width >= 700. { 48. } else { 32. })
                    .width(Fill),
                text::p("Composable components for native and web applications, built with Rust and iced.")
                    .style(theme::text::Muted)
                    .width(Fill),
                container(destinations).padding([24, 0]).width(Fill),
            ]
            .spacing(16)
            .width(Fill)
            .max_width(880.),
        )
        .padding(if width >= 700. { 64 } else { 24 })
        .center_x(Fill),
    )
    .height(Fill)
    .width(Fill)
    .into()
}

fn destination<'a>(
    label: &'static str,
    description: &'static str,
    path: &'static str,
) -> Element<'a, Message, Theme> {
    button(
        column![
            text::h3(label).width(Fill),
            text::p(description).style(theme::text::Muted).width(Fill),
        ]
        .spacing(8)
        .width(Fill),
    )
    .style(button::Outline)
    .padding(24)
    .height(Shrink)
    .width(Fill)
    .on_press(Message::GoTo(path))
    .into()
}
