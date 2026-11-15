//! Bookmarkable destinations shared by native navigation and browser history.

#[cfg(target_arch = "wasm32")]
mod history;

use iced::Subscription;

use super::Message;
use crate::page;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Route {
    Page(usize),
    Examples,
    Receipts,
}

impl Route {
    pub(super) fn parse(
        path: &str,
        pages: &[Box<dyn page::Page>],
        component_count: usize,
    ) -> Option<Self> {
        let path = path.trim_end_matches('/');
        match path {
            "" => Some(Self::Page(0)),
            "/examples" => Some(Self::Examples),
            "/examples/receipts" => Some(Self::Receipts),
            _ => {
                let (slug, components) = path
                    .strip_prefix("/components/")
                    .map(|slug| (slug, true))
                    .or_else(|| path.strip_prefix("/docs/").map(|slug| (slug, false)))?;
                pages.iter().enumerate().find_map(|(index, page)| {
                    ((index < component_count) == components
                        && page::slug(page.meta().title) == slug)
                        .then_some(Self::Page(index))
                })
            }
        }
    }

    pub(super) fn path(self, pages: &[Box<dyn page::Page>], component_count: usize) -> String {
        match self {
            Self::Page(index) => {
                let section = if index < component_count {
                    "components"
                } else {
                    "docs"
                };
                format!("/{section}/{}", page::slug(pages[index].meta().title))
            }
            Self::Examples => "/examples".into(),
            Self::Receipts => "/examples/receipts".into(),
        }
    }
}

pub(super) fn location() -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    return history::location();
    #[cfg(not(target_arch = "wasm32"))]
    None
}

pub(super) fn push(path: &str) {
    #[cfg(target_arch = "wasm32")]
    history::push(path);
    #[cfg(not(target_arch = "wasm32"))]
    let _ = path;
}

pub(super) fn set_title(title: &str) {
    #[cfg(target_arch = "wasm32")]
    if let Some(document) = web_sys::window().and_then(|window| window.document()) {
        document.set_title(title);
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = title;
}

pub(super) fn subscription() -> Subscription<Message> {
    #[cfg(target_arch = "wasm32")]
    return history::subscription();
    #[cfg(not(target_arch = "wasm32"))]
    Subscription::none()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::Route;
    use crate::{Browser, Message, examples};

    #[test]
    fn every_registered_page_has_a_unique_round_trip_path() {
        let (browser, _) = Browser::new();
        let app = browser.app;
        let mut paths = HashSet::new();
        for index in 0..app.pages.len() {
            let destination = Route::Page(index);
            let path = destination.path(&app.pages, app.component_count);
            assert!(paths.insert(path.clone()), "duplicate route: {path}");
            assert_eq!(
                Route::parse(&path, &app.pages, app.component_count),
                Some(destination)
            );
            assert_eq!(
                Route::parse(&format!("{path}/"), &app.pages, app.component_count),
                Some(destination)
            );
        }
        for destination in [Route::Examples, Route::Receipts] {
            let path = destination.path(&app.pages, app.component_count);
            assert_eq!(
                Route::parse(&path, &app.pages, app.component_count),
                Some(destination)
            );
        }
        assert_eq!(
            Route::parse("/", &app.pages, app.component_count),
            Some(Route::Page(0))
        );
        for path in [
            "/unknown",
            "/docs/accordion",
            "/components/introduction",
            "/examples/missing",
            "/components/button/extra",
            "/missing.js",
        ] {
            assert_eq!(
                Route::parse(path, &app.pages, app.component_count),
                None,
                "{path}"
            );
        }
    }

    #[test]
    fn history_restores_components_guides_gallery_and_receipts() {
        let (browser, _) = Browser::new();
        let mut app = browser.app;
        let _ = app.update(Message::Example(examples::Event::OpenReceipts));
        assert_eq!(app.route(), Route::Receipts);
        assert_eq!(app.title(), "Receipts — icedcn");
        let _ = app.update(Message::Example(examples::Event::Gallery));
        assert_eq!(app.route(), Route::Examples);
        let _ = app.update(Message::History("/examples/receipts".into()));
        assert_eq!(app.route(), Route::Receipts);
        let _ = app.update(Message::History("/examples/".into()));
        assert_eq!(app.route(), Route::Examples);
        let _ = app.update(Message::History("/docs/installation".into()));
        assert_eq!(app.title(), "Installation — icedcn");
        assert!(!app.showing_examples);
        let _ = app.update(Message::History("/components/button".into()));
        assert_eq!(app.title(), "Button — icedcn");
        let selected = app.route();
        let _ = app.update(Message::SelectPage(app.current_page));
        assert_eq!(app.route(), selected);
        let _ = app.update(Message::History("/missing".into()));
        assert_eq!(app.route(), Route::Page(0));
    }
}
