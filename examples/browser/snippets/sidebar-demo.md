```rust
use icedxdy::widget::{column, container, row, scrollable, sidebar};

sidebar::provider(move |s| {
    let compact = !s.is_open() && !s.is_mobile();
    let nav = sidebar::content([
        platform(compact, state.selected),
        if compact {
            Space::new().into()
        } else {
            projects(state.selected)
        },
    ])
    .height(iced::Shrink);
    let side = sidebar::fixed(
        column![
            sidebar::header([teams(state.team, compact)]),
            scrollable(nav).width(Fill).height(Fill),
            sidebar::footer([account(compact)])
        ]
        .width(Fill)
        .height(Fill),
    );
    row![
        s.sidebar(side),
        if s.is_mobile() {
            Space::new().into()
        } else {
            sidebar::rail_trigger()
        },
        sidebar::main(container(sidebar::trigger()).padding([8, 16]))
    ]
    .width(Fill)
    .height(Fill)
    .into()
})
.collapsible(sidebar::Collapsible::Icon)
```
