//! Menu items, labels, separators, and submenus shared by
//! [`DropdownMenu`](crate::DropdownMenu) and [`ContextMenu`](crate::ContextMenu).
//!
//! ```ignore
//! use icedcn::widget::menu;
//!
//! column![
//!     menu::group([
//!         menu::label("My Account"),
//!         menu::item("Profile").shortcut("⇧⌘P").into(),
//!         menu::item("Billing").on_select(Message::Billing).into(),
//!     ]),
//!     menu::separator(),
//!     menu::item("Log out").style(menu::Destructive).into(),
//!     menu::item("API").disabled(true).into(),
//! ]
//! ```
//!
//! Selecting an enabled item publishes its message and closes the menu. Disabled items ignore
//! selection.

use crate::tooltip::interaction::{Interaction, Mode};
use std::time::Duration;

pub(crate) mod list;

use iced_core::Length::Fill;
use iced_core::text::{IntoFragment, LineHeight};
use iced_core::{Alignment, Element, Padding, mouse};
use iced_widget::{Renderer, Space, Widget, anchored};
use icedcn_core::Theme;
use icedcn_core::theme::menu::item;
use icedcn_core::theme::popover::content;

use crate::button::widget::{self, Button};
use crate::compose::{Compose, Inherited, Slot};
use crate::container::Container;
use crate::icon::Icon;
use crate::marker::{self, Press};
use crate::popover::Signal;
use crate::{column, container, row, text};

pub use icedcn_core::theme::menu::item::{Destructive, Variant};

/// A `MenuItem`: a row that publishes its message and closes the menu
/// when selected.
pub struct Item<'a, Message> {
    content: Slot<'a, Message>,
    shortcut: Option<&'a str>,
    on_select: Option<Message>,
    on_focus: Option<Message>,
    disabled: bool,
    inset: bool,
    highlighted: bool,
    variant: Variant,
    indicator: Indicator,
    closes: bool,
    anchor: bool,
}

/// What an item shows in its trailing indicator slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Indicator {
    /// A plain item: no slot.
    None,
    /// A checkbox or radio item: a check while checked.
    Check(bool),
    /// A submenu trigger: a chevron.
    Chevron,
}

/// Create a [`menu::Item`](Item) showing `content`.
pub fn item<'a, Message>(content: impl Into<Slot<'a, Message>>) -> Item<'a, Message> {
    Item {
        content: content.into(),
        shortcut: None,
        on_select: None,
        on_focus: None,
        disabled: false,
        inset: false,
        highlighted: false,
        variant: Variant::Default,
        indicator: Indicator::None,
        closes: true,
        anchor: false,
    }
}

/// A `MenuCheckboxItem`: shows a check while `checked`; selecting it
/// publishes `on_checked_change` with the flipped value.
pub fn checkbox_item<'a, Message>(
    content: impl Into<Slot<'a, Message>>,
    checked: bool,
    on_checked_change: impl Fn(bool) -> Message,
) -> Item<'a, Message> {
    Item {
        on_select: Some(on_checked_change(!checked)),
        indicator: Indicator::Check(checked),
        ..item(content)
    }
}

/// A `MenuRadioItem` for `value`: shows a check while `selected` holds it;
/// selecting it publishes `on_select(value)`.
pub fn radio_item<'a, Message, V>(
    content: impl Into<Slot<'a, Message>>,
    value: V,
    selected: Option<V>,
    on_select: impl Fn(V) -> Message,
) -> Item<'a, Message>
where
    V: Eq + Copy,
{
    Item {
        on_select: Some(on_select(value)),
        indicator: Indicator::Check(selected == Some(value)),
        ..item(content)
    }
}

impl<'a, Message> Item<'a, Message> {
    /// Show a `MenuShortcut` at the end of the item.
    pub fn shortcut(mut self, shortcut: &'a str) -> Self {
        self.shortcut = Some(shortcut);
        self
    }

    /// Mark a navigation item as a link with a pointer cursor.
    /// The caller supplies its destination message with `on_select`.
    pub fn anchor(mut self) -> Self {
        self.anchor = true;
        self
    }

    /// Set the message published when the item is selected.
    pub fn on_select(mut self, message: Message) -> Self {
        self.on_select = Some(message);
        self
    }

    /// Publish when pointer or keyboard navigation highlights this item.
    /// Menu focus is separate from selection; use this for reversible previews.
    pub fn on_focus(mut self, message: Message) -> Self {
        self.on_focus = Some(message);
        self
    }

    /// Disable the item: it dims and ignores the pointer.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Indent the item to line up with items that have an icon
    /// (`inset`).
    pub fn inset(mut self, inset: bool) -> Self {
        self.inset = inset;
        self
    }

    /// Paint this item as the active option of a keyboard-controlled list.
    pub fn highlighted(mut self, highlighted: bool) -> Self {
        self.highlighted = highlighted;
        self
    }

    /// Whether selecting the item closes the menu through its marker (a
    /// select closes itself instead).
    pub(crate) fn closes(mut self, closes: bool) -> Self {
        self.closes = closes;
        self
    }

    /// Set the item's variant (`.style(menu::Destructive)`).
    pub fn style(mut self, variant: impl Into<Variant>) -> Self {
        self.variant = variant.into();
        self
    }
}

