//! A Nova code panel with selectable output, a keyboard viewport and optional slots.
mod floating;
mod frame;
mod lines;
mod selection;
mod viewport;

use std::borrow::Cow;

use iced_core::Length::{Fill, Shrink};
use iced_core::alignment::{Horizontal, Vertical};
use iced_core::text::{LineHeight, Wrapping};
use iced_core::widget::Id;
use iced_core::{Element, Font, Padding};
use iced_widget::{Renderer, Widget, text};
use icedxdy_core::theme::container::{Style, StyleFn};
use icedxdy_core::{Border, Theme, radius};

use crate::compose::{Compose, Inherited, Slot};
use crate::scrollable::{Direction, scrollbar};
use crate::text::Ellipsis;
use crate::{Column, Weight, column, container, row, scrollable};

/// Presentation of a line. Added/removed markers are separate from selectable code.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub enum LineStyle {
    #[default]
    Default,
    Highlighted,
    Added,
    Removed,
}

/// Nova's error surface.
#[derive(Debug, Clone, Copy)]
pub struct Destructive;

impl<'a> From<Destructive> for StyleFn<'a, Theme> {
    fn from(_: Destructive) -> Self {
        Box::new(|theme| Style {
            text_color: Some(theme.destructive.base),
            border: Border {
                color: theme
                    .destructive
                    .base
                    .alpha(if theme.neutral.is_dark { 0.4 } else { 0.3 }),
                ..surface(theme).border
            },
            ..Style::default()
        })
    }
}

fn surface(theme: &Theme) -> Style {
    Style {
        text_color: Some(theme.neutral.foreground),
        background: theme
            .neutral
            .is_dark
            .then(|| theme.input.scale_alpha(0.3).into()),
        border: Border {
            width: 1.,
            radius: radius::LG.into(),
            color: theme.input,
        },
        ..Style::default()
    }
}

enum Content<'a, Message> {
    Text(Cow<'a, str>),
    Lines(Vec<(LineStyle, Slot<'a, Message>)>),
}

/// Plain output stays one selectable paragraph unless a line gutter is requested.
pub fn code_block<'a, Message>(source: impl Into<Cow<'a, str>>) -> CodeBlock<'a, Message> {
    CodeBlock {
        content: Content::Text(source.into()),
        title: None,
        actions: None,
        footer: None,
        start_line: None,
        wrap: false,
        compact: false,
        height: None,
        max_height: None,
        collapsed: false,
        id: None,
        focusable: true,
        style: Box::new(surface),
        font: Font::MONOSPACE,
    }
}

/// A title that truncates within the available header width.
pub fn title<'a>(value: impl iced_core::text::IntoFragment<'a>) -> crate::Text<'a> {
    crate::text(value)
        .medium()
        .width(Fill)
        .wrapping(Wrapping::None)
        .ellipsis(Ellipsis::End)
}

/// Builder for the surface, header, selectable content and footer.
pub struct CodeBlock<'a, Message> {
    content: Content<'a, Message>,
    title: Option<Slot<'a, Message>>,
    actions: Option<Slot<'a, Message>>,
    footer: Option<Slot<'a, Message>>,
    start_line: Option<usize>,
    wrap: bool,
    compact: bool,
    height: Option<f32>,
    max_height: Option<f32>,
    collapsed: bool,
    id: Option<Id>,
    focusable: bool,
    style: StyleFn<'static, Theme>,
    font: Font,
}

