//! A code or terminal-output panel with optional actions, a line gutter and a footer.
//! Supply plain text for streaming output, or individual lines for a highlighted listing.

use std::borrow::Cow;

use iced_core::Length::{Fill, Shrink};
use iced_core::alignment::Horizontal;
use iced_core::text::Wrapping;
use iced_core::{Element, Font};
use iced_widget::{Renderer, Widget, hover, text};
use icedxdy_core::Theme;
use icedxdy_core::theme;
use icedxdy_core::theme::container::{Spec, Style};

use crate::compose::{Compose, Inherited, Slot};
use crate::scrollable::{Direction, scrollbar};
use crate::{Column, column, container, row, scrollable, separator, space};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub enum LineStyle {
    #[default]
    Default,
    Highlighted,
    Added,
    Removed,
}

enum Content<'a, Message> {
    Text(Cow<'a, str>),
    Lines(Vec<(LineStyle, Slot<'a, Message>)>),
}

/// A string is displayed as one text widget unless the line gutter is enabled.
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
        style: theme::container::CodeBlock.into(),
        font: Font::MONOSPACE,
    }
}

pub struct CodeBlock<'a, Message> {
    content: Content<'a, Message>,
    title: Option<Slot<'a, Message>>,
    actions: Option<Slot<'a, Message>>,
    footer: Option<Slot<'a, Message>>,
    start_line: Option<usize>,
    wrap: bool,
    compact: bool,
    height: Option<f32>,
    style: Spec,
    font: Font,
}

impl<'a, Message: 'a> CodeBlock<'a, Message> {
    /// Use pre-styled line contents such as syntax-highlighted spans.
    pub fn lines(lines: impl IntoIterator<Item = (LineStyle, Slot<'a, Message>)>) -> Self {
        Self {
            content: Content::Lines(lines.into_iter().collect()),
            ..code_block("")
        }
    }

    pub fn title(mut self, title: impl Into<Slot<'a, Message>>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn actions(mut self, actions: impl Into<Slot<'a, Message>>) -> Self {
        self.actions = Some(actions.into());
        self
    }

    pub fn footer(mut self, footer: impl Into<Slot<'a, Message>>) -> Self {
        self.footer = Some(footer.into());
        self
    }

    /// Enable line numbers starting at the supplied number.
    pub fn start_line(mut self, first: usize) -> Self {
        self.start_line = Some(first);
        self
    }

    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }

    pub fn compact(mut self, compact: bool) -> Self {
        self.compact = compact;
        self
    }

    /// Scroll the output independently inside a fixed-height viewport.
    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn font(mut self, font: Font) -> Self {
        self.font = font;
        self
    }

    pub fn style(mut self, style: impl Into<Spec>) -> Self {
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
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let Self {
            content,
            title,
            mut actions,
            footer,
            start_line,
            wrap,
            compact,
            height,
            style,
            font,
        } = *self;
        let size = if compact { 12.0 } else { 14.0 };
        let padding = if compact { 8 } else { 12 };
        let wrapping = if wrap {
            Wrapping::WordOrGlyph
        } else {
            Wrapping::None
        };
        let plain = |s: Cow<'a, str>| {
            text(s)
                .size(size)
                .font(font)
                .wrapping(wrapping)
                .selectable(true)
        };
        let mut panel = column![];
        if title.is_some() {
            let mut header = row![];
            if let Some(title) = title {
                header = header.push(title);
            }
            header = header.push(space::horizontal().into());
            if let Some(actions) = actions.take() {
                header = header.push(actions);
            }
            panel = panel
                .push(container(header).padding([4, padding]).width(Fill).into())
                .push(separator().into());
        }
        let content_width = if wrap { Fill } else { Shrink };
        let render_lines = |lines: Vec<(LineStyle, Slot<'a, Message>)>| {
            let last = start_line
                .unwrap_or(1)
                .saturating_add(lines.len().saturating_sub(1));
            let gutter = (last.to_string().len() + 1) as f32 * size * 0.65;
            Column::with_children(lines.into_iter().enumerate().map(|(index, (style, line))| {
                let mut entry = row![].spacing(8);
                if let Some(first) = start_line {
                    let number = text(first.saturating_add(index).to_string())
                        .size(size)
                        .font(font)
                        .width(gutter)
                        .align_x(Horizontal::Right)
                        .style(|theme: &Theme| iced_widget::text::Style {
                            color: Some(theme.muted.foreground.into()),
                            ..Default::default()
                        });
                    entry = entry.push(number.into());
                }
                container(entry.push(line).width(content_width))
                    .width(content_width)
                    .padding([0, padding])
                    .style(move |theme| line_style(theme, style))
                    .into()
            }))
            .width(content_width)
        };
        let body: Slot<'_, Message> = match content {
            Content::Text(value) if start_line.is_none() => container(plain(value))
                .width(content_width)
                .padding([0, padding])
                .into(),
            Content::Text(value) => render_lines(
                value
                    .split('\n')
                    .map(|s| (LineStyle::Default, plain(Cow::Owned(s.to_owned())).into()))
                    .collect(),
            )
            .into(),
            Content::Lines(lines) => render_lines(lines).into(),
        };
        let body = container(body).padding([padding, 0]).width(content_width);
        let body: Slot<'_, Message> = match (height, wrap) {
            (None, true) => body.into(),
            (height, wrap) => {
                let direction = match (height, wrap) {
                    (Some(_), false) => Direction::Both {
                        vertical: scrollbar(),
                        horizontal: scrollbar(),
                    },
                    (_, true) => Direction::Vertical(scrollbar()),
                    (_, false) => Direction::Horizontal(scrollbar()),
                };
                let mut viewport = scrollable(body).direction(direction).width(Fill);
                if let Some(height) = height {
                    viewport = viewport.height(height);
                }
                viewport.into()
            }
        };
        let body = if let Some(actions) = actions {
            let top = container(actions)
                .width(Fill)
                .align_x(Horizontal::Right)
                .padding(4);
            Slot::from(hover(body.compose(), Element::from(top)).boxed())
        } else {
            body
        };
        panel = panel.push(body);
        if let Some(footer) = footer {
            panel = panel
                .push(separator().into())
                .push(container(footer).padding([4, padding]).width(Fill).into());
        }
        container(panel).width(Fill).clip(true).class(style).into()
    }
}

fn line_style(theme: &Theme, style: LineStyle) -> Style {
    let tint = match style {
        LineStyle::Default => return Style::default(),
        LineStyle::Highlighted => theme.muted.foreground,
        LineStyle::Added => theme.success.base,
        LineStyle::Removed => theme.destructive.base,
    };
    Style {
        background: Some(tint.alpha(0.13).into()),
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
