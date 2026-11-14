//! An anchored list for selecting one item.
//!
//! A trigger showing the chosen item's label (or a muted placeholder) that
//! opens a modal list of items anchored below it, at least as wide as the
//! trigger, with a check beside the chosen one. An iced `Component` that
//! owns its open state and, uncontrolled, its value:
//!
//! ```ignore
//! use icedcn::widget::select;
//!
//! select([select::group([
//!     select::label("Fruits"),
//!     select::item("apple", "Apple"),
//!     select::item("banana", "Banana"),
//! ])])
//! .placeholder("Select a fruit")
//! .width(192)
//! // Controlled: .value(self.fruit.clone()).on_value_change(Message::Fruit)
//! ```
//!
//! The selected row aligns with the trigger. Choose [`Position::Popper`] for
//! an edge-anchored panel with a four-pixel gap.

use iced_core::Length::Fill;
use iced_core::{Alignment, Element, Length, Padding};
use iced_widget::component::{self, Component};
use iced_widget::text::LineHeight;
use iced_widget::{Renderer, Space, Widget, anchored};
use icedcn_core::Theme;
use icedcn_core::theme::container::Style;
use icedcn_core::theme::popover::content;
use icedcn_core::theme::select::trigger;
use std::rc::Rc;

use crate::button::widget::{self, Button};
use crate::compose::{Compose, Inherited, Slot};
use crate::container::Container;
use crate::{Size, column, menu, row, scrollable, text};

pub use iced_widget::anchored::{Align, Side};
pub use icedcn_core::theme::select::{Invalid, Variant};

/// A part of a select's list.
#[derive(Debug, Clone, PartialEq)]
pub enum Entry {
    /// A `SelectItem`: a value and its label.
    Item {
        /// The value published when chosen.
        value: String,
        /// The label shown in the list and in the trigger.
        label: String,
        /// Whether the item ignores the pointer.
        disabled: bool,
    },
    /// A `SelectLabel`.
    Label(String),
    /// A `SelectSeparator`.
    Separator,
    /// A `SelectGroup`.
    Group(Vec<Entry>),
}

/// A `SelectItem` publishing `value`, shown as `label`.
pub fn item(value: impl Into<String>, label: impl Into<String>) -> Entry {
    Entry::Item {
        value: value.into(),
        label: label.into(),
        disabled: false,
    }
}

/// A `SelectLabel`.
pub fn label(label: impl Into<String>) -> Entry {
    Entry::Label(label.into())
}

/// A `SelectSeparator`.
pub fn separator() -> Entry {
    Entry::Separator
}

/// A `SelectGroup`.
pub fn group(entries: impl IntoIterator<Item = Entry>) -> Entry {
    Entry::Group(entries.into_iter().collect())
}

impl Entry {
    /// Disable an item: it dims and cannot be chosen.
    pub fn disabled(self, disabled: bool) -> Self {
        match self {
            Entry::Item { value, label, .. } => Entry::Item {
                value,
                label,
                disabled,
            },
            other => other,
        }
    }

    fn items(&self) -> Box<dyn Iterator<Item = (&str, &str)> + '_> {
        match self {
            Entry::Item { value, label, .. } => {
                Box::new(std::iter::once((value.as_str(), label.as_str())))
            }
            Entry::Group(entries) => Box::new(entries.iter().flat_map(Entry::items)),
            Entry::Label(_) | Entry::Separator => Box::new(std::iter::empty()),
        }
    }
}

/// Placement of the list relative to its trigger.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Position {
    #[default]
    ItemAligned,
    Popper,
}

type ValueLabel<'a> = Box<dyn Fn(&str, &str) -> String + 'a>;
type ItemContent<'a> = Box<dyn Fn(&str, &str) -> Slot<'a, Event> + 'a>;

