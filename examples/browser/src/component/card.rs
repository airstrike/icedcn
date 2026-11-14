// The upstream RTL example is outside this browser’s scope.

use iced::Alignment::Center;
use iced::Task;
use iced::Widget;
use iced::border;
use iced::widget::component::{Component, component};
use iced::widget::image;
use iced::{ContentFit, Element, Fill};
use icedxdy::theme::container::Style;
use std::sync::LazyLock;

use icedxdy::card;
use icedxdy::widget::{
    aspect_ratio, badge, button, column, container, field, input, row, scroll_area, separator,
    space, text, toggle_group,
};
use icedxdy::{Gap, Size, Slot, Theme, radius};

use crate::Message;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Card",
    description: "Displays a card with header, content, and footer.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/card-demo.md",
    },
    page::Block::Md("examples/browser/snippets/card-installation.md"),
    page::Block::Md("examples/browser/snippets/card-usage.md"),
    page::Block::Md("examples/browser/snippets/card-composition.md"),
    page::Block::Md("examples/browser/snippets/card-size-guide.md"),
    page::Block::Example {
        id: "small",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/card-small.md",
    },
    page::Block::Md("examples/browser/snippets/card-spacing-guide.md"),
    page::Block::Example {
        id: "spacing",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/card-spacing.md",
    },
    page::Block::Md("examples/browser/snippets/card-edge-guide.md"),
    page::Block::Example {
        id: "edge-to-edge",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/card-edge-to-edge.md",
    },
    page::Block::Example {
        id: "image",
        title: "Image",
        description: Some("Add an image before the card header to create a card with an image."),
        snippet: "examples/browser/snippets/card-image.md",
    },
    page::Block::Group {
        title: "API Reference",
        children: &[
            page::Block::Md("examples/browser/snippets/card-api-reference.md"),
            page::Block::Md("examples/browser/snippets/card-api-header.md"),
            page::Block::Md("examples/browser/snippets/card-api-title.md"),
            page::Block::Md("examples/browser/snippets/card-api-description.md"),
            page::Block::Md("examples/browser/snippets/card-api-action.md"),
            page::Block::Md("examples/browser/snippets/card-api-content.md"),
            page::Block::Md("examples/browser/snippets/card-api-footer.md"),
        ],
    },
    page::Block::Group {
        title: "Changelog",
        children: &[page::Block::Md(
            "examples/browser/snippets/card-spacing-variable.md",
        )],
    },
];

fn login_card<'a, M: Clone + 'a>(spacing: f32, forgot: M) -> Slot<'a, M> {
    let password_label = row![
        text::label("Password"),
        space::horizontal(),
        button(
            text::small("Forgot your password?")
                .weight(icedxdy::Weight::Normal)
                .line_height(iced::Pixels(20.0))
        )
        .style(button::Link)
        .anchor()
        .translate(false)
        .height(20)
        .padding(0)
        .on_press(forgot)
    ]
    .align_y(Center)
    .width(Fill);
    card(
        column![
            card::header("Login to your account")
                .description("Enter your email below to login to your account")
                .action(button("Sign Up").style(button::Link))
                .inset(spacing),
            card::content(
                column![
                    field(input("m@example.com").width(Fill)).label("Email"),
                    field(input("").secure(true).width(Fill)).label(Slot::from(password_label))
                ]
                .spacing(24)
                .width(Fill)
            )
            .padding([0.0, spacing]),
            card::footer_with_spacing(
                column![
                    button("Login").width(Fill),
                    button("Login with Google")
                        .style(button::Outline)
                        .width(Fill)
                ]
                .spacing(8)
                .width(Fill),
                spacing
            )
        ]
        .width(Fill)
        .spacing(spacing)
        .padding(iced::Padding::ZERO.top(spacing)),
    )
    .width(Fill.max(384))
    .boxed()
    .into()
}
#[derive(Clone)]
enum Event {
    Spacing(Vec<String>),
    Forgot,
}
struct Spacing {
    page: usize,
}
impl<'a> Component<'a, Message, Theme> for Spacing {
    type State = Option<u16>;
    type Event = Event;
    fn update(&self, state: &mut Self::State, event: Event, _: &iced::Renderer) -> Option<Message> {
        match event {
            Event::Spacing(value) => {
                if let Some(n) = value.first().and_then(|v| v.parse::<u16>().ok()) {
                    *state = Some(n);
                }
                None
            }
            Event::Forgot => Some(Message::Page(
                self.page,
                page::Message::LinkClicked("#".into()),
            )),
        }
    }
    fn view(&self, state: &Self::State) -> impl Widget<Event, Theme> + 'a {
        let spacing = state.unwrap_or(16);
        Element::from(
            column![
                toggle_group(|| [16, 20, 24, 32]
                    .map(|n| toggle_group::item(n.to_string(), text::small(format!("{n}px")))))
                .style(toggle_group::Outline)
                .size(Size::Sm)
                .value([spacing.to_string()])
                .on_value_change(Event::Spacing),
                login_card(f32::from(spacing), Event::Forgot),
            ]
            .spacing(16)
            .width(Fill.max(384)),
        )
    }
}
fn terms<'a>() -> Slot<'a, Message> {
    const TERMS: [&str; 4] = [
        "These terms govern your use of the workspace, including access to shared documents, project files, and collaboration tools.",
        "You are responsible for the content you upload and for ensuring that your team has the appropriate permissions to view or edit it.",
        "We may update features or limits as the service evolves. When those changes materially affect your workflow, we will notify your workspace administrators.",
        "By continuing, you agree to keep your account credentials secure and to follow your organization's acceptable use policies.",
    ];
    let body = column(TERMS.map(|p| text::small(p).line_height(1.625).into()))
        .spacing(16)
        .padding(16)
        .width(Fill);
    let terms = container(scroll_area(body).height(192))
        .width(Fill)
        .style(|t| Style {
            background: Some(t.muted.base.alpha(0.5).into()),
            ..Style::default()
        });
    card(
        column![
            container(
                card::header("Terms of Service")
                    .description("Review the terms before accepting the agreement.")
            )
            .padding(iced::Padding::ZERO.bottom(16)),
            separator(),
            terms,
            card::footer(
                row![
                    space::horizontal(),
                    button("Decline").style(button::Outline),
                    button("Accept")
                ]
                .spacing(8)
                .width(Fill)
            )
        ]
        .width(Fill)
        .padding(iced::Padding::ZERO.top(16)),
    )
    .width(Fill.max(384))
    .boxed()
    .into()
}

