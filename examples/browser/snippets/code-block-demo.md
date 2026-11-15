```rust
use crate::icon;
use iced::Task;
use icedcn::theme::tweak::{Fg, Token, hover};
use icedcn::widget::{CodeBlock, button, code_block, row, text, tooltip};
use icedcn::{Size, Slot};
#[derive(Clone, Debug)]
pub enum Message {
    Wrap,
    Save,
    Saved(Result<bool, String>),
}
#[derive(Default)]
pub struct Demo {
    wrap: bool,
    status: Option<String>,
}
impl Demo {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Wrap => self.wrap = !self.wrap,
            Message::Save => {
                return Task::perform(
                    async {
                        let Some(file) = rfd::AsyncFileDialog::new()
                            .set_file_name("view.rs")
                            .save_file()
                            .await
                        else {
                            return Ok(false);
                        };
                        file.write(DEMO.as_bytes())
                            .await
                            .map(|()| true)
                            .map_err(|error| error.to_string())
                    },
                    Message::Saved,
                );
            }
            Message::Saved(Ok(true)) => self.status = Some("Saved view.rs".into()),
            Message::Saved(Ok(false)) => {}
            Message::Saved(Err(error)) => {
                self.status = Some(format!("Could not save view.rs: {error}"))
            }
        }
        Task::none()
    }
    pub fn view(&self) -> CodeBlock<'_, Message> {
        let mut block = code_block(DEMO).language("rust").start_line(1);
        for n in 6..=10 {
            block = block.line_style(n, code_block::LineStyle::Highlighted);
        }
        let mut block = block
            .font("Geist Mono")
            .wrap(self.wrap)
            .title(row![icon::file_code().size(14), code_block::title("src/view.rs")].spacing(6))
            .actions(
                row![
                    action(
                        icon::wrap_text().size(14),
                        "Wrap lines",
                        Message::Wrap,
                        self.wrap
                    ),
                    action(icon::download().size(14), "Download", Message::Save, false)
                ]
                .spacing(4),
            )
            .copyable(true);
        if let Some(status) = &self.status {
            block = block.footer(text::muted(status.as_str()));
        }
        block
    }
}
fn action(
    glyph: impl Into<Slot<'static, Message>>,
    label: &'static str,
    event: Message,
    active: bool,
) -> Slot<'static, Message> {
    let ink = if active {
        Token::Foreground
    } else {
        Token::MutedForeground
    };
    tooltip(
        button(glyph)
            .square()
            .style(button::Ghost + Fg(ink) + hover(Fg(Token::Foreground)))
            .size(Size::Xs)
            .on_press(event),
        label,
    )
    .into()
}
const DEMO: &str = r##"use icedcn::{Slot, widget::{column, input, text}};

#[derive(Clone, Debug)]
pub enum Message { SearchChanged(String) }

pub fn view<'a>(query: &'a str, titles: &[&'a str]) -> Slot<'a, Message> {
    let search = input("Find a notebook").value(query.to_owned()).on_input(Message::SearchChanged);
    let mut results = column![search].spacing(8);
    for title in titles.iter().copied().filter(|title| title.contains(query)) {
        results = results.push(text::p(title).into());
    }
    results.into()
}"##;
```
