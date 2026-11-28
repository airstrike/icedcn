//! Local choices retain their selected value in the widget tree.

use iced::Fill;
use icedcn::widget::{View, native_select};

use crate::Message;

pub fn trigger<'a>(selected: &str, options: &[&str]) -> View<'a, Message> {
    native_select(
        options
            .iter()
            .map(|option| native_select::item(*option, *option)),
    )
    .default_value(selected)
    .width(Fill)
    .into()
}
