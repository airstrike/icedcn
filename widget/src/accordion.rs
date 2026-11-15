//! Stacked disclosures with positional or stable keyed selection.
//!
//! Use the accordion macro for fixed children and the accordion function for iterators.
//! Single selection uses Option<Key> in controlled values and change handlers.
//! Call .multiple() before selection builders to use a complete open set.
//! Dynamic records use keyed with (key, item) pairs.

#[cfg(feature = "svg")]
mod chevron;
pub(crate) mod element;
pub mod widget;

use widget::Mode;
pub use widget::{Catalog, StyleFn};

use crate::compose::Compose;
use std::marker::PhantomData;

use iced_core::Padding;
#[cfg(not(feature = "svg"))]
use iced_core::text::Renderer as _;
use iced_widget::Renderer;
use icedcn_core::Theme;
use icedcn_core::theme;

use crate::{Inherited, Slot, Weight};

/// Builder for one disclosure item. Constructed via [`item`].
pub struct Item<'a, Message> {
    trigger: Slot<'a, Message>,
    chevron_closed: Option<Slot<'a, Message>>,
    chevron_open: Option<Slot<'a, Message>>,
    body: Slot<'a, Message>,
    disabled: bool,
    inset: f32,
}

impl<'a, Message: 'a> Item<'a, Message> {
    /// Replace the body content.
    pub fn content(mut self, content: impl Into<Slot<'a, Message>>) -> Self {
        self.body = content.into();
        self
    }

    /// Override the default downward chevron.
    pub fn chevron_closed(mut self, icon: impl Into<Slot<'a, Message>>) -> Self {
        self.chevron_closed = Some(icon.into());
        self
    }

    /// Override the default upward chevron.
    pub fn chevron_open(mut self, icon: impl Into<Slot<'a, Message>>) -> Self {
        self.chevron_open = Some(icon.into());
        self
    }

    /// Inset the trigger and content horizontally while keeping the divider full width.
    pub fn inset(mut self, pixels: f32) -> Self {
        self.inset = pixels.max(0.0);
        self
    }

    /// Disable and dim this item's trigger. Its open content remains interactive.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Construct an accordion item from its trigger and body.
pub fn item<'a, Message: 'a>(
    trigger: impl Into<Slot<'a, Message>>,
    body: impl Into<Slot<'a, Message>>,
) -> Item<'a, Message> {
    Item {
        trigger: trigger.into(),
        chevron_closed: None,
        chevron_open: None,
        body: body.into(),
        disabled: false,
        inset: 0.0,
    }
}

impl<'a, Message: 'a, Trigger, Body> From<(Trigger, Body)> for Item<'a, Message>
where
    Trigger: Into<Slot<'a, Message>>,
    Body: Into<Slot<'a, Message>>,
{
    fn from((trigger, body): (Trigger, Body)) -> Self {
        item(trigger, body)
    }
}

/// At most one open item.
pub struct Single;

/// Any number of open items.
pub struct Multiple;

/// Item positions identify selection and child state.
pub struct Positional;

/// Caller-supplied keys identify selection and child state.
pub struct Keyed;

/// Construct a positional accordion with literal items.
#[macro_export]
macro_rules! accordion {
    () => { $crate::accordion::accordion(::std::iter::empty::<$crate::accordion::Item<'_, _>>()) };
    ($first:expr $(, $rest:expr)* $(,)?) => {
        $crate::accordion::accordion([$crate::accordion::Item::from($first)])$(.push($rest))*
    };
}

/// Top-level builder for the accordion widget.
pub struct Accordion<'a, Message, Key = usize, Selection = Single, Identity = Positional> {
    items: Vec<(Key, Item<'a, Message>)>,
    mode: Mode,
    value: Option<Vec<Key>>,
    default_value: Vec<Key>,
    on_value_change: Option<Box<dyn Fn(Vec<Key>) -> Message + 'a>>,
    style: Option<theme::accordion::StyleFn<'a, Theme>>,
    outer_padding: Padding,
    selection: PhantomData<(Selection, Identity)>,
}

/// Construct a positional accordion. Values are zero-based item positions.
pub fn accordion<'a, Message: 'a>(
    items: impl IntoIterator<Item = impl Into<Item<'a, Message>>>,
) -> Accordion<'a, Message> {
    Accordion::from_items(items.into_iter().enumerate())
}

/// Construct an accordion whose selection and entire child trees follow stable keys.
/// Keys must be unique. Strings, integer IDs and domain enums retain their own types.
pub fn keyed<'a, Message: 'a, Key: Clone + PartialEq + 'static>(
    items: impl IntoIterator<Item = (Key, impl Into<Item<'a, Message>>)>,
) -> Accordion<'a, Message, Key, Single, Keyed> {
    Accordion::from_items(items)
}

