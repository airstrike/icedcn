use crate::{Message, page};
use iced::Task;
use iced::widget::button::StyleFn;
use iced::widget::component::{Component, component};
use iced::{Color, Element, Fill, Widget};
use icedcn::Theme;
use icedcn::core::Border;
use icedcn::theme::container::Style;
use icedcn::widget::chart::{Chart, Series};
use icedcn::widget::{button, chart, column, container, row, separator, space, text};
use icedcn::{Slot, card};

pub static META: page::Meta = page::Meta {
    title: "Chart",
    description: "Interactive charts built with iced.",
};
pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/chart-demo.md",
    },
    page::Block::Md("examples/browser/snippets/chart-composition.md"),
    page::Block::Group {
        title: "Your First Chart",
        children: &[
            page::Block::Example {
                id: "example",
                title: "",
                description: Some("Start with desktop and mobile visitors grouped by month."),
                snippet: "examples/browser/snippets/chart-example.md",
            },
            page::Block::Example {
                id: "example-grid",
                title: "Add a Grid",
                description: None,
                snippet: "examples/browser/snippets/chart-example-grid.md",
            },
            page::Block::Example {
                id: "example-axis",
                title: "Add an Axis",
                description: None,
                snippet: "examples/browser/snippets/chart-example-axis.md",
            },
            page::Block::Example {
                id: "example-tooltip",
                title: "Add Tooltip",
                description: None,
                snippet: "examples/browser/snippets/chart-example-tooltip.md",
            },
            page::Block::Example {
                id: "example-legend",
                title: "Add Legend",
                description: None,
                snippet: "examples/browser/snippets/chart-example-legend.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/chart-config.md"),
    page::Block::Md("examples/browser/snippets/chart-guide-theming.md"),
    page::Block::Example {
        id: "tooltip",
        title: "Tooltip",
        description: None,
        snippet: "examples/browser/snippets/chart-tooltip.md",
    },
    page::Block::Md("examples/browser/snippets/chart-api-reference.md"),
    page::Block::Md("examples/browser/snippets/chart-guide-accessibility.md"),
];
const LABELS: [&str; 6] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun"];
const DESKTOP: [f32; 6] = [186.0, 305.0, 237.0, 73.0, 209.0, 214.0];
const MOBILE: [f32; 6] = [80.0, 200.0, 120.0, 190.0, 130.0, 140.0];
const BLUE: Color = Color::from_rgb8(37, 99, 235);
const LIGHT_BLUE: Color = Color::from_rgb8(96, 165, 250);
fn monthly() -> Chart<'static> {
    chart(
        &LABELS,
        [
            Series::new("Desktop", BLUE, &DESKTOP),
            Series::new("Mobile", LIGHT_BLUE, &MOBILE),
        ],
    )
}
const DAYS: [&str; 30] = [
    "Apr 1", "Apr 2", "Apr 3", "Apr 4", "Apr 5", "Apr 6", "Apr 7", "Apr 8", "Apr 9", "Apr 10",
    "Apr 11", "Apr 12", "Apr 13", "Apr 14", "Apr 15", "Apr 16", "Apr 17", "Apr 18", "Apr 19",
    "Apr 20", "Apr 21", "Apr 22", "Apr 23", "Apr 24", "Apr 25", "Apr 26", "Apr 27", "Apr 28",
    "Apr 29", "Apr 30",
];
const DESKTOP_DAILY: [f32; 30] = [
    222.0, 97.0, 167.0, 242.0, 373.0, 301.0, 245.0, 409.0, 59.0, 261.0, 327.0, 292.0, 342.0, 137.0,
    120.0, 138.0, 446.0, 364.0, 243.0, 89.0, 137.0, 224.0, 138.0, 387.0, 215.0, 75.0, 383.0, 122.0,
    315.0, 454.0,
];
const MOBILE_DAILY: [f32; 30] = [
    150.0, 180.0, 120.0, 260.0, 290.0, 340.0, 180.0, 320.0, 110.0, 190.0, 350.0, 210.0, 380.0,
    220.0, 170.0, 190.0, 360.0, 410.0, 180.0, 150.0, 200.0, 170.0, 230.0, 290.0, 250.0, 130.0,
    420.0, 180.0, 240.0, 380.0,
];
type ButtonStyle = StyleFn<'static, Theme>;
struct Visitors;
impl<'a> Component<'a, Message, Theme> for Visitors {
    type State = bool;
    type Event = bool;
    fn update(&self, state: &mut bool, mobile: bool, _: &iced::Renderer) -> Option<Message> {
        *state = mobile;
        None
    }
    fn view(&self, mobile: &bool) -> impl Widget<bool, Theme> + 'a {
        let metric = |label: &'static str, total: &'static str, is_mobile: bool| {
            let selected = *mobile == is_mobile;
            button(
                column![
                    text::muted(label).size(12.0),
                    text::large(total).size(24.0).bold()
                ]
                .spacing(4),
            )
            .style(if selected {
                ButtonStyle::from(button::Secondary)
            } else {
                ButtonStyle::from(button::Ghost)
            })
            .translate(false)
            .on_press(is_mobile)
            .height(72)
            .inset([8, 12])
            .map_style(|mut style| {
                style.border.radius = 0.into();
                style
            })
        };
        let series = if *mobile {
            Series::new("Page Views", LIGHT_BLUE, &MOBILE_DAILY)
        } else {
            Series::new("Page Views", BLUE, &DESKTOP_DAILY)
        };
        Element::from(
            card(
                column![
                    row![
                        column![
                            card::title("Bar Chart - Interactive"),
                            card::description("Showing total visitors for the last 3 months")
                        ]
                        .spacing(4)
                        .width(Fill)
                        .padding(12),
                        metric("Desktop", "7,324", false),
                        metric("Mobile", "7,250", true)
                    ]
                    .width(Fill),
                    separator(),
                    container(
                        chart(&DAYS, [series])
                            .height(250)
                            .grid(true)
                            .axis(true)
                            .tooltip(true)
                            .radius(0.0)
                    )
                    .padding(12)
                    .width(Fill),
                ]
                .width(Fill),
            )
            .gap(0)
            .padding(0)
            .width(Fill),
        )
    }
}
fn sample(
    title: Option<&'static str>,
    rows: &[(&'static str, &'static str, Color)],
    style: &str,
) -> Slot<'static, Message> {
    let mut items = Vec::new();
    if let Some(title) = title {
        items.push(text::small(title).medium().into());
    }
    for &(name, value, color) in rows {
        let swatch = |width, height| {
            container(space::horizontal())
                .width(width)
                .height(height)
                .style(move |_| Style {
                    background: Some(icedcn::Color::from(color).into()),
                    border: Border {
                        radius: 2.into(),
                        ..Default::default()
                    },
                    ..Style::default()
                })
        };
        let indicator: Slot<'static, Message> = match style {
            "dashed" => column![swatch(4, 3), swatch(4, 3), swatch(4, 3)]
                .spacing(2)
                .into(),
            "line" => swatch(4, 20).into(),
            _ => swatch(9, 9).into(),
        };
        items.push(
            row![
                indicator,
                text::muted(name).size(12.0).width(Fill),
                text::small(value).medium()
            ]
            .spacing(6)
            .width(Fill)
            .into(),
        );
    }
    container(column(items).spacing(4).width(Fill))
        .width(132)
        .padding(9)
        .style(|t| Style {
            background: Some(t.neutral.base.into()),
            border: Border {
                width: 1.0,
                color: t.neutral.border,
                radius: 8.into(),
            },
            ..Default::default()
        })
        .into()
}
fn anatomy() -> Slot<'static, Message> {
    let first = sample(
        Some("Page Views"),
        &[("Desktop", "186", BLUE), ("Mobile", "80", LIGHT_BLUE)],
        "dot",
    );
    let second = sample(
        None,
        &[("Chrome", "1,286", BLUE), ("Firefox", "1,000", LIGHT_BLUE)],
        "dashed",
    );
    let third = sample(Some("Page Views"), &[("Desktop", "12,486", BLUE)], "line");
    let fourth = sample(None, &[("Chrome", "1,286", LIGHT_BLUE)], "dot");
    column![
        row![
            column![text::small("Label"), first].spacing(8),
            column![text::small("Name"), second].spacing(8)
        ]
        .spacing(24),
        row![third, column![fourth, text::small("Indicator")].spacing(8)].spacing(24),
    ]
    .spacing(18)
    .into()
}
pub struct ChartPage {
    state: page::State,
}

