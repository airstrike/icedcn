//! Typography components
//!
//! Typography presets inherit the surrounding foreground. Semantic colors such as
//! [`muted`] and [`error`], and an explicit `.style(...)`, override that color.
//!
//! # Example
//!
//! ```ignore
//! use icedcn::text;
//!
//! text::h1("Page Title")
//! text::muted("Secondary text")
//! text::p("Bold text").bold()
//! ```

use iced_widget::Widget;
use std::marker::PhantomData;

use crate::core::Length;
use crate::core::Length::Fill;
use crate::core::alignment::Horizontal;
use crate::core::font;
use crate::core::font::Weight;
pub use crate::core::text::{Ellipsis, LineHeight, Wrapping};
use crate::core::text::{Fragment, IntoFragment};
use iced_widget::Renderer;
use iced_widget::text::StyleFn;

use crate::Size;
use crate::compose::{Compose, Inherited, Slot};
use crate::rule;
use icedcn_core::Theme;
use icedcn_core::theme;

/// The `text-*` steps of Tailwind v4's `theme.css`: each sets a font size
/// and its unitless leading, which descendants scale by their own size.
pub mod size {
    use crate::Size;

    const fn step(size: f32, leading: f32) -> Size {
        Size::Text { size, leading }
    }

    pub const XS: Size = step(12.0, 1.0 / 0.75);
    pub const SM: Size = step(14.0, 1.25 / 0.875);
    pub const BASE: Size = step(16.0, 1.5);
    pub const LG: Size = step(18.0, 1.75 / 1.125);
    pub const XL: Size = step(20.0, 1.75 / 1.25);
    pub const XL2: Size = step(24.0, 2.0 / 1.5);
    pub const XL3: Size = step(30.0, 2.25 / 1.875);
    pub const XL4: Size = step(36.0, 2.5 / 2.25);
    pub const XL5: Size = step(48.0, 1.0);
    pub const XL6: Size = step(60.0, 1.0);
    pub const XL7: Size = step(72.0, 1.0);
    pub const XL8: Size = step(96.0, 1.0);
    pub const XL9: Size = step(128.0, 1.0);
}

/// The `leading-*` ratios of Tailwind v4. `leading-N` is a length
/// (`N * 4px`): pass `Pixels(..)` for it, which inherits as pixels.
pub mod leading {
    use super::LineHeight;

    pub const NONE: LineHeight = LineHeight::Relative(1.0);
    pub const TIGHT: LineHeight = LineHeight::Relative(1.25);
    pub const SNUG: LineHeight = LineHeight::Relative(1.375);
    pub const NORMAL: LineHeight = LineHeight::Relative(1.5);
    pub const RELAXED: LineHeight = LineHeight::Relative(1.625);
    pub const LOOSE: LineHeight = LineHeight::Relative(2.0);
}

/// Text widget wrapper with semantic styling
pub struct Text<'a> {
    fragment: Fragment<'a>,
    font: Option<iced_core::Font>,
    size: Option<f32>,
    /// The leading of the `text-*` step or the cascade; `line_height` wins.
    leading: Option<LineHeight>,
    card_title: bool,
    weight: Option<Weight>,
    italic: bool,
    letter_spacing: Option<iced_core::Em>,
    line_height: Option<LineHeight>,
    style: Option<StyleFn<'a, Theme>>,
    width: Option<Length>,
    height: Length,
    selectable: Option<bool>,
    wrapping: Wrapping,
    ellipsis: Ellipsis,
    align_x: Option<iced_core::text::Alignment>,
}

impl<'a> Text<'a> {
    pub fn new(content: impl IntoFragment<'a>) -> Self {
        Self {
            fragment: content.into_fragment(),
            font: None,
            size: None,
            leading: None,
            card_title: false,
            weight: None,
            italic: false,
            letter_spacing: None,
            line_height: None,
            style: None,
            width: None,
            height: Length::Shrink,
            selectable: None,
            wrapping: Wrapping::default(),
            ellipsis: Ellipsis::None,
            align_x: None,
        }
    }

