```rust
use icedcn::widget::{text, trace};

sheet(
    |s| {
        s.trigger(trace::action(
            line("Explored 4 files", icon::folder_search)
                .trailing(icon::chevron_right().size(16)),
            None,
        ))
    },
    |_| {
        column![
            sheet::header(
                "Explored 4 files",
                "Files read while preparing the response."
            ),
            column![
                text("examples/browser/src/page.rs"),
                text("examples/browser/src/component.rs"),
                text("examples/browser/src/main.rs"),
                text("Cargo.toml")
            ]
            .spacing(8)
            .padding([0, 16])
        ]
        .into()
    },
)
.into()
```
