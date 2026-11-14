use super::demo::Event;
use super::fixtures::{Role, Turn};
use iced::alignment::Horizontal::{Left, Right};
use icedxdy::Slot;
use icedxdy::widget::{bubble, message, status_marker, text};

impl Turn {
    pub(super) fn view(self, value: &'static str) -> Slot<'static, Event> {
        if self.role == Role::Marker {
            return status_marker(text::small(value))
                .style(status_marker::Separator)
                .into();
        }
        let user = self.role == Role::User;
        let alignment = if user { Right } else { Left };
        let content = bubble(text::small(value))
            .style(match self.role {
                Role::User => bubble::Muted,
                Role::Guest => bubble::Tinted,
                _ => bubble::Ghost,
            })
            .align(alignment);
        let mut turn_view = message(content).align(alignment);
        if !user && !self.speaker.is_empty() {
            turn_view = turn_view.header(text::xs(self.speaker).semibold());
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