    /// Set the font size. A `text-*` step (`text::size::LG`, a control
    /// [`Size`]) also sets its leading; a number sets only the size and
    /// keeps the inherited leading, like `text-[28px]`.
    pub fn size(mut self, size: impl Into<Size>) -> Self {
        let size = size.into();
        self.size = Some(size.text_size());
        self.leading = size.leading().map(LineHeight::Relative);
        self
    }

    /// Set the font used for this text.
    pub fn font(mut self, font: impl Into<iced_core::Font>) -> Self {
        self.font = Some(font.into());
        self
    }

    /// Use Card title sizing from the nearest Card, unless `.size` is explicit.
    pub fn card_title(mut self) -> Self {
        self.card_title = true;
        self
    }

    pub fn weight(mut self, weight: impl Into<crate::Weight>) -> Self {
        self.weight = Some(weight.into().into());
        self
    }

    pub fn medium(self) -> Self {
        self.weight(crate::Weight::Medium)
    }

    pub fn semibold(self) -> Self {
        self.weight(crate::Weight::Semibold)
    }

    pub fn bold(self) -> Self {
        self.weight(crate::Weight::Bold)
    }

    /// Apply italic font style. Used by `blockquote` to match nova's
    /// `border-l-2 pl-6 italic` cascade.
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    pub fn letter_spacing(mut self, spacing: impl Into<iced_core::Em>) -> Self {
        self.letter_spacing = Some(spacing.into());
        self
    }

    pub fn line_height(mut self, line_height: impl Into<LineHeight>) -> Self {
        self.line_height = Some(line_height.into());
        self
    }

    /// Override the text widget's layout width. Defaults to `Shrink`,
    /// matching `Text::new()`. Add `.width(Fill)` at the call site for
    /// paragraph copy that should occupy the parent's full width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Constrain the paragraph's height, for example to two line heights.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Allow native text selection. Defaults to true outside button labels.
    pub fn selectable(mut self, selectable: bool) -> Self {
        self.selectable = Some(selectable);
        self
    }

    /// Choose the paragraph's native line-breaking strategy.
    pub fn wrapping(mut self, wrapping: Wrapping) -> Self {
        self.wrapping = wrapping;
        self
    }

    /// Truncate the last visible line using the paragraph's shaped glyphs.
    pub fn ellipsis(mut self, ellipsis: Ellipsis) -> Self {
        self.ellipsis = ellipsis;
        self
    }

    /// Align the lines within the text's width (`text-center`); pair it
    /// with `.width(Fill)` so there is room to align in.
    pub fn align_x(mut self, alignment: impl Into<iced_core::text::Alignment>) -> Self {
        self.align_x = Some(alignment.into());
        self
    }

    /// Set the style.
    ///
    /// Accepts any text-style variant struct (`theme::text::Foreground`,
    /// `theme::text::Muted`, etc.), a `Spec` (`Foreground + Alpha(0.5)`),
    /// or an explicit `Box::new(closure)`.
    pub fn style(mut self, style: impl Into<StyleFn<'a, Theme>>) -> Self {
        self.style = Some(style.into());
        self
    }

    /// Build the iced text widget
    fn build(self) -> iced_widget::Text<'a, Theme> {
        #[cfg(feature = "symbols")]
        crate::font::load();
        let mut widget = iced_widget::text(self.fragment)
            .selectable(self.selectable.unwrap_or(true))
            .height(self.height)
            .wrapping(self.wrapping)
            .ellipsis(self.ellipsis);

        if let Some(size) = self
            .size
            .or(self.card_title.then_some(size::BASE.text_size()))
        {
            widget = widget.size(size);
        }

        // Italic is folded into the font (no Format-level override exists),
        // so we layer it onto whichever font is set. Defaulting to
        // `Font::DEFAULT` here would clobber `font::set_defaults`, but we
        // only do it when the caller actually wants italic — the same
        // tradeoff `bold()` would have if we modeled it that way.
        let font = match (self.font, self.italic) {
            (Some(f), true) => Some(f.style(font::Style::Italic)),
            (Some(f), false) => Some(f),
            (None, true) => Some(iced_core::Font::DEFAULT.style(font::Style::Italic)),
            (None, false) => None,
        };

