```rust
use iced::Fill;
use icedcn::widget::{accordion, container};

let items = [
    (
        "How do I reset my password?",
        "Click on 'Forgot Password' on the login page, enter your email \
     address, and we'll send you a link to reset your password. The link \
     will expire in 24 hours.",
    ),
    (
        "Can I change my subscription plan?",
        "Yes, you can upgrade or downgrade your plan at any time from your \
     account settings. Changes will be reflected in your next billing cycle.",
    ),
    (
        "What payment methods do you accept?",
        "We accept all major credit cards, PayPal, and bank transfers. All \
     payments are processed securely through our payment partners.",
    ),
];

let items = accordion(items).default_value(0);

container(items).width(Fill).max_width(512)
```