impl<'a, Message: 'a, Key, Identity> Accordion<'a, Message, Key, Single, Identity> {
    fn from_items(items: impl IntoIterator<Item = (Key, impl Into<Item<'a, Message>>)>) -> Self {
        Self {
            items: items
                .into_iter()
                .map(|(key, item)| (key, item.into()))
                .collect(),
            mode: Mode::Single { collapsible: true },
            value: None,
            default_value: Vec::new(),
            on_value_change: None,
            style: None,
            outer_padding: Padding::ZERO,
            selection: PhantomData,
        }
    }
}

impl<'a, Message: 'a, Selection> Accordion<'a, Message, usize, Selection, Positional> {
    /// Append an item at the next position.
    pub fn push(mut self, item: impl Into<Item<'a, Message>>) -> Self {
        self.items.push((self.items.len(), item.into()));
        self
    }

    /// Append items in iteration order.
    pub fn extend(self, items: impl IntoIterator<Item = impl Into<Item<'a, Message>>>) -> Self {
        items
            .into_iter()
            .fold(self, |accordion, item| accordion.push(item))
    }
}

impl<'a, Message: 'a, Key, Selection> Accordion<'a, Message, Key, Selection, Keyed> {
    /// Append a keyed item.
    pub fn push(mut self, key: Key, item: impl Into<Item<'a, Message>>) -> Self {
        self.items.push((key, item.into()));
        self
    }

    /// Append keyed items in iteration order.
    pub fn extend(
        self,
        items: impl IntoIterator<Item = (Key, impl Into<Item<'a, Message>>)>,
    ) -> Self {
        items
            .into_iter()
            .fold(self, |accordion, (key, item)| accordion.push(key, item))
    }
}

impl<'a, Message: 'a, Key: 'a, Identity> Accordion<'a, Message, Key, Single, Identity> {
    /// Control the open item. None closes all items.
    pub fn value(mut self, value: Option<Key>) -> Self {
        self.value = Some(value.into_iter().collect());
        self
    }

    /// Seed one open item in an uncontrolled accordion.
    pub fn default_value(mut self, value: Key) -> Self {
        self.default_value = vec![value];
        self
    }

    /// Receive the new open key, or None when the open item closes.
    pub fn on_value_change(mut self, on_change: impl Fn(Option<Key>) -> Message + 'a) -> Self {
        self.on_value_change = Some(Box::new(move |values| on_change(values.into_iter().next())));
        self
    }

    /// Allow the open trigger to close itself. Defaults to true.
    pub fn collapsible(mut self, collapsible: bool) -> Self {
        self.mode = Mode::Single { collapsible };
        self
    }

    /// Allow several items to stay open. Configure change handlers after this call.
    /// An existing single-selection handler is cleared.
    pub fn multiple(self) -> Accordion<'a, Message, Key, Multiple, Identity> {
        Accordion {
            items: self.items,
            mode: Mode::Multiple,
            value: self.value,
            default_value: self.default_value,
            on_value_change: None,
            style: self.style,
            outer_padding: self.outer_padding,
            selection: PhantomData,
        }
    }
}

impl<'a, Message: 'a, Key, Identity> Accordion<'a, Message, Key, Multiple, Identity> {
    /// Control the complete open set. Repeated keys are ignored.
    pub fn value(mut self, value: impl IntoIterator<Item = Key>) -> Self {
        self.value = Some(value.into_iter().collect());
        self
    }

    /// Seed an uncontrolled open set. Later rebuilds preserve interaction state.
    pub fn default_value(mut self, value: impl IntoIterator<Item = Key>) -> Self {
        self.default_value = value.into_iter().collect();
        self
    }

    /// Receive the complete new open set after an accepted toggle.
    pub fn on_value_change(mut self, on_change: impl Fn(Vec<Key>) -> Message + 'a) -> Self {
        self.on_value_change = Some(Box::new(on_change));
        self
    }
}

