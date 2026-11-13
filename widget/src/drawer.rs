//! A window-edge panel that supports touch dismissal.
use crate::compose::Slot;
pub use crate::dialog::Side;
use crate::dialog::{Dialog, Handle, Placement};
pub use crate::sheet::header;

/// Open a bottom drawer. Use `.side(..)` to choose another edge.
pub fn drawer<'a, Message>(
    trigger: impl Fn(&Handle) -> Slot<'a, Message> + 'a,
    content: impl Fn(&Handle) -> Slot<'a, Message> + 'a,
) -> Dialog<'a, Message> {
    Dialog::new(trigger, content, Placement::Side(Side::Bottom))
        .drawer(0.8)
        .show_close_button(false)
}
/// Bottom actions, padded and stacked at the end of the drawer.
pub fn footer<'a, Message: 'a>(
    actions: impl IntoIterator<Item = Slot<'a, Message>>,
) -> Slot<'a, Message> {
    use crate::column::Column;
    use iced_core::Length::Fill;
    Column::with_children(actions)
        .width(Fill)
        .spacing(8)
        .padding(16)
        .into()
}
