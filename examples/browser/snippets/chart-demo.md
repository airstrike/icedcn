```rust
const LABELS: [&str; 6] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun"];
const DESKTOP: [f32; 6] = [186.0, 305.0, 237.0, 73.0, 209.0, 214.0];
const MOBILE: [f32; 6] = [80.0, 200.0, 120.0, 190.0, 130.0, 140.0];
const BLUE: Color = Color::from_rgb8(37, 99, 235);
const LIGHT_BLUE: Color = Color::from_rgb8(96, 165, 250);
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
            .width(Fill)
            .boxed(),
        )
    }
}
```
