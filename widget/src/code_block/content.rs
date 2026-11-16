//! The scrolling content: a source string or a list of lines.
use std::borrow::Cow;
use std::ops::{Range, RangeInclusive};

use iced_core::Length::{Fill, Shrink};
use iced_core::widget::Id;
use iced_core::{Font, Pixels};
use iced_widget::Widget;

use super::editor::Editor;
use super::listing::{self, Options, View};
use super::{LineStyle, Viewport, plain};
use crate::compose::{Inherited, Slot};
use crate::container;

/// A string, or one [`Line`] per row.
pub enum Body<'a, Message> {
    Text(Cow<'a, str>),
    Lines(Vec<Line<'a, Message>>),
}

impl<'a, Message> From<&'a str> for Body<'a, Message> {
    fn from(source: &'a str) -> Self {
        Body::Text(source.into())
    }
}
impl<'a, Message> From<String> for Body<'a, Message> {
    fn from(source: String) -> Self {
        Body::Text(source.into())
    }
}
impl<'a, Message> From<&'a String> for Body<'a, Message> {
    fn from(source: &'a String) -> Self {
        Body::Text(source.as_str().into())
    }
}
impl<'a, Message> From<Cow<'a, str>> for Body<'a, Message> {
    fn from(source: Cow<'a, str>) -> Self {
        Body::Text(source)
    }
}
impl<'a, Message> From<Vec<Line<'a, Message>>> for Body<'a, Message> {
    fn from(lines: Vec<Line<'a, Message>>) -> Self {
        Body::Lines(lines)
    }
}

/// One line of code, for a custom row.
pub struct Line<'a, Message> {
    content: Slot<'a, Message>,
    style: LineStyle,
}

/// A line of code. Leave diff markers out of the content: the line paints its own.
pub fn line<'a, Message>(content: impl Into<Slot<'a, Message>>) -> Line<'a, Message> {
    Line {
        content: content.into(),
        style: LineStyle::Default,
    }
}

impl<Message> Line<'_, Message> {
    /// The state of the line: [`Added`](super::Added), [`Removed`](super::Removed)
    /// or [`Highlighted`](super::Highlighted).
    pub fn style(mut self, style: LineStyle) -> Self {
        self.style = style;
        self
    }
}

/// One-based source lines: `7`, `6..=10` or `6..11`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineRange(RangeInclusive<usize>);

impl From<usize> for LineRange {
    fn from(line: usize) -> Self {
        LineRange(line..=line)
    }
}
impl From<RangeInclusive<usize>> for LineRange {
    fn from(lines: RangeInclusive<usize>) -> Self {
        LineRange(lines)
    }
}
impl From<Range<usize>> for LineRange {
    fn from(lines: Range<usize>) -> Self {
        LineRange(lines.start..=lines.end.saturating_sub(1))
    }
}

// Unsuffixed literals (`7`, `6..=10`) infer as `i32`.
fn index(line: i32) -> usize {
    usize::try_from(line).unwrap_or(0)
}
impl From<i32> for LineRange {
    fn from(line: i32) -> Self {
        LineRange::from(index(line))
    }
}
impl From<RangeInclusive<i32>> for LineRange {
    fn from(lines: RangeInclusive<i32>) -> Self {
        LineRange(index(*lines.start())..=index(*lines.end()))
    }
}
impl From<Range<i32>> for LineRange {
    fn from(lines: Range<i32>) -> Self {
        LineRange::from(index(lines.start)..index(lines.end))
    }
}

/// The content that scrolls or wraps.
pub struct Content<'a, Message> {
    body: Body<'a, Message>,
    options: Options<'a>,
    listing: Option<View<'a, Message>>,
    pub(super) wrap: bool,
    line_numbers: bool,
    start_line: usize,
    pub(super) height: Option<Pixels>,
    pub(super) max_height: Option<Pixels>,
    pub(super) collapsed: bool,
    id: Option<Id>,
    focusable: bool,
    font: Font,
    on_input: Option<Box<dyn Fn(String) -> Message + 'a>>,
    default_value: Option<String>,
    disabled: bool,
}

/// Content from a string, or from a list of [`line`]s.
pub fn content<'a, Message>(body: impl Into<Body<'a, Message>>) -> Content<'a, Message> {
    Content {
        body: body.into(),
        options: Options::default(),
        listing: None,
        wrap: false,
        line_numbers: false,
        start_line: 1,
        height: None,
        max_height: None,
        collapsed: false,
        id: None,
        focusable: true,
        font: Font::MONOSPACE,
        on_input: None,
        default_value: None,
        disabled: false,
    }
}

