use crate::icon;
use crate::{Message, page};
use iced::widget::component::{Component, component};
use iced::{Color, Element, Fill, Pixels, Task, Widget, color};
use icedcn::theme::tweak::Bg;
use icedcn::widget::chart::{Data, axis, bar, bars, data, hover, hyozu, legend, tooltip};
use icedcn::widget::{button, chart, column, container, row, separator, text};
use icedcn::{Slot, Theme, Vertical, theme};
use icedcn::{card, radius};
use std::borrow::Cow;

pub static META: page::Meta = page::Meta {
    title: "Chart",
    description: "Interactive charts built with Hyozu.",
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
const LABELS: [&str; 6] = ["January", "February", "March", "April", "May", "June"];
const DESKTOP: [f32; 6] = [186.0, 305.0, 237.0, 73.0, 209.0, 214.0];
const MOBILE: [f32; 6] = [80.0, 200.0, 120.0, 190.0, 130.0, 140.0];
const BLUE: Color = color!(0x2563eb);
const LIGHT_BLUE: Color = color!(0x60a5fa);

fn monthly() -> Data {
    data(
        bars([
            bar(DESKTOP).with_name("Desktop").with_color(BLUE),
            bar(MOBILE).with_name("Mobile").with_color(LIGHT_BLUE),
        ])
        .data_labels(None),
    )
    .x_axis_labels(LABELS)
    .x_axis(|axis| {
        axis.none().labels(|month: f64| {
            LABELS
                .get(month.round() as usize)
                .unwrap_or(&"")
                .chars()
                .take(3)
                .collect::<String>()
        })
    })
    .y_axis(|axis| axis.none())
    .show_tooltip(false)
}

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
struct Visitors<'a> {
    series: &'a [Data; 2],
    theme: Theme,
    compact: bool,
}
impl<'a> Component<'a, Message, Theme> for Visitors<'a> {
    type State = bool;
    type Event = bool;
    fn update(&self, state: &mut bool, mobile: bool, _: &iced::Renderer) -> Option<Message> {
        *state = mobile;
        None
    }
    fn view(&self, mobile: &bool) -> impl Widget<bool, Theme> + 'a {
        let metric = |label: &'static str, total: &'static str, is_mobile: bool| {
            button(
                column![
                    text::muted(label).size(12).line_height(Pixels(16.0)),
                    text(total)
                        .size(if self.compact { 18 } else { 30 })
                        .bold()
                        .line_height(Pixels(if self.compact { 18.0 } else { 30.0 })),
                ]
                .spacing(4)
                .width(Fill),
            )
            .style(
                button::Ghost
                    + Bg::from(if *mobile == is_mobile {
                        self.theme.muted.base.scale_alpha(0.5)
                    } else {
                        icedcn::Color::TRANSPARENT
                    })
                    + radius(0),
            )
            .translate(false)
            .on_press(is_mobile)
            .width(if self.compact { Fill } else { 144.into() })
            .height(Fill)
            .padding(if self.compact { [16, 24] } else { [24, 32] })
            .inset(0)
        };
        let title = container(
            column![
                card::title("Bar Chart - Interactive"),
                card::description("Showing total visitors for the last 3 months"),
            ]
            .spacing(4),
        )
        .height(Fill)
        .align_y(iced::Center)
        .padding([0, 24])
        .width(Fill);
        let metrics = row![
            metric("Desktop", "7,324", false),
            separator().orientation(Vertical),
            metric("Mobile", "7,250", true),
        ]
        .height(Fill);
        let header: Element<'_, bool, Theme> = if self.compact {
            column![
                container(title).height(68),
                separator(),
                metrics.height(70).width(Fill)
            ]
            .width(Fill)
            .into()
        } else {
            row![title, separator().orientation(Vertical), metrics]
                .height(98)
                .width(Fill)
                .into()
        };
        Element::from(
            card(
                column![
                    header,
                    separator(),
                    container(
                        chart(&self.series[usize::from(*mobile)])
                            .height(250)
                            .padding([0, 12])
                            .style(chart::transparent)
                            .corners(0)
                            .hover_group(self.theme.clone(), |group| {
                                hover::Annotation::new(Element::from(
                                    chart::tooltip(group.entries.iter())
                                        .label(format!("{}, 2024", group.label))
                                        .font("Geist Mono")
                                        .width(150),
                                ))
                                .transparent()
                                .padding(0)
                            }),
                    )
                    .padding(if self.compact { [0, 8] } else { [24, 24] })
                    .width(Fill),
                ]
                .width(Fill),
            )
            .gap(0)
            .padding([0, 0, 16, 0])
            .width(Fill),
        )
    }
}
const APRIL: f64 = 1_711_929_600.0;

