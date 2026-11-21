//! Theme-aware rich-text widget.
//!
//! Build inline runs of differently-styled text without resorting to a
//! `row![text, text]` hack. Each [`Span`] carries an optional theme-aware
//! color [`Token`] that is resolved against the active palette at draw time
//! — so the same widget renders correctly in light and dark modes without
//! the user ever touching a concrete `Color`.
//!
//! # Example
//!
//! ```ignore
//! use icedcn::rich_text;
//! use icedcn::widget::{rich_text::span, text};
//! use icedcn::theme::tweak::Token;
//!
//! rich_text![
//!     span("Required Field ").size(text::size::SM).medium(),
//!     span("*").size(text::size::SM).color(Token::Destructive),
//! ]
//! ```
//!
//! # Implementation
//!
//! Iced 0.15's stock `iced_widget::text::Rich` takes per-span concrete
//! `Color` at construction time. Per-span theme-resolved colors require
//! reaching `&Theme` after the spans were built, which the stock widget
//! doesn't expose. We vendor the minimal `Rich::layout` / `Rich::draw`
//! algorithm here and resolve `Token` → `Color` at draw time, rebuilding
//! the cached paragraph when the palette changes.

use crate::text::leading;
use std::cell::RefCell;
use std::ops::Add;

use crate::core::alignment;
use crate::core::font;
use crate::core::layout;
use crate::core::mouse;
use crate::core::renderer;
use crate::core::text;
use crate::core::text::paragraph::Paragraph as _;
use crate::core::widget::text::{Alignment, Catalog, Shaping, Wrapping};
use crate::core::widget::tree::{self, Tree};
use crate::core::widget::{Meta, Operation, operation};
use crate::core::{
    Border, Color, Element, Em, Layout, Length, Padding, Pixels, Point, Rectangle, Size, Text,
    Vector, Widget,
};
use iced_widget::Renderer;
use icedcn_core::Theme;
use icedcn_core::theme::tweak::Token;

use crate::compose::{Compose, Inherited, Slot};

/// Local alias for iced's text fragment so callers don't have to type
/// the iced-core path inline.
type Fragment<'a> = text::Fragment<'a>;

/// One inline run of text inside a [`RichText`].
///
/// Mirrors the subset of iced's text `Span` we need, plus a
/// [`Token`]-typed color that resolves against the active theme palette
/// at draw time. Underline, strikethrough and link semantics are punted
/// for now — no current call site needs them.
#[derive(Debug, Clone)]
pub struct Span<'a> {
    fragment: Fragment<'a>,
    size: Option<Pixels>,
    line_height: Option<text::LineHeight>,
    weight: Option<font::Weight>,
    font: Option<iced_core::Font>,
    italic: bool,
    color: Option<(Token, Token)>,
    letter_spacing: Option<Em>,
    padding: Padding,
    background: Option<Token>,
    border: Option<Border>,
}

/// Create a [`Span`] with the given text fragment.
pub fn span<'a>(fragment: impl text::IntoFragment<'a>) -> Span<'a> {
    Span {
        fragment: fragment.into_fragment(),
        size: None,
        line_height: None,
        weight: None,
        font: None,
        italic: false,
        color: None,
        letter_spacing: None,
        padding: Padding::ZERO,
        background: None,
        border: None,
    }
}

impl<'a> Span<'a> {
    /// Set the font size of this span.
    pub fn size(mut self, size: impl Into<crate::Size>) -> Self {
        self.size = Some(Pixels(size.into().text_size()));
        self
    }

    /// Override the font for this run, for example for inline code.
    pub fn font(mut self, font: impl Into<iced_core::Font>) -> Self {
        self.font = Some(font.into());
        self
    }

    /// Set the font weight of this span.
    pub fn weight(mut self, weight: impl Into<crate::Weight>) -> Self {
        self.weight = Some(weight.into().into());
        self
    }

    /// Shorthand for [`Self::weight`] with [`Weight::Medium`].
    pub fn medium(self) -> Self {
        self.weight(crate::Weight::Medium)
    }

    /// Shorthand for [`Self::weight`] with [`Weight::Semibold`].
    pub fn semibold(self) -> Self {
        self.weight(crate::Weight::Semibold)
    }

