//! Breadcrumb navigation from a list of [`Item`] values. The widget inserts separators between
//! the items.
//!
//! ```ignore
//! use icedcn::widget::breadcrumb;
//!
//! breadcrumb([
//!     breadcrumb::link("Home", Message::GoHome),
//!     breadcrumb::link("Components", Message::GoComponents),
//!     breadcrumb::page("Breadcrumb"),
//! ])
//! ```
//!
//! Use [`link`] for an earlier page and [`page`] for the current one. Use [`item`] for custom
//! content such as a menu. [`ellipsis`] accepts an optional message for revealing omitted
//! pages.
//!
//! # Color cascade
//!
//! Items and separators inherit the muted text color. The current page uses the foreground
//! color.
//!
//! Links use the native button's keyboard focus, release activation and
//! animated hover color. Disabled ancestors disable every interactive item.
//!
//! # Separator
//!
//! With the `svg` feature, the default chevron and ellipsis use Lucide paths
//! without an icon font. Without SVG, text glyphs occupy the same boxes.
//! Use [`Breadcrumb::separator`] for a custom separator.

#[cfg(feature = "svg")]
mod icon;

use crate::Weight;
use crate::button::widget;
use crate::compose::{Compose, Inherited, Slot};
use crate::row::Row;
#[cfg(not(feature = "svg"))]
use crate::text;
#[cfg(not(feature = "svg"))]
use crate::text::Text;
use iced_core::border;
use iced_core::mouse::Interaction;
use iced_widget::Widget;
use iced_widget::container::Style;
use iced_widget::{Renderer, container};
use icedcn_core::Theme;

/// A composable breadcrumb part, usable in a trail or as another widget's child.
///
/// Construct with [`item`], [`link`], [`page`] or [`ellipsis`]. Parts preserve
/// inherited typography and disabling until their Compose boundary.
pub struct Item<'a, Message> {
    kind: Kind<'a, Message>,
    inherited: Inherited,
}

enum Kind<'a, Message> {
    Content(Slot<'a, Message>),
    Link(Slot<'a, Message>, Message),
    Page(Slot<'a, Message>),
    Ellipsis(Option<Message>),
}

impl<'a, Message> Item<'a, Message> {
    fn new(kind: Kind<'a, Message>) -> Self {
        Self {
            kind,
            inherited: Inherited::default(),
        }
    }
}

impl<'a, Message: Clone + 'a> Compose<Message> for Item<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.inherited = self.inherited.or(*from);
    }

    fn compose<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        render_item(self.kind, &self.inherited).compose()
    }
}

impl<'a, Message: Clone + 'a> From<Item<'a, Message>> for Slot<'a, Message> {
    fn from(item: Item<'a, Message>) -> Self {
        Slot::new(item)
    }
}

impl<'a, Message: Clone + 'a> From<Item<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(item: Item<'a, Message>) -> Self {
        Box::new(item).compose()
    }
}

/// Place a widget in the trail, preserving its own interactions and styling.
pub fn item<'a, Message: 'a>(content: impl Into<Slot<'a, Message>>) -> Item<'a, Message> {
    Item::new(Kind::Content(content.into()))
}

/// Build a clickable breadcrumb part.
pub fn link<'a, Message: 'a>(
    text: impl Into<Slot<'a, Message>>,
    on_press: Message,
) -> Item<'a, Message> {
    Item::new(Kind::Link(text.into(), on_press))
}

/// Build a non-clickable current-page part.
pub fn page<'a, Message: 'a>(text: impl Into<Slot<'a, Message>>) -> Item<'a, Message> {
    Item::new(Kind::Page(text.into()))
}

/// Build an ellipsis part. Pass `Some(message)` to make it
/// clickable, `None` for a static visual placeholder. The static part also
/// composes inside a button: `button(breadcrumb::ellipsis(None))`.
pub fn ellipsis<'a, Message: 'a>(on_press: Option<Message>) -> Item<'a, Message> {
    Item::new(Kind::Ellipsis(on_press))
}

/// Build a [`Breadcrumb`] with a muted chevron between adjacent items.
pub fn breadcrumb<'a, Message: 'a>(
    items: impl IntoIterator<Item = Item<'a, Message>>,
) -> Breadcrumb<'a, Message> {
    Breadcrumb::new(items)
}

/// Lazily-evaluated separator factory.
///
/// `Slot` is not `Clone`, so the separator must be re-built once per
/// gap rather than stored once and copied. The factory closure is invoked
/// `items.len() - 1` times at `compose` time.
type SeparatorFn<'a, Message> = Box<dyn Fn() -> Slot<'a, Message> + 'a>;

/// A breadcrumb trail. Build via [`breadcrumb`].
pub struct Breadcrumb<'a, Message> {
    items: Vec<Item<'a, Message>>,
    separator: Option<SeparatorFn<'a, Message>>,
    inherited: Inherited,
}

impl<'a, Message: 'a> Breadcrumb<'a, Message> {
    /// Build a breadcrumb from an iterator of items. Prefer [`breadcrumb`]
    /// at call sites for symmetry with other helpers.
    pub fn new(items: impl IntoIterator<Item = Item<'a, Message>>) -> Self {
        Self {
            items: items.into_iter().collect(),
            separator: None,
            inherited: Inherited::default(),
        }
    }

    /// Override the default separator. The closure is called once per gap
    /// between adjacent items (Slot values are not `Clone`, so a fresh
    /// instance is built for each insertion).
    ///
    /// ```ignore
    /// breadcrumb([...])
    ///     .separator(|| icon::chevron_right().size(14).into())
    /// ```
    pub fn separator<F>(mut self, factory: F) -> Self
    where
        F: Fn() -> Slot<'a, Message> + 'a,
    {
        self.separator = Some(Box::new(factory));
        self
    }
}