        if let Some(font) = font {
            widget = widget.font(font);
        }

        if let Some(weight) = self.weight {
            widget = widget.weight(weight);
        }

        if let Some(letter_spacing) = self.letter_spacing {
            widget = widget.letter_spacing(letter_spacing);
        }

        // Preflight's `html { line-height: 1.5 }` is the root of the cascade.
        widget = widget.line_height(self.line_height.or(self.leading).unwrap_or(leading::NORMAL));

        if let Some(style) = self.style {
            widget = widget.style(style);
        }

        if let Some(width) = self.width {
            widget = widget.width(width);
        }

        if let Some(alignment) = self.align_x {
            widget = widget.align_x(alignment);
        }

        widget
    }
}

impl<'a, Message: 'a> From<Text<'a>> for iced_core::Element<'a, Message, Theme, Renderer> {
    fn from(t: Text<'a>) -> Self {
        t.build().boxed()
    }
}

impl<'a, Message: 'a> From<Text<'a>> for Slot<'a, Message> {
    fn from(widget: Text<'a>) -> Self {
        Slot::new(widget)
    }
}

impl<'a, Message: 'a> Compose<Message> for Text<'a> {
    fn inherit(&mut self, from: &Inherited) {
        self.selectable = self.selectable.or(from.selectable());
        self.size = self.size.or(if self.card_title {
            from.card_title_size().or(Some(size::BASE.text_size()))
        } else {
            from.size().map(Size::text_size)
        });
        self.weight = self.weight.or(from.weight().map(Into::into));
        self.font = self.font.or(from.font());
        // A relative leading scales by this text's own size, as CSS does.
        self.leading = self.leading.or(from.line_height());
    }

    fn compose<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        self.build().boxed()
    }
}

pub fn h1<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content)
        .size(size::XL3)
        .semibold()
        .letter_spacing(-0.025)
}

pub fn h2<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content)
        .size(size::XL)
        .medium()
        .letter_spacing(-0.025)
}

pub fn h3<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content)
        .size(size::LG)
        .medium()
        .letter_spacing(-0.025)
}

pub fn h4<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content)
        .size(size::BASE)
        .medium()
        .letter_spacing(-0.025)
}

pub fn p<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content).size(size::BASE)
}

pub fn lead<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content).size(size::XL).style(theme::text::Muted)
}

pub fn large<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content).size(size::LG).semibold()
}

pub fn small<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content).size(size::SM)
}

/// Alias for [`small`]: 14-pixel text that inherits its parent color.
pub fn sm<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    small(content)
}

/// Alias for [`p`]: 16-pixel text that inherits its parent color.
pub fn md<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    p(content)
}

/// Alias for [`large`]: 18-pixel semibold text that inherits its parent color.
pub fn lg<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    large(content)
}

/// 20-pixel text that inherits its parent color.
pub fn xl<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content).size(size::XL)
}

/// 24-pixel text that inherits its parent color.
pub fn xl2<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content).size(size::XL2)
}

/// 30-pixel text that inherits its parent color.
pub fn xl3<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content).size(size::XL3)
}

/// Medium-weight field label with a tight line height for alignment beside controls.
pub fn label<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content)
        .size(size::SM)
        .medium()
        .line_height(LineHeight::Relative(1.0))
}

pub fn muted<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content).size(size::SM).style(theme::text::Muted)
}

/// Field error text: `text-sm` in the destructive color.
pub fn error<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content)
        .size(size::SM)
        .style(theme::text::Destructive)
}

pub fn xs<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content).size(size::XS)
}

/// An italic blockquote with a two-pixel left rule and a 24-pixel inset. Set vertical spacing
/// on the enclosing column.
pub struct Blockquote<'a> {
    body: Text<'a>,
}

impl<'a> Blockquote<'a> {
    /// Override the body text size. Defaults to `size::BASE` (16px).
    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.body = self.body.size(size);
        self
    }

    /// Override the body text style. Defaults to `theme::text::Foreground`.
    pub fn style(mut self, style: impl Into<StyleFn<'a, Theme>>) -> Self {
        self.body = self.body.style(style);
        self
    }
}

