use crate::icon;
use crate::{Message, page};
use iced::alignment::Horizontal;
use iced::widget::component::{Component, component};
use iced::{Element, Task};
use iced::{Fill, Widget};
use icedcn::Theme;
use icedcn::widget::input_otp::Pattern;
use icedcn::widget::{button, column, container, field, input_otp, row, space, text};
use icedcn::{Size, View, card};

pub static META: page::Meta = page::Meta {
    title: "Input OTP",
    description: "Accessible one-time password component with copy-paste functionality.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/input-otp-demo.md",
    },
    page::Block::Md("examples/browser/snippets/input-otp-composition.md"),
    page::Block::Md("examples/browser/snippets/input-otp-guide-about.md"),
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "pattern",
                title: "Pattern",
                description: None,
                snippet: "examples/browser/snippets/input-otp-pattern.md",
            },
            page::Block::Example {
                id: "separator",
                title: "Separator",
                description: None,
                snippet: "examples/browser/snippets/input-otp-separator.md",
            },
            page::Block::Example {
                id: "disabled",
                title: "Disabled",
                description: None,
                snippet: "examples/browser/snippets/input-otp-disabled.md",
            },
            page::Block::Example {
                id: "controlled",
                title: "Controlled",
                description: None,
                snippet: "examples/browser/snippets/input-otp-controlled.md",
            },
            page::Block::Example {
                id: "invalid",
                title: "Invalid",
                description: None,
                snippet: "examples/browser/snippets/input-otp-invalid.md",
            },
            page::Block::Example {
                id: "four-digits",
                title: "Four Digits",
                description: None,
                snippet: "examples/browser/snippets/input-otp-four-digits.md",
            },
            page::Block::Example {
                id: "alphanumeric",
                title: "Alphanumeric",
                description: None,
                snippet: "examples/browser/snippets/input-otp-alphanumeric.md",
            },
            page::Block::Example {
                id: "form",
                title: "Form",
                description: None,
                snippet: "examples/browser/snippets/input-otp-form.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/input-otp-api-reference.md"),
];
pub struct InputOtpPage {
    state: page::State,
}

impl InputOtpPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

impl page::Page for InputOtpPage {
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
            "demo" => input_otp(6).default_value("123456").into(),
            "pattern" => field(input_otp(6).pattern(Pattern::Digits))
                .label("Digits Only")
                .into(),
            "separator" => input_otp(6).group_size(2).into(),
            "disabled" => input_otp(6)
                .group_size(3)
                .value("123456")
                .disabled(true)
                .into(),
            "controlled" => component(Verification { form: false }).boxed().into(),
            "invalid" => input_otp(6)
                .group_size(2)
                .default_value("000000")
                .invalid(true)
                .into(),
            "four-digits" => input_otp(4).pattern(Pattern::Digits).into(),
            "alphanumeric" => input_otp(6)
                .group_size(3)
                .pattern(Pattern::Alphanumeric)
                .into(),
            "form" => component(Verification { form: true }).boxed().into(),
            _ => unreachable!("unknown preview: {id}"),
        }
    }
}

struct Verification {
    form: bool,
}

#[derive(Default)]
struct VerificationState {
    code: String,
    attempted: bool,
}

#[derive(Debug, Clone)]
enum Edit {
    Code(String),
    Resend,
    Verify,
}

impl<'a> Component<'a, Message, Theme> for Verification {
    type State = VerificationState;
    type Event = Edit;

    fn update(
        &self,
        state: &mut VerificationState,
        edit: Edit,
        _: &iced::Renderer,
    ) -> Option<Message> {
        match edit {
            Edit::Code(code) => {
                state.code = code;
                state.attempted = false;
            }
            Edit::Resend => *state = VerificationState::default(),
            Edit::Verify => state.attempted = true,
        }
        None
    }

    fn view(&self, state: &VerificationState) -> impl Widget<Edit, Theme> + 'a {
        let entry = input_otp(6)
            .value(state.code.clone())
            .on_value_change(Edit::Code);
        if !self.form {
            let caption = if state.code.is_empty() {
                "Enter your one-time password.".to_owned()
            } else {
                format!("You entered: {}", state.code)
            };
            return Element::from(
                column![entry, text::small(caption)]
                    .spacing(8)
                    .align_x(Horizontal::Center),
            );
        }
        let invalid = state.attempted && state.code.len() != 6;
        let mut code = field(entry.group_size(3).cell_size(44., 48.).invalid(invalid))
            .label(View::from(row![
                text::label("Verification code"),
                space::horizontal(),
                button(row![icon::refresh_ccw(), "Resend Code"].spacing(4))
                    .size(Size::Xs)
                    .style(button::Outline)
                    .on_press(Edit::Resend)
            ]))
            .description("I no longer have access to this email address.");
        if invalid {
            code = code.error("Enter the six-character code.");
        }
        let action = if state.attempted && !invalid {
            "Verified"
        } else {
            "Verify"
        };
        Element::from(
            container(
                card(column![
                    card::header("Verify your login").description(
                        "Enter the verification code we sent to your email address: m@example.com."
                    ),
                    card::content(code),
                    card::footer(
                        column![
                            button(action).on_press(Edit::Verify).width(Fill),
                            text::muted("Having trouble signing in? Contact support")
                        ]
                        .spacing(8)
                    )
                ])
                .gap(0)
                .padding(0),
            )
            .width(360),
        )
    }
}