    /// Shorthand for [`Self::weight`] with [`Weight::Bold`].
    pub fn bold(self) -> Self {
        self.weight(crate::Weight::Bold)
    }

    /// Render this span in italic.
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    /// Override the line height of this span.
    pub fn line_height(mut self, line_height: impl Into<text::LineHeight>) -> Self {
        self.line_height = Some(line_height.into());
        self
    }

    /// Override the letter spacing of this span.
    pub fn letter_spacing(mut self, letter_spacing: impl Into<Em>) -> Self {
        self.letter_spacing = Some(letter_spacing.into());
        self
    }

    /// Set native span padding. Horizontal padding participates in shaping;
    /// vertical padding extends the highlight above and below the line box.
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    /// Paint a theme-aware background behind this span's wrapped regions.
    pub fn background(mut self, background: impl Into<Token>) -> Self {
        self.background = Some(background.into());
        self
    }

    /// Set the native border of the span highlight.
    pub fn border(mut self, border: impl Into<Border>) -> Self {
        self.border = Some(border.into());
        self
    }

    /// Apply a theme-aware color to this span.
    ///
    /// Resolves against the active palette at draw time, so the same
    /// span renders correctly in both light and dark modes.
    pub fn color(mut self, color: impl Into<Token>) -> Self {
        let color = color.into();
        self.color = Some((color, color));
        self
    }

    /// Use different span colors for light and dark themes.
    pub fn color_pair(mut self, light: impl Into<Token>, dark: impl Into<Token>) -> Self {
        self.color = Some((light.into(), dark.into()));
        self
    }

    /// Set the size iff this span has none.
    fn fallback_size(mut self, size: f32) -> Self {
        if self.size.is_none() {
            self.size = Some(Pixels(size));
        }
        self
    }

    /// Set the weight iff this span has none.
    fn fallback_weight(mut self, weight: font::Weight) -> Self {
        if self.weight.is_none() {
            self.weight = Some(weight);
        }
        self
    }

    /// Build the iced span used for *layout* (color stripped — color is a
    /// rendering concern and including it would needlessly invalidate the
    /// paragraph cache when the palette changes between draws).
    fn to_layout_span(&self, default_font: iced_core::Font) -> text::Span<'static, ()> {
        let mut font = self.font;
        if self.italic {
            font = Some(font.unwrap_or(default_font).style(font::Style::Italic));
        }
        if let Some(weight) = self.weight {
            let base = font.unwrap_or(default_font);
            font = Some(base.weight(weight));
        }
        let mut span = text::Span::new(self.fragment.clone().into_owned()).padding(self.padding);
        if let Some(size) = self.size {
            span = span.size(size);
        }
        if let Some(lh) = self.line_height {
            span = span.line_height(lh);
        }
        if let Some(font) = font {
            span = span.font(font);
        }
        if let Some(letter_spacing) = self.letter_spacing {
            span = span.letter_spacing(letter_spacing);
        }
        span
    }
}

// `&str` / `String` ergonomics: `rich_text![span("..."), "literal"]` works.
impl<'a> From<&'a str> for Span<'a> {
    fn from(s: &'a str) -> Self {
        span(s)
    }
}

impl From<String> for Span<'_> {
    fn from(s: String) -> Self {
        span(s)
    }
}

// Compose with a `Token` via `+` — `span("hi") + Token::Destructive`.
impl<'a> Add<Token> for Span<'a> {
    type Output = Span<'a>;
    fn add(self, rhs: Token) -> Span<'a> {
        self.color(rhs)
    }
}

/// A bunch of inline [`Span`]s rendered as a single text widget.
///
/// Theme-aware: each span's [`Token`] color is resolved against the active
/// palette at draw time, so swapping themes re-colors spans correctly.
pub struct RichText<'a> {
    font: Option<iced_core::Font>,
    spans: Vec<Span<'a>>,
    size: Option<Pixels>,
    line_height: Option<text::LineHeight>,
    /// The leading of the `text-*` step or the cascade; `line_height` wins.
    leading: Option<text::LineHeight>,
    width: Length,
    height: Length,
    align_x: Alignment,
    align_y: alignment::Vertical,
    selectable: Option<bool>,
    wrapping: Wrapping,
    letter_spacing: Em,
    /// Default color applied to spans that lack their own `color`. Driven
    /// by the catalog (so call sites can `style(theme::text::Foreground)`
    /// to set a baseline) — but most sites don't need it.
    class: <Theme as Catalog>::Class<'a>,
}

