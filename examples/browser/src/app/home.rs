//! The library landing page: real, interactive compositions of the public widgets.

use std::sync::LazyLock;

use iced::widget::component::{Component, component};
use iced::{Center, Element, Fill, Pixels, Widget};
use icedcn::widget::calendar::{Selection, today};
use icedcn::widget::chart::{Data, bar, bars};
use icedcn::widget::{
    alert_dialog, avatar, badge, button, calendar, chart, checkbox, column, container, field,
    input, progress, row, scrollable, separator, slider, space, switch, text, textarea,
};
use icedcn::{Size, Slot, Theme, card, theme};

use super::Message;
use crate::icon;

static CONTRIBUTIONS: LazyLock<Data> = LazyLock::new(|| {
    chart::data(
        bars([bar([1200, 1650, 1350, 1950, 1100, 2100]).with_name("Contributions")])
            .data_labels(None),
    )
    .x_axis_labels(["Dec", "Jan", "Feb", "Mar", "Apr", "May"])
    .x_axis(|axis| axis.show_line(false).show_grid(false))
    .y_axis(|axis| axis.none().show_grid(false))
    .value_format(|value| format!("${value:.0}"))
});

pub(super) fn view<'a>(width: f32) -> Element<'a, Message, Theme> {
    let desktop = width >= 1280.;
    let mobile = width < 720.;
    let hero = column![
        button(row![text::xs("Native components. Ready for the web."), icon::arrow_right().size(12)]
            .spacing(6).align_y(Center))
            .style(button::Secondary).height(24).padding([0, 10])
            .on_press(Message::GoTo("/docs/introduction")),
        text::h1("The foundation for your next Rust app.")
            .size(if mobile { 34. } else { 48. })
            .semibold().letter_spacing(-0.045)
            .line_height(Pixels(if mobile { 38. } else { 53. }))
            .align_x(Center).width(Fill),
        container(text::p("Beautifully designed components, built with Rust and iced.\nCompose, customize, and make them your own.")
            .size(if mobile { 16. } else { 18. })
            .line_height(Pixels(28.)).align_x(Center).width(Fill))
            .max_width(720).center_x(Fill),
        container(row![
            button("Get Started").on_press(Message::GoTo("/docs/installation")),
            button("View Components").style(button::Secondary)
                .on_press(Message::GoTo("/docs/components")),
        ].spacing(8).align_y(Center)).padding([8, 0]),
    ].spacing(16).align_x(Center).width(Fill);

    let first = column![controls(), shortcuts(), savings()]
        .spacing(24)
        .width(Fill);
    let second = column![contributions(), team()].spacing(24).width(Fill);
    let third = column![component(Milestone).boxed(), preferences()]
        .spacing(24)
        .width(Fill);
    let fourth = column![dates(), customize()].spacing(24).width(Fill);
    let gallery: Slot<'_, Message> = if desktop {
        row![first, second, third, fourth]
            .spacing(24)
            .width(Fill)
            .into()
    } else if !mobile {
        row![
            column![first, third].spacing(24).width(Fill),
            column![second, fourth].spacing(24).width(Fill)
        ]
        .spacing(24)
        .width(Fill)
        .into()
    } else {
        column![first, second, third, fourth]
            .spacing(24)
            .width(Fill)
            .into()
    };

    let footer_label = text::small("Built with iced. Made to be yours.").style(theme::text::Muted);
    let footer_link = button("Explore examples")
        .style(button::Link)
        .on_press(Message::GoTo("/examples"));
    let footer: Slot<'_, Message> = if mobile {
        column![footer_label, footer_link]
            .spacing(8)
            .align_x(Center)
            .width(Fill)
            .into()
    } else {
        row![footer_label, space::horizontal(), footer_link]
            .spacing(12)
            .align_y(Center)
            .width(Fill)
            .into()
    };

    scrollable(
        container(
            column![
                container(hero)
                    .padding(
                        iced::Padding::ZERO
                            .top(if mobile { 40. } else { 72. })
                            .bottom(if mobile { 40. } else { 64. })
                    )
                    .width(Fill),
                gallery,
                container(footer).padding([32, 0]),
            ]
            .width(Fill)
            .max_width(1552.),
        )
        .padding([0, if mobile { 16 } else { 24 }])
        .center_x(Fill),
    )
    .height(Fill)
    .width(Fill)
    .into()
}

