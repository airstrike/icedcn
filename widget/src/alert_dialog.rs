//! A modal confirmation prompt.
//!
//! A [`Dialog`] that interrupts with a question: it has no corner close
//! button and a press outside it does not dismiss it; its cancel and action
//! buttons (and `Escape`) do. Two closures receive a [`Handle`], like the
//! dialog's:
//!
//! ```ignore
//! use icedcn::widget::{alert_dialog, button, column};
//!
//! alert_dialog(
//!     |a| a.trigger(button("Show Dialog").style(button::Outline)),
//!     |a| {
//!         column![
//!             a.header("Are you absolutely sure?", "This action cannot be undone."),
//!             a.footer([
//!                 a.cancel(button("Cancel").style(button::Outline)),
//!                 a.action(button("Continue").on_press(Message::Delete)),
//!             ]),
//!         ]
//!         .width(Fill)
//!         .into()
//!     },
//! )
//! .size(Size::Sm)
//! ```
//!
//! `Size::Sm` narrows the panel, centers the header and splits the footer
//! into equal halves.

use iced_core::Element;
use iced_core::Length::Fill;
use iced_core::alignment::{Horizontal, Vertical};
use iced_widget::Renderer;
use icedcn_core::Theme;
use icedcn_core::theme::alert_dialog;
use icedcn_core::theme::alert_dialog::Spec;
use icedcn_core::theme::text::Foreground;

use crate::Size;
use crate::button::Button;
use crate::column::Column;
use crate::compose::{Compose, Inherited, Slot};
use crate::container::Container;
use crate::dialog::{self, Dialog, Placement};
use crate::row::Row;
use crate::separator::separator;
use crate::text;

pub use icedcn_core::theme::alert_dialog::{Destructive, Media};

#[cfg(test)]
mod tests;

/// A closure that builds part of an [`AlertDialog`].
type Body<'a, Message> = Box<dyn Fn(&Handle) -> Slot<'a, Message> + 'a>;

/// A modal question with cancel and action buttons.
pub struct AlertDialog<'a, Message> {
    trigger: Body<'a, Message>,
    content: Body<'a, Message>,
    size: Size,
    open: Option<bool>,
    default_open: bool,
    on_open_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    inherited: Inherited,
}

/// What an [`AlertDialog`]'s closures receive: the trigger, cancel and
/// action wrappers, and the header and footer builders for its size.
#[derive(Debug, Clone, Copy)]
pub struct Handle {
    dialog: dialog::Handle,
    is_small: bool,
    is_mobile: bool,
}

/// Create a closed, uncontrolled [`AlertDialog`]: `trigger` builds what
/// sits in place, `content` the panel shown while open.
pub fn alert_dialog<'a, Message>(
    trigger: impl Fn(&Handle) -> Slot<'a, Message> + 'a,
    content: impl Fn(&Handle) -> Slot<'a, Message> + 'a,
) -> AlertDialog<'a, Message> {
    AlertDialog {
        trigger: Box::new(trigger),
        content: Box::new(content),
        size: Size::Default,
        open: None,
        default_open: false,
        on_open_change: None,
        inherited: Inherited::default(),
    }
}

impl<'a, Message> AlertDialog<'a, Message> {
    /// Choose the default or small dialog size.
    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }

    /// Control the dialog: it is open exactly when `open` is.
    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    /// Whether an uncontrolled dialog starts open.
    pub fn default_open(mut self, open: bool) -> Self {
        self.default_open = open;
        self
    }

    /// Set the message published with the new open state.
    pub fn on_open_change(mut self, on_change: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_open_change = Some(Box::new(on_change));
        self
    }
}

impl Handle {
    /// Whether the dialog is open.
    pub fn is_open(&self) -> bool {
        self.dialog.is_open()
    }

    /// Make `child` open the dialog when pressed (`AlertDialogTrigger`).
    pub fn trigger<'a, Message: 'a>(
        &self,
        child: impl Into<Slot<'a, Message>>,
    ) -> Slot<'a, Message> {
        self.dialog.trigger(child)
    }

    /// Closes the dialog when the supplied button is pressed. Style the button at the call
    /// site.
    pub fn cancel<'a, Message: Clone + 'a>(
        &self,
        button: Button<'a, Message>,
    ) -> Slot<'a, Message> {
        self.dialog.close(self.fit(button))
    }

    /// An `AlertDialogAction`: `button` closes the dialog and still
    /// publishes its own `on_press`.
    pub fn action<'a, Message: Clone + 'a>(
        &self,
        button: Button<'a, Message>,
    ) -> Slot<'a, Message> {
        self.dialog.close(self.fit(button))
    }

    /// An `AlertDialogHeader`: an `AlertDialogTitle` over an
    /// `AlertDialogDescription`, optionally beside an `AlertDialogMedia`.
    pub fn header<'a, Message: 'a>(
        &self,
        title: &'a str,
        description: &'a str,
    ) -> Header<'a, Message> {
        let alignment = if self.is_small || self.is_mobile {
            Horizontal::Center
        } else {
            Horizontal::Left
        };
        self.header_content(title, text::muted(description).align_x(alignment))
    }

    /// Build a header with a composed description, such as rich text with a link.
    pub fn header_content<'a, Message: 'a>(
        &self,
        title: &'a str,
        description: impl Into<Slot<'a, Message>>,
    ) -> Header<'a, Message> {
        Header {
            title,
            description: description.into(),
            media: None,
            media_style: Media.into(),
            is_small: self.is_small || self.is_mobile,
        }
    }

    /// An `AlertDialogFooter`: its buttons on a `bg-muted/50` band, at
    /// the end (default) or in equal halves (`Size::Sm`). Narrow default
    /// dialogs stack actions in reverse visual order, preserving focus order.
    pub fn footer<'a, Message: 'a>(
        &self,
        buttons: impl IntoIterator<Item = Slot<'a, Message>>,
    ) -> Slot<'a, Message> {
        let buttons: Vec<_> = buttons.into_iter().collect();
        let actions: Slot<'a, Message> = if self.is_mobile && !self.is_small {
            Column::with_children(buttons)
                .reverse()
                .spacing(8)
                .width(Fill)
                .into()
        } else {
            let row = Row::with_children(buttons).spacing(8);
            if self.is_small {
                row.width(Fill).into()
            } else {
                row.into()
            }
        };

        Column::with_children([
            Slot::from(separator()),
            Slot::from(
                Container::new(actions)
                    .width(Fill)
                    .padding(16)
                    .align_x(Horizontal::Right)
                    .style(alert_dialog::footer),
            ),
        ])
        .width(Fill)
        .into()
    }

    /// Small actions fill their half; narrow default actions fill the column.
    fn fit<'a, Message: Clone + 'a>(&self, button: Button<'a, Message>) -> Button<'a, Message> {
        if self.is_small || self.is_mobile {
            button.width(Fill)
        } else {
            button
        }
    }
}