type TriggerPaint<'a> = Rc<dyn Fn(&Theme, widget::Status, bool, bool) -> widget::Style + 'a>;
/// A field choosing one value from a list.
pub struct Select<'a, Message> {
    entries: Vec<Entry>,
    position: Position,
    side: Side,
    align: Align,
    placeholder: String,
    font: Option<iced_core::Font>,
    item_content: Option<ItemContent<'a>>,
    value_label: Option<ValueLabel<'a>>,
    value: Option<String>,
    default_value: Option<String>,
    on_value_change: Option<Box<dyn Fn(String) -> Message + 'a>>,
    disabled: bool,
    width: Length,
    min_width: f32,
    padding: Padding,
    size: Size,
    variant: Variant,
    trigger_paint: Option<TriggerPaint<'a>>,
    icon: Option<Box<dyn Fn() -> Slot<'a, Event> + 'a>>,
    inherited: Inherited,
}

/// The select's private events.
#[derive(Debug, Clone)]
pub enum Event {
    /// The trigger was pressed.
    Toggle,
    /// The list asked to open or close.
    Set(bool),
    /// An item was chosen.
    Choose(String),
}

/// The open state and, uncontrolled, the value of a select.
#[derive(Debug, Default)]
pub struct State {
    open: bool,
    value: Option<String>,
}

/// Create an uncontrolled [`Select`] over `entries`, showing its
/// placeholder while nothing is chosen.
pub fn select<'a, Message>(entries: impl IntoIterator<Item = Entry>) -> Select<'a, Message> {
    Select {
        entries: entries.into_iter().collect(),
        position: Position::ItemAligned,
        side: Side::Bottom,
        align: Align::Center,
        placeholder: String::new(),
        value_label: None,
        font: None,
        item_content: None,
        value: None,
        default_value: None,
        on_value_change: None,
        disabled: false,
        width: Length::Shrink,
        min_width: 144.,
        // Nova's pl-2.5 / pr-2 plus the one-pixel border.
        padding: Padding::default().left(11).right(9),
        size: Size::Default,
        variant: Variant::Default,
        trigger_paint: None,
        icon: None,
        inherited: Inherited::default(),
    }
}

impl<'a, Message> Select<'a, Message> {
    /// Format the collapsed trigger independently from the labels in the menu.
    pub fn format_value(mut self, format: impl Fn(&str, &str) -> String + 'a) -> Self {
        self.value_label = Some(Box::new(format));
        self
    }
    /// Use selected-row alignment or popper placement.
    pub fn position(mut self, position: Position) -> Self {
        self.position = position;
        self
    }

    /// Choose the preferred side for a popper-positioned list.
    pub fn side(mut self, side: Side) -> Self {
        self.side = side;
        self
    }

    /// Align a popper-positioned list with the start, center, or end of its trigger.
    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    /// Set the text shown while nothing is chosen (`SelectValue
    /// placeholder`).
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Control the select: it shows exactly `value`, and choosing an item
    /// only publishes [`on_value_change`](Self::on_value_change).
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// The value an uncontrolled select starts with.
    pub fn default_value(mut self, value: impl Into<String>) -> Self {
        self.default_value = Some(value.into());
        self
    }

    /// Set the message published with the chosen value.
    pub fn on_value_change(mut self, on_change: impl Fn(String) -> Message + 'a) -> Self {
        self.on_value_change = Some(Box::new(on_change));
        self
    }

