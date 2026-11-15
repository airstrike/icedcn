## Controlled Sidebar

Store the desktop preference in your application and use the requested value when updating it. The phone sheet keeps a separate temporary open state.

```rust
use icedcn::Slot;
use icedcn::widget::{row, sidebar};

pub fn view<'a>(open: bool) -> Slot<'a, bool> {
    sidebar::provider(|s| {
        row![
            s.sidebar(sidebar::fixed("Workspace")),
            sidebar::main(sidebar::trigger()),
        ].width(iced::Fill).height(iced::Fill).into()
    }).open(open).on_open_change(|next| next).into()
}
```
