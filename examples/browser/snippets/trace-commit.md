```rust
use crate::icon;
use icedcn::widget::trace;

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
                "src/login.rs",
                "Edited src/login.rs",
                "+2 −2",
                "2 additions, 2 deletions",
                LOGIN_DIFF,
                icon::file_pen,
            ),
            file_diff(
                "src/auth/redirect.rs",
                "Added src/auth/redirect.rs",
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
