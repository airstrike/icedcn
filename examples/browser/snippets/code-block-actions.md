```rust
use iced::widget::svg;
use iced::{Font, Task, Widget};
use icedxdy::widget::{CodeBlock, button, code_block, row, text, tooltip};
use icedxdy::{Size, Slot, Theme};
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
        let block = code_block(ACTIONS);
        let mut block = block
            .font(Font::new("Geist Mono"))
            .wrap(self.wrap)
            .title(code_block::terminal_title("Terminal"))
            .actions(
                row![
                    action(icon(WRAP_TEXT), "Wrap lines", Message::Wrap),
                    action(icon(DOWNLOAD), "Download", Message::Save),
                    code_block::copy_button(ACTIONS)
                ]
                .spacing(4),
            );
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
) -> Slot<'static, Message> {
    tooltip(
        button(glyph)
            .square()
            .style(button::Ghost)
            .size(Size::Xs)
            .on_press(event),
        label,
    )
    .into()
}
fn icon(path: &str) -> Slot<'static, Message> {
    let source = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">{path}</svg>"#
    );
    svg(svg::Handle::from_memory(source.into_bytes()))
        .width(14)
        .height(14)
        .style(|theme: &Theme, _| svg::Style {
            color: Some(theme.neutral.foreground.into()),
        })
        .boxed()
        .into()
}
const WRAP_TEXT: &str = r#"<path d="M3 6h18M3 12h15a3 3 0 0 1 0 6h-4m2-2-2 2 2 2M3 18h7"/>"#;
const DOWNLOAD: &str =
    r#"<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4M7 10l5 5 5-5M12 15V3"/>"#;

const ACTIONS: &str = r##"$ cargo clippy -- -D warnings
    Checking notebook v0.1.0
error: writing `&String` instead of `&str` involves a new object where a slice will do
 --> src/search.rs:4:18
  |
4 | fn matches(query: &String, title: &str) -> bool {
  |                   ^^^^^^^ help: change this to: `&str`"##;
```