fn daily(values: [f32; 30], color: hyozu::Color) -> Data {
    let points = values
        .into_iter()
        .enumerate()
        .map(|(day, value)| (APRIL + day as f64 * 86_400.0, value))
        .collect::<Vec<_>>();
    data(bars([bar(points).with_name("Page Views").with_color(color)]).data_labels(None))
        .x_axis(|axis| {
            axis.with_kind(axis::Kind::Time)
                .show_line(false)
                .show_grid(false)
                .labels(|seconds: f64| {
                    format!("Apr {}", ((seconds - APRIL) / 86_400.0).round() as i32 + 1)
                })
        })
        .y_axis(|axis| axis.none().show_grid(true))
        .x_axis_bounds(APRIL - 43_200.0, APRIL + 29.5 * 86_400.0)
        .tooltip(
            hyozu::Tooltip::default()
                .tracking_line(false)
                .category_band(hyozu::Color::themed(color!(0xf5f5f5), color!(0x262626))),
        )
}

fn anatomy<'a>(compact: bool) -> Element<'a, Message, Theme> {
    let [first, second, third, fourth, _] = theme::chart::COLORS;
    let panel = |content: Slot<'a, Message>| {
        container(content)
            .width(224)
            .height(137)
            .center_x(224)
            .center_y(137)
    };
    let label = panel(
        row![
            text("Label").size(14).medium(),
            icon::arrow_right(),
            chart::tooltip([("Desktop", 186, first), ("Mobile", 80, second)])
                .label("Page Views")
                .font("Geist Mono")
                .width(128),
        ]
        .spacing(8)
        .align_y(iced::Center)
        .into(),
    );
    let name = panel(
        column![
            row![text("Name").size(14).medium(), icon::arrow_down()].spacing(8),
            chart::tooltip([("Chrome", 1286, third), ("Firefox", 1000, fourth)])
                .label("Browser")
                .hide_label(true)
                .indicator(tooltip::Dashed)
                .font("Geist Mono")
                .width(128),
        ]
        .spacing(8)
        .into(),
    );
    let line = panel(
        chart::tooltip([("Desktop", 12486, third)])
            .label("Page Views")
            .indicator(tooltip::Line)
            .font("Geist Mono")
            .width(144)
            .into(),
    );
    let indicator = panel(
        column![
            chart::tooltip([("Chrome", 1286, first)])
                .label("Browser")
                .hide_label(true)
                .font("Geist Mono")
                .width(128),
            row![icon::arrow_up(), text("Indicator").size(14).medium()].spacing(8),
        ]
        .spacing(8)
        .into(),
    );
    if compact {
        column![label, name, indicator].into()
    } else {
        column![row![label, name], row![line, indicator]].into()
    }
}

pub struct ChartPage {
    state: page::State,
    monthly: [Data; 5],
    daily: [Data; 2],
    window_width: f32,
}

