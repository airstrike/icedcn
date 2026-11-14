mod demo;
mod samples;
mod sections;
use crate::{Message, page};
use iced::time::Instant;
use iced::widget::Id;
use iced::{Element, Fill, Task, clipboard};
use icedcn::{Theme, widget::container};
use sections::BLOCKS;
pub static META: page::Meta = page::Meta {
    title: "Code Block",
    description: "Displays code, terminal output and diffs with an optional header, line numbers and a copy button.",
};

#[derive(Clone, Debug)]
pub enum Event {
    Copy {
        target: Id,
        id: &'static str,
        value: String,
    },
    Save {
        target: Id,
        name: &'static str,
        value: &'static str,
    },
    Complete(demo::Completion),
    Acknowledged(Vec<Id>),
}
pub struct CodeBlockPage {
    state: page::State,
    completions: Vec<demo::Completion>,
}
impl CodeBlockPage {
    pub fn new(index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(index, BLOCKS),
            completions: Vec::new(),
        })
    }
    fn effect(&mut self, event: Event) -> Task<Message> {
        let index = self.state.page_index;
        let send = move |target, message| {
            Message::Page(
                index,
                page::Message::CodeBlock(Event::Complete(demo::Completion {
                    target,
                    serial: Id::unique(),
                    message,
                })),
            )
        };
        match event {
            Event::Copy { target, id, value } => clipboard::write(value).then(move |result| {
                if result.is_ok() {
                    Task::done(send(
                        target.clone(),
                        demo::Message::Copied(id, Instant::now()),
                    ))
                } else {
                    Task::none()
                }
            }),
            Event::Save {
                target,
                name,
                value,
            } => Task::perform(
                async move {
                    let file = rfd::AsyncFileDialog::new()
                        .set_file_name(name)
                        .save_file()
                        .await;
                    match file {
                        Some(file) => file
                            .write(value.as_bytes())
                            .await
                            .map(|()| true)
                            .map_err(|e| e.to_string()),
                        None => Ok(false),
                    }
                },
                move |result| send(target.clone(), demo::Message::Saved(name, result)),
            ),
            Event::Acknowledged(ids) => {
                self.completions
                    .retain(|completion| !ids.contains(&completion.serial));
                Task::none()
            }
            Event::Complete(completion) => {
                self.completions.push(completion);
                Task::none()
            }
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
        self.completions.clear();
    }
    fn backdrop(&self) -> page::Backdrop {
        page::Backdrop {
            min_height: 0.,
            padding: 40.into(),
            ..Default::default()
        }
    }
    fn docs_preview(&self, id: &str, theme: &Theme) -> Element<'_, Message, Theme> {
        container(demo::show(
            id,
            theme.neutral.is_dark,
            self.state.page_index,
            self.completions.clone(),
        ))
        .width(Fill)
        .max_width(if id == "demo" { 512 } else { 448 })
        .into()
    }
    fn preview(&self, id: &str) -> Element<'_, Message, Theme> {
        container(demo::show(
            id,
            false,
            self.state.page_index,
            self.completions.clone(),
        ))
        .width(Fill)
        .max_width(if id == "demo" { 512 } else { 448 })
        .into()
    }
}