impl<'a, Message: 'a, Key, Selection, Identity> Accordion<'a, Message, Key, Selection, Identity> {
    /// Override the accordion's style. Accepts our OKLCH-native variants
    /// (`Default`, `Card`) or any `Spec`.
    pub fn style(mut self, style: impl Into<theme::accordion::StyleFn<'a, Theme>>) -> Self {
        self.style = Some(style.into());
        self
    }

    /// Override outer padding. Defaults to zero — the wrapping container
    /// (e.g. card) is responsible for outer breathing room. Surfaced as
    /// a knob so the `borders` example can supply `px-4`.
    pub fn padding(mut self, padding: impl Into<icedcn_core::Padding>) -> Self {
        let padding: icedcn_core::Padding = padding.into();
        self.outer_padding = padding.into();
        self
    }
}

#[cfg(not(feature = "svg"))]
fn chevron<'a, Message: 'a>(open: bool) -> Slot<'a, Message> {
    let icon = if open {
        Renderer::SCROLL_UP_ICON
    } else {
        Renderer::SCROLL_DOWN_ICON
    };
    iced_widget::text(icon.to_string())
        .font(Renderer::ICON_FONT)
        .size(10)
        .width(16)
        .height(16)
        .align_x(iced_core::Alignment::Center)
        .align_y(iced_core::Alignment::Center)
        .into()
}

#[cfg(feature = "svg")]
fn chevron<'a, Message: 'a>(open: bool) -> Slot<'a, Message> {
    use iced_widget::Widget;
    chevron::Icon::new(open).boxed().into()
}

impl<'a, Message: 'a, Key: Clone + PartialEq + 'static, Selection: 'a, Identity: 'a>
    From<Accordion<'a, Message, Key, Selection, Identity>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(a: Accordion<'a, Message, Key, Selection, Identity>) -> Self {
        let label = Inherited::default()
            .with_size(14)
            .with_weight(Weight::Medium)
            .with_line_height(20.0)
            .with_selectable(false);
        let content = Inherited::default().with_size(14).with_line_height(20.0);
        let items = a
            .items
            .into_iter()
            .map(|(id, mut it)| {
                it.trigger.inherit(&label.with_disabled(it.disabled));
                let mut body = it.body;
                body.inherit(&content);
                widget::Item {
                    id,
                    trigger: it.trigger.compose(),
                    chevron_closed: it
                        .chevron_closed
                        .unwrap_or_else(|| chevron(false))
                        .compose(),
                    chevron_open: it.chevron_open.unwrap_or_else(|| chevron(true)).compose(),
                    body: body.compose(),
                    disabled: it.disabled,
                    inset: it.inset,
                }
            })
            .collect();

        // Resolve the styling class. Per accordion's element.rs comment,
        // the widget's Style IS core's accordion::Style — no `from_core`
        // bridge needed; the core StyleFn is the widget StyleFn.
        let class: <Theme as widget::Catalog>::Class<'a> = match a.style {
            Some(style_fn) => style_fn,
            None => <Theme as widget::Catalog>::default(),
        };

        let is_controlled = a.value.is_some();
        let widget = widget::Accordion {
            items,
            mode: a.mode,
            open: a.value.unwrap_or(a.default_value),
            is_controlled,
            on_value_change: a.on_value_change,
            class,
            outer_padding: a.outer_padding,
        };
        widget.into()
    }
}

impl<'a, Message: 'a, Key: Clone + PartialEq + 'static, Selection: 'a, Identity: 'a>
    From<Accordion<'a, Message, Key, Selection, Identity>> for Slot<'a, Message>
{
    fn from(widget: Accordion<'a, Message, Key, Selection, Identity>) -> Self {
        crate::Slot::new(widget)
    }
}

impl<'a, Message: 'a, Key: Clone + PartialEq + 'static, Selection: 'a, Identity: 'a>
    Compose<Message> for Accordion<'a, Message, Key, Selection, Identity>
{
    fn inherit(&mut self, from: &Inherited) {
        for (_, item) in &mut self.items {
            item.trigger.inherit(from);
            item.disabled |= from.is_disabled();
            item.body.inherit(from);
        }
    }

    fn compose<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        (*self).into()
    }
}
