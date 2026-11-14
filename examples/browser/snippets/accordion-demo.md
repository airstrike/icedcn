```rust
use iced::Fill;
use icedcn::widget::{accordion, container};

let items = accordion([
    accordion::item("shipping", "What are your shipping options?")
        .content("We offer standard (5-7 days), express (2-3 days), and overnight \
             shipping. Free shipping on international orders."),
    accordion::item("returns", "What is your return policy?")
        .content("Returns accepted within 30 days. Items must be unused and in \
             original packaging. Refunds processed within 5-7 business days."),
    accordion::item("support", "How can I contact customer support?")
        .content("Reach us via email, live chat, or phone. We respond within 24 \
             hours during business days."),
])
.mode(accordion::Mode::Single { collapsible: true })
.default_value(["shipping"]);

container(items).width(Fill).max_width(512)
```
