```rust
use icedcn::widget::native_select;

native_select([
    native_select::item("", "Select department"),
    native_select::group(
        "Engineering",
        [
            native_select::item("frontend", "Frontend"),
            native_select::item("backend", "Backend"),
            native_select::item("devops", "DevOps"),
        ],
    ),
    native_select::group(
        "Sales",
        [
            native_select::item("sales-rep", "Sales Rep"),
            native_select::item("account-manager", "Account Manager"),
            native_select::item("sales-director", "Sales Director"),
        ],
    ),
    native_select::group(
        "Operations",
        [
            native_select::item("support", "Customer Support"),
            native_select::item("product-manager", "Product Manager"),
            native_select::item("ops-manager", "Operations Manager"),
        ],
    ),
])
```