/// Create a [`Blockquote`] for the given quote text. Italic, foreground
/// color, fill width — plus the 2px left rule + 24px left padding that
/// the bare `Text` helper alone can't express.
pub fn blockquote<'a>(content: impl IntoFragment<'a>) -> Blockquote<'a> {
    Blockquote {
        body: Text::new(content)
            .size(size::BASE)
            .italic()
            .line_height(LineHeight::Relative(1.5))
            .style(theme::text::Foreground)
            .width(Fill),
    }
}

impl<'a, Message: 'a> From<Blockquote<'a>> for iced_core::Element<'a, Message, Theme, Renderer> {
    fn from(b: Blockquote<'a>) -> Self {
        Box::new(b).compose()
    }
}

impl<'a, Message: 'a> From<Blockquote<'a>> for Slot<'a, Message> {
    fn from(widget: Blockquote<'a>) -> Self {
        Slot::new(widget)
    }
}

impl<'a, Message: 'a> Compose<Message> for Blockquote<'a> {
    fn compose<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let Blockquote { body } = *self;

        iced_widget::Row::new()
            .spacing(24)
            .height(Length::Shrink)
            .push(rule::vertical(2).boxed())
            .push(body.build().boxed())
            .boxed()
    }
}

/// Semibold monospace inline code with native span padding and a muted highlight.
/// The highlight follows wrapped lines; no surrounding container is needed.
pub fn inline_code<'a>(content: impl IntoFragment<'a>) -> crate::rich_text::RichText<'a> {
    use crate::core::border;
    use crate::rich_text::{rich_text, span};
    use icedcn_core::theme::tweak::Token;

    rich_text([span(content)
        .semibold()
        .padding([2, 5])
        .background(Token::Muted)
        .border(border::rounded(4))])
    .font(iced_core::Font::MONOSPACE)
    .wrapping(Wrapping::WordOrGlyph)
    .size(size::SM)
    .style(theme::text::Foreground)
}

/// A ruled article table. Columns share their intrinsic widths, header cells wrap,
/// and alternate body rows use the muted background.
pub fn table<'a, M: 'a, const N: usize>(
    headers: [impl IntoFragment<'a>; N],
    rows: impl IntoIterator<Item = [impl IntoFragment<'a>; N]>,
) -> crate::container::Container<'a, M> {
    use crate::table::{self, Track};
    use icedcn_core::Border;
    use icedcn_core::theme::container::Style;

    let header = table::header([table::row(headers.map(|value| {
        table::head(p(value).bold().line_height(LineHeight::Relative(1.5)))
            .height(Length::Shrink)
            .padding([8, 16])
    }))]);
    let body = table::body(rows.into_iter().map(|values| {
        table::row(values.map(|value| {
            table::cell(p(value).line_height(LineHeight::Relative(1.5))).padding([8, 16])
        }))
    }));
    let grid = table::grid([Track::Auto; N])
        .header(header)
        .body(body)
        .ruled();
    crate::container(grid)
        .width(Fill)
        .padding(1)
        .style(|theme| Style {
            border: Border {
                width: 1.,
                color: theme.neutral.border,
                ..Default::default()
            },
            ..Default::default()
        })
}

/// A bulleted list with a 24-pixel indent and eight pixels between items. Set surrounding
/// spacing on the enclosing column.
pub struct List<'a, Message> {
    items: Vec<Text<'a>>,
    _phantom: PhantomData<Message>,
}

/// Create a [`List`] from an iterable of fragments. Empty input
/// produces an empty list (no panic).
pub fn list<'a, Message, I, F>(items: I) -> List<'a, Message>
where
    I: IntoIterator<Item = F>,
    F: IntoFragment<'a>,
{
    let items = items
        .into_iter()
        .map(|f| {
            Text::new(f)
                .size(size::BASE)
                .line_height(LineHeight::Relative(1.5))
                .style(theme::text::Foreground)
                .width(Fill)
        })
        .collect();

    List {
        items,
        _phantom: PhantomData,
    }
}

impl<'a, Message: 'a> From<List<'a, Message>> for iced_core::Element<'a, Message, Theme, Renderer> {
    fn from(list: List<'a, Message>) -> Self {
        Box::new(list).compose()
    }
}

