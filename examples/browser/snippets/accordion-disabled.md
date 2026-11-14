```rust
use icedcn::widget::accordion;

accordion([
    accordion::item("item-1", "Can I access my account history?")
        .content("Yes, you can view your complete account history including all \
             transactions, plan changes, and support tickets in the Account \
             History section of your dashboard."),
    accordion::item("item-2", "Premium feature information")
        .content("This section contains information about premium features. \
             Upgrade your plan to access this content.")
        .disabled(true),
    accordion::item("item-3", "How do I update my email address?")
        .content("You can update your email address in your account settings. \
             You'll receive a verification email at your new address to \
             confirm the change."),
])
.mode(accordion::Mode::Single { collapsible: true })
```
