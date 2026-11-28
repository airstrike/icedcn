```rust
use iced::Task;
use icedcn::View;
use icedcn::widget::{field, input, input_group, text};
use rfd::AsyncFileDialog;

#[derive(Clone, Debug)]
enum Message {
    Choose,
    Selected(Option<String>),
}

#[derive(Default)]
struct Picture {
    name: Option<String>,
}

impl Picture {
    fn view(&self) -> View<'_, Message> {
        field(
            input_group(input("").value(self.name.as_deref().unwrap_or("No file chosen"))).addon(
                input_group::addon(
                    input_group::button(text::small("Choose File").medium())
                        .padding([0, 3])
                        .on_press(Message::Choose),
                ),
            ),
        )
        .label("Picture")
        .description("Select a picture to upload.")
        .width(320)
        .into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Choose => Task::perform(
                async {
                    AsyncFileDialog::new()
                        .pick_file()
                        .await
                        .map(|file| file.file_name())
                },
                Message::Selected,
            ),
            Message::Selected(name) => {
                if name.is_some() {
                    self.name = name;
                }
                Task::none()
            }
        }
    }
}
```
