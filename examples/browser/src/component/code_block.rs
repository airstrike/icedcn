mod demo;
mod samples;
mod sections;
use crate::{Message, page};
use iced::widget::Id;
use iced::{Element, Fill, Task};
use icedcn::{Theme, widget::container};
use sections::BLOCKS;
pub static META: page::Meta = page::Meta {
    title: "Code Block",
    description: "Displays code, terminal output and diffs with an optional header, line numbers and a copy button.",
};

#[derive(Clone, Debug)]
pub enum Event {
    Save {
        name: &'static str,
        value: &'static str,
    },
}
pub struct CodeBlockPage {
    state: page::State,
}
impl CodeBlockPage {
    pub fn new(index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(index, BLOCKS),
        })
    }
    fn effect(&mut self, event: Event) -> Task<Message> {
        match event {
            Event::Save { name, value } => Task::future(async move {
                if let Some(file) = rfd::AsyncFileDialog::new()
                    .set_file_name(name)
                    .save_file()
                    .await
                {
                    let _ = file.write(value.as_bytes()).await;
                }
            })
            .discard(),
        }
    }
}
impl page::Page for CodeBlockPage {
    fn meta(&self) -> &'static page::Meta {
        &META
    }
    fn blocks(&self) -> &'static [page::Block] {
        BLOCKS
    }
    fn page_index(&self) -> usize {
        self.state.page_index
    }
    fn state(&self) -> &page::State {
        &self.state
    }
    fn scrollable_id(&self) -> Id {
        self.state.scrollable_id.clone()
    }
    fn init(&self) -> Task<Message> {
        self.state.init()
    }
    fn update(&mut self, event: page::Message) -> Task<Message> {
        match event {
            page::Message::CodeBlock(event) => self.effect(event),
            other => self.state.update(&other),
        }
    }
    fn reset_volatile(&mut self) {
        self.state.reset_volatile();
    }
    fn backdrop(&self) -> page::Backdrop {
        page::Backdrop {
            min_height: 0.,
            padding: 40.into(),
            ..Default::default()
        }
    }
    fn docs_preview(&self, id: &str, theme: &Theme) -> Element<'_, Message, Theme> {
        container(demo::show(id, theme.neutral.is_dark, self.state.page_index))
            .width(Fill)
            .max_width(if id == "demo" { 512 } else { 448 })
            .into()
    }
    fn preview(&self, id: &str) -> Element<'_, Message, Theme> {
        container(demo::show(id, false, self.state.page_index))
            .width(Fill)
            .max_width(if id == "demo" { 512 } else { 448 })
            .into()
    }
}
