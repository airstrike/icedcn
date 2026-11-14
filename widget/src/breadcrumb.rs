//! Breadcrumb navigation — list-of-items API with auto separator interspersing.
//!
//! Mirrors shadcn's `<Breadcrumb><BreadcrumbList>...</BreadcrumbList></Breadcrumb>`
//! composition. The shadcn API exposes seven primitives (`Breadcrumb`,
//! `BreadcrumbList`, `BreadcrumbItem`, `BreadcrumbLink`, `BreadcrumbPage`,
//! `BreadcrumbSeparator`, `BreadcrumbEllipsis`); we collapse the JSX nesting
//! into a flat list of [`Item`] values plus an auto-interspersed separator.
//!
//! ```ignore
//! use icedxdy::widget::breadcrumb;
//!
//! breadcrumb([
//!     breadcrumb::link("Home", Message::GoHome),
//!     breadcrumb::link("Components", Message::GoComponents),
//!     breadcrumb::page("Breadcrumb"),
//! ])
//! ```
//!
//! Item kinds:
//!
//! - [`item`] — custom content, such as a dropdown menu, inside the trail.
//! - [`link`] — clickable. Cursor changes to `Pointer`; emits the supplied
//!   message on left-button release. Mirrors `<BreadcrumbLink>`.
//! - [`page`] — current page. Renders content as-is, no click affordance.
//!   Mirrors `<BreadcrumbPage>`.
//! - [`ellipsis`] — collapsed-items indicator. Pass
//!   `Some(message)` to make it clickable (e.g. to open a dropdown of the
//!   elided crumbs); pass `None` for a purely visual placeholder.
//!   Mirrors `<BreadcrumbEllipsis>`.
//!
//! # Color cascade
//!
//! Mirrors shadcn: `<BreadcrumbList>` applies `text-muted-foreground` and
//! `<BreadcrumbPage>` overrides back to `text-foreground`. Internally we
//! wrap the row in a Container with the muted text-color cascade and wrap
//! page items in an inner Container with foreground — so unstyled children
//! (separator icons, raw-string links) inherit muted at draw time without
//! callers threading colors manually.
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
use icedxdy_core::Theme;

/// One entry in a breadcrumb trail.
///
/// Public via the `breadcrumb::Item` namespace path (per RUST_STYLE
/// no-composite-names: callers write `breadcrumb::Item`, never
/// `BreadcrumbItem`). Construct via the [`item`], [`link`], [`page`], and [`ellipsis`]
/// helpers rather than the variants directly.
pub enum Item<'a, Message> {
    /// Custom content that participates in the trail as a breadcrumb item.
    Content(Slot<'a, Message>),
    /// A clickable crumb. Mirrors `<BreadcrumbLink>`.
    Link(Slot<'a, Message>, Message),
    /// The current page. Mirrors `<BreadcrumbPage>`.
    Page(Slot<'a, Message>),
    /// An "..." indicator standing in for elided intermediate crumbs.
    /// Mirrors `<BreadcrumbEllipsis>`.
    Ellipsis(Option<Message>),
}

/// Place a widget in the trail, preserving its own interactions and styling.
pub fn item<'a, Message: 'a>(content: impl Into<Slot<'a, Message>>) -> Item<'a, Message> {
    Item::Content(content.into())
}

/// Build a clickable [`Item::Link`] crumb.
pub fn link<'a, Message: 'a>(
    text: impl Into<Slot<'a, Message>>,
    on_press: Message,
) -> Item<'a, Message> {
    Item::Link(text.into(), on_press)
}

/// Build a non-clickable [`Item::Page`] crumb (the current page).
pub fn page<'a, Message: 'a>(text: impl Into<Slot<'a, Message>>) -> Item<'a, Message> {
    Item::Page(text.into())
}

/// Build an [`Item::Ellipsis`] crumb. Pass `Some(message)` to make it
/// clickable, `None` for a static visual placeholder.
pub fn ellipsis<'a, Message: 'a>(on_press: Option<Message>) -> Item<'a, Message> {
    Item::Ellipsis(on_press)
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
        // shadcn `<BreadcrumbList>` uses `gap-1.5` (6px) between adjacent
        // children (items + separators).
        let mut row = Row::new().spacing(6.0).wrap();

        for (idx, item) in items.into_iter().enumerate() {
            row = row.push(render_item(item, &inherited));
            if idx + 1 < item_count {
                let mut separator = make_separator();
                separator.inherit(&inherited);
                row = row.push(separator);
            }
        }

        // shadcn `<BreadcrumbList>` cascades `text-muted-foreground` to
        // unstyled descendants; Container's text_color is iced's only
        // mechanism for the same. `<BreadcrumbPage>` overrides in `render_page`.
        container(Box::new(row).compose())
            .style(|t: &Theme| Style::default().color(t.muted.foreground))
            .boxed()
    }
}

/// Render a single [`Item`] to a [`Slot`]. Extracted so the dispatch
/// reads as a flat match in one place.
fn render_item<'a, Message: Clone + 'a>(
    item: Item<'a, Message>,
    inherited: &Inherited,
) -> Slot<'a, Message> {
    match item {
        Item::Content(mut content) => {
            content.inherit(inherited);
            content
        }
        Item::Link(content, on_press) => render_link(content, on_press, inherited),
        Item::Page(mut content) => {
            content.inherit(inherited);
            render_page(content)
        }
        Item::Ellipsis(on_press) => render_ellipsis(on_press, inherited),
    }
}

/// Render the current-page crumb. shadcn's `<BreadcrumbPage>` wraps the
/// label in `text-foreground` to override the list's muted cascade.
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
