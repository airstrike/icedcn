//! Export the live route registry for static HTML and social previews.

use std::io;

use serde::Serialize;

use super::{App, route};
use crate::{GH_DARK, GH_LIGHT, page, parse_code_map};

#[derive(Serialize)]
struct Route<'a> {
    path: &'a str,
    title: &'a str,
    description: &'a str,
    blocks: Vec<Block>,
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
enum Block {
    Markdown {
        path: &'static str,
    },
    Example {
        title: &'static str,
        description: Option<&'static str>,
        snippet: &'static str,
    },
    Group {
        title: &'static str,
        children: Vec<Block>,
    },
}

fn blocks(source: &[page::Block]) -> Vec<Block> {
    source
        .iter()
        .filter_map(|block| match block {
            page::Block::Md(path) => Some(Block::Markdown { path }),
            page::Block::Example {
                title,
                description,
                snippet,
                ..
            } => Some(Block::Example {
                title,
                description: *description,
                snippet,
            }),
            page::Block::Group { title, children } => Some(Block::Group {
                title,
                children: blocks(children),
            }),
            // Inline blocks contain live widgets rather than exportable prose.
            page::Block::Inline { .. } => None,
        })
        .collect()
}

/// Write page metadata to stdout without opening a window or running startup tasks.
pub fn write_routes() -> serde_json::Result<()> {
    let (app, _) = App::new(parse_code_map(GH_LIGHT), parse_code_map(GH_DARK));
    let routes: Vec<_> = app
        .routes
        .routes()
        .map(|definition| {
            let description = match definition.destination {
                route::Route::Page(index) => app.pages[index].meta().description,
                route::Route::Home => {
                    "Composable components, thoughtful defaults, and copyable Rust for native iced applications."
                }
                route::Route::Create => {
                    "Customize colors and fonts, preview components, and export a Rust theme."
                }
                route::Route::Examples => {
                    "Explore complete interfaces built with icedcn components and Rust."
                }
                route::Route::Receipts => {
                    "A sales notebook built with icedcn, bringing a complete Rust application into the component browser."
                }
                route::Route::NotFound(_) => "This page could not be found.",
            };
            Route {
                path: &definition.path,
                title: definition.title.unwrap_or(definition.name),
                description,
                blocks: match definition.destination {
                    route::Route::Page(index) => blocks(app.pages[index].blocks()),
                    _ => Vec::new(),
                },
            }
        })
        .collect();
    serde_json::to_writer(io::stdout().lock(), &routes)
}
