use iced::widget::{rich_text, span};
use iced::{Fill, Widget};
use icedcn::theme::text::muted;
use icedcn::widget::{
    button, checkbox, column, field, input, radio_group, row, select, separator, switch, text,
    textarea,
};
use icedcn::{Horizontal, Slot};

pub(super) fn credentials<M: Clone + 'static>(text_size: f32) -> Slot<'static, M> {
    field::group([
        field(input("Max Leiter").width(Fill).text_size(text_size))
            .label("Username")
            .description("Choose a unique username for your account.")
            .into(),
        field(
            input("••••••••")
                .width(Fill)
                .text_size(text_size)
                .secure(true),
        )
        .label("Password")
        .description("Must be at least 8 characters long.")
        .description_above()
        .into(),
    ])
    .width(320)
    .into()
}

pub(super) fn feedback<M: Clone + 'static>() -> Slot<'static, M> {
    field(textarea("Your feedback helps us improve..."))
        .label("Feedback")
        .description("Share your thoughts about our service.")
        .width(320)
        .into()
}

pub(super) fn department<M: Clone + 'static>() -> Slot<'static, M> {
    let options = [
        ("engineering", "Engineering"),
        ("design", "Design"),
        ("marketing", "Marketing"),
        ("sales", "Sales"),
        ("support", "Customer Support"),
        ("hr", "Human Resources"),
        ("finance", "Finance"),
        ("operations", "Operations"),
    ];
    field(
        select(options.map(|(id, name)| select::item(id, name)))
            .placeholder("Choose department")
            .width(Fill),
    )
    .label("Department")
    .description("Select your department or area of work.")
    .width(320)
    .into()
}

pub(super) fn address<M: Clone + 'static>(text_size: f32) -> Slot<'static, M> {
    let address = |label, hint| {
        field(input(hint).width(Fill).text_size(text_size))
            .label(label)
            .width(Fill)
    };
    field::set(
        field::legend("Address Information"),
        Some("We need your address to deliver your order."),
        field::group([
            address("Street Address", "123 Main St").into(),
            row![address("City", "New York"), address("Postal Code", "90502")]
                .width(Fill)
                .spacing(16)
                .into(),
        ]),
    )
    .max_width(384.)
    .into()
}

pub(super) fn authentication<M: Clone + 'static>() -> Slot<'static, M> {
    field(switch())
        .orientation(Horizontal)
        .control_last()
        .label("Multi-factor authentication")
        .into()
}

pub(super) fn environment<M: Clone + 'static>() -> Slot<'static, M> {
    let environments = [
        (
            "kubernetes",
            "Kubernetes",
            "Run GPU workloads on a K8s cluster.",
        ),
        (
            "vm",
            "Virtual Machine",
            "Access a cluster to run GPU workloads.",
        ),
    ];
    field::set(
        field::label("Compute Environment"),
        Some("Select the compute environment for your cluster."),
        radio_group(move |g| {
            field::group(environments.map(|(id, name, detail)| {
                field(g.item(id))
                    .width(iced::Fill)
                    .orientation(Horizontal)
                    .control_last()
                    .label(name)
                    .description(detail)
                    .card()
                    .into()
            }))
            .width(320)
            .spacing(8)
            .into()
        })
        .default_value("kubernetes"),
    )
    .max_width(320.)
    .into()
}

pub(super) fn devices<M: Clone + 'static>() -> Slot<'static, M> {
    let devices = [
        "Hard disks",
        "External disks",
        "CDs, DVDs, and iPods",
        "Connected servers",
    ];
    let items = field::group(devices.map(|name| {
        field(checkbox())
            .orientation(Horizontal)
            .label(text::small(name))
            .into()
    }))
    .spacing(12);
    field::group([
    field::set(field::label("Show these items on the desktop"),
        Some("Select the items you want to show on the desktop."), items).into(),
    separator().into(),
    field(checkbox().default_checked(true)).orientation(Horizontal).label("Sync Desktop & Documents folders")
        .description("Your Desktop & Documents folders are being synced with iCloud Drive. You can access them from other devices.").into(),
]).width(320).into()
}

pub(super) fn plans<M: Clone + 'static>() -> Slot<'static, M> {
    let plans = [
        ("monthly", "Monthly ($9.99/month)"),
        ("yearly", "Yearly ($99.99/year)"),
        ("lifetime", "Lifetime ($299.99)"),
    ];
    field::set(
        field::label("Subscription Plan"),
        Some("Yearly and lifetime plans offer significant savings."),
        radio_group(move |g| {
            field::group(plans.map(|(id, name)| {
                field(g.item(id))
                    .orientation(Horizontal)
                    .label(text::small(name))
                    .into()
            }))
            .spacing(8)
            .into()
        })
        .default_value("monthly"),
    )
    .max_width(320.)
    .into()
}

pub(super) fn notifications<M: Clone + 'static>(manage: M) -> Slot<'static, M> {
    let description = rich_text([
        span("Get notified when tasks you've created have updates. "),
        span("Manage tasks").link(()).underline(true),
    ])
    .size(14)
    .line_height(1.5)
    .width(Fill)
    .style(|theme| muted(theme).into())
    .on_link_click(move |()| manage.clone())
    .boxed();
    let notifications = field::group(["Push notifications", "Email notifications"].map(|name| {
        field(checkbox())
            .orientation(Horizontal)
            .label(text::small(name))
            .into()
    }))
    .spacing(12);
    field::group([
    column![field::label("Responses"),
        text::muted("Get notified when ChatGPT responds to requests that take time, like research or image generation.").line_height(1.5),
    ].width(Fill).spacing(8).into(),
    field(checkbox().default_checked(true).disabled(true)).orientation(Horizontal).label(text::muted("Push notifications")).into(),
    separator().into(),
    column![field::label("Tasks"), description].width(Fill).spacing(8).into(),
    notifications.into(),
]).width(320).into()
}

pub(super) fn payment<M: Clone + 'static>(
    required: impl Fn(&'static str, &'static str) -> field::Field<'static, M>,
    submit: M,
) -> Slot<'static, M> {
    let date = |range: std::ops::RangeInclusive<u16>, hint| {
        select(range.map(|n| {
            let label = format!("{n:02}");
            select::item(label.clone(), label)
        }))
        .placeholder(hint)
        .width(Fill)
    };
    column![
        field::set(
            field::legend("Payment Method"),
            Some("All transactions are secure and encrypted"),
            field::group([
                required("Name on Card", "Evil Rabbit").into(),
                required("Card Number", "1234 5678 9012 3456")
                    .description("Enter your 16-digit card number")
                    .into(),
                row![
                    field(date(1..=12, "MM")).label("Month").width(Fill),
                    field(date(2024..=2029, "YYYY")).label("Year").width(Fill),
                    required("CVV", "123").width(Fill)
                ]
                .width(Fill)
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
        row![
            button("Submit").on_press(submit),
            button("Cancel").style(button::Outline)
        ]
        .spacing(8),
    ]
    .spacing(20)
    .width(Fill)
    .max_width(448.)
    .into()
}

pub(super) fn profile<M: Clone + 'static>(
    name: field::Field<'static, M>,
    submit: M,
) -> Slot<'static, M> {
    field::set(
        field::legend("Profile"),
        Some("Fill in your profile information."),
        field::group([
            name.responsive()
                .description("Provide your full name for identification")
                .into(),
            field::actions([
                button("Submit").on_press(submit).into(),
                button("Cancel").style(button::Outline).into(),
            ])
            .into(),
        ]),
    )
    .max_width(512.)
    .into()
}
