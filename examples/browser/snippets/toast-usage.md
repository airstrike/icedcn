## Usage

Place the application's content inside a toaster. A button can raise a toast directly.

```rust
use icedcn::{
    Slot,
    widget::{button, toast, toaster},
};

fn view<'a>() -> Slot<'a, ()> {
    let save = toast::trigger(button("Save"), toast::toast("Saved"));
    toaster(save).position(toast::Position::TopCenter).into()
}
```

When a notification is visible, press **Alt+T** to focus its list (**Option+T** on macOS). Press **Tab** to focus the notification and **Tab** again to reach its action. **Enter** activates the action. Shift+Tab moves back through the list. Activating the action or tabbing out of the list returns focus to the previous control.

From an application update, return a task to show the result of an operation.

```rust
use iced::Task;
use icedcn::widget::toast;

fn saved<Message: Send + 'static>() -> Task<Message> {
    toast::show(toast::toast::<Message>("Saved").kind(toast::Kind::Success))
}
```