impl<'a, Message: Clone + 'a> Compose<Message> for Item<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.disabled |= from.is_disabled();
        self.content.inherit(from);
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let Item {
            mut content,
            shortcut,
            on_select,
            on_focus,
            disabled,
            inset,
            highlighted,
            variant,
            indicator,
            closes,
            anchor,
        } = *self;

        content.inherit(
            &Inherited::default()
                .with_size(crate::Size::Default)
                .with_selectable(false),
        );
        let mut line = row![container(content).width(Fill)]
            .width(Fill)
            .spacing(6)
            .align_y(Alignment::Center);

        if let Some(shortcut) = shortcut {
            // `ml-auto text-xs tracking-widest text-muted-foreground`.
            line = line.push(Slot::from(
                text::muted(shortcut)
                    .size(text::size::XS)
                    .letter_spacing(0.1),
            ));
        }

        match indicator {
            Indicator::None => {}
            Indicator::Check(checked) => {
                line = line.push(Slot::from(
                    Container::new(if checked {
                        Slot::from(Icon::Check.boxed())
                    } else {
                        Slot::from(Space::new())
                    })
                    .width(16),
                ));
            }
            Indicator::Chevron => {
                line = line.push(Slot::from(Icon::ChevronRight.boxed()));
            }
        }

        // `px-1.5 py-1`, `pl-7` when inset, and a check item's `pr-8`
        // holding its indicator at `right-2`.
        let padding = Padding {
            top: 4.0,
            bottom: 4.0,
            left: if inset { 28.0 } else { 6.0 },
            right: if let Indicator::Check(_) = indicator {
                8.0
            } else {
                6.0
            },
        };

        let style: widget::StyleFn<'b, Theme> =
            Box::new(move |theme: &Theme, status: widget::Status| {
                let status = if !disabled
                    && (highlighted || matches!(status, widget::Status::Focused { .. }))
                {
                    widget::Status::Hovered
                } else {
                    status
                };
                item::style(theme, status.into(), variant).into()
            });

        let mut button = Button::new(Slot::from(line).compose())
            .padding(padding)
            .width(Fill)
            .translate(false)
            .disabled(disabled)
            .on_press_maybe(on_select)
            .style(style);
        if anchor {
            button = button.interaction(mouse::Interaction::Pointer);
        }
        if let Some(message) = on_focus {
            button = button.on_focus(message);
        }

        if closes {
            marker::marker(button.boxed(), Signal::Close, Press::Captured).into()
        } else {
            button.boxed()
        }
    }
}

impl<'a, Message: Clone + 'a> From<Item<'a, Message>> for Slot<'a, Message> {
    fn from(item: Item<'a, Message>) -> Self {
        Slot::new(item)
    }
}

/// A `MenuLabel`: a muted `text-xs font-medium` heading.
pub fn label<'a, Message: 'a>(label: impl IntoFragment<'a>) -> Slot<'a, Message> {
    Container::new(
        text::muted(label)
            .size(text::size::XS)
            .line_height(LineHeight::Absolute(16.0.into()))
            .medium(),
    )
    .padding([4, 6])
    .into()
}

/// A `MenuSeparator`: a 1px rule with `my-1`.
pub fn separator<'a, Message: 'a>() -> Slot<'a, Message> {
    Container::new(crate::separator::separator())
        .width(Fill)
        .padding([4, 0])
        .into()
}

/// A leading icon and its label, aligned like a menu item.
pub fn caption<'a, Message: 'a>(
    icon: impl Into<Slot<'a, Message>>,
    label: impl Into<Slot<'a, Message>>,
) -> Slot<'a, Message> {
    let icon = Container::new(icon).center_x(16).center_y(16);
    row::Row::with_children([icon.into(), label.into()])
        .spacing(6)
        .align_y(Alignment::Center)
        .into()
}

/// Group menu children of different types.
#[macro_export]
macro_rules! menu_group {
    ($($item:expr),* $(,)?) => {
        $crate::menu::group([$(Into::<$crate::Slot<'_, _>>::into($item)),*])
    };
}

/// Open a submenu containing children of different types.
#[macro_export]
macro_rules! menu_sub {
    ($trigger:expr, $($item:expr),+ $(,)?) => {
        $crate::menu::sub($trigger, [$(Into::<$crate::Slot<'_, _>>::into($item)),+])
    };
}

/// A `MenuGroup`: its children stacked.
pub fn group<'a, Message: 'a>(
    children: impl IntoIterator<Item = Slot<'a, Message>>,
) -> Slot<'a, Message> {
    column::Column::with_children(children).width(Fill).into()
}

/// A `MenuSub`: an item labelled `content` that opens `items` beside it
/// while hovered. Selecting an item inside closes the whole menu.
pub fn sub<'a, Message: Clone + 'a>(
    content: impl Into<Slot<'a, Message>>,
    items: impl IntoIterator<Item = Slot<'a, Message>>,
) -> Slot<'a, Message> {
    Slot::new(Sub {
        trigger: Item {
            indicator: Indicator::Chevron,
            closes: false,
            ..item(content)
        },
        items: group(items),
    })
}

/// A submenu: its trigger item and its panel.
struct Sub<'a, Message> {
    trigger: Item<'a, Message>,
    items: Slot<'a, Message>,
}

impl<'a, Message: Clone + 'a> Compose<Message> for Sub<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.trigger.inherit(from);
        self.items.inherit(from);
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        // `MenuSubContent`: `min-w-[96px] p-1 shadow-lg`, beside the
        // trigger, its first item level with it.
        let panel = Container::new(self.items)
            .padding(4)
            .width(Fill)
            .style(content);

        let body = list::List {
            content: Slot::from(panel).compose(),
            horizontal: Some(false),
            autofocus: false.into(),
            on_move: None,
        }
        .boxed();
        let position = anchored::Position::new(anchored::Side::Right)
            .align(anchored::Align::Start)
            .offset(0);
        let sub = Interaction::new(
            Slot::from(self.trigger).compose(),
            body,
            position,
            (Duration::ZERO, Duration::from_millis(150)),
            Mode::Submenu,
            None,
            None,
        );
        marker::relay::<Signal, _, _, _>(sub.boxed()).into()
    }
}
