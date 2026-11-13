use iced::Task;
use iced::Widget;
use iced::widget::component::{Component, component};
use iced::{Element, Fill};
use icedxdy::widget::slider;
use icedxdy::widget::{button, column, radio_group, row, select, separator, text, textarea};
use icedxdy::{Horizontal, Slot};

use icedxdy::Theme;

use icedxdy::widget::{checkbox, field, input, switch};

use crate::Message;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Field",
    description: "Combine labels, controls, and help text to compose accessible form fields and grouped inputs.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/field-demo.md",
    },
    page::Block::Md("examples/browser/snippets/field-composition.md"),
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "input",
                title: "Input",
                description: None,
                snippet: "examples/browser/snippets/field-input.md",
            },
            page::Block::Example {
                id: "textarea",
                title: "Textarea",
                description: None,
                snippet: "examples/browser/snippets/field-textarea.md",
            },
            page::Block::Example {
                id: "select",
                title: "Select",
                description: None,
                snippet: "examples/browser/snippets/field-select.md",
            },
            page::Block::Example {
                id: "slider",
                title: "Slider",
                description: None,
                snippet: "examples/browser/snippets/field-slider.md",
            },
            page::Block::Example {
                id: "fieldset",
                title: "Fieldset",
                description: None,
                snippet: "examples/browser/snippets/field-fieldset.md",
            },
            page::Block::Example {
                id: "checkbox",
                title: "Checkbox",
                description: None,
                snippet: "examples/browser/snippets/field-checkbox.md",
            },
            page::Block::Example {
                id: "radio",
                title: "Radio",
                description: None,
                snippet: "examples/browser/snippets/field-radio.md",
            },
            page::Block::Example {
                id: "switch",
                title: "Switch",
                description: None,
                snippet: "examples/browser/snippets/field-switch.md",
            },
            page::Block::Example {
                id: "choice-card",
                title: "Choice Card",
                description: None,
                snippet: "examples/browser/snippets/field-choice-card.md",
            },
            page::Block::Example {
                id: "group",
                title: "Field Group",
                description: None,
                snippet: "examples/browser/snippets/field-group.md",
            },
            page::Block::Example {
                id: "responsive",
                title: "Responsive Layout",
                description: None,
                snippet: "examples/browser/snippets/field-responsive.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/field-api-reference.md"),
];

pub struct FieldPage {
    state: page::State,
}

impl FieldPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

impl page::Page for FieldPage {
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
            "demo" => checkout(),
            "input" => field::group([
    field(input("Max Leiter")).label("Username").description("Choose a unique username for your account.").into(),
    field(input("••••••••").secure(true)).label("Password").description("Must be at least 8 characters long.").description_above().into(),
]).width(320).into(),
            "textarea" => field(textarea("Your feedback helps us improve...").min_height(88.)).label("Feedback").description("Share your thoughts about our service.").width(320).into(),
            "select" => field(select([
    ("engineering", "Engineering"), ("design", "Design"), ("marketing", "Marketing"),
    ("sales", "Sales"), ("support", "Customer Support"), ("hr", "Human Resources"),
    ("finance", "Finance"), ("operations", "Operations"),
].map(|(value, label)| select::item(value, label))).placeholder("Choose department"))
.label("Department").description("Select your department or area of work.").width(320).into(),
            "slider" => component(Budget).boxed(),
            "fieldset" => field::set(field::legend("Address Information"), Some("We need your address to deliver your order."),
    field::group([
        field(input("123 Main St")).label("Street Address").into(),
        row![field(input("New York")).label("City").width(Fill), field(input("90502")).label("Postal Code").width(Fill)].spacing(16).into(),
    ])).max_width(384.).into(),
            "switch" => field(switch()).orientation(Horizontal).control_last().label("Multi-factor authentication").width(222).into(),
            "choice-card" => field::set(text::label("Compute Environment"), Some("Select the compute environment for your cluster."),
    radio_group(|g| field::group([
        field(g.item("kubernetes")).orientation(Horizontal).control_last().label("Kubernetes").description("Run GPU workloads on a K8s cluster.").card().into(),
        field(g.item("vm")).orientation(Horizontal).control_last().label("Virtual Machine").description("Access a cluster to run GPU workloads.").card().into(),
    ]).spacing(8).into()).default_value("kubernetes")).max_width(320.).into(),
            "group" => field::group([
    field::set(text::label("Responses"), Some("Get notified when ChatGPT responds to requests that take time, like research or image generation."),
        field(checkbox().default_checked(true).disabled(true)).orientation(Horizontal).label(text::small("Push notifications"))).into(),
    separator().into(),
    field::set(text::label("Tasks"), Some("Get notified when tasks you've created have updates. Manage tasks"),
        field::group(["Push notifications", "Email notifications"].map(|label|
            field(checkbox()).orientation(Horizontal).label(text::small(label)).into()
        )).spacing(12)).into(),
]).width(320).into(),
            "responsive" => field::set(field::legend("Profile"), Some("Fill in your profile information."), field::group([
    field(input("Evil Rabbit")).responsive().label("Name").description("Provide your full name for identification").into(),
    field::actions([button("Submit").into(), button("Cancel").style(button::Outline).into()]).into(),
])).max_width(512.).into(),
            "checkbox" => field::group([
    desktop_items(false).into(), separator().into(),
    field(checkbox().default_checked(true)).orientation(Horizontal).label("Sync Desktop & Documents folders")
        .description("Your Desktop & Documents folders are being synced with iCloud Drive. You can access them from other devices.").into(),
]).width(320).into(),
            "radio" => subscription_plan().into(),
            _ => unreachable!("unknown field preview id: {id}"),
        }
    }
}

