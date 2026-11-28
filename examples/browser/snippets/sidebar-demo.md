```rust
use iced::widget::Space;
use iced::{Fill, Shrink};
use icedcn::View;
use icedcn::widget::{column, container, row, scrollable, sidebar};

pub fn workspace<'a, M: Clone + 'static>(
    teams: impl Fn(&sidebar::Handle<'_, 'a, M>) -> View<'a, M> + 'a,
    links: impl Fn(&sidebar::Handle<'_, 'a, M>) -> View<'a, M> + 'a,
    account: impl Fn(&sidebar::Handle<'_, 'a, M>) -> View<'a, M> + 'a,
) -> View<'a, M> {
    sidebar::provider(move |s| {
        let panel = sidebar::fixed(
            column![
                sidebar::header([teams(s)]),
                scrollable(sidebar::content([links(s)]).height(Shrink))
                    .height(Fill)
                    .width(Fill),
                sidebar::footer([account(s)]),
            ]
            .height(Fill)
            .width(Fill),
        );
        let rail = if s.is_mobile() {
            Space::new().into()
        } else {
            sidebar::rail_trigger()
        };
        row![
            s.sidebar(panel),
            rail,
            sidebar::main(container(sidebar::trigger()).center_y(64)),
        ]
        .width(Fill)
        .height(Fill)
        .into()
    })
    .collapsible(sidebar::Collapsible::Icon)
    .into()
}
```

This layout takes the workspace switcher, navigation and account controls from the application. Each closure receives the provider handle, so it can show icons when collapsed and place dropdowns below their triggers on mobile.
