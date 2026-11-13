```rust
use icedxdy::widget::{trace, text};

dialog(
    |d| {
        d.trigger(trace::action(
            line("Searched the web for cache keys", icon::globe)
                .trailing(icon::chevron_right().size(16)),
            None,
        ))
    },
    |_| {
        column![
            dialog::header(
                "Searched the web for cache keys",
                "Three results were used in the response."
            ),
            trace([
                source(
                    "Caching build outputs",
                    "https://nextjs.org/docs/app/guides/caching"
                ),
                source(
                    "Image optimization reference",
                    "https://nextjs.org/docs/app/api-reference/components/image"
                ),
                source(
                    "Deploy hooks and cache keys",
                    "https://vercel.com/docs/deploy-hooks"
                )
            ])
        ]
        .spacing(16)
        .into()
    },
)
.into()
```
