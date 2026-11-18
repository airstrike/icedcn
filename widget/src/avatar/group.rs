//! Overlapping avatars with an optional trailing count.

use iced_core::Element;
use iced_widget::{Renderer, Widget, row};
use icedcn_core::Theme;
use icedcn_core::theme::avatar::group_count_style;

use crate::compose::{Compose, Inherited, Slot};

use super::{Avatar, Geometry, Size, avatar};

/// Construct a group from avatars or image handles, each converted on its own.
///
/// ```ignore
/// avatar::group![avatar(portrait).fallback("AT"), another_portrait].count("+3")
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! __avatar_group {
    () => {
        $crate::avatar::group(std::iter::empty::<$crate::avatar::Avatar<'_, _>>())
    };
    ($($avatar:expr),+ $(,)?) => {
        $crate::avatar::group([
            $(Into::<$crate::avatar::Avatar<'_, _>>::into($avatar)),*
        ])
    };
}

/// A group of avatars, including avatars with badges.
pub struct Group<'a, Message> {
    avatars: Vec<Avatar<'a, Message>>,
    count: Option<Slot<'a, Message>>,
    inherited: Inherited,
}

/// Arrange avatars with 8px overlap and a 2px outer background ring.
pub fn group<'a, Message: 'a>(
    avatars: impl IntoIterator<Item = impl Into<Avatar<'a, Message>>>,
) -> Group<'a, Message> {
    Group {
        avatars: avatars.into_iter().map(Into::into).collect(),
        count: None,
        inherited: Inherited::default(),
    }
}

impl<'a, Message: 'a> Group<'a, Message> {
    /// Append a count such as `"+3"`, or an explicitly sized stock iced icon.
    ///
    /// The chip matches any small/large member; small wins in a mixed group.
    /// Text stays 14px. Source icon sizes are 12/16/20px for small/default/large.
    pub fn count(mut self, content: impl Into<Slot<'a, Message>>) -> Self {
        self.count = Some(content.into());
        self
    }

    /// Geometry of the count chip, following the source's group selectors.
    pub fn count_geometry(&self) -> Geometry {
        let mut geometry = Geometry::DEFAULT;
        for avatar in &self.avatars {
            let member = avatar.geometry();
            if member == Geometry::SM {
                return Geometry {
                    font_size: 14.0,
                    ..Geometry::SM
                };
            }
            if member == Geometry::LG {
                geometry = Geometry::LG;
            }
        }
        geometry
    }
}

impl<'a, Message: 'a> Compose<Message> for Group<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.inherited = self.inherited.or(*from);
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let geometry = self.count_geometry();
        let Self {
            avatars,
            count,
            inherited,
        } = *self;
        let mut children: Vec<Element<'_, Message, Theme, Renderer>> = avatars
            .into_iter()
            .map(|mut avatar| {
                avatar.inherit(&inherited);
                avatar.ring(true).into()
            })
            .collect();
        if let Some(count) = count {
            let mut chip = avatar(None)
                .fallback(count)
                .size(Size(geometry))
                .ring(true)
                .style(|theme| {
                    let mut style = group_count_style(theme);
                    style.border.width = 0.0;
                    style
                });
            chip.inherit(&inherited);
            children.push(chip.into());
        }
        row(children).spacing(-8.0).boxed()
    }
}

impl<'a, Message: 'a> From<Group<'a, Message>> for Element<'a, Message, Theme, Renderer> {
    fn from(group: Group<'a, Message>) -> Self {
        Box::new(group).compose()
    }
}

impl<'a, Message: 'a> From<Group<'a, Message>> for Slot<'a, Message> {
    fn from(group: Group<'a, Message>) -> Self {
        Slot::new(group)
    }
}
