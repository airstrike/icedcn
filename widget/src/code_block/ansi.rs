//! Terminal output with SGR foreground colors and intensity, for `.language("ansi")`.
use std::sync::Arc;

use iced_core::text::{LineHeight, Wrapping};
use iced_core::{Font, Length};
use icedcn_core::Color;

use super::LineStyle;
use crate::{View, rich_text, span};

/// A parsed transcript retains plain text for copying and colored runs for each line.
#[derive(Clone)]
pub struct Output(Arc<Transcript>);

struct Transcript {
    plain: String,
    lines: Vec<Vec<Run>>,
}

struct Run {
    end: usize,
    ink: Ink,
}

#[derive(Clone, Copy, Default, PartialEq)]
struct Ink {
    index: Option<usize>,
    dim: bool,
}

/// The text of a transcript without its SGR escape sequences, for a copy button.
pub fn strip(source: &str) -> String {
    parse(source).0.plain.clone()
}

/// Parse SGR foreground colors, dim intensity and resets. Each new line starts in the default color.
pub fn parse(source: &str) -> Output {
    let mut plain = String::with_capacity(source.len());
    let mut lines = Vec::new();
    for (index, line) in source.split('\n').enumerate() {
        if index != 0 {
            plain.push('\n');
        }
        let mut runs = Vec::new();
        let mut ink = Ink::default();
        let mut rest = line;
        while let Some((before, escape)) = rest.split_once("\x1b[") {
            append(&mut plain, &mut runs, before, ink);
            let length = escape
                .bytes()
                .take_while(|b| b.is_ascii_digit() || *b == b';')
                .count();
            if let Some(after) = escape[length..].strip_prefix('m') {
                ink.apply(&escape[..length]);
                rest = after;
            } else {
                append(&mut plain, &mut runs, "\x1b[", ink);
                rest = escape;
            }
        }
        append(&mut plain, &mut runs, rest, ink);
        lines.push(runs);
    }
    Output(Arc::new(Transcript { plain, lines }))
}

fn append(plain: &mut String, runs: &mut Vec<Run>, text: &str, ink: Ink) {
    if !text.is_empty() {
        plain.push_str(text);
        runs.push(Run {
            end: plain.len(),
            ink,
        });
    }
}

impl Ink {
    fn apply(&mut self, codes: &str) {
        let mut codes = codes.split(';').map(|code| {
            if code.is_empty() {
                Some(0)
            } else {
                code.parse::<u16>().ok()
            }
        });
        while let Some(code) = codes.next() {
            match code {
                Some(0) => *self = Self::default(),
                Some(2) => self.dim = true,
                Some(22) => self.dim = false,
                Some(n @ 30..=37) => self.index = Some(usize::from(n - 30)),
                Some(n @ 90..=97) => self.index = Some(8 + usize::from(n - 90)),
                Some(39) => self.index = None,
                // Extended foreground/background parameters belong to this one instruction.
                Some(38 | 48 | 58) => match codes.next() {
                    Some(Some(2)) => {
                        codes.by_ref().take(3).for_each(drop);
                    }
                    Some(Some(5)) => {
                        codes.next();
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    fn colors(self) -> (Color, Color) {
        const COLORS: [(u32, u32); 16] = [
            (0x24292e, 0x586069),
            (0xd73a49, 0xea4a5a),
            (0x28a745, 0x34d058),
            (0xdbab09, 0xffea7f),
            (0x0366d6, 0x2188ff),
            (0x5a32a3, 0xb392f0),
            (0x1b7c83, 0x39c5cf),
            (0x6a737d, 0xd1d5da),
            (0x959da5, 0x959da5),
            (0xcb2431, 0xf97583),
            (0x22863a, 0x85e89d),
            (0xb08800, 0xffea7f),
            (0x005cc5, 0x79b8ff),
            (0x5a32a3, 0xb392f0),
            (0x3192aa, 0x56d4dd),
            (0xd1d5da, 0xfafbfc),
        ];
        let (light, dark) = self
            .index
            .map_or((0x24292e, 0xe1e4e8), |index| COLORS[index]);
        let alpha = if self.dim { 128. / 255. } else { 1. };
        let rgba = |n| Color::from(iced_core::Color::from_packed_rgba8(n, alpha));
        (rgba(light), rgba(dark))
    }
}

impl Output {
    /// The complete transcript without SGR escape sequences, including blank lines.
    pub fn as_str(&self) -> &str {
        &self.0.plain
    }

    pub(super) fn lines<'a, M: 'a>(
        &self,
        font: Font,
        wrapping: Wrapping,
        width: Length,
        size: f32,
    ) -> Vec<(LineStyle, View<'a, M>)> {
        let mut start = 0;
        self.0
            .lines
            .iter()
            .map(|runs| {
                let spans = runs.iter().map(|run| {
                    let text = &self.0.plain[start..run.end];
                    start = run.end;
                    let (light, dark) = run.ink.colors();
                    span(text.to_owned()).color_pair(light, dark)
                });
                let line = rich_text(spans)
                    .font(font)
                    .size(size)
                    .line_height(LineHeight::Absolute((size * 1.625).into()))
                    .wrapping(wrapping)
                    .width(width);
                start += 1;
                (LineStyle::Default, line.into())
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reset_and_dim_colors_match_the_terminal_palette_in_both_themes() {
        let output = parse("λ\x1b[2;31mred\x1b[22m!\x1b[39m.\n\n\x1b[32mgreen\nplain\n");
        assert_eq!(output.as_str(), "λred!.\n\ngreen\nplain\n");
        let rows = &output.0.lines;
        assert_eq!(rows.len(), 5);
        assert!(rows[1].is_empty() && rows[4].is_empty());
        assert_eq!(
            rows[0].iter().map(|run| run.end).collect::<Vec<_>>(),
            [2, 5, 6, 7]
        );
        let rgba = |run: &Run| {
            let (light, dark) = run.ink.colors();
            (
                iced_core::Color::from(light).into_rgba8(),
                iced_core::Color::from(dark).into_rgba8(),
            )
        };
        assert_eq!(rgba(&rows[0][1]), ([215, 58, 73, 128], [234, 74, 90, 128]));
        assert_eq!(rgba(&rows[0][2]), ([215, 58, 73, 255], [234, 74, 90, 255]));
        assert_eq!(rgba(&rows[2][0]), ([40, 167, 69, 255], [52, 208, 88, 255]));
        assert_eq!(rgba(&rows[3][0]), ([36, 41, 46, 255], [225, 228, 232, 255]));
    }

    #[test]
    fn unsupported_color_parameters_do_not_set_dim_or_an_unrelated_color() {
        let output = parse("\x1b[38;2;31;32;2mRGB\x1b[0m\x1b[");
        assert_eq!(output.as_str(), "RGB\x1b[");
        assert!(!output.0.lines[0][0].ink.dim);
        assert!(output.0.lines[0][0].ink.index.is_none());
    }
}
