```rust
use iced::Fill;
use icedcn::widget::{accordion, container};

let items = accordion![
    (
        "What are your shipping options?",
        "We offer standard (5-7 days), express (2-3 days), and overnight \
         shipping. Free shipping on international orders."
    ),
    (
        "What is your return policy?",
        "Returns accepted within 30 days. Items must be unused and in \
         original packaging. Refunds processed within 5-7 business days."
    ),
    (
        "How can I contact customer support?",
        "Reach us via email, live chat, or phone. We respond within 24 \
         hours during business days."
    ),
]
.default_value(0);

container(items).width(Fill).max_width(512)
```
