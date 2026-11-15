```rust
use iced::Fill;
use iced::widget::text::LineHeight;
use icedcn::card;
use icedcn::widget::{accordion, column};

let items = [
    (
        "What subscription plans do you offer?",
        "We offer three subscription tiers: Starter ($9/month), Professional \
     ($29/month), and Enterprise ($99/month). Each plan includes \
     increasing storage limits, API access, priority support, and team \
     collaboration features.",
    ),
    (
        "How does billing work?",
        "Billing occurs automatically at the start of each billing cycle. \
     We accept all major credit cards, PayPal, and ACH transfers for \
     enterprise customers. You'll receive an invoice via email after \
     each payment.",
    ),
    (
        "How do I cancel my subscription?",
        "You can cancel your subscription anytime from your account \
     settings. There are no cancellation fees or penalties. Your access \
     will continue until the end of your current billing period.",
    ),
];

let inner = accordion(items).default_value(0);

let description = card::description(
    "Common questions about your account, plans, payments and cancellations.",
)
.line_height(LineHeight::Absolute(20.into()));
let header = card::header("Subscription & Billing").description(description);

card(
    column![header, card::content(inner)]
        .spacing(16)
        .padding([16, 0]),
)
.gap(0)
.padding(0)
.width(Fill.max(384))
```
