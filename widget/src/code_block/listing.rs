//! Selectable source rows and their outside-the-selection decorations.
use super::{LineStyle, ansi, lines};
use crate::compose::Inherited;
use crate::{Column, View, container, row};
use iced_core::Length::{Fill, Shrink};
use iced_core::alignment::Horizontal;
use iced_core::text::{LineHeight, Wrapping};
use iced_core::{Element, Font, Padding};
use iced_widget::{Renderer, Widget, text};
use icedcn_core::Theme;
use std::borrow::Cow;
use std::collections::BTreeMap;

// Only listings use iced's lazy widget, whose cached element requires a static message.
// Plain blocks and user-supplied slots continue to accept borrowed messages.
pub(super) type Render<'a, Message> = fn(Options<'a>, Cow<'a, str>, Layout) -> View<'a, Message>;

#[derive(Debug, Default, Hash)]
pub(super) struct Options<'a> {
    pub styles: BTreeMap<usize, LineStyle>,
    pub language: Option<Cow<'a, str>>,
}

impl<'a> Options<'a> {
    pub fn view<Message: 'static>(self, source: Cow<'a, str>, layout: Layout) -> View<'a, Message> {
        let body = iced_widget::lazy((source, self, layout), |(source, options, layout)| {
            // Parse the visible source once per change, including an unfinished final line.
            let lines = options
                .rows(source, *layout)
                .into_iter()
                .enumerate()
                .map(|(index, line)| {
                    let style = options
                        .styles
                        .get(&(index + 1))
                        .copied()
                        .unwrap_or_default();
                    (style, line)
                })
                .collect();
            layout.render::<Message>(lines)
        });
        body.boxed().into()
    }

    fn rows<Message: 'static>(&self, source: &str, layout: Layout) -> Vec<View<'static, Message>> {
        if self
            .language
            .as_deref()
            .is_some_and(|language| language.eq_ignore_ascii_case("ansi"))
        {
            let width = if layout.wrap { Fill } else { Shrink };
            return ansi::parse(source)
                .lines(layout.font, layout.wrapping(), width, layout.size())
                .into_iter()
                .map(|(_, line)| line)
                .collect();
        }
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

#[derive(Debug, Clone, Copy, Hash)]
pub(super) struct Layout {
    pub font: Font,
    pub wrap: bool,
    pub compact: bool,
    pub start_line: Option<usize>,
}
impl Layout {
    pub(super) fn size(self) -> f32 {
        if self.compact { 12.0 } else { 14.0 }
    }
    pub(super) fn line_height(self) -> f32 {
        self.size() * 1.625
    }
    pub(super) fn wrapping(self) -> Wrapping {
        if self.wrap {
            Wrapping::WordOrGlyph
        } else {
            Wrapping::None
        }
    }
    pub fn render<'a, Message: 'a>(
        self,
        lines: Vec<(LineStyle, View<'a, Message>)>,
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
        // The line numbers stay put while the code scrolls sideways, so they
        // and the diff marks are drawn by `Lines`, apart from the rows.
        let pin = start_line.map(|_| padding + gutter + 12.);
        let styles: Vec<LineStyle> = lines.iter().map(|(style, _)| *style).collect();
        let mark = move |variant: LineStyle| -> Option<Element<'a, Message, Theme, Renderer>> {
            let glyph = match variant {
                LineStyle::Added => "+",
                LineStyle::Removed => "−",
                _ => return None,
            };
            Some(
                container(
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
                .into(),
            )
        };
        let numbers = start_line.map(|first| {
            styles
                .iter()
                .enumerate()
                .map(|(index, variant)| {
                    let number: Element<'a, Message, Theme, Renderer> = container(
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
                            }),
                    )
                    .padding(Padding {
                        left: padding,
                        ..Padding::ZERO
                    })
                    .into();
                    match mark(*variant) {
                        Some(mark) => iced_widget::stack![number, mark].boxed(),
                        None => number,
                    }
                })
                .collect::<Vec<_>>()
        });
        let column = Column::with_children(lines.into_iter().map(|(variant, mut line)| {
            line.inherit(
                &Inherited::default()
                    .with_size(size)
                    .with_line_height(LineHeight::Absolute(line_height.into()))
                    .with_font(font)
                    .with_selectable(true),
            );
            let row = container(row![line].width(content_width))
                .width(content_width)
                .min_height(line_height)
                .padding(Padding {
                    left: pin.unwrap_or(padding),
                    right: padding,
                    ..Padding::ZERO
                });
            match mark(variant).filter(|_| pin.is_none()) {
                Some(mark) => View::from(iced_widget::stack![Element::from(row), mark].boxed()),
                None => row.into(),
            }
        }))
        .width(content_width);
        lines::Lines::new(
            column.into(),
            styles,
            numbers.unwrap_or_default(),
            pin.unwrap_or(0.),
        )
        .boxed()
    }
}