fn cover_image<'a>(theme: &Theme) -> Element<'a, Message, Theme> {
    static LIGHT: LazyLock<image::Handle> = LazyLock::new(|| {
        image::Handle::from_bytes(include_bytes!("../../assets/card-cover-light.png").as_slice())
    });
    static DARK: LazyLock<image::Handle> = LazyLock::new(|| {
        image::Handle::from_bytes(include_bytes!("../../assets/card-cover-dark.png").as_slice())
    });
    image(if theme.neutral.is_dark {
        DARK.clone()
    } else {
        LIGHT.clone()
    })
    .width(Fill)
    .height(Fill)
    .content_fit(ContentFit::Cover)
    .border_radius(border::Radius {
        top_left: radius::XL,
        top_right: radius::XL,
        ..Default::default()
    })
    .boxed()
}

pub struct CardPage {
    state: page::State,
}

impl CardPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

impl page::Page for CardPage {
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

    fn backdrop(&self) -> page::Backdrop {
        page::Backdrop {
            padding: 40.into(),
            ..Default::default()
        }
    }

    fn docs_preview(&self, id: &str, outer: &Theme) -> Element<'_, Message, Theme> {
        let height = match id {
            "demo" => 480.,
            "spacing" => 544.,
            "image" => 512.,
            _ => 384.,
        };
        container(self.themed_preview(id, outer))
            .center_x(Fill)
            .center_y(height - 80.)
            .into()
    }

    fn preview(&self, id: &str) -> Element<'_, Message, Theme> {
        self.themed_preview(id, &Theme::default())
    }

    fn themed_preview(&self, id: &str, theme: &Theme) -> Element<'_, Message, Theme> {
        let pi = self.state.page_index;
        match id {
            "demo" => login_card(16.0,Message::Page(pi,page::Message::LinkClicked("#".into()))).compose(),

            "small" => card(column![
                card::header(card::title("Small Card").size(14.0)).inset(12.0)
                    .description("This card uses the small size variant."),
                card::content(text::small(
                    "The card component supports a size prop that can be set to \"sm\" for a more compact appearance."
                ).line_height(iced::Pixels(20.0))).padding([0,12]),
                card::footer_with_spacing(
                    button("Action").style(button::Outline)
                        .size(Size::Sm)
                        .on_press(Message::Page(pi, page::Message::Noop))
                        .width(Fill),12.0
                ),
            ]
            .spacing(12)
            .padding(iced::Padding::ZERO.top(12))
            .width(Fill))
            .width(Fill.max(384))
            .boxed(),

            "image" => {
                let header: Slot<'_, Message> = card::header("Design systems meetup")
                    .description(
                        "A practical talk on component APIs, accessibility, and shipping faster.",
                    )
                    .action(card::action(badge("Featured").style(badge::Secondary)))
                    .into();

                card(column![
                    aspect_ratio(cover_image(theme), 16.0 / 9.0),
                    header,
                    card::footer(
                        button("View Event")
                            .on_press(Message::Page(pi, page::Message::Noop))
                            .width(Fill)
                    ),
                ]
                .spacing(Gap::G4)

                .width(Fill))
                .width(Fill.max(384))
                .clip(true)
                .boxed()
            }

            "spacing" => component(Spacing{page:pi}).boxed().into(),

            "edge-to-edge" => terms().into(),

            _ => unreachable!("unknown card preview id: {id}"),
        }
    }
}
