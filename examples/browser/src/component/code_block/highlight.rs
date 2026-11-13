//! Parse the static samples once; the theme resolves their colors at display time.
use super::samples::*;
use crate::{Message, font};
use iced::highlighter::{Parser, Settings};
use iced::widget::text::Wrapping;
use iced::{Code, Fill, Font, Shrink};
use icedxdy::Slot;
use icedxdy::theme::tweak::Token;
use icedxdy::widget::code_block::{CodeBlock, LineStyle};
use icedxdy::widget::{rich_text, span};
use std::collections::HashMap;
use std::ops::Range;
use std::sync::LazyLock;

pub const STREAMS: &[(&str, &str, &str)] = &[
    ("hooks/use-debounce.ts", "ts", STREAMING_0),
    ("Terminal", "sh", STREAMING_1),
    ("components/search.tsx", "tsx", STREAMING_2),
    ("app/api/chat/route.ts", "ts", STREAMING_3),
    (".env.local", "sh", STREAMING_4),
];
type Runs = Vec<Vec<(Range<usize>, Code)>>;
static TOKENS: LazyLock<HashMap<&'static str, Runs>> = LazyLock::new(|| {
    let mut sources = vec![
        (DEMO, "ts"),
        (START_LINE, "tsx"),
        (HIGHLIGHTED_LINES, "js"),
        (LINE_NUMBERS, "json"),
        (HIGHLIGHTING, "json"),
        (CHAT, "ts"),
        (TYPESET, "ts"),
        (SHOW_MORE, "tsx"),
        (TABS_0, "ts"),
        (TABS_1, "tsx"),
        (TABS_2, "sh"),
        (LANGUAGES_0, "css"),
        (LANGUAGES_1, "py"),
        (LANGUAGES_2, "txt"),
    ];
    sources.extend(STREAMS.iter().map(|(_, lang, source)| (*source, *lang)));
    sources
        .into_iter()
        .map(|(source, lang)| {
            let mut parser = Parser::new(&Settings {
                token: lang.to_string(),
            });
            let lines = source
                .split('\n')
                .map(|line| parser.parse_line(line).collect())
                .collect();
            (source, lines)
        })
        .collect()
});

fn color(class: Code) -> Token {
    match class {
        Code::Comment => Token::MutedForeground,
        Code::String => Token::Success,
        Code::Type | Code::Path | Code::Function => Token::Warning,
        Code::Keyword | Code::Support | Code::Constant => Token::Info,
        Code::Invalid => Token::Destructive,
        _ => Token::CodeForeground,
    }
}

/// Render a prefix so one cached parse can also serve a streamed listing.
pub fn listing(
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
    let tokens = TOKENS.get(source);
    let mut end = visible.min(source.len());
    while !source.is_char_boundary(end) {
        end -= 1;
    }
    let lines = source[..end].split('\n').enumerate().map(|(i, line)| {
        let mut spans = Vec::new();
        if let Some(runs) = tokens.and_then(|all| all.get(i)) {
            let mut pos = 0;
            for (range, class) in runs {
                let start = range.start.min(line.len());
                let end = range.end.min(line.len());
                if start > pos {
                    spans.push(span(&line[pos..start]));
                }
                if end > start {
                    spans.push(span(&line[start..end]).color(color(*class)));
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
            spans = vec![span(" ")];
        }
        let text = rich_text(spans)
            .font(Font::new(font::GEIST_MONO_NAME))
            .size(13)
            .wrapping(wrapping)
            .width(if wrap { Fill } else { Shrink });
        (styles(i, line), Slot::from(text))
    });
    CodeBlock::lines(lines).wrap(wrap)
}

pub fn ansi() -> (CodeBlock<'static, Message>, String) {
    let mut stripped = Vec::new();
    let lines = ANSI
        .lines()
        .map(|line| {
            let mut color = Token::CodeForeground;
            let mut spans = Vec::new();
            let mut plain = String::new();
            let mut pieces = line.split("\x1b[");
            let prefix = pieces.next().unwrap_or_default();
            plain.push_str(prefix);
            spans.push(span(prefix));
            for piece in pieces {
                if let Some((codes, value)) = piece.split_once('m') {
                    for code in codes.split(';') {
                        color = match code {
                            "2" => Token::MutedForeground,
                            "31" => Token::Destructive,
                            "32" => Token::Success,
                            "0" | "22" | "39" => Token::CodeForeground,
                            _ => color,
                        };
                    }
                    plain.push_str(value);
                    spans.push(span(value).color(color));
                }
            }
            stripped.push(plain);
            (
                LineStyle::Default,
                Slot::from(
                    rich_text(spans)
                        .size(13)
                        .font(Font::new(font::GEIST_MONO_NAME)),
                ),
            )
        })
        .collect::<Vec<_>>();
    (CodeBlock::lines(lines).wrap(true), stripped.join("\n"))
}