impl<'a, Message: 'a> CodeBlock<'a, Message> {
    /// Supply styled content for each line. Do not include diff markers in the slots.
    pub fn lines(lines: impl IntoIterator<Item = (LineStyle, Slot<'a, Message>)>) -> Self {
        Self {
            content: Content::Lines(lines.into_iter().collect()),
            ..code_block("")
        }
    }
    /// Place a title in the header. Use [`title`] for a long filename.
    pub fn title(mut self, title: impl Into<Slot<'a, Message>>) -> Self {
        self.title = Some(title.into());
        self
    }
    /// Header controls, or floating controls when the header is absent.
    pub fn actions(mut self, actions: impl Into<Slot<'a, Message>>) -> Self {
        self.actions = Some(actions.into());
        self
    }
    /// Controls below the content, such as a Show More button.
    pub fn footer(mut self, footer: impl Into<Slot<'a, Message>>) -> Self {
        self.footer = Some(footer.into());
        self
    }
    /// Number rows starting at this value. The gutter is not selectable.
    pub fn start_line(mut self, first: usize) -> Self {
        self.start_line = Some(first);
        self
    }
    /// Wrap long lines instead of scrolling horizontally. Defaults to false.
    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }
    /// Use 12px code and 10px padding. Defaults to false (14px and 12px).
    pub fn compact(mut self, compact: bool) -> Self {
        self.compact = compact;
        self
    }
    /// Set an exact content viewport height.
    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height.max(0.));
        self
    }
    /// Cap the content height without enlarging short output.
    pub fn max_height(mut self, height: f32) -> Self {
        self.max_height = Some(height.max(0.));
        self
    }
    /// Clip a capped listing without vertical scrolling, for collapsed Show More.
    pub fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }
    /// Identify the focusable content for native focus/scroll operations.
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }
    /// Include the content in keyboard traversal. Defaults to true.
    pub fn focusable(mut self, focusable: bool) -> Self {
        self.focusable = focusable;
        self
    }
    /// Set the code and gutter font. Styled child slots may override it explicitly.
    pub fn font(mut self, font: Font) -> Self {
        self.font = font;
        self
    }
    /// Set the panel style, for example [`Destructive`], a container variant or a closure.
    pub fn style(mut self, style: impl Into<StyleFn<'static, Theme>>) -> Self {
        self.style = style.into();
        self
    }
}

