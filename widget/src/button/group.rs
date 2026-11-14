//! Related controls joined through the ordinary Compose/Slot contract.
use crate::compose::{Compose, Inherited, Slot};
use iced_core::Length;
use iced_widget::{Renderer, Widget};
use icedcn_core::Theme;

pub(crate) mod join;
pub mod separator;
pub mod text;
mod widget;
pub use icedcn_core::{Horizontal, Orientation, Vertical};
pub use join::Join;
pub use separator::Separator;
pub use text::Text;

/// A row or column of joined controls, or spaced nested groups.
pub struct ButtonGroup<'a, Message> {
    children: Vec<Slot<'a, Message>>,
    orientation: Orientation,
    width: Length,
    inherited: Inherited,
}

/// Group related controls. Use [`crate::button_group!`] for mixed child types.
pub fn group<'a, Message: 'a>(
    children: impl IntoIterator<Item = Slot<'a, Message>>,
) -> ButtonGroup<'a, Message> {
    ButtonGroup {
        children: children.into_iter().collect(),
        orientation: Horizontal,
        width: Length::Shrink,
        inherited: Inherited::default(),
    }
}
/// A divider inside a button group; vertical by default.
pub fn separator() -> Separator {
    Separator::new()
}
/// Muted text or a custom label inside a button group.
pub fn text<'a, Message: 'a>(content: impl Into<Slot<'a, Message>>) -> Text<'a, Message> {
    Text::new(content)
}

impl<'a, Message: 'a> ButtonGroup<'a, Message> {
    /// Set the group axis; horizontal by default.
    pub fn orientation(mut self, orientation: impl Into<Orientation>) -> Self {
        self.orientation = orientation.into();
        self
    }
    /// Set the group's width; shrink to its controls by default.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }
}
impl<'a, Message: 'a> Compose<Message> for ButtonGroup<'a, Message> {
    fn is_button_group(&self) -> bool {
        true
    }
    fn inherit(&mut self, from: &Inherited) {
        self.inherited = self.inherited.or(from.without_join());
    }
    fn compose<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let total = self.children.len();
        let spacing = if self.children.iter().any(Slot::is_button_group) {
            8.
        } else {
            0.
        };
        let inherited = self.inherited.without_join();
        let orientation = self.orientation;
        let children = self
            .children
            .into_iter()
            .enumerate()
            .map(|(index, mut child)| {
                child.inherit(&inherited.with_join(Join {
                    orientation,
                    first: index == 0,
                    last: index + 1 == total,
                }));
                child.compose()
            })
            .collect();
        widget::Group::new(children, orientation, self.width, spacing).boxed()
    }
}
impl<'a, Message: 'a> From<ButtonGroup<'a, Message>> for Slot<'a, Message> {
    fn from(group: ButtonGroup<'a, Message>) -> Self {
        Slot::new(group)
    }
}
impl<'a, Message: 'a> From<ButtonGroup<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(group: ButtonGroup<'a, Message>) -> Self {
        Box::new(group).compose()
    }
}
/// Compose related controls with the same children accepted by rows and columns.
#[macro_export]
macro_rules! button_group {
    () => { $crate::button::group::group(::core::iter::empty()) };
    ($($child:expr),+ $(,)?) => { $crate::button::group::group([
        $(Into::<$crate::compose::Slot<'_, _>>::into($child)),+
    ]) };
}