/// Create a [`RichText`] from an iterator of spans.
pub fn rich_text<'a>(spans: impl IntoIterator<Item = Span<'a>>) -> RichText<'a> {
    RichText {
        font: None,
        spans: spans.into_iter().collect(),
        size: None,
        line_height: None,
        leading: None,
        width: Length::Shrink,
        height: Length::Shrink,
        align_x: Alignment::Default,
        align_y: alignment::Vertical::Top,
        selectable: None,
        wrapping: Wrapping::default(),
        letter_spacing: Em::default(),
        class: <Theme as Catalog>::default(),
    }
}

impl<'a> RichText<'a> {
    fn format<R: text::Renderer>(&self, renderer: &R, bounds: Size) -> Text<()> {
        Text {
            content: (),
            bounds,
            size: self.size.unwrap_or_else(|| renderer.text_size()),
            font: self.font.unwrap_or_else(|| text::Renderer::font(renderer)),
            line_height: self.line_height.or(self.leading).unwrap_or(leading::NORMAL),
            align_x: self.align_x,
            align_y: self.align_y,
            shaping: Shaping::Advanced,
            wrapping: self.wrapping,
            ellipsis: text::Ellipsis::default(),
            letter_spacing: self.letter_spacing,
            font_features: Vec::new(),
            font_variations: Vec::new(),
            weight: None,
            hint_factor: renderer.hint_factor(),
        }
    }

    /// Set the base font used to shape and draw this paragraph.
    pub fn font(mut self, font: impl Into<iced_core::Font>) -> Self {
        self.font = Some(font.into());
        self
    }

    /// Set the default font size of the rich text. Spans may override
    /// per-span. A `text-*` step also sets its leading, like
    /// [`Text::size`](crate::text::Text).
    pub fn size(mut self, size: impl Into<crate::Size>) -> Self {
        let size = size.into();
        self.size = Some(Pixels(size.text_size()));
        self.leading = size.leading().map(text::LineHeight::Relative);
        self
    }

    /// Set the line height.
    pub fn line_height(mut self, line_height: impl Into<text::LineHeight>) -> Self {
        self.line_height = Some(line_height.into());
        self
    }

    /// Set the width of the layout box.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Set the height of the layout box.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Set the horizontal alignment.
    pub fn align_x(mut self, alignment: impl Into<Alignment>) -> Self {
        self.align_x = alignment.into();
        self
    }

    /// Set the vertical alignment.
    pub fn align_y(mut self, alignment: impl Into<alignment::Vertical>) -> Self {
        self.align_y = alignment.into();
        self
    }

    /// Allow native text selection. Defaults to true outside button labels.
    pub fn selectable(mut self, selectable: bool) -> Self {
        self.selectable = Some(selectable);
        self
    }

    /// Set the wrapping strategy.
    pub fn wrapping(mut self, wrapping: Wrapping) -> Self {
        self.wrapping = wrapping;
        self
    }

    /// Set the letter spacing.
    pub fn letter_spacing(mut self, letter_spacing: impl Into<Em>) -> Self {
        self.letter_spacing = letter_spacing.into();
        self
    }

    /// Set the catalog class for the default span color (analogous to
    /// iced's `Rich::style`).
    pub fn style(mut self, style: impl Into<iced_widget::text::StyleFn<'a, Theme>>) -> Self {
        self.class = style.into();
        self
    }
}

// Selection belongs to the laid-out paragraph. The renderer retains a weak
// reference to the colored paragraph, so keep its owner through presentation.
struct State<P: text::Paragraph> {
    spans: Vec<text::Span<'static, ()>>,
    paragraph: P,
    colored: RefCell<Option<(Vec<Option<Color>>, P)>>,
}

struct Operand<'a, P: text::Paragraph> {
    state: &'a mut State<P>,
    anchor: Point,
    selectable: bool,
}