pub(super) fn desktop_items<'a>(checked: bool) -> Slot<'a, Message> {
    let choices = [
        "Hard disks",
        "External disks",
        "CDs, DVDs, and iPods",
        "Connected servers",
    ];
    let legend = if checked {
        "Show these items on the desktop:"
    } else {
        "Show these items on the desktop"
    };
    field::set(
        text::label(legend),
        Some("Select the items you want to show on the desktop."),
        field::group(choices.into_iter().enumerate().map(|(index, label)| {
            field(checkbox().default_checked(checked && index < 2))
                .orientation(Horizontal)
                .label(text::small(label))
                .into()
        }))
        .spacing(12),
    )
    .into()
}

pub(super) fn subscription_plan<'a>() -> Slot<'a, Message> {
    field::set(
        text::label("Subscription Plan"),
        Some("Yearly and lifetime plans offer significant savings."),
        radio_group(|g| {
            field::group(
                [
                    ("monthly", "Monthly ($9.99/month)"),
                    ("yearly", "Yearly ($99.99/year)"),
                    ("lifetime", "Lifetime ($299.99)"),
                ]
                .map(|(value, label)| {
                    field(g.item(value))
                        .orientation(Horizontal)
                        .label(text::small(label))
                        .into()
                }),
            )
            .spacing(8)
            .into()
        })
        .default_value("monthly"),
    )
    .max_width(320.)
    .into()
}

pub(super) fn checkout<'a>() -> Element<'a, Message, Theme> {
    let month = select((1..=12).map(|n| {
        let m = format!("{n:02}");
        select::item(m.clone(), m)
    }))
    .placeholder("MM")
    .width(Fill);
    let year = select((2024..=2029).map(|n| select::item(n.to_string(), n.to_string())))
        .placeholder("YYYY")
        .width(Fill);
    column![
        field::set(
            field::legend("Payment Method"),
            Some("All transactions are secure and encrypted"),
            field::group([
                field(input("Evil Rabbit")).label("Name on Card").into(),
                field(input("1234 5678 9012 3456"))
                    .label("Card Number")
                    .description("Enter your 16-digit card number")
                    .into(),
                row![
                    field(month).label("Month").width(Fill),
                    field(year).label("Year").width(Fill),
                    field(input("123")).label("CVV").width(Fill)
                ]
                .spacing(16)
                .into(),
            ])
        ),
        separator(),
        field::set(
            field::legend("Billing Address"),
            Some("The billing address associated with your payment method"),
            field(checkbox().default_checked(true))
                .orientation(Horizontal)
                .label(text::small("Same as shipping address"))
        ),
        field(textarea("Add any additional comments")).label("Comments"),
        row![button("Submit"), button("Cancel").style(button::Outline)].spacing(8)
    ]
    .spacing(20)
    .width(Fill)
    .max_width(448.)
    .into()
}

struct Budget;

impl<'a> Component<'a, Message, Theme> for Budget {
    type State = Option<[f32; 2]>;
    type Event = [f32; 2];

    fn update(
        &self,
        state: &mut Self::State,
        next: Self::Event,
        _: &iced::Renderer,
    ) -> Option<Message> {
        *state = Some(next);
        None
    }

    fn view(&self, state: &Self::State) -> impl Widget<Self::Event, Theme> + 'a {
        let [low, high] = state.unwrap_or([200., 800.]);
        Element::from(
            field(
                slider::range(0.0..=1000.0)
                    .step(10.)
                    .value([low, high])
                    .on_value_change(|values| values),
            )
            .label("Price Range")
            .description(format!("Set your budget range (${low:.0} - ${high:.0})."))
            .description_above()
            .width(320),
        )
    }
}