    /// Disable the select: it dims and does not open.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set the trigger's width (`w-fit` by default).
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Set the space between the trigger's outer edge and its content.
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    /// Set the trigger's size: `Size::Sm` is `h-7`, the default `h-8`.
    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }

    /// Customize the trigger surface for its button, open, and selection states.
    pub fn trigger_style(
        mut self,
        paint: impl Fn(&Theme, widget::Status, bool, bool) -> widget::Style + 'a,
    ) -> Self {
        self.trigger_paint = Some(Rc::new(paint));
        self
    }

    /// Set the selected value's font (for example, a monospace currency symbol).
    pub fn font(mut self, font: impl Into<iced_core::Font>) -> Self {
        self.font = Some(font.into());
        self
    }

    /// Compose the option's label while preserving its value and selection behavior.
    pub fn format_item<Content: Into<Slot<'a, Event>> + 'a>(
        mut self,
        content: impl Fn(&str, &str) -> Content + 'a,
    ) -> Self {
        self.item_content = Some(Box::new(move |value, label| content(value, label).into()));
        self
    }

    /// Set the option panel's minimum width, constrained to the viewport.
    pub fn min_width(mut self, width: f32) -> Self {
        self.min_width = width;
        self
    }

    /// Set the trigger's variant (`.style(select::Invalid)`).
    pub fn style(mut self, variant: impl Into<Variant>) -> Self {
        self.variant = variant.into();
        self
    }

    /// Replace the chevron with a 16-pixel icon from an icon font.
    pub fn icon(self, icon: impl Fn() -> iced_widget::Text<'a, Theme> + 'a) -> Self {
        self.icon_content(move || icon().size(16).line_height(LineHeight::Relative(1.)))
    }

    /// Replace the trigger chevron with custom content, such as an SVG.
    pub fn icon_content<Content: Into<Slot<'a, Event>> + 'a>(
        mut self,
        icon: impl Fn() -> Content + 'a,
    ) -> Self {
        self.icon = Some(Box::new(move || icon().into()));
        self
    }

    fn current<'s>(&'s self, state: &'s State) -> Option<&'s str> {
        self.value
            .as_deref()
            .or(state.value.as_deref())
            .or(self.default_value.as_deref())
    }

    fn list(&self, entries: &[Entry], current: Option<&str>) -> Vec<Slot<'a, Event>> {
        entries
            .iter()
            .map(|entry| match entry {
                Entry::Item {
                    value,
                    label,
                    disabled,
                } => {
                    let chosen = current == Some(value.as_str());
                    let content = self.item_content.as_ref().map_or_else(
                        || Slot::from(text::small(label.clone())),
                        |content| content(value, label),
                    );
                    let value = value.clone();

                    Slot::from(
                        menu::checkbox_item(content, chosen, move |_| Event::Choose(value.clone()))
                            .disabled(*disabled)
                            .closes(false),
                    )
                }
                Entry::Label(label) => menu::label(label.clone()),
                Entry::Separator => menu::separator(),
                Entry::Group(entries) => menu::group(self.list(entries, current)),
            })
            .collect()
    }
}

// Options use 14px text at 20px line height plus 4px vertical padding.
// Headings use 12px text at 16px line height, separators 1px with 4px margins.
fn row_center(entries: &[Entry], selected: &str, y: &mut f32) -> Option<f32> {
    for entry in entries {
        match entry {
            Entry::Item { value, .. } if value == selected => return Some(*y + 14.0),
            Entry::Item { .. } => *y += 28.0,
            Entry::Label(_) => *y += 24.0,
            Entry::Separator => *y += 9.0,
            Entry::Group(group) => {
                if let Some(center) = row_center(group, selected, y) {
                    return Some(center);
                }
            }
        }
    }
    None
}

impl<'a, Message: 'static> Component<'a, Message, Theme, Renderer> for Select<'a, Message> {
    type State = State;
    type Event = Event;

    fn update(&self, state: &mut State, event: Event, _renderer: &Renderer) -> Option<Message> {
        match event {
            Event::Toggle => {
                state.open = !state.open;
                None
            }
            Event::Set(open) => {
                state.open = open;
                None
            }
            Event::Choose(value) => {
                state.open = false;

                if self.value.is_none() {
                    state.value = Some(value.clone());
                }

                self.on_value_change
                    .as_ref()
                    .map(|on_change| on_change(value))
            }
        }
    }

    fn view(&self, state: &State) -> impl Widget<Event, Theme, Renderer> + 'a {
        let current = self.current(state);
        let chosen = current.and_then(|current| {
            self.entries
                .iter()
                .flat_map(Entry::items)
                .find(|(value, _)| *value == current)
                .map(|(value, label)| {
                    self.value_label
                        .as_ref()
                        .map_or_else(|| label.to_owned(), |f| f(value, label))
                })
        });

        let has_value = chosen.is_some();
        let value = match chosen {
            Some(label) => text::small(label),
            None => text::muted(self.placeholder.clone()),
        }
        .line_height(LineHeight::Absolute(20.into()));

        let value = if let Some(font) = self.font {
            value.font(font)
        } else {
            value
        };