impl<'a, Message: 'a> Content<'a, Message> {
    /// Highlight the source with a language or alias such as `"rust"`, `"toml"` or
    /// `"sh"`, or render terminal colors with `"ansi"`. Other languages show plain
    /// text. Syntax colors need the `highlighter` or `markdown` feature.
    pub fn language(mut self, language: impl Into<Cow<'a, str>>) -> Self
    where
        Message: 'static,
    {
        self.listing = Some(Options::view);
        self.options.language = Some(language.into());
        self
    }

    /// Set the state of one-based source lines, such as `6..=10`.
    pub fn line_style(mut self, lines: impl Into<LineRange>, style: LineStyle) -> Self
    where
        Message: 'static,
    {
        self.listing = Some(Options::view);
        for line in lines.into().0 {
            self.options.styles.insert(line, style);
        }
        self
    }

    /// Wrap long lines instead of scrolling. Defaults to `false`.
    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }

    /// Number each line. The numbers are not selectable. Defaults to `false`.
    pub fn line_numbers(mut self, line_numbers: bool) -> Self {
        self.line_numbers = line_numbers;
        self
    }

    /// The number of the first line, when line numbers are on. Defaults to `1`.
    pub fn start_line(mut self, first: usize) -> Self {
        self.start_line = first;
        self
    }

    /// An exact content height.
    pub fn height(mut self, height: impl Into<Pixels>) -> Self {
        self.height = Some(Pixels(height.into().0.max(0.)));
        self
    }

    /// Cap the content height without enlarging short output, like `max-h-48`.
    pub fn max_height(mut self, height: impl Into<Pixels>) -> Self {
        self.max_height = Some(Pixels(height.into().0.max(0.)));
        self
    }

    /// Clip capped content instead of scrolling it vertically (`overflow-y-hidden`).
    pub fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }

    /// Identify the focusable content for focus and scroll operations.
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Include the content in keyboard traversal. Defaults to `true`.
    pub fn focusable(mut self, focusable: bool) -> Self {
        self.focusable = focusable;
        self
    }

    /// The code and line number font. Defaults to [`Font::MONOSPACE`].
    pub fn font(mut self, font: impl Into<Font>) -> Self {
        self.font = font.into();
        self
    }

    /// Edit the source in place. The source is the controlled value: the editor
    /// always shows it, and an edit publishes the complete new source.
    pub fn on_input(mut self, on_input: impl Fn(String) -> Message + 'a) -> Self {
        self.on_input = Some(Box::new(on_input));
        self
    }

    /// Edit in place, starting from `source`. The editor keeps its own text and
    /// [`on_input`](Self::on_input) only reports edits; the content's own source
    /// is unused.
    pub fn default_value(mut self, source: impl Into<String>) -> Self {
        self.default_value = Some(source.into());
        self
    }

    pub(super) fn inherit(&mut self, from: &Inherited) {
        self.disabled |= from.is_disabled();
        if let Body::Lines(lines) = &mut self.body {
            for line in lines {
                line.content.inherit(from);
            }
        }
    }

    pub(super) fn view(self, compact: bool, disabled: bool) -> (Slot<'a, Message>, Viewport) {
        let Self {
            body,
            options,
            listing,
            wrap,
            line_numbers,
            start_line,
            height,
            max_height,
            collapsed,
            id,
            focusable,
            font,
            on_input,
            default_value,
            disabled: _,
        } = self;
        let layout = listing::Layout {
            font,
            wrap,
            compact,
            start_line: line_numbers.then_some(start_line),
        };
        let padding = if compact { 10. } else { 12. };
        let width = if wrap { Fill } else { Shrink };
        let editable =
            matches!(body, Body::Text(_)) && (on_input.is_some() || default_value.is_some());
        let body: Slot<'a, Message> = match (body, listing) {
            (Body::Text(source), _) if editable => {
                let value = default_value.is_none().then(|| source.into_owned());
                Editor::new(
                    value,
                    default_value.unwrap_or_default(),
                    on_input,
                    options.language.map(Cow::into_owned),
                    options.styles,
                    layout,
                    id.clone(),
                    disabled,
                )
                .boxed()
                .into()
            }
            (Body::Text(source), Some(view)) => view(options, source, layout),
            (Body::Text(source), None) if !line_numbers => container(plain(source, layout))
                .width(width)
                .padding([0., padding])
                .into(),
            (Body::Text(source), None) => layout
                .render(
                    source
                        .strip_suffix('\n')
                        .unwrap_or(&source)
                        .split('\n')
                        .map(|row| {
                            let row = plain(Cow::Owned(row.to_owned()), layout);
                            (LineStyle::Default, row.into())
                        })
                        .collect(),
                )
                .into(),
            (Body::Lines(lines), _) => layout
                .render(
                    lines
                        .into_iter()
                        .enumerate()
                        .map(|(index, line)| {
                            let style = match line.style {
                                LineStyle::Default => options
                                    .styles
                                    .get(&(index + 1))
                                    .copied()
                                    .unwrap_or_default(),
                                style => style,
                            };
                            (style, line.content)
                        })
                        .collect(),
                )
                .into(),
        };
        let viewport = Viewport {
            height,
            max_height,
            collapsed,
            // The editor takes the focus and the id itself.
            id: if editable { None } else { id },
            focusable: focusable && !disabled && !editable,
            editable,
        };
        (body, viewport)
    }
}
