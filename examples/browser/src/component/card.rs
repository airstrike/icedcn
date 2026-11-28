use iced::Alignment::{Center, End, Start};
use iced::Task;
use iced::Widget;
use iced::border;
use iced::widget::component::{Component, component};
use iced::widget::{image, responsive};
use iced::{ContentFit, Element, Fill, Shrink};
use icedcn::theme::container::Style;
use std::sync::LazyLock;

use icedcn::card;
use icedcn::widget::{
    aspect_ratio, badge, button, column, container, field, input, row, scroll_area, separator,
    space, text, toggle_group,
};
use icedcn::{Size, Slot, Theme, radius};

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

fn login_card<'a, M: Clone + 'a>(input_size: f32, forgot: M) -> card::Card<'a, M> {
    let password_label = responsive(move |area| {
        // The 14px password label and recovery link occupy 211px on one line.
        let wraps = area.width < 211.;
        let link = button(text::small("Forgot your password?").weight(icedcn::Weight::Normal))
            .style(button::Link)
            .anchor()
            .translate(false)
            .height(if wraps { 40 } else { 20 })
            .padding(0)
            .on_press(forgot.clone());
        Element::from(
            row![
                text::label("Password"),
                container(link)
                    .width(Fill)
                    .align_x(if wraps { Start } else { End })
            ]
            .align_y(Center)
            .width(Fill),
        )
    })
    .height(Shrink)
    .boxed();
    card(
        column![
            card::header("Login to your account")
                .description("Enter your email below to login to your account")
                .action(button("Sign Up").style(button::Link)),
            card::content(
                column![
                    field(input("m@example.com").text_size(input_size).width(Fill))
                        .label(text::label("Email")),
                    field(input("").secure(true).text_size(input_size).width(Fill))
                        .label(Slot::from(password_label))
                ]
                .spacing(24)
                .width(Fill)
            ),
            card::footer(
                column![
                    button("Login").width(Fill),
                    button("Login with Google")
                        .style(button::Outline)
                        .width(Fill)
                ]
                .spacing(8)
                .width(Fill)
            )
        ]
        .width(Fill),
    )
    .width(Fill.max(384))
}
#[derive(Clone)]
enum Event {
    Spacing(u16),
    Forgot,
}
struct Spacing {
    page: usize,
    input_size: f32,
}
impl<'a> Component<'a, Message, Theme> for Spacing {
    type State = Option<u16>;
    type Event = Event;
    fn update(&self, state: &mut Self::State, event: Event, _: &iced::Renderer) -> Option<Message> {
        match event {
            Event::Spacing(value) => {
                *state = Some(value);
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
                .spacing(8)
                .value([spacing.to_string()])
                .on_value_change(move |selected| {
                    let value = selected
                        .first()
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(spacing);
                    Event::Spacing(value)
                }),
                login_card(self.input_size, Event::Forgot).spacing(f32::from(spacing)),
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
        .style(|t: &Theme| Style {
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
        .spacing(0),
    )
    .width(Fill.max(384))
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
    input_size: f32,
    state: page::State,
}

impl CardPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            input_size: 14.,
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

    fn window_width(&mut self, width: f32) {
        self.input_size = if width < 768. { 16. } else { 14. };
    }

    fn backdrop(&self) -> page::Backdrop {
        page::Backdrop {
            padding: [0, 40].into(),
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
            .min_height(height)
            .align_y(Center)
            .into()
    }

    fn preview(&self, id: &str) -> Element<'_, Message, Theme> {
        self.themed_preview(id, &Theme::default())
    }

    fn themed_preview(&self, id: &str, theme: &Theme) -> Element<'_, Message, Theme> {
        let pi = self.state.page_index;
        match id {
            "demo" => login_card(
                self.input_size,
                Message::Page(pi, page::Message::LinkClicked("#".into())),
            )
            .into(),

            "small" => card(
                column![
                    card::header("Small Card")
                        .description("This card uses the small size variant."),
                    card::content(
                        text::small(
                            "The card component supports a size that can be set to Size::Sm for a more compact appearance."
                        )
                        .line_height(iced::Pixels(20.0))
                    ),
                    card::footer(
                        button("Action")
                            .style(button::Outline)
                            .size(Size::Sm)
                                                        .width(Fill)
                    ),
                ]
                .width(Fill),
            )
            .size(Size::Sm)
            .width(Fill.max(384))
            .into(),

            "image" => {
                let header = card::header("Design systems meetup")
                    .description(
                        "A practical talk on component APIs, accessibility, and shipping faster.",
                    )
                    .action(badge("Featured").style(badge::Secondary));

                card(
                    column![
                        aspect_ratio(cover_image(theme), 16.0 / 9.0),
                        header,
                        card::footer(
                            button("View Event")
                                                                .width(Fill)
                        ),
                    ]
                    .width(Fill),
                )
                .padding(0)
                .width(Fill.max(384))
                .into()
            }

            "spacing" => component(Spacing { page: pi, input_size: self.input_size }).boxed(),

            "edge-to-edge" => terms().into(),

            _ => unreachable!("unknown card preview id: {id}"),
        }
    }
}
