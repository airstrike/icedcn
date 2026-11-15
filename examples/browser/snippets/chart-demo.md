```rust
use iced::widget::component::{Component, component};
use iced::{Element, Fill, Pixels, Widget, color};
use icedcn::theme::tweak::Bg;
use icedcn::widget::chart::{Data, axis, bar, bars, data, hover, hyozu};
use icedcn::widget::{button, chart, column, container, row, separator, text};
use icedcn::{Theme, Vertical, card, radius};

type Message = ();

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

fn visitors() -> [Data; 2] {
    [
        daily(DESKTOP_DAILY, hyozu::Color::Secondary),
        daily(MOBILE_DAILY, hyozu::Color::Primary),
    ]
}

fn view<'a>(
    series: &'a [Data; 2],
    theme: &Theme,
    window_width: f32,
) -> Element<'a, Message, Theme> {
    component(Visitors {
        series,
        theme: theme.clone(),
        compact: window_width < 640.0,
    })
    .boxed()
}
```
