use super::demo::Event;
use super::fixtures::{Role, Turn};
use iced::alignment::Horizontal::{Left, Right};
use icedcn::Slot;
use icedcn::radius;
use icedcn::widget::{bubble, column, message, motion, status_marker};

impl Turn {
    pub(super) fn motion(
        self,
        value: &'static str,
        preset: motion::Preset,
        animate: bool,
    ) -> Slot<'static, Event> {
        motion(self.view(value))
            .preset(preset)
            .animate(animate)
            .theme(bubble::fade_muted_on_card)
            .into()
    }

    pub(super) fn view(self, value: &'static str) -> Slot<'static, Event> {
        if self.role == Role::Marker {
            return status_marker(value).style(status_marker::Separator).into();
        }
        let user = self.role == Role::User;
        let alignment = if user { Right } else { Left };
        let paragraphs = value.split("\n\n").map(str::trim).filter(|s| !s.is_empty());
        let mut content = bubble(column(paragraphs.map(Into::into)).spacing(8));
        content = match self.role {
            Role::User | Role::Guest => content
                .style(if user { bubble::Muted } else { bubble::Tinted })
                .radius(radius::XL3)
                .padding([10, 12]),
            _ => content.style(bubble::Ghost),
        };
        let mut turn_view = message(content).align(alignment);
        if !user && !self.speaker.is_empty() {
            turn_view = turn_view.header(self.speaker);
            if self.role == Role::Assistant {
                turn_view = turn_view.metadata_padding(0);
            }
        }
        turn_view.into()
    }

    pub(super) fn short(self) -> String {
        let mut chars = self.text.chars();
        let mut text: String = chars.by_ref().take(39).collect();
        if chars.take(4).count() == 4 {
            text.push_str("...");
            text
        } else {
            self.text.to_owned()
        }
    }
}
