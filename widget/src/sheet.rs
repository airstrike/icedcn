//! A modal panel attached to a window edge.
//!
//! The same component as [`Dialog`]: two closures receive a
//! [`Handle`](crate::dialog::Handle), `s.trigger` opens it and `s.close`
//! dismisses it. The panel spans the window along its [`Side`] and pads its
//! sections itself, through [`header`] and [`footer`]:
//!
//! ```ignore
//! use icedcn::widget::{button, column, sheet};
//!
//! sheet(
//!     |s| s.trigger(button("Open").style(button::Outline)),
//!     |s| {
//!         column![
//!             sheet::header("Edit profile", "Make changes to your profile here."),
//!             body(),
//!             sheet::footer([
//!                 button("Save changes").into(),
//!                 s.close(button("Close").style(button::Outline)),
//!             ]),
//!         ]
//!         .into()
//!     },
//! )
//! .side(sheet::Side::Left)
//! ```

use iced_core::Length::Fill;
use iced_core::alignment::Vertical;
use iced_core::{Element, Length};
use iced_widget::Renderer;
use icedcn_core::Theme;
use icedcn_core::theme;
use icedcn_core::theme::{container::Style, dialog};

use crate::column::Column;
use crate::compose::{Compose, Inherited, View};
use crate::container::Container;
use crate::dialog::{Dialog, Handle, Placement};
use crate::{column, text};

pub use crate::dialog::Side;

/// A panel that slides in from a window edge.
pub struct Sheet<'a, Message>(Dialog<'a, Message>);

/// Create a closed, uncontrolled [`Sheet`] on the right: `trigger` builds
/// what sits in place, `content` the panel shown while open.
pub fn sheet<'a, Message>(
    trigger: impl Fn(&Handle) -> View<'a, Message> + 'a,
    content: impl Fn(&Handle) -> View<'a, Message> + 'a,
) -> Sheet<'a, Message> {
    Sheet(Dialog::new(trigger, content, Placement::Side(Side::Right)))
}

impl<'a, Message> Sheet<'a, Message> {
    /// Set the edge the sheet is pinned to.
    pub fn side(self, side: Side) -> Self {
        Self(self.0.side(side))
    }

    /// Limit a top or bottom sheet to a fraction of the viewport height.
    pub fn max_height_ratio(mut self, fraction: f32) -> Self {
        self.0.sheet_height = fraction.clamp(0., 1.);
        self
    }

    /// Control the sheet: it is open exactly when `open` is.
    pub fn open(self, open: bool) -> Self {
        Self(self.0.open(open))
    }

    /// Whether an uncontrolled sheet starts open.
    pub fn default_open(self, open: bool) -> Self {
        Self(self.0.default_open(open))
    }

    /// Set the message published with the new open state when the sheet is
    /// opened or dismissed.
    pub fn on_open_change(self, on_change: impl Fn(bool) -> Message + 'a) -> Self {
        Self(self.0.on_open_change(on_change))
    }

    /// Whether the panel shows its corner close button (on by default).
    pub fn show_close_button(self, show: bool) -> Self {
        Self(self.0.show_close_button(show))
    }
}

/// Paint the one-pixel border only on the edge facing the document.
pub(crate) fn panel<'a, Message: 'a>(
    body: View<'a, Message>,
    side: Side,
) -> Container<'a, Message> {
    let (height, edge) = match side {
        Side::Left => (Fill, [0, 1, 0, 0]),
        Side::Right => (Fill, [0, 0, 0, 1]),
        Side::Top => (Length::Shrink, [0, 0, 1, 0]),
        Side::Bottom => (Length::Shrink, [1, 0, 0, 0]),
    };
    let surface = Container::new(body)
        .width(Fill)
        .height(height)
        .style(|theme: &Theme| Style {
            background: Some(theme.popover.base.into()),
            text_color: Some(theme.popover.foreground),
            ..Style::default()
        });
    Container::new(surface)
        .width(Fill)
        .height(height)
        .padding(edge)
        .style(|theme: &Theme| Style {
            background: Some(theme.neutral.border.into()),
            ..dialog::sheet(theme)
        })
}

/// A `SheetHeader`: a `SheetTitle` over a `SheetDescription`, `p-4 gap-0.5`.
pub fn header<'a, Message: 'a>(title: &'a str, description: &'a str) -> View<'a, Message> {
    Container::new(
        column![
            text::Text::new(title)
                .size(text::size::BASE)
                .medium()
                .style(theme::text::Foreground),
            text::muted(description),
        ]
        .spacing(2)
        .width(Fill),
    )
    .width(Fill)
    .padding(16)
    .into()
}

/// A `SheetFooter`: its actions stacked, `p-4 gap-2`, at the bottom of the
/// panel (`mt-auto`).
pub fn footer<'a, Message: 'a>(
    actions: impl IntoIterator<Item = View<'a, Message>>,
) -> View<'a, Message> {
    actions_column(actions)
        .height(Fill)
        .align_y(Vertical::Bottom)
        .into()
}

/// Lay out a header, expanding body and fixed footer. A scrollable body scrolls
/// independently of the heading and actions. The body has 16px side padding.
pub fn content<'a, Message: 'a>(
    header: impl Into<View<'a, Message>>,
    body: impl Into<View<'a, Message>>,
    actions: impl IntoIterator<Item = View<'a, Message>>,
) -> View<'a, Message> {
    let header = header.into();
    column![
        header,
        Container::new(body)
            .width(Fill)
            .height(Fill)
            .padding([0, 16]),
        actions_column(actions),
    ]
    .width(Fill)
    .height(Length::Fit)
    .spacing(16)
    .into()
}

fn actions_column<'a, Message: 'a>(
    actions: impl IntoIterator<Item = View<'a, Message>>,
) -> Container<'a, Message> {
    Container::new(
        Column::with_children(actions)
            .stretch()
            .width(Fill)
            .spacing(8),
    )
    .width(Fill)
    .height(Length::Fit)
    .padding(16)
}

impl<'a, Message: Clone + 'static> Compose<Message> for Sheet<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        Compose::inherit(&mut self.0, from);
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        Box::new(self.0).compose()
    }
}

impl<'a, Message: Clone + 'static> From<Sheet<'a, Message>> for View<'a, Message> {
    fn from(sheet: Sheet<'a, Message>) -> Self {
        View::new(sheet)
    }
}

impl<'a, Message: Clone + 'static> From<Sheet<'a, Message>>
    for Element<'a, Message, Theme, Renderer>
{
    fn from(sheet: Sheet<'a, Message>) -> Self {
        sheet.0.into()
    }
}
