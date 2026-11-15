## Usage

Place the application's content inside a toaster. A button can raise a toast directly.

```rust
use icedcn::{Slot, widget::{button, toast, toaster}};

fn view() -> Slot<'static, ()> {
    let save = toast::trigger(button("Save"), toast::toast("Saved"));
    toaster(save).position(toast::Position::TopCenter).into()
}
```

From an application update, return a task to show the result of an operation.

```rust
use iced::Task;
use icedcn::widget::toast;

fn saved<Message: Send + 'static>() -> Task<Message> {
    toast::show(toast::toast::<Message>("Saved").kind(toast::Kind::Success))
}
```
