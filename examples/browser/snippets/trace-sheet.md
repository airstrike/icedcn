```rust
use icedxdy::widget::{trace, text};

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
                text("apps/v4/lib/docs.ts"),
                text("apps/v4/lib/components.ts"),
                text("apps/v4/next.config.mjs"),
                text("apps/v4/package.json")
            ]
            .spacing(8)
            .padding([0, 16])
        ]
        .into()
    },
)
.into()
```