impl<P: text::Paragraph> operation::Text for Operand<'_, P> {
    fn text(&self) -> Fragment<'_> {
        self.state
            .spans
            .iter()
            .map(|span| span.text.as_ref())
            .collect::<String>()
            .into()
    }

    fn select(&mut self, start: Point, end: Point, target: text::Target) {
        if self.selectable {
            let offset = self.anchor - Point::ORIGIN;
            self.state
                .paragraph
                .select(start - offset, end - offset, target);
        }
    }

    fn select_all(&mut self) {
        if self.selectable {
            self.state.paragraph.select_all();
        }
    }

    fn deselect(&mut self) {
        self.state.paragraph.deselect();
    }

    fn copy(&mut self) -> Option<String> {
        if self.selectable {
            self.state.paragraph.copy()
        } else {
            None
        }
    }
}

impl Meta for RichText<'_> {}

// Generic over the renderer, as iced's text widgets are, so the paragraph is
// the renderer's own type rather than a concrete one fixed by features.
impl<'a, Message: 'a, R: text::Renderer> Widget<Message, Theme, R> for RichText<'a> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State<R::Paragraph>>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State {
            spans: Vec::<text::Span<'static, ()>>::new(),
            paragraph: R::Paragraph::default(),
            colored: RefCell::default(),
        })
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &R, limits: &layout::Limits) {
        // We don't have the theme here, so layout uses each span's
        // *layout* shape (no color). Color is rendering-only and gets
        // applied in `draw` by reshaping the paragraph against the live
        // palette. The cached paragraph's dimensions are color-independent.
        let state = tree.state.downcast_mut::<State<R::Paragraph>>();
        let default_font = self.font.unwrap_or_else(|| text::Renderer::font(renderer));
        let layout_spans: Vec<_> = self
            .spans
            .iter()
            .map(|span| span.to_layout_span(default_font))
            .collect();

        tree.size = layout::sized(limits, self.width, self.height, |limits| {
            let bounds = limits.bounds();
            let text = self
                .format(renderer, bounds)
                .with_content(layout_spans.as_slice());
            let difference = if state.spans == layout_spans {
                state.paragraph.compare(text.with_content(()))
            } else {
                text::Difference::Shape
            };
            match difference {
                text::Difference::None => {}
                text::Difference::Bounds => {
                    state.paragraph.resize(bounds);
                    *state.colored.get_mut() = None;
                }
                text::Difference::Shape => {
                    state.paragraph = text::Paragraph::with_spans(text);
                    state.spans = layout_spans;
                    *state.colored.get_mut() = None;
                }
            }
            state.paragraph.min_bounds()
        });
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut R,
        theme: &Theme,
        defaults: &renderer::Style,
        layout: Layout,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        if !layout.bounds().intersects(viewport) {
            return;
        }
        let state = tree.state.downcast_ref::<State<R::Paragraph>>();

        let paragraph = &state.paragraph;
        let anchor = layout
            .bounds()
            .anchor(paragraph.min_bounds(), self.align_x, self.align_y);
        // Match iced's native rich-text highlights: only the first and last
        // wrapped regions receive horizontal padding and rounded outer ends.
        for (index, span) in self.spans.iter().enumerate() {
            if span.background.is_none() && span.border.is_none() {
                continue;
            }
            let highlight = text::Highlight {
                background: span
                    .background
                    .map(|token| token.resolve(theme).into())
                    .unwrap_or(Color::TRANSPARENT.into()),
                border: span.border.unwrap_or_default(),
            };
            let regions = paragraph.span_bounds(index);
            for (i, bounds) in regions.iter().enumerate() {
                use crate::core::border::Radius;

                let starts = i == 0;
                let ends = i + 1 == regions.len();
                let left = if starts { span.padding.left } else { 0.0 };
                let right = if ends { span.padding.right } else { 0.0 };
                let bounds = Rectangle::new(
                    bounds.position() - Vector::new(left, span.padding.top),
                    bounds.size() + Size::new(left + right, span.padding.y()),
                );
                let radius = Radius {
                    top_left: highlight.border.radius.top_left * f32::from(starts),
                    bottom_left: highlight.border.radius.bottom_left * f32::from(starts),
                    top_right: highlight.border.radius.top_right * f32::from(ends),
                    bottom_right: highlight.border.radius.bottom_right * f32::from(ends),
                };
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: bounds + (anchor - Point::ORIGIN),
                        border: Border {
                            radius,
                            ..highlight.border
                        },
                        ..Default::default()
                    },
                    highlight.background,
                );
            }
        }
        let colors = || {
            self.spans.iter().map(|span| {
                span.color.map(|(light, dark)| {
                    let color = if theme.neutral.is_dark { dark } else { light };
                    color.resolve(theme).into()
                })
            })
        };
        let mut colored = state.colored.borrow_mut();
        let ink = if self.spans.iter().any(|span| span.color.is_some()) {
            colored.take_if(|(palette, _)| palette.iter().copied().ne(colors()));
            let (_, ink) = colored.get_or_insert_with(|| {
                let palette: Vec<_> = colors().collect();
                let mut spans = state.spans.clone();
                for (span, color) in spans.iter_mut().zip(&palette) {
                    span.color = *color;
                }
                let text = self
                    .format(renderer, paragraph.bounds())
                    .with_content(spans.as_slice());
                (palette, text::Paragraph::with_spans(text))
            });
            ink
        } else {
            paragraph
        };
        let style = <Theme as Catalog>::style(theme, &self.class);
        renderer.fill_paragraph(
            ink,
            anchor,
            style.color.unwrap_or(defaults.text_color),
            *viewport,
        );
        for region in paragraph.selection() {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: *region + (anchor - Point::ORIGIN),
                    ..Default::default()
                },
                style.selection.unwrap_or_else(|| theme.selection()),
            );
        }
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        _viewport: &Rectangle,
        _renderer: &R,
        operation: &mut dyn Operation,
    ) {
        let state = tree.state.downcast_mut::<State<R::Paragraph>>();
        let bounds = layout.bounds();
        let anchor = bounds.anchor(state.paragraph.min_bounds(), self.align_x, self.align_y);
        operation.text(
            None,
            bounds,
            &mut Operand {
                state,
                anchor,
                selectable: self.selectable.unwrap_or(true),
            },
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        _renderer: &R,
    ) -> mouse::Interaction {
        let state = tree.state.downcast_ref::<State<R::Paragraph>>();
        let bounds = layout.bounds();
        let anchor = bounds.anchor(state.paragraph.min_bounds(), self.align_x, self.align_y);
        if self.selectable.unwrap_or(true)
            && cursor.is_over(*viewport)
            && cursor.is_over(bounds)
            && cursor
                .position_from(anchor)
                .is_some_and(|point| state.paragraph.hit_glyph(point))
        {
            mouse::Interaction::Text
        } else {
            mouse::Interaction::None
        }
    }
}