impl<'a, Message: Clone + 'a> From<Breadcrumb<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(b: Breadcrumb<'a, Message>) -> Self {
        Box::new(b).compose()
    }
}

impl<'a, Message: Clone + 'a> From<Breadcrumb<'a, Message>> for Slot<'a, Message> {
    fn from(widget: Breadcrumb<'a, Message>) -> Self {
        Slot::new(widget)
    }
}

impl<'a, Message: Clone + 'a> Compose<Message> for Breadcrumb<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.inherited = self.inherited.or(*from);
    }

    fn compose<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let Breadcrumb {
            items,
            separator,
            inherited,
        } = *self;
        let inherited = inherited.or(Inherited::default()
            .with_size(14.0)
            .with_line_height(20.0)
            .with_weight(Weight::Normal));

        let make_separator: SeparatorFn<'a, Message> =
            separator.unwrap_or_else(|| Box::new(default_separator));

        let item_count = items.len();
        let mut row = Row::new().spacing(6.0).wrap();

        for (idx, mut item) in items.into_iter().enumerate() {
            item.inherit(&inherited);
            row = row.push(item.into());
            if idx + 1 < item_count {
                let mut separator = make_separator();
                separator.inherit(&inherited);
                row = row.push(separator);
            }
        }

        // Set the inherited color once. The current page overrides it in `render_page`.
        container(Box::new(row).compose())
            .style(|t: &Theme| Style::default().color(t.muted.foreground))
            .boxed()
    }
}

/// Render a single [`Item`] to a [`Slot`]. Extracted so the dispatch
/// reads as a flat match in one place.
fn render_item<'a, Message: Clone + 'a>(
    item: Kind<'a, Message>,
    inherited: &Inherited,
) -> Slot<'a, Message> {
    match item {
        Kind::Content(mut content) => {
            content.inherit(inherited);
            content
        }
        Kind::Link(content, on_press) => render_link(content, on_press, inherited),
        Kind::Page(mut content) => {
            content.inherit(inherited);
            render_page(content)
        }
        Kind::Ellipsis(on_press) => render_ellipsis(on_press, inherited),
    }
}

/// Render the current page in the foreground color.
fn render_page<'a, Message: Clone + 'a>(content: Slot<'a, Message>) -> Slot<'a, Message> {
    container(content.compose())
        .style(|t: &Theme| Style::default().color(t.neutral.foreground))
        .into()
}

/// The existing native button owns release, cancellation and keyboard input.
fn render_link<'a, Message: Clone + 'a>(
    mut content: Slot<'a, Message>,
    on_press: Message,
    inherited: &Inherited,
) -> Slot<'a, Message> {
    content.inherit(&Inherited::default().with_selectable(false).or(*inherited));
    let style: widget::StyleFn<'a, Theme> = Box::new(link_style);
    widget::Button::new(content.compose())
        .padding(0)
        .translate(false)
        .disabled(inherited.is_disabled())
        .interaction(Interaction::Pointer)
        .on_press(on_press)
        .style(style)
        .boxed()
        .into()
}

fn link_style(theme: &Theme, status: widget::Status) -> widget::Style {
    let hovered = matches!(
        status,
        widget::Status::Hovered
            | widget::Status::Pressed
            | widget::Status::Focused { is_hovered: true }
    );
    let color = if hovered {
        theme.neutral.foreground
    } else {
        theme.muted.foreground
    };
    widget::Style {
        text_color: if status == widget::Status::Disabled {
            color.alpha(0.5).into()
        } else {
            color.into()
        },
        ring: matches!(status, widget::Status::Focused { .. })
            .then(|| border::width(2).color(theme.neutral.foreground).rounded(2)),
        ..widget::Style::default()
    }
}

/// Render an ellipsis indicator. `on_press = Some(_)` makes it clickable.
///
/// A 16px Lucide icon inside the source's 20px square; the text fallback
/// retains that square when the SVG feature is not enabled.
fn render_ellipsis<'a, Message: Clone + 'a>(
    on_press: Option<Message>,
    inherited: &Inherited,
) -> Slot<'a, Message> {
    #[cfg(feature = "svg")]
    let glyph: Slot<'a, Message> = icon::Icon::Ellipsis.boxed().into();
    #[cfg(not(feature = "svg"))]
    let glyph: Slot<'a, Message> = Text::new("\u{2026}").size(text::size::SM).into();
    let glyph = container(glyph.compose()).center_x(20).center_y(20).into();

    match on_press {
        Some(message) => render_link(glyph, message, inherited),
        None => glyph,
    }
}

/// Default 14px chevron. Its color comes from the list's muted cascade.
fn default_separator<'a, Message: 'a>() -> Slot<'a, Message> {
    #[cfg(feature = "svg")]
    {
        icon::Icon::Chevron.boxed().into()
    }
    #[cfg(not(feature = "svg"))]
    {
        container(Slot::<Message>::from(Text::new("\u{203A}").size(text::size::SM)).compose())
            .center_x(14)
            .center_y(14)
            .boxed()
            .into()
    }
}