impl<'a, Message: 'a> Compose<Message> for CodeBlock<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        for part in [&mut self.title, &mut self.actions, &mut self.footer]
            .into_iter()
            .flatten()
        {
            part.inherit(from);
        }
        if let Content::Lines(lines) = &mut self.content {
            for (_, line) in lines {
                line.inherit(from);
            }
        }
        if from.is_disabled() {
            self.focusable = false;
        }
    }
    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let Self {
            content,
            mut title,
            mut actions,
            mut footer,
            start_line,
            wrap,
            compact,
            height,
            max_height,
            collapsed,
            id,
            focusable,
            style,
            font,
        } = *self;
        let size = if compact { 12. } else { 14. };
        let padding = if compact { 10. } else { 12. };
        let line_height = size * 1.625;
        let has_header = title.is_some();
        let has_footer = footer.is_some();
        let wrapping = if wrap {
            Wrapping::WordOrGlyph
        } else {
            Wrapping::None
        };
        let plain = |s: Cow<'a, str>| {
            text(s)
                .size(size)
                .font(font)
                .line_height(LineHeight::Absolute(line_height.into()))
                .wrapping(wrapping)
                .selectable(true)
        };
        let mut panel = column![];
        if let Some(mut title) = title.take() {
            title.inherit(
                &Inherited::default()
                    .with_size(size)
                    .with_weight(Weight::Medium)
                    .with_line_height(if compact { 16. } else { 20. }),
            );
            let mut header = row![container(title).width(Fill).clip(true)]
                .spacing(8)
                .align_y(Vertical::Center);
            if let Some(actions) = actions.take() {
                header = header.push(frame::Frame::actions(actions.compose()).boxed().into());
            }
            panel = panel.push(
                container(header)
                    .padding(Padding {
                        top: padding,
                        left: padding,
                        right: padding,
                        bottom: 0.,
                    })
                    .width(Fill)
                    .into(),
            );
        }
        let content_width = if wrap { Fill } else { Shrink };
        let render_lines = |lines: Vec<(LineStyle, Slot<'a, Message>)>| {
            let last = start_line
                .unwrap_or(1)
                .saturating_add(lines.len().saturating_sub(1));
            let gutter = (last.to_string().len() as f32 * size * 0.61).max(24.);
            let styles = lines.iter().map(|(style, _)| *style).collect();
            let column = Column::with_children(lines.into_iter().enumerate().map(
                |(index, (variant, mut line))| {
                    line.inherit(
                        &Inherited::default()
                            .with_size(size)
                            .with_line_height(line_height),
                    );
                    let mut entry = row![].spacing(12);
                    if let Some(first) = start_line {
                        entry = entry.push(
                            text(first.saturating_add(index).to_string())
                                .size(size)
                                .font(font)
                                .line_height(LineHeight::Absolute(line_height.into()))
                                .width(gutter)
                                .align_x(Horizontal::Right)
                                .selectable(false)
                                .style(|theme: &Theme| iced_widget::text::Style {
                                    color: Some(theme.muted.foreground.into()),
                                    ..Default::default()
                                })
                                .into(),
                        );
                    }
                    let row = container(entry.push(line).width(content_width))
                        .width(content_width)
                        .min_height(line_height)
                        .padding([0., padding]);
                    let glyph = match variant {
                        LineStyle::Added => Some("+"),
                        LineStyle::Removed => Some("−"),
                        _ => None,
                    };
                    if let Some(glyph) = glyph {
                        let mark: Element<'_, Message, Theme, Renderer> = container(
                            text(glyph)
                                .size(size)
                                .font(font)
                                .line_height(LineHeight::Absolute(line_height.into()))
                                .selectable(false)
                                .style(move |t: &Theme| iced_widget::text::Style {
                                    color: Some(
                                        if variant == LineStyle::Removed {
                                            t.destructive.base
                                        } else {
                                            t.muted.foreground
                                        }
                                        .into(),
                                    ),
                                    ..Default::default()
                                }),
                        )
                        .padding([0, 0, 0, 4])
                        .into();
                        Slot::from(iced_widget::stack![Element::from(row), mark].boxed())
                    } else {
                        row.into()
                    }
                },
            ))
            .width(content_width);
            lines::Lines::new(column.into(), styles).boxed()
        };
        let body: Slot<'_, Message> = match content {
            Content::Text(value) if start_line.is_none() => container(plain(value))
                .width(content_width)
                .padding([0., padding])
                .into(),
            Content::Text(value) => render_lines(
                value
                    .strip_suffix('\n')
                    .unwrap_or(&value)
                    .split('\n')
                    .map(|s| (LineStyle::Default, plain(Cow::Owned(s.to_owned())).into()))
                    .collect(),
            )
            .into(),
            Content::Lines(lines) => render_lines(lines).into(),
        };
        let body = container(body)
            .padding(Padding {
                top: if has_header {
                    if compact { 6. } else { 8. }
                } else {
                    padding
                },
                bottom: if has_footer { 8. } else { padding },
                left: 0.,
                right: 0.,
            })
            .width(content_width);
        let direction = if wrap {
            Direction::Vertical(scrollbar())
        } else {
            Direction::Both {
                vertical: scrollbar(),
                horizontal: scrollbar(),
            }
        };
        let mut view = scrollable(body).direction(direction).width(Fill);
        if let Some(height) = height {
            view = view.height(height);
        } else if let Some(max) = max_height {
            view = view.height(Shrink.max(max));
        }
        let body: Element<'_, Message, Theme, Renderer> = viewport::Viewport::new(
            view.into(),
            id,
            focusable,
            collapsed,
            wrap || height.is_some() || max_height.is_some(),
        )
        .boxed();
        let body: Slot<'_, Message> = if let Some(actions) = actions {
            let top: Element<'_, Message, Theme, Renderer> = container(actions)
                .width(Fill)
                .align_x(Horizontal::Right)
                .padding(if compact { 6 } else { 8 })
                .into();
            floating::Hover::new(body, top).boxed().into()
        } else {
            body.into()
        };
        panel = panel.push(body);
        if let Some(mut footer) = footer.take() {
            footer.inherit(&Inherited::default().with_size(size));
            panel = panel.push(
                container(footer)
                    .padding(Padding {
                        bottom: padding,
                        left: padding,
                        right: padding,
                        top: 0.,
                    })
                    .width(Fill)
                    .into(),
            );
        }
        frame::Frame::new(
            container(panel)
                .padding(1)
                .width(Fill)
                .clip(true)
                .style(style)
                .into(),
        )
        .boxed()
    }
}

fn line_style(theme: &Theme, style: LineStyle) -> Style {
    let color = match style {
        LineStyle::Default => return Style::default(),
        LineStyle::Highlighted | LineStyle::Added => theme.muted.base,
        LineStyle::Removed => theme.destructive.base.alpha(0.1),
    };
    Style {
        background: Some(color.into()),
        ..Style::default()
    }
}
impl<'a, Message: 'a> From<CodeBlock<'a, Message>> for Slot<'a, Message> {
    fn from(block: CodeBlock<'a, Message>) -> Self {
        Slot::new(block)
    }
}
impl<'a, Message: 'a> From<CodeBlock<'a, Message>> for Element<'a, Message, Theme, Renderer> {
    fn from(block: CodeBlock<'a, Message>) -> Self {
        Box::new(block).compose()
    }
}