impl ChartPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

impl page::Page for ChartPage {
    fn page_index(&self) -> usize {
        self.state.page_index
    }
    fn meta(&self) -> &'static page::Meta {
        &META
    }
    fn blocks(&self) -> &'static [page::Block] {
        BLOCKS
    }
    fn state(&self) -> &page::State {
        &self.state
    }
    fn scrollable_id(&self) -> iced::widget::Id {
        self.state.scrollable_id.clone()
    }

    fn init(&self) -> Task<Message> {
        self.state.init()
    }

    fn update(&mut self, event: page::Message) -> Task<Message> {
        self.state.update(&event)
    }

    fn reset_volatile(&mut self) {
        self.state.reset_volatile();
    }

    fn preview(&self, id: &str) -> iced::Element<'_, Message, Theme> {
        match id {
            "demo" => component(Visitors).boxed().into(),
            "example" => monthly().into(),
            "example-grid" => monthly().grid(true).into(),
            "example-axis" => monthly().grid(true).axis(true).into(),
            "example-tooltip" => monthly().grid(true).axis(true).tooltip(true).into(),
            "example-legend" => monthly()
                .grid(true)
                .axis(true)
                .tooltip(true)
                .legend(true)
                .into(),
            "tooltip" => anatomy().into(),
            _ => unreachable!("unknown chart preview id: {id}"),
        }
    }
}
