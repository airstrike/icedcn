//! Selectable source rows and their outside-the-selection decorations.
use super::{LineStyle, lines};
use crate::compose::Inherited;
use crate::{Column, Slot, container, row};
use iced_core::Length::{Fill, Shrink};
use iced_core::alignment::Horizontal;
use iced_core::text::{LineHeight, Wrapping};
use iced_core::{Element, Font};
use iced_widget::{Renderer, Widget, text};
use icedcn_core::Theme;
use std::borrow::Cow;
use std::collections::BTreeMap;

// Only listings use iced's lazy widget, whose cached element requires a static message.
// Plain blocks and user-supplied slots continue to accept borrowed messages.
pub(super) type View<'a, Message> = fn(Options<'a>, Cow<'a, str>, Layout) -> Slot<'a, Message>;

#[derive(Debug, Default, Hash)]
pub(super) struct Options<'a> {
    pub styles: BTreeMap<usize, LineStyle>,
    pub diff: bool,
    pub language: Option<Cow<'a, str>>,
}

impl<'a> Options<'a> {
    pub fn view<Message: 'static>(self, source: Cow<'a, str>, layout: Layout) -> Slot<'a, Message> {
        let body = iced_widget::lazy((source, self, layout), |(source, options, layout)| {
            // Parse the visible source once per change, including an unfinished final line.
            let mut styles = Vec::new();
            let source = if options.diff {
                Cow::Owned(
                    source
                        .split('\n')
                        .map(|row| {
                            let (style, text) = diff_line(row);
                            styles.push(style);
                            text
                        })
                        .collect::<Vec<_>>()
                        .join("\n"),
                )
            } else {
                Cow::Borrowed(source.as_ref())
            };
            let lines = options.rows(&source, *layout);
            let lines = lines
                .into_iter()
                .enumerate()
                .map(|(index, line)| {
                    let style = options
                        .styles
                        .get(&(index + 1))
                        .or_else(|| styles.get(index))
                        .copied()
                        .unwrap_or_default();
                    (style, line)
                })
                .collect();
            layout.render::<Message>(lines)
        });
        body.boxed().into()
    }

    fn rows<Message: 'static>(&self, source: &str, layout: Layout) -> Vec<Slot<'static, Message>> {
        #[cfg(feature = "highlighter")]
        if let Some(language) = &self.language {
            return super::syntax::rows(source, language)
                .into_iter()
                .map(|spans| {
                    crate::rich_text(spans)
                        .font(layout.font)
                        .size(layout.size())
                        .line_height(LineHeight::Absolute(layout.line_height().into()))
                        .wrapping(layout.wrapping())
                        .width(if layout.wrap { Fill } else { Shrink })
                        .into()
                })
                .collect();
        }
        source
            .split('\n')
            .map(|line| {
                text(line.to_owned())
                    .font(layout.font)
                    .size(layout.size())
                    .line_height(LineHeight::Absolute(layout.line_height().into()))
                    .wrapping(layout.wrapping())
                    .selectable(true)
                    .into()
            })
            .collect()
    }
}

pub(super) fn diff_line(line: &str) -> (LineStyle, &str) {
    match line.as_bytes().first() {
        Some(b'+') => (LineStyle::Added, &line[1..]),
        Some(b'-') => (LineStyle::Removed, &line[1..]),
        Some(b' ') => (LineStyle::Default, &line[1..]),
        _ => (LineStyle::Default, line),
    }
}

#[derive(Debug, Clone, Copy, Hash)]
pub(super) struct Layout {
    pub font: Font,
    pub wrap: bool,
    pub compact: bool,
    pub start_line: Option<usize>,
}
impl Layout {
    fn size(self) -> f32 {
        if self.compact { 12.0 } else { 14.0 }
    }
    fn line_height(self) -> f32 {
        self.size() * 1.625
    }
    fn wrapping(self) -> Wrapping {
        if self.wrap {
            Wrapping::WordOrGlyph
        } else {
            Wrapping::None
        }
    }
    pub fn render<'a, Message: 'a>(
        self,
        lines: Vec<(LineStyle, Slot<'a, Message>)>,
    ) -> Element<'a, Message, Theme, Renderer> {
        let Self {
            font,
            start_line,
            wrap,
            compact,
        } = self;
        let size = self.size();
        let line_height = self.line_height();
        let padding = if compact { 10.0 } else { 12.0 };
        let content_width = if wrap { Fill } else { Shrink };
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
    }
}
