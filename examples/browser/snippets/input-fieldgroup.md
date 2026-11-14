```rust
use iced::Fill;
use icedcn::Slot;
use icedcn::widget::{button, field, input, row};

#[derive(Clone, Debug)]
enum Edit { Name(String), Email(String), Reset, Submit }

#[derive(Default)]
struct Contact { name: String, email: String }

impl Contact {
    fn update(&mut self, edit: Edit) {
        match edit {
            Edit::Name(value) => self.name = value,
            Edit::Email(value) => self.email = value,
            Edit::Reset => *self = Self::default(),
            Edit::Submit => {}
        }
    }

    fn view(&self) -> Slot<'static, Edit> {
        field::group([
            field(input("Jordan Lee").width(Fill).value(self.name.clone()).on_input(Edit::Name))
                .label("Name").into(),
            field(input("name@example.com").width(Fill).value(self.email.clone()).on_input(Edit::Email))
                .label("Email").description("We'll send updates to this address.").into(),
            row![button("Reset").style(button::Outline).on_press(Edit::Reset), button("Submit").on_press(Edit::Submit)]
                .spacing(8).into(),
        ]).width(320).into()
    }
}
```