impl<'a, Message: 'a> From<List<'a, Message>> for Slot<'a, Message> {
    fn from(widget: List<'a, Message>) -> Self {
        Slot::new(widget)
    }
}

impl<'a, Message: 'a> Compose<Message> for List<'a, Message> {
    fn compose<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let List { items, .. } = *self;

        let rows = items.into_iter().map(|item| {
            let marker = Text::new("•")
                .size(size::BASE)
                .width(24)
                .line_height(LineHeight::Relative(1.5))
                .align_x(Horizontal::Center);
            iced_widget::Row::new()
                .push(marker.build())
                .push(item.build())
                .width(Fill)
                .boxed()
        });
        iced_widget::column(rows).spacing(8).width(Fill).boxed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::badge::badge;
    use crate::button::button;
    use iced_core::Pixels;

    // Observe the actual Text configuration after its parent has composed it.
    struct Label {
        text: Text<'static>,
        check: fn(&Text<'_>),
    }

    impl Compose<()> for Label {
        fn inherit(&mut self, from: &Inherited) {
            <Text as Compose<()>>::inherit(&mut self.text, from);
        }

        fn compose<'a>(self: Box<Self>) -> iced_core::Element<'a, (), Theme, Renderer>
        where
            Self: 'a,
        {
            (self.check)(&self.text);
            self.text.build().boxed()
        }
    }

    #[test]
    fn plain_text_inherits_button_foreground() {
        let label = Label {
            text: crate::text("←").align_x(Horizontal::Center),
            check: |text| {
                assert!(
                    text.style.is_none(),
                    "plain text must inherit its parent's color"
                );
                assert_eq!(text.size, Some(14.0));
            },
        };
        let _ = Box::new(button(Slot::new(label)).width(40)).compose();
    }

    #[test]
    fn typography_presets_inherit_selected_sidebar_foreground() {
        for text in [
            h1("Dashboard"),
            h2("Dashboard"),
            h3("Dashboard"),
            h4("Dashboard"),
            p("Dashboard"),
            large("Dashboard"),
            small("Dashboard"),
            xl("Dashboard"),
            xl2("Dashboard"),
            xl3("Dashboard"),
            label("Dashboard"),
            xs("Dashboard"),
        ] {
            let label = Label {
                text,
                check: |text| {
                    assert!(
                        text.style.is_none(),
                        "typography must inherit the selected color"
                    );
                },
            };
            let _: iced_core::Element<'_, (), Theme, Renderer> =
                crate::sidebar::menu_button(Slot::new(label))
                    .active(true)
                    .into();
        }
    }

    #[test]
    fn semantic_and_explicit_typography_colors_remain_overrides() {
        for theme in [Theme::light(), Theme::dark()] {
            for text in [
                small("Label").style(theme::text::Foreground),
                muted("Label"),
                error("Label"),
            ] {
                let style = text
                    .style
                    .expect("semantic and explicit styles are preserved");
                assert!(style(&theme).color.is_some());
            }
        }
    }

    #[test]
    fn explicit_text_foreground_survives_button_composition() {
        let label = Label {
            text: crate::text("←").style(theme::text::Foreground),
            check: |text| {
                let style = text.style.as_ref().expect("explicit style remains");
                for theme in [Theme::light(), Theme::dark()] {
                    assert_eq!(style(&theme).color, Some(theme.neutral.foreground.into()));
                }
            },
        };
        let _ = Box::new(button(Slot::new(label)).width(40)).compose();
    }

    /// The line box a text gets once `from` has cascaded into it.
    fn line_box(text: Text<'static>, from: &Inherited) -> f32 {
        let mut text = text;
        <Text as Compose<()>>::inherit(&mut text, from);
        let size = text.size.expect("a sized text");
        text.line_height
            .or(text.leading)
            .unwrap_or(leading::NORMAL)
            .to_absolute(Pixels(size))
            .0
    }

    fn card() -> Inherited {
        Inherited::default().with_size(Size::Default)
    }

    fn assert_px(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() < 0.01,
            "line box {actual}px, expected {expected}px"
        );
    }

    #[test]
    fn same_size_text_takes_the_cascaded_ratio() {
        assert_px(line_box(Text::new("Body"), &card()), 20.0);
        assert_px(line_box(small("Body"), &card()), 20.0);
    }

    #[test]
    fn custom_size_scales_the_inherited_ratio() {
        // `text-[28px]` inside a `text-sm` card: 28 * 1.25 / 0.875.
        assert_px(line_box(h3("Start with a style.").size(28), &card()), 40.0);
        assert_px(line_box(Text::new("Caption").size(12.8), &card()), 18.2857);
    }

    #[test]
    fn token_size_sets_its_own_ratio() {
        assert_px(line_box(xs("Native components."), &card()), 16.0);
        assert_px(line_box(h3("Title"), &card()), 28.0);
        assert_px(line_box(p("Body"), &card()), 24.0);
        assert_px(line_box(xl3("Title"), &card()), 36.0);
        assert_px(line_box(Text::new("Body").size(size::XL4), &card()), 40.0);
    }

    #[test]
    fn explicit_line_height_wins() {
        let text = h3("Title").size(28).line_height(Pixels(32.0));
        assert_px(line_box(text, &card()), 32.0);
        let text = small("Label").line_height(leading::NONE);
        assert_px(line_box(text, &card()), 14.0);
        // Order does not matter: the `leading-*` wins over a later `text-*`.
        let text = Text::new("Title")
            .line_height(leading::SNUG)
            .size(size::BASE);
        assert_px(line_box(text, &card()), 22.0);
    }

    #[test]
    fn absolute_cascade_inherits_as_pixels() {
        let leading_6 = Inherited::default()
            .with_size(Size::Custom(14.0))
            .with_line_height(Pixels(24.0));
        assert_px(line_box(Text::new("Body"), &leading_6), 24.0);
        assert_px(line_box(Text::new("Big").size(28), &leading_6), 24.0);
        // A `text-*` step replaces the inherited length.
        assert_px(line_box(xs("Small"), &leading_6), 16.0);
    }

    #[test]
    fn nested_text_size_replaces_an_ancestor_leading() {
        let outer = Inherited::default().with_line_height(Pixels(24.0));
        let inner = Inherited::default().with_size(Size::Xs).or(outer);
        assert_px(line_box(Text::new("Badge"), &inner), 16.0);
        let inner = outer.with_size(Size::Default);
        assert_px(line_box(Text::new("Card"), &inner), 20.0);
    }

    #[test]
    fn uncascaded_custom_size_uses_the_root_leading() {
        assert_px(
            line_box(Text::new("Body").size(20), &Inherited::default()),
            30.0,
        );
    }

    #[test]
    fn badge_typography_overrides_card_cascade() {
        let label = Label {
            text: Text::new("Forms"),
            check: |text| {
                assert_eq!(text.size, Some(12.0));
                assert_eq!(text.line_height, None);
                assert_eq!(text.leading, Some(LineHeight::Relative(1.0 / 0.75)));
                assert_eq!(text.weight, Some(Weight::Medium));
                assert_eq!(text.selectable, Some(false));
            },
        };
        let mut badge = badge(Slot::new(label)).on_press(());
        badge.inherit(
            &Inherited::default()
                .with_size(Size::Default)
                .with_line_height(Pixels(20.0))
                .with_weight(crate::Weight::Bold)
                .with_selectable(true),
        );
        let _ = Box::new(badge).compose();
    }

    #[test]
    fn badge_preserves_explicit_label_typography() {
        let label = Label {
            text: Text::new("Forms")
                .size(10.0)
                .line_height(Pixels(14.0))
                .bold(),
            check: |text| {
                assert_eq!(text.size, Some(10.0));
                assert_eq!(text.line_height, Some(LineHeight::Absolute(Pixels(14.0))));
                assert_eq!(text.weight, Some(Weight::Bold));
            },
        };
        let mut badge = badge(Slot::new(label));
        badge.inherit(
            &Inherited::default()
                .with_size(Size::Default)
                .with_line_height(Pixels(20.0)),
        );
        let _ = Box::new(badge).compose();
    }
}
