```rust
use iced::Fill;
use icedxdy::core::Border;
use icedxdy::theme::container::Style;
use icedxdy::widget::{accordion, container};
use icedxdy::{Theme, radius};

let items = [
    (
        "billing",
        "How does billing work?",
        "We offer monthly and annual subscription plans. Billing is charged \
         at the beginning of each cycle, and you can cancel anytime. All \
         plans include automatic backups, 24/7 support, and unlimited team \
         members.",
    ),
    (
        "security",
        "Is my data secure?",
        "Yes. We use end-to-end encryption, SOC 2 Type II compliance, and \
         regular third-party security audits. All data is encrypted at rest \
         and in transit using industry-standard protocols.",
    ),
    (
        "integration",
        "What integrations do you support?",
        "We integrate with 500+ popular tools including Slack, Zapier, \
         Salesforce, HubSpot, and more. You can also build custom \
         integrations using our REST API and webhooks.",
    ),
];

let inner = accordion(items.into_iter().map(|(id, trigger, body)| {
    accordion::item(id, trigger).content(body).inset(16.0)
}))
.mode(accordion::Mode::Single { collapsible: true })
.default_value(["billing"]);

container(inner)
    .max_width(512)
    .width(Fill)
    .padding(1)
    .style(|theme: &Theme| Style {
        border: Border {
            width: 1.0,
            color: theme.neutral.border,
            radius: radius::LG.into(),
        },
        ..Style::default()
    })
```
