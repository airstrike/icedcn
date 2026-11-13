```rust
use icedxdy::widget::{trace, text};

trace::disclosure(
    || {
        line(
            "Committed a1b2c3: Fix the login redirect",
            icon::git_commit_horizontal,
        )
    },
    || {
        trace([
            file_diff(
                "app/login/page.tsx",
                "Edited app/login/page.tsx",
                "+2 −2",
                "2 additions, 2 deletions",
                LOGIN_DIFF,
                icon::file_pen,
            ),
            file_diff(
                "lib/auth/redirect.ts",
                "Added lib/auth/redirect.ts",
                "+7 −0",
                "7 additions, 0 deletions",
                AUTH_DIFF,
                icon::file_plus,
            ),
        ])
        .into()
    },
)
.default_open(true)
.into()
```