impl<'a, Message: 'a> From<RichText<'a>> for Element<'a, Message, Theme, Renderer> {
    fn from(rich: RichText<'a>) -> Self {
        rich._boxed()
    }
}

impl<'a, Message: 'a> From<RichText<'a>> for Slot<'a, Message> {
    fn from(widget: RichText<'a>) -> Self {
        Slot::new(widget)
    }
}

impl<'a, Message: 'a> Compose<Message> for RichText<'a> {
    fn inherit(&mut self, from: &Inherited) {
        self.selectable = self.selectable.or(from.selectable());
        self.font = self.font.or(from.font());
        self.leading = self.leading.or(from.line_height());
        // Spans that set no size or weight of their own take the inherited
        // one, so `.size(Sm)` on an enclosing button cascades into them.
        if let Some(size) = from.size() {
            let pixels = size.text_size();
            if self.size.is_none() {
                self.size = Some(Pixels(pixels));
            }
            for s in self.spans.iter_mut() {
                *s = std::mem::replace(s, span("")).fallback_size(pixels);
            }
        }
        if let Some(weight) = from.weight() {
            let w: font::Weight = weight.into();
            for s in self.spans.iter_mut() {
                *s = std::mem::replace(s, span("")).fallback_weight(w);
            }
        }
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        (*self).into()
    }
}

/// Build a [`RichText`] from a comma-separated list of [`Span`]s. Mirrors
/// the shape of `row!` / `column!`.
///
/// ```ignore
/// rich_text![
///     span("Required Field ").medium(),
///     span("*").color(Token::Destructive),
/// ]
/// ```
#[macro_export]
macro_rules! rich_text {
    () => {
        $crate::rich_text::rich_text(::std::iter::empty::<$crate::rich_text::Span<'_>>())
    };
    ($($span:expr),+ $(,)?) => {
        $crate::rich_text::rich_text([
            $(::core::convert::Into::<$crate::rich_text::Span<'_>>::into($span)),+
        ])
    };
}
