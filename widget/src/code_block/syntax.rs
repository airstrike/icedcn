//! Parse source into theme-independent tokens before the listing paints its rows.
use crate::highlighter::{self, CAPTURE_NAMES, Highlighter};
use crate::{Span, span};
use iced_core::Code;
use iced_highlighter::{Parser, Settings};
use icedxdy_core::Color;

pub(super) fn rows(source: &str, language: &str) -> Vec<Vec<Span<'static>>> {
    tokenize(source, language)
        .into_iter()
        .map(|line| {
            line.into_iter()
                .map(|(text, token)| {
                    let (light, dark) = colors(token);
                    span(text).color_pair(rgb(light), rgb(dark))
                })
                .collect()
        })
        .collect()
}

fn tokenize(source: &str, language: &str) -> Vec<Vec<(String, Code)>> {
    // Registered tree-sitter grammars take precedence over iced's language set.
    if highlighter::config_for(language).is_some() {
        let mut rows: Vec<_> = Highlighter::new()
            .highlight(source, language)
            .into_iter()
            .map(|line| {
                line.into_iter()
                    .map(|span| {
                        let token = span
                            .capture
                            .and_then(|id| CAPTURE_NAMES.get(id as usize))
                            .map_or(Code::Other, |name| classify(name));
                        (span.text, token)
                    })
                    .collect()
            })
            .collect();
        if source.ends_with('\n') {
            rows.push(Vec::new());
        }
        rows
    } else {
        let mut parser = Parser::new(&Settings {
            token: language.to_owned(),
        });
        source
            .split('\n')
            .map(|line| {
                parser
                    .parse_line(line)
                    .map(|(range, code)| (line[range].to_owned(), code))
                    .collect()
            })
            .collect()
    }
}

fn classify(name: &str) -> Code {
    match name.split('.').next().unwrap_or(name) {
        "keyword" | "operator" => Code::Keyword,
        "function" | "constructor" => Code::Function,
        "type" | "number" | "constant" | "label" => Code::Constant,
        "comment" => Code::Comment,
        "string" => Code::String,
        _ => Code::Other,
    }
}

// GitHub's classic light/dark syntax colors. Keep unclassified punctuation and text
// in the same foreground as the rest of a highlighted line.
fn colors(code: Code) -> (u32, u32) {
    match code {
        Code::Comment => (0x6a737d, 0x6a737d),
        Code::Keyword | Code::Invalid => (0xd73a49, 0xf97583),
        Code::String => (0x032f62, 0x9ecbff),
        Code::Function => (0x6f42c1, 0xb392f0),
        Code::Constant | Code::Type | Code::Support => (0x005cc5, 0x79b8ff),
        Code::Other | Code::Variable | Code::Punctuation | Code::Path => (0x24292e, 0xe1e4e8),
    }
}

fn rgb(hex: u32) -> Color {
    Color::from(iced_core::Color::from_rgb8(
        (hex >> 16) as u8,
        (hex >> 8) as u8,
        hex as u8,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arbitrary_rust_uses_both_themes_and_preserves_text() {
        let source = "let café = \"東京\"; // greeting\nlet limit = 42;";
        let tokens = tokenize(source, "rust");
        for (text, expected) in [
            ("let", Code::Keyword),
            ("東京", Code::String),
            ("greeting", Code::Comment),
            ("42", Code::Constant),
        ] {
            let token = tokens
                .iter()
                .flatten()
                .find(|(s, _)| s.contains(text))
                .expect("token");
            assert_eq!(token.1, expected);
        }
        assert_eq!(colors(Code::String), (0x032f62, 0x9ecbff));
        assert_eq!(colors(Code::Constant), (0x005cc5, 0x79b8ff));
        assert_eq!(colors(Code::Comment), (0x6a737d, 0x6a737d));
        assert_eq!(text(&tokens), source);
    }

    #[test]
    fn partial_streams_and_plain_fallback_preserve_every_character() {
        for (language, source) in [
            ("rs", "let café = \"日\n本\";\n\n"),
            ("toml", "[package]\nname = \"café\"\n"),
            ("sh", "echo \"δ\"\n"),
            ("unknown-language", "日本語\n\n"),
        ] {
            for end in source.char_indices().map(|(i, _)| i).chain([source.len()]) {
                let prefix = &source[..end];
                assert_eq!(text(&tokenize(prefix, language)), prefix);
            }
        }
        for (language, source) in [("toml", "label = \"hello\""), ("sh", "echo \"hello\"")] {
            assert!(
                tokenize(source, language)
                    .iter()
                    .flatten()
                    .any(|(_, token)| *token == Code::String)
            );
        }
    }
    fn text(rows: &[Vec<(String, Code)>]) -> String {
        rows.iter()
            .map(|line| {
                line.iter()
                    .map(|(text, _)| text.as_str())
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}
