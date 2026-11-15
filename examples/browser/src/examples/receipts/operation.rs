//! Keep the original receipt form's focus traversal inside its hosted viewport.
use iced::Task;
use iced::advanced::widget::operate;
use iced::advanced::widget::operation::{focusable, scope};

pub(super) use iced::widget::operation::focus;

pub fn focus_next<Message: Send + 'static>() -> Task<Message> {
    operate(scope(super::ROOT.into(), focusable::focus_next()))
}

pub fn focus_previous<Message: Send + 'static>() -> Task<Message> {
    operate(scope(super::ROOT.into(), focusable::focus_previous()))
}
