use crate::{Message, icon, page};
use drawer::Side;
use iced::widget::component::{Component, component};
use iced::widget::{Space, scrollable};
use iced::{Alignment, Element, Fill, Task, Widget};
use icedxdy::theme::container::Style;
use icedxdy::widget::{button, column, container, dialog, drawer, field, input, row, text};
use icedxdy::{Slot, Theme};
pub static META: page::Meta = page::Meta {
    title: "Drawer",
    description: "A drawer component.",
};
pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/drawer-demo.md",
    },
    page::Block::Md("examples/browser/snippets/drawer-about.md"),
    page::Block::Md("examples/browser/snippets/drawer-composition.md"),
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "scrollable-content",
                title: "Scrollable Content",
                description: Some("Keep actions visible while the content scrolls."),
                snippet: "examples/browser/snippets/drawer-scrollable-content.md",
            },
            page::Block::Example {
                id: "sides",
                title: "Sides",
                description: Some("Use the direction to set the side of the drawer."),
                snippet: "examples/browser/snippets/drawer-sides.md",
            },
            page::Block::Example {
                id: "dialog",
                title: "Responsive Dialog",
                description: Some("Render a dialog on desktop and a drawer on mobile."),
                snippet: "examples/browser/snippets/drawer-dialog.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/drawer-api-reference.md"),
];
pub struct DrawerPage {
    state: page::State,
}

impl DrawerPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

#[derive(Default)]
struct Goal(Option<i32>);
struct Activity;
impl<'a> Component<'a, Message, Theme> for Activity {
    type State = Goal;
    type Event = i32;
    fn update(&self, state: &mut Goal, next: i32, _: &iced::Renderer) -> Option<Message> {
        state.0 = Some(next.clamp(200, 400));
        None
    }
    fn view(&self, state: &Goal) -> impl Widget<i32, Theme> + 'a {
        let goal = state.0.unwrap_or(350);
        let chart = [
            400, 300, 200, 300, 200, 278, 189, 239, 300, 200, 278, 189, 349,
        ];
        Element::from(drawer(
            |d| d.trigger(button("Open Drawer").style(button::Outline)),
            move |d| {
                let bars = row::Row::with_children(chart.into_iter().map(|v| {
                    container(Space::new())
                        .width(Fill)
                        .height(v as f32 * 0.3)
                        .style(|t: &Theme| {
                            Style {
                                background: Some(t.primary.base.into()),
                                ..Style::default()
                            }
                            .into()
                        })
                        .into()
                }))
                .spacing(5)
                .height(120)
                .align_y(Alignment::End);
                let controls = row![
                    button(icon::minus())
                        .size(icedxdy::Size::Sm)
                        .square()
                        .rounded()
                        .style(button::Outline)
                        .disabled(goal <= 200)
                        .on_press(goal - 10),
                    container(
                        column![
                            text(goal.to_string()).size(72).bold(),
                            text::muted("CALORIES/DAY").size(11)
                        ]
                        .align_x(Alignment::Center)
                    )
                    .center_x(Fill),
                    button(icon::plus())
                        .size(icedxdy::Size::Sm)
                        .square()
                        .rounded()
                        .style(button::Outline)
                        .disabled(goal >= 400)
                        .on_press(goal + 10),
                ]
                .spacing(8)
                .align_y(Alignment::Center);
                container(
                    column![
                        drawer::header("Move Goal", "Set your daily activity goal."),
                        container(column![controls, bars].spacing(12)).padding([0, 16]),
                        drawer::footer([
                            d.close(button("Submit").width(Fill)),
                            d.close(button("Cancel").style(button::Outline).width(Fill))
                        ]),
                    ]
                    .width(Fill.max(384.)),
                )
                .center_x(Fill)
                .into()
            },
        ))
    }
}
const LOREM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
fn panel(label: &'static str, side: Side, fraction: f32) -> Slot<'static, Message> {
    drawer(
        move |d| d.trigger(button(label).style(button::Outline)),
        |d| {
            let body = column::Column::with_children((0..10).map(|_| text(LOREM).size(14).into()))
                .spacing(16)
                .padding([0, 16]);
            column![
                drawer::header("Move Goal", "Set your daily activity goal."),
                scrollable(Slot::from(body).compose()).height(Fill).boxed(),
                drawer::footer([
                    d.close(button("Submit").width(Fill)),
                    d.close(button("Cancel").style(button::Outline).width(Fill))
                ])
            ]
            .width(Fill)
            .into()
        },
    )
    .side(side)
    .height_fraction(fraction)
    .into()
}
fn profile() -> Slot<'static, Message> {
    let trigger = |d: &dialog::Handle| d.trigger(button("Edit Profile").style(button::Outline));
    let form = |d: &dialog::Handle| {
        column![
            field(input("").default_value("shadcn@example.com")).label("Email"),
            field(input("").default_value("@shadcn")).label("Username"),
            d.close(button("Save changes").width(Fill))
        ]
        .spacing(24)
    };
    let description = "Make changes to your profile here. Click save when you're done.";
    drawer(trigger, move |d| {
        if d.is_mobile() {
            column![
                drawer::header("Edit profile", description),
                container(form(d)).padding([0, 16]),
                drawer::footer([d.close(button("Cancel").width(Fill).style(button::Outline))])
            ]
            .into()
        } else {
            column![dialog::header("Edit profile", description), form(d)]
                .spacing(16)
                .into()
        }
    })
    .desktop_dialog(425.)
    .into()
}
impl page::Page for DrawerPage {
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
            "demo" => component(Activity).boxed(),
            "scrollable-content" => panel("Scrollable Content", Side::Right, 0.8).compose(),
            "sides" => row![
                panel("Top", Side::Top, 0.5),
                panel("Right", Side::Right, 0.5),
                panel("Bottom", Side::Bottom, 0.5),
                panel("Left", Side::Left, 0.5)
            ]
            .spacing(8)
            .into(),
            "dialog" => profile().compose(),
            _ => unreachable!("unknown drawer preview: {id}"),
        }
    }
}