        let chevron: Slot<'a, Event> = match &self.icon {
            Some(icon) => {
                let mut icon = icon();
                icon.inherit(
                    &Inherited::default()
                        .with_size(Size::Default)
                        .with_line_height(16.),
                );
                icon
            }
            None => text::muted("\u{2304}").into(),
        };
        let chevron = Container::new(chevron).style(|theme| Style {
            text_color: Some(theme.muted.foreground),
            ..Style::default()
        });
        let mut line: Slot<'a, Event> = row![Container::new(value).width(Fill), chevron]
            .spacing(6)
            .align_y(Alignment::Center)
            .into();
        line.inherit(&self.inherited.without_join());

        let variant = self.variant;
        let custom = self.trigger_paint.clone();
        let opened = state.open;
        let style: widget::StyleFn<'a, Theme> =
            Box::new(move |theme: &Theme, status: widget::Status| {
                custom.as_ref().map_or_else(
                    || {
                        let mut style: widget::Style =
                            trigger(theme, status.into(), variant).into();
                        if matches!(status, widget::Status::Focused { .. }) {
                            let color = match variant {
                                Variant::Invalid => theme.destructive.base,
                                _ => theme.neutral.ring,
                            };
                            style.border.color = color.into();
                            let mut radius = style.border.radius;
                            radius.top_left += 3.;
                            radius.top_right += 3.;
                            radius.bottom_left += 3.;
                            radius.bottom_right += 3.;
                            style.ring = Some(iced_core::Border {
                                color: color.alpha(0.5).into(),
                                width: 3.,
                                radius,
                            });
                        }
                        style
                    },
                    |paint| paint(theme, status, opened, has_value),
                )
            });

        // `h-8` (`h-7` small).
        let height = if self.size == Size::Sm { 28.0 } else { 32.0 };
        let button = Button::new(line.compose())
            .padding(
                self.inherited
                    .join()
                    .map_or(self.padding, |join| join.padding(self.padding, 1.)),
            )
            .height(height)
            .width(self.width)
            .translate(false)
            .border_box(true)
            .join(self.inherited.join())
            .disabled(self.disabled || self.inherited.is_disabled())
            .on_press(Event::Toggle)
            .style(style);

        let is_open = state.open && !self.disabled && !self.inherited.is_disabled();

        let list: Element<'a, Event, Theme, Renderer> = if is_open {
            let items =
                column::Column::with_children(self.list(&self.entries, current)).width(Fill);
            let panel = Container::new(
                scrollable(Container::new(items).width(Fill).padding(4)).width(Fill),
            )
            .width(Fill)
            .style(content);

            Slot::from(panel).compose()
        } else {
            Space::new().boxed()
        };

        // Use the horizontal side for item alignment. This makes the vertical
        // placement the anchored cross axis, which clamps to the window even
        // when the selected item is near the top or bottom of the screen.
        let trigger = button.boxed();
        let anchored = anchored(trigger, list);
        let anchored = if self.position == Position::ItemAligned && current.is_some() {
            let mut y = 4.0;
            let row = current
                .and_then(|value| row_center(&self.entries, value, &mut y))
                .unwrap_or(14.0);
            anchored
                .side(anchored::Side::Right)
                .at([0.0, height / 2.0])
                .align(anchored::Align::Start)
                .align_offset(-row)
                .flip(false)
        } else {
            anchored.side(self.side).align(self.align).offset(4)
        };
        anchored
            .width(anchored::Width::AtLeastAnchor)
            .min_width(self.min_width)
            .modal(true)
            .open(is_open)
            .on_toggle(Event::Set)
    }
}

impl<'a, Message: 'static> Compose<Message> for Select<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.inherited = self.inherited.or(*from);
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        component::component(*self).boxed()
    }
}

impl<'a, Message: 'static> From<Select<'a, Message>> for Slot<'a, Message> {
    fn from(select: Select<'a, Message>) -> Self {
        Slot::new(select)
    }
}

impl<'a, Message: 'static> From<Select<'a, Message>> for Element<'a, Message, Theme, Renderer> {
    fn from(select: Select<'a, Message>) -> Self {
        component::component(select).boxed()
    }
}
