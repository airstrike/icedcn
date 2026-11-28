use super::items;
use crate::{Message, page};
use iced::widget::component::{self, Component};
use iced::{Element, Fill, Widget};
use icedcn::core::Border;
use icedcn::theme::container::Style;
use icedcn::widget::questionnaire::{Answers, Invalid, Progress, Shortcuts};
use icedcn::widget::{
    Row, button, column, container, dialog, native_select, questionnaire, row, space, text,
};
use icedcn::{Theme, View, card};

pub(super) fn preview<'a>(id: &'static str, page: usize) -> Element<'a, Message, Theme> {
    component::component(Example { id, page }).boxed()
}

struct Example {
    id: &'static str,
    page: usize,
}
#[derive(Default)]
struct State {
    active: Option<String>,
    shortcuts: Option<String>,
    open: bool,
}
#[derive(Debug, Clone)]
enum Event {
    Navigate(String),
    Shortcuts(String),
    Submit(Answers),
    Reset,
    Open(bool),
}

fn validate(answers: &Answers) -> Option<Invalid> {
    if answers
        .get("audience")
        .is_some_and(|a| a.contains("public"))
        && answers.get("detail").is_some_and(|a| a.contains("summary"))
    {
        Some(Invalid {
            item: "detail".into(),
            message: "Public answers need enough context. Choose a complete answer.".into(),
        })
    } else {
        None
    }
}
fn segmented<'a>(p: Progress) -> View<'a, questionnaire::Event> {
    let segments = (0..p.total).map(|i| {
        container(space::horizontal())
            .width(Fill)
            .height(6)
            .style(move |theme: &Theme| Style {
                background: Some(
                    if i < p.current {
                        theme.primary.base
                    } else {
                        theme.muted.base
                    }
                    .into(),
                ),
                border: Border::default().rounded(3),
                ..Style::default()
            })
            .into()
    });
    column![
        Row::with_children(segments).spacing(6).width(Fill),
        text::muted(format!("Checkpoint {} of {}", p.current, p.total)).size(12)
    ]
    .spacing(8)
    .into()
}

impl Example {
    fn form<'a>(&self, state: &State) -> View<'a, Event> {
        let id = self.id;
        let mut q = questionnaire(items::items(id))
            .default_answers(items::saved(id))
            .submit_label(items::submit(id))
            .on_submit(Event::Submit);
        if matches!(id, "controlled" | "navigation-state" | "validation") {
            let first = match id {
                "controlled" => "scope",
                "navigation-state" => "permission",
                _ => "detail",
            };
            q = q
                .item(state.active.as_deref().unwrap_or(first))
                .on_item_change(Event::Navigate);
        }
        if matches!(id, "demo" | "multiple" | "freeform" | "shortcuts") {
            q = q.shortcuts(match state.shortcuts.as_deref() {
                Some("none") => Shortcuts::None,
                Some("numbers") => Shortcuts::Numbers,
                _ => Shortcuts::Letters,
            });
        }
        if matches!(id, "multiple" | "freeform" | "shortcuts") {
            q = q.hide_progress();
        }
        match id {
            "demo" | "skip" => q = q.allow_skip(true),
            "progress" => q = q.progress(segmented),
            "animated" => q = q.animated(true),
            "resume" => {
                q = q
                    .default_item("verification")
                    .reset_action("Reset changes")
                    .on_reset(|| Event::Reset)
            }
            "validation" => {
                q = q.validate(validate).progress(|p| {
                    text::muted(format!("{} / {}", p.current, p.total))
                        .size(12)
                        .into()
                })
            }
            "navigation-state" => q = q.block_unanswered(true),
            _ => {}
        }
        let result: View<'a, Event> = if matches!(id, "validation" | "card") {
            card::card(q).width(Fill).padding(20).into()
        } else {
            q.into()
        };
        let mut col = column![].width(Fill).spacing(24);
        if id == "shortcuts" {
            let selector = native_select([
                native_select::item("none", "No shortcuts"),
                native_select::item("letters", "Letters"),
                native_select::item("numbers", "Numbers"),
            ])
            .value(state.shortcuts.as_deref().unwrap_or("letters"))
            .on_value_change(Event::Shortcuts);
            col = col.push(row![space::horizontal(), selector].width(Fill).into());
        } else if id == "controlled" {
            let label = match state.active.as_deref() {
                Some("checks") => "Verification",
                Some("output") => "Final output",
                _ => "Change scope",
            };
            col = col.push(
                row![
                    space::horizontal(),
                    text::muted(format!("Current checkpoint: {label}"))
                ]
                .width(Fill)
                .into(),
            );
        }
        container(col.push(result))
            .width(Fill)
            .max_width(448.)
            .into()
    }
}
impl<'a> Component<'a, Message, Theme> for Example {
    type State = State;
    type Event = Event;
    fn update(&self, state: &mut State, event: Event, _: &iced::Renderer) -> Option<Message> {
        match event {
            Event::Navigate(name) => state.active = Some(name),
            Event::Shortcuts(mode) => state.shortcuts = Some(mode),
            Event::Open(open) => state.open = open,
            Event::Reset => {
                return Some(Message::Page(
                    self.page,
                    page::Message::Notice("Saved answers restored".into(), String::new()),
                ));
            }
            Event::Submit(answers) => {
                state.open = false;
                let detail = items::summary(self.id, &answers, state.shortcuts.as_deref());
                return Some(Message::Page(
                    self.page,
                    page::Message::Notice(items::notice(self.id).into(), detail),
                ));
            }
        }
        None
    }
    fn view(&self, state: &State) -> impl Widget<Event, Theme> + 'a {
        if self.id == "dialog" {
            let id = self.id;
            let page = self.page;
            Element::from(
                dialog(
                    |d| d.trigger(button("Open clarification").style(button::Outline)),
                    move |d| {
                        let form = Example { id, page }.form(&State::default());
                        column![form, d.close(button("Cancel").style(button::Outline))]
                            .spacing(12)
                            .into()
                    },
                )
                .open(state.open)
                .on_open_change(Event::Open),
            )
        } else {
            self.form(state).compose()
        }
    }
}
