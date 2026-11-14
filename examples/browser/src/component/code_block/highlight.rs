//! Render retained pinned Shiki runs for these fixed fixtures; no parsing in view.
use super::samples::*;
use super::tokens;
use crate::font;
use iced::widget::text::{LineHeight, Wrapping};
use iced::{Fill, Font, Shrink};
use icedxdy::theme::tweak::Token;
use icedxdy::widget::code_block::{CodeBlock, LineStyle};
use icedxdy::widget::{rich_text, span};
use icedxdy::{Color, Slot};

pub const STREAMS: &[(&str, &str, &str)] = &[
    ("hooks/use-debounce.ts", "ts", STREAMING_0),
    ("Terminal", "sh", STREAMING_1),
    ("components/search.tsx", "tsx", STREAMING_2),
    ("app/api/chat/route.ts", "ts", STREAMING_3),
    (".env.local", "sh", STREAMING_4),
];
fn rgb(hex: u32) -> Token {
    Color::from(iced::Color::from_rgba8(
        (hex >> 24) as u8,
        (hex >> 16) as u8,
        (hex >> 8) as u8,
        (hex & 255) as f32 / 255.,
    ))
    .into()
}

/// Render a retained prefix, including the grammar of an unfinished final line.
pub fn listing<Message: 'static>(
    dark: bool,
    source: &'static str,
    visible: usize,
    wrap: bool,
    styles: impl Fn(usize, &str) -> LineStyle,
) -> CodeBlock<'static, Message> {
    let wrapping = if wrap {
        Wrapping::WordOrGlyph
    } else {
        Wrapping::None
    };
    let mut end = visible.min(source.len());
    while !source.is_char_boundary(end) {
        end -= 1;
    }
    let last = source[..end].bytes().filter(|b| *b == b'\n').count();
    let lines = source[..end].split('\n').enumerate().map(|(i, line)| {
        let original = line;
        let line = if source == DIFF || source == STREAMING_2 {
            line.get(1..).unwrap_or("")
        } else {
            line
        };
        let mut spans = Vec::new();
        if let Some(runs) = tokens::row(source, i, end, i == last) {
            let mut pos = 0;
            for &(end, light, night) in runs {
                let start = pos;
                let end = end.min(line.len());
                if start > pos {
                    spans.push(span(&line[pos..start]));
                }
                if end > start {
                    spans.push(span(&line[start..end]).color(rgb(if dark {
                        night
                    } else {
                        light
                    })));
                }
                pos = end;
            }
            if pos < line.len() {
                spans.push(span(&line[pos..]));
            }
        } else {
            spans.push(span(line));
        }
        if line.is_empty() {
            spans = vec![span("")];
        }
        let text = rich_text(spans)
            .font(Font::new(font::GEIST_MONO_NAME))
            .size(14)
            .line_height(LineHeight::Absolute(22.75.into()))
            .wrapping(wrapping)
            .width(if wrap { Fill } else { Shrink });
        (styles(i, original), Slot::from(text))
    });
    CodeBlock::lines(lines)
        .wrap(wrap)
        .font(Font::new(font::GEIST_MONO_NAME))
}

pub fn ansi<Message: 'static>(dark: bool) -> (CodeBlock<'static, Message>, String) {
    (
        listing(dark, tokens::ANSI_PLAIN, usize::MAX, true, |_, _| {
            LineStyle::Default
        }),
        tokens::ANSI_PLAIN.to_owned(),
    )
}
