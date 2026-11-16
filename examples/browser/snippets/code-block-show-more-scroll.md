```rust
use icedcn::widget::code_block;

code_block::show_more(256, || {
    code_block![
        code_block::header![
            "src/notebook.rs",
            code_block::actions![code_block::copy_button(CODE)],
        ],
        code_block::content(CODE)
            .language("rust")
            .line_numbers(true)
            .max_height(384),
    ]
})

const CODE: &str = r##"use icedcn::widget::{button, column, input, text};
use icedcn::Slot;

#[derive(Default)]
pub struct Notebook {
    draft: String,
    notes: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Edited(String),
    Add,
}

impl Notebook {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Edited(draft) => self.draft = draft,
            Message::Add if !self.draft.trim().is_empty() => {
                self.notes.push(std::mem::take(&mut self.draft));
            }
            Message::Add => {}
        }
    }

    pub fn view(&self) -> Slot<'_, Message> {
        let draft = input("Write a note")
            .value(self.draft.clone())
            .on_input(Message::Edited);
        let add = button("Add").on_press(Message::Add);
        let mut notes = column![draft, add].spacing(8);
        for note in &self.notes {
            notes = notes.push(text::p(note.as_str()).into());
        }
        notes.into()
    }
}"##;
```
