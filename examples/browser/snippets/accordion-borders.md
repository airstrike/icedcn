```rust
use iced::Fill;
use icedcn::radius;
use icedcn::theme::container::Transparent;
use icedcn::theme::tweak::{Token, border};
use icedcn::widget::{accordion, container};

let items = [
    (
        "How does billing work?",
        "We offer monthly and annual subscription plans. Billing is charged \
 at the beginning of each cycle, and you can cancel anytime. All \
 plans include automatic backups, 24/7 support, and unlimited team \
 members.",
    ),
    (
        "Is my data secure?",
        "Yes. We use end-to-end encryption, SOC 2 Type II compliance, and \
 regular third-party security audits. All data is encrypted at rest \
 and in transit using industry-standard protocols.",
    ),
    (
        "What integrations do you support?",
        "We integrate with 500+ popular tools including Slack, Zapier, \
 Salesforce, HubSpot, and more. You can also build custom \
 integrations using our REST API and webhooks.",
    ),
];

let inner = accordion(
    items
        .into_iter()
        .map(|(trigger, body)| accordion::item(trigger, body).inset(16.0)),
)
.default_value(0);

container(inner)
    .width(Fill.max(512))
    .padding(1)
    .class(Transparent + border::Width(1.) + border::Color(Token::Border) + radius(radius::LG))
```
