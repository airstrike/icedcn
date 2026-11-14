```rust
use iced::widget::svg;
use iced::{Fill, Font, Subscription, Widget, time};
use icedxdy::widget::{CodeBlock, animation::cubic_bezier, button, code_block, container, row};
use icedxdy::{Size, Slot, Theme};
use std::f32::consts::PI;
use std::time::{Duration, Instant};
#[derive(Clone, Debug)]
pub enum Message {
    Toggle,
    Tick(Instant),
}
#[derive(Default)]
pub struct Demo {
    expanded: bool,
    turn: Turn,
}
impl Demo {
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::Toggle => {
                self.expanded = !self.expanded;
                self.turn
                    .retarget(if self.expanded { 1. } else { 0. }, Instant::now());
            }
            Message::Tick(now) => self.turn.advance(now),
        }
    }
    pub fn subscription(&self) -> Subscription<Message> {
        if self.turn.start.is_some() {
            time::every(Duration::from_millis(16)).map(Message::Tick)
        } else {
            Subscription::none()
        }
    }
    pub fn view(&self) -> CodeBlock<'static, Message> {
        let toggle = button(
            row![
                chevron(self.turn.value),
                if self.expanded {
                    "Show less"
                } else {
                    "Show more"
                }
            ]
            .spacing(4),
        )
        .style(button::Ghost)
        .size(Size::Xs)
        .on_press(Message::Toggle);
        let mut block = code_block(SHOW_MORE)
            .language("rust")
            .font(Font::new("Geist Mono"))
            .title(code_block::title("src/notebook.rs"))
            .start_line(1)
            .actions(code_block::copy_button(SHOW_MORE))
            .footer(container(toggle).center_x(Fill));
        if !self.expanded {
            block = block.max_height(256.).collapsed(true);
        } else {
            block = block.max_height(384.);
        }
        block
    }
}
#[derive(Default)]
struct Turn {
    value: f32,
    from: f32,
    to: f32,
    start: Option<Instant>,
}
impl Turn {
    fn retarget(&mut self, to: f32, now: Instant) {
        self.advance(now);
        self.from = self.value;
        self.to = to;
        self.start = Some(now);
    }
    fn advance(&mut self, now: Instant) {
        if let Some(start) = self.start {
            let p = (now.saturating_duration_since(start).as_secs_f32() / 0.15).min(1.);
            self.value = self.from + (self.to - self.from) * cubic_bezier(0.4, 0., 0.2, 1., p);
            if p == 1. {
                self.start = None;
                self.value = self.to;
            }
        }
    }
}

fn chevron(turn: f32) -> Slot<'static, Message> {
    svg(svg::Handle::from_memory(br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m6 9 6 6 6-6"/></svg>"#))
        .width(14).height(14).rotation(turn * PI)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) }).boxed().into()
}

const SHOW_MORE: &str = r##"use icedxdy::widget::{button, column, input, text};
use icedxdy::Slot;

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