/// An `AlertDialogHeader`, built by [`Handle::header`].
pub struct Header<'a, Message> {
    title: &'a str,
    description: Slot<'a, Message>,
    media: Option<Slot<'a, Message>>,
    media_style: Spec,
    is_small: bool,
}

impl<'a, Message: 'a> Header<'a, Message> {
    /// Show `content` (usually an icon, drawn at 24px) on an
    /// `AlertDialogMedia` tile.
    pub fn media(mut self, content: impl Into<Slot<'a, Message>>) -> Self {
        self.media = Some(content.into());
        self
    }

    /// Restyle the media tile: `alert_dialog::Media` with tweaks.
    pub fn media_style(mut self, style: impl Into<Spec>) -> Self {
        self.media_style = style.into();
        self
    }
}

impl<'a, Message: 'a> Compose<Message> for Header<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.description.inherit(from);
        if let Some(media) = &mut self.media {
            media.inherit(from);
        }
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let Header {
            title,
            description,
            media,
            media_style,
            is_small,
        } = *self;

        // `text-base font-medium` over `text-sm text-muted-foreground`,
        // centered in a small dialog.
        let alignment = if is_small {
            Horizontal::Center
        } else {
            Horizontal::Left
        };
        let title = text::Text::new(title)
            .size(text::size::BASE)
            .medium()
            .style(Foreground)
            .align_x(alignment);
        let copy = Column::with_children([Slot::from(title), description])
            .spacing(6)
            .width(Fill)
            .align_x(alignment);

        // `size-10 rounded-md`, icons at `size-6`.
        let media = media.map(|mut content| {
            content.inherit(&Inherited::default().with_size(Size::Custom(24.0)));

            Container::new(content)
                .width(40)
                .height(40)
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center)
                .class(media_style)
        });

        let header: Slot<'b, Message> = match (media, is_small) {
            (None, _) => copy.into(),
            (Some(media), true) => Column::with_children([
                Slot::from(Container::new(media).padding(iced_core::Padding::ZERO.bottom(8))),
                Slot::from(copy),
            ])
            .spacing(6)
            .width(Fill)
            .align_x(Horizontal::Center)
            .into(),
            (Some(media), false) => Row::with_children([Slot::from(media), Slot::from(copy)])
                .spacing(16)
                .align_y(Vertical::Top)
                .width(Fill)
                .into(),
        };

        Container::new(header).width(Fill).padding(16).into()
    }
}

impl<'a, Message: 'a> From<Header<'a, Message>> for Slot<'a, Message> {
    fn from(header: Header<'a, Message>) -> Self {
        Slot::new(header)
    }
}

impl<'a, Message: Clone + 'static> Compose<Message> for AlertDialog<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.inherited = self.inherited.or(*from);
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let AlertDialog {
            trigger,
            content,
            size,
            open,
            default_open,
            on_open_change,
            inherited,
        } = *self;

        let is_small = size == Size::Sm;
        // `max-w-xs` for a small panel, `sm:max-w-sm` otherwise.
        let max_width = if is_small { 320.0 } else { 384.0 };

        let mut dialog = Dialog::new(
            move |d| {
                trigger(&Handle {
                    dialog: *d,
                    is_small,
                    is_mobile: d.is_mobile(),
                })
            },
            move |d| {
                content(&Handle {
                    dialog: *d,
                    is_small,
                    is_mobile: d.is_mobile(),
                })
            },
            Placement::Center { max_width },
        )
        .compact(320.0, 640.0)
        .show_close_button(false)
        .dismiss_outside(false)
        .padding(0)
        .default_open(default_open);

        if let Some(open) = open {
            dialog = dialog.open(open);
        }

        if let Some(on_change) = on_open_change {
            dialog = dialog.on_open_change(on_change);
        }

        Compose::inherit(&mut dialog, &inherited);

        Slot::from(dialog).compose()
    }
}

impl<'a, Message: Clone + 'static> From<AlertDialog<'a, Message>> for Slot<'a, Message> {
    fn from(dialog: AlertDialog<'a, Message>) -> Self {
        Slot::new(dialog)
    }
}

impl<'a, Message: Clone + 'static> From<AlertDialog<'a, Message>>
    for Element<'a, Message, Theme, Renderer>
{
    fn from(dialog: AlertDialog<'a, Message>) -> Self {
        Box::new(dialog).compose()
    }
}
