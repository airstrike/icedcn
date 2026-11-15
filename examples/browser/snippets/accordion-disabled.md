```rust
use icedcn::widget::accordion;

accordion![
    accordion::item(
        "Can I access my account history?",
        "Yes, you can view your complete account history including all \
         transactions, plan changes, and support tickets in the Account \
         History section of your dashboard."
    ),
    accordion::item(
        "Premium feature information",
        "This section contains information about premium features. \
         Upgrade your plan to access this content."
    )
    .disabled(true),
    accordion::item(
        "How do I update my email address?",
        "You can update your email address in your account settings. \
         You'll receive a verification email at your new address to \
         confirm the change."
    ),
]
```