impl ChartPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
            monthly: [
                monthly(),
                monthly().y_axis(|axis| axis.show_grid(true)),
                monthly()
                    .y_axis(|axis| axis.show_grid(true))
                    .x_axis(|axis| axis.show_labels(true)),
                monthly()
                    .y_axis(|axis| axis.show_grid(true))
                    .x_axis(|axis| axis.show_labels(true))
                    .tooltip(
                        hyozu::Tooltip::default()
                            .tracking_line(false)
                            .category_band(hyozu::Color::themed(
                                color!(0xf5f5f5),
                                color!(0x262626),
                            )),
                    ),
                monthly()
                    .y_axis(|axis| axis.show_grid(true))
                    .x_axis(|axis| axis.show_labels(true))
                    .tooltip(
                        hyozu::Tooltip::default()
                            .tracking_line(false)
                            .category_band(hyozu::Color::themed(
                                color!(0xf5f5f5),
                                color!(0x262626),
                            )),
                    )
                    .legend(legend::Config::below().size(12).interactive(true)),
            ],
            daily: [
                daily(DESKTOP_DAILY, hyozu::Color::Secondary),
                daily(MOBILE_DAILY, hyozu::Color::Primary),
            ],
            window_width: 1440.0,
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

    fn window_width(&mut self, width: f32) {
        self.window_width = width;
    }

    fn preview(&self, id: &str) -> Element<'_, Message, Theme> {
        self.themed_preview(id, &Theme::default())
    }

    fn themed_preview(&self, id: &str, theme: &Theme) -> Element<'_, Message, Theme> {
        if id == "demo" {
            return component(Visitors {
                series: &self.daily,
                theme: theme.clone(),
                compact: self.window_width < 640.0,
            })
            .boxed();
        }
        if id == "tooltip" {
            return anatomy(self.window_width < 768.0);
        }
        let index = match id {
            "example" => 0,
            "example-grid" => 1,
            "example-axis" => 2,
            "example-tooltip" => 3,
            "example-legend" => 4,
            _ => unreachable!("unknown chart preview id: {id}"),
        };
        let chart = chart(&self.monthly[index])
            .height(200)
            .padding(0)
            .style(chart::transparent)
            .design(Cow::<Theme>::Owned(theme.clone()));
        if index >= 3 {
            chart
                .hover_group(theme.clone(), |group| {
                    hover::Annotation::new(Element::from(
                        chart::tooltip(group.entries.iter())
                            .label(&group.label)
                            .font("Geist Mono"),
                    ))
                    .transparent()
                    .padding(0)
                })
                .boxed()
        } else {
            chart.boxed()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::Rectangle;
    use iced::advanced::widget::{Id, Operation};
    use iced_test::core::Settings;
    use iced_test::simulator::Simulator;

    fn settings() -> Settings {
        Settings {
            font: "Geist".into(),
            fonts: vec![
                include_bytes!("../../fonts/Geist.ttf").as_slice().into(),
                crate::font::GEIST_MONO.into(),
            ],
            ..Settings::default()
        }
    }

    #[test]
    fn narrow_previews_keep_the_header_and_tooltip_anatomy_inside_the_viewport() {
        let mut page = ChartPage::new(0);
        page.window_width(375.0);
        let theme = Theme::light();
        let mut ui = Simulator::with_size(
            settings(),
            [343.0, 600.0],
            page.themed_preview("demo", &theme),
        );
        for label in [
            "Bar Chart - Interactive",
            "Showing total visitors for the last 3 months",
            "Desktop",
            "Mobile",
            "7,324",
            "7,250",
        ] {
            let bounds = ui.find(label).unwrap().bounds();
            assert!(
                bounds.x >= 0.0 && bounds.x + bounds.width <= 343.1,
                "{label}: {bounds:?}"
            );
        }
        let description = ui
            .find("Showing total visitors for the last 3 months")
            .unwrap()
            .bounds();
        assert!(ui.find("Desktop").unwrap().bounds().y >= description.y + description.height);

        let mut ui = Simulator::with_size(settings(), [343.0, 600.0], anatomy(true));
        assert!(ui.find("12,486").is_err());
        for (index, label) in ["Label", "Name", "Indicator"].into_iter().enumerate() {
            let bounds = ui.find(label).unwrap().bounds();
            assert!(
                bounds.x >= 0.0 && bounds.x + bounds.width <= 343.1,
                "{label}: {bounds:?}"
            );
            assert!(
                bounds.y >= index as f32 * 137.0
                    && bounds.y + bounds.height <= (index + 1) as f32 * 137.0,
                "{label}: {bounds:?}"
            );
        }
    }

    #[test]
    fn tooltip_rows_keep_names_and_values_separate_with_real_fonts() {
        #[derive(Default)]
        struct Bounds(Option<Rectangle>);
        impl Operation for Bounds {
            fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
                visit(self);
            }
            fn container(&mut self, _: Option<&Id>, bounds: Rectangle, _: &Rectangle) {
                self.0.get_or_insert(bounds);
            }
        }
        for (width, indicator, label, entries) in [
            (
                Some(150),
                tooltip::Dot,
                "Apr 11, 2024",
                vec![("Page Views", 327, "327")],
            ),
            (
                None,
                tooltip::Dot,
                "January",
                vec![("Desktop", 186, "186"), ("Mobile", 80, "80")],
            ),
            (
                Some(144),
                tooltip::Line,
                "Page Views",
                vec![("Desktop", 12486, "12,486")],
            ),
            (
                None,
                tooltip::Dot,
                "September",
                vec![("International Desktop Visitors", 1234567, "1,234,567")],
            ),
        ] {
            let mut content =
                chart::tooltip(entries.iter().map(|(name, value, _)| (*name, *value, BLUE)))
                    .label(label)
                    .indicator(indicator)
                    .font("Geist Mono");
            if let Some(width) = width {
                content = content.width(width);
            }
            let content: Element<'_, (), Theme> = content.into();
            let mut ui = Simulator::with_size(settings(), [640.0, 480.0], content);
            let mut bounds = Bounds::default();
            ui.operate(&mut bounds);
            let surface = bounds.0.expect("tooltip surface");
            if let Some(width) = width {
                assert_eq!(surface.width, width as f32);
            } else {
                let expected = if label == "September" {
                    220.0..=320.0
                } else {
                    128.0..=150.0
                };
                assert!(expected.contains(&surface.width), "{surface:?}");
            }
            let heading = ui.find(label).unwrap().bounds();
            assert!(
                heading.height <= 16.1,
                "{label} {indicator:?}: heading {heading:?}, surface {surface:?}"
            );
            for (name, _, value) in entries {
                let name = ui.find(name).unwrap().bounds();
                let value = ui.find(value).unwrap().bounds();
                assert!(name.height <= 16.1, "name wrapped: {name:?}");
                assert!(
                    name.x + name.width <= value.x + 0.1,
                    "name {name:?}, value {value:?}"
                );
                assert!(
                    (value.x + value.width - (surface.x + surface.width - 10.0)).abs() < 0.1,
                    "value {value:?}, surface {surface:?}"
                );
            }
        }
    }
}