fn panel<'a, M: 'a>(content: impl Into<Slot<'a, M>>) -> Slot<'a, M> {
    card(container(content).padding(20).width(Fill))
        .padding(0)
        .width(Fill)
        .into()
}

fn controls<'a>() -> Slot<'a, Message> {
    panel(column![
        row![
            button(row!["Button", icon::arrow_right().size(14)].spacing(6))
                .size(Size::Sm).on_press(Message::GoTo("/components/button")),
            button("Secondary").style(button::Secondary).size(Size::Sm)
                .on_press(Message::GoTo("/components/button")),
            button("Outline").style(button::Outline).size(Size::Sm)
                .on_press(Message::GoTo("/components/button")),
        ].spacing(8),
        input("Name").width(Fill),
        textarea("Message").resizable(false),
        row![badge("Badge"), badge("Secondary").style(badge::Secondary),
            space::horizontal(), checkbox().default_checked(true), switch().default_checked(true)]
            .spacing(8).align_y(Center).width(Fill),
        row![
            alert_dialog(
                |a| a.trigger(button("Alert Dialog").style(button::Outline).size(Size::Sm)),
                |a| column![
                    a.header("Make it your own", "Every preview on this page uses the same components you can use in your application."),
                    a.footer([
                        a.cancel(button("Close").style(button::Outline)),
                        a.action(button("View components").on_press(Message::GoTo("/docs/components"))),
                    ]),
                ].width(Fill).into(),
            ).size(Size::Sm),
            space::horizontal(),
            button(row!["Button Group", icon::chevron_up().size(14)].spacing(8))
                .style(button::Outline).size(Size::Sm)
                .on_press(Message::GoTo("/components/button-group")),
        ].align_y(Center).width(Fill),
    ].spacing(24).width(Fill))
}

fn shortcuts<'a>() -> Slot<'a, Message> {
    panel(
        column![
            text::small("Explore the library").medium(),
            route(
                "Components",
                icon::gallery_vertical_end(),
                "/docs/components"
            ),
            route("Installation", icon::terminal(), "/docs/installation"),
            route("Charts", icon::chart_pie(), "/components/chart"),
            route("Examples", icon::panels_top_left(), "/examples"),
        ]
        .spacing(8)
        .width(Fill),
    )
}

fn route<'a>(
    label: &'static str,
    glyph: impl Into<Slot<'a, Message>>,
    path: &'static str,
) -> Slot<'a, Message> {
    button(
        row![
            glyph.into(),
            text::small(label),
            space::horizontal(),
            icon::arrow_up_right().size(14)
        ]
        .spacing(12)
        .align_y(Center)
        .width(Fill),
    )
    .style(button::Ghost)
    .width(Fill)
    .on_press(Message::GoTo(path))
    .into()
}

fn savings<'a>() -> Slot<'a, Message> {
    panel(
        column![
            text::p("Savings targets").medium(),
            text::muted("A little progress, every month.").width(Fill),
            row![
                text::small("Next adventure"),
                space::horizontal(),
                badge("65%").style(badge::Secondary)
            ]
            .width(Fill),
            progress(65.).width(Fill),
            row![
                text::muted("$3,250 saved"),
                space::horizontal(),
                text::small("$5,000")
            ]
            .width(Fill),
        ]
        .spacing(16)
        .width(Fill),
    )
}

fn contributions<'a>() -> Slot<'a, Message> {
    panel(
        column![
            text::p("Contribution history").medium(),
            text::muted("Last 6 months of activity"),
            chart(&CONTRIBUTIONS)
                .height(208)
                .padding(0)
                .style(chart::transparent),
            row![
                detail("Upcoming", "May 2027", "Scheduled"),
                detail("Savings plan", "Accelerated", "Recurring"),
            ]
            .spacing(12)
            .width(Fill),
            button("Explore charts")
                .width(Fill)
                .on_press(Message::GoTo("/components/chart")),
        ]
        .spacing(16)
        .width(Fill),
    )
}

fn detail<'a>(label: &'static str, value: &'static str, note: &'static str) -> Slot<'a, Message> {
    container(
        column![
            text::xs(label).style(theme::text::Muted),
            text::p(value).semibold(),
            text::muted(note)
        ]
        .spacing(6)
        .width(Fill),
    )
    .padding(14)
    .width(Fill)
    .class(theme::container::CardMuted)
    .into()
}

fn team<'a>() -> Slot<'a, Message> {
    panel(
        column![
            text::p("Your team").medium(),
            text::muted("Great things are built together.").width(Fill),
            member("AL", "Ada Lovelace", "Admin"),
            member("GH", "Grace Hopper", "Developer"),
            member("AT", "Alan Turing", "Developer"),
            separator(),
            button("Build your workspace")
                .style(button::Outline)
                .width(Fill)
                .on_press(Message::GoTo("/create")),
        ]
        .spacing(20)
        .width(Fill),
    )
}

