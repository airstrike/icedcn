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
pub struct Demo {
    wrap: bool,
    status: Option<String>,
}
impl Default for Demo {
    fn default() -> Self {
        Self {
            wrap: true,
            status: None,
        }
    }
}
impl Demo {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Wrap => self.wrap = !self.wrap,
            Message::Save => {
                return Task::perform(
                    async {
                        let Some(file) = rfd::AsyncFileDialog::new()
                            .set_file_name("clippy.log")
                            .save_file()
                            .await
                        else {
                            return Ok(false);
                        };
                        file.write(ACTIONS.as_bytes())
                            .await
                            .map(|()| true)
                            .map_err(|error| error.to_string())
                    },
                    Message::Saved,
                );
            }
            Message::Saved(Ok(true)) => self.status = Some("Saved clippy.log".into()),
            Message::Saved(Ok(false)) => {}
            Message::Saved(Err(error)) => {
                self.status = Some(format!("Could not save clippy.log: {error}"))
            }
        }
        Task::none()
    }
    pub fn view(&self) -> CodeBlock<'_, Message> {
        let mut block = code_block(ACTIONS)
            .font("Geist Mono")
            .wrap(self.wrap)
            .title(code_block::terminal_title("Terminal"))
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
const ACTIONS: &str = r##"$ cargo clippy -- -D warnings
    Checking notebook v0.1.0
error: writing `&String` instead of `&str` involves a new object where a slice will do
 --> src/search.rs:4:18
  |
4 | fn matches(query: &String, title: &str) -> bool {
  |                   ^^^^^^^ help: change this to: `&str`"##;
```
