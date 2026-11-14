```rust
use iced::Fill;
use icedcn::widget::{accordion, container};

let items = [
    (
        "notifications",
        "Notification Settings",
        "Manage how you receive notifications. You can enable email alerts \
         for updates or push notifications for mobile devices.",
    ),
    (
        "privacy",
        "Privacy & Security",
        "Control your privacy settings and security preferences. Enable \
         two-factor authentication, manage connected devices, review active \
         sessions, and configure data sharing preferences. You can also \
         download your data or delete your account.",
    ),
    (
        "billing",
        "Billing & Subscription",
        "View your current plan, payment history, and upcoming invoices. \
         Update your payment method, change your subscription tier, or cancel \
         your subscription.",
    ),
];

let items = accordion(items.into_iter().map(|(id, trigger, body)| {
    accordion::item(id, trigger).content(body)
}))
.mode(accordion::Mode::Multiple)
.default_value(["notifications"]);

container(items).width(Fill).max_width(512)
```