fn member<'a>(initials: &'static str, name: &'static str, role: &'static str) -> Slot<'a, Message> {
    row![
        avatar(None).fallback(initials),
        column![
            text::small(name).medium(),
            text::xs(role).style(theme::text::Muted)
        ]
        .spacing(3),
        space::horizontal(),
        icon::ellipsis().size(16)
    ]
    .spacing(12)
    .align_y(Center)
    .width(Fill)
    .into()
}

fn preferences<'a>() -> Slot<'a, Message> {
    panel(
        column![
            text::p("Make yourself at home").medium(),
            text::muted("Small details. Thoughtful defaults.").width(Fill),
            field(switch().default_checked(true))
                .label("Notifications")
                .orientation(field::Horizontal),
            separator(),
            field(switch())
                .label("Focus mode")
                .orientation(field::Horizontal),
            separator(),
            text::small("Volume").medium(),
            slider(0.0..=100.0).default_value(65.).width(Fill),
        ]
        .spacing(20)
        .width(Fill),
    )
}

fn dates<'a>() -> Slot<'a, Message> {
    panel(
        column![
            text::p("Make time for something new").medium().width(Fill),
            text::muted("Pick a day to get started.").width(Fill),
            container(
                calendar()
                    .default_selected(Selection::single(today()))
                    .cell_size(36.)
                    .padding(0.)
            )
            .center_x(Fill),
            button("Explore calendars")
                .style(button::Outline)
                .width(Fill)
                .on_press(Message::GoTo("/components/calendar")),
        ]
        .spacing(16)
        .width(Fill),
    )
}

fn customize<'a>() -> Slot<'a, Message> {
    panel(
        column![
            row![
                icon::palette().size(22),
                space::horizontal(),
                badge("Yours").style(badge::Secondary)
            ]
            .align_y(Center)
            .width(Fill),
            text::h3("Start with a style.\nMake it your own.")
                .size(28)
                .width(Fill),
            text::muted(
                "Choose your colors, typography, and radius. See every component come together."
            )
            .width(Fill),
            button(row!["Create your theme", icon::arrow_right().size(14)].spacing(8))
                .width(Fill)
                .on_press(Message::GoTo("/create")),
        ]
        .spacing(20)
        .width(Fill),
    )
}

struct Milestone;

struct State {
    name: String,
    amount: String,
    date: String,
    saved: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            name: String::new(),
            amount: "$15,000".into(),
            date: "Dec 2027".into(),
            saved: false,
        }
    }
}

#[derive(Clone)]
enum Event {
    Name(String),
    Amount(String),
    Date(String),
    Save,
    Reset,
}

impl<'a> Component<'a, Message, Theme> for Milestone {
    type State = State;
    type Event = Event;

    fn update(&self, state: &mut State, event: Event, _: &iced::Renderer) -> Option<Message> {
        state.saved = false;
        match event {
            Event::Name(value) => state.name = value,
            Event::Amount(value) => state.amount = value,
            Event::Date(value) => state.date = value,
            Event::Save => state.saved = !state.name.trim().is_empty(),
            Event::Reset => *state = State::default(),
        }
        None
    }

    fn view(&self, state: &State) -> impl Widget<Event, Theme> + 'a {
        Element::from(panel(
            column![
                text::p(if state.saved {
                    "Milestone created"
                } else {
                    "Set a new milestone"
                })
                .medium(),
                text::muted(if state.saved {
                    "Your next chapter starts here."
                } else {
                    "Define your target and take the first step."
                })
                .width(Fill),
                field(
                    input("e.g. My next adventure")
                        .value(state.name.clone())
                        .on_input(Event::Name)
                        .width(Fill)
                )
                .label("Goal name")
                .width(Fill),
                row![
                    field(
                        input("")
                            .value(state.amount.clone())
                            .on_input(Event::Amount)
                            .width(Fill)
                    )
                    .label("Target amount")
                    .width(Fill),
                    field(
                        input("")
                            .value(state.date.clone())
                            .on_input(Event::Date)
                            .width(Fill)
                    )
                    .label("Target date")
                    .width(Fill),
                ]
                .spacing(12)
                .width(Fill),
                column![
                    button(if state.saved { "Saved" } else { "Create goal" })
                        .width(Fill)
                        .disabled(state.name.trim().is_empty() || state.saved)
                        .on_press(Event::Save),
                    button("Reset")
                        .style(button::Outline)
                        .width(Fill)
                        .on_press(Event::Reset),
                ]
                .spacing(8)
                .width(Fill),
            ]
            .spacing(20)
            .width(Fill),
        ))
    }
}
