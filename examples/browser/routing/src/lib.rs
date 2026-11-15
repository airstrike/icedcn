//! Bookmarkable destinations shared by native navigation and browser history.

#[cfg(target_arch = "wasm32")]
mod history;

use iced_futures::Subscription;

/// Metadata supplied by an application for one navigable destination.
pub trait Route {
    /// Stable, human-readable name for the destination.
    fn name(&self) -> &str;

    /// Canonical absolute pathname, such as `/components/button`.
    fn path(&self) -> &str;

    /// Optional page title; callers can fall back to the route name.
    fn title(&self) -> Option<&str> {
        None
    }

    /// Additional pathnames that resolve to this same destination.
    fn aliases(&self) -> &[&str] {
        &[]
    }
}

/// Lookup over application-owned route definitions, independent of view state.
pub struct Router<R> {
    routes: Vec<R>,
}

impl<R: Route> Router<R> {
    /// Register route definitions in iteration order.
    pub fn new(routes: impl IntoIterator<Item = R>) -> Self {
        Self {
            routes: routes.into_iter().collect(),
        }
    }

    /// Find a destination, accepting trailing slashes without rewriting the URL.
    pub fn find(&self, path: &str) -> Option<&R> {
        let path = normalize(path);
        self.routes.iter().find(|route| {
            normalize(route.path()) == path
                || route.aliases().iter().any(|alias| normalize(alias) == path)
        })
    }

    /// Iterate over the original definitions, preserving application identity.
    pub fn routes(&self) -> impl Iterator<Item = &R> {
        self.routes.iter()
    }
}

fn normalize(path: &str) -> &str {
    let trimmed = path.trim_end_matches('/');
    if !path.is_empty() && trimmed.is_empty() {
        "/"
    } else {
        trimmed
    }
}

/// Read the current browser pathname; native applications have no location.
pub fn location() -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    return history::location();
    #[cfg(not(target_arch = "wasm32"))]
    None
}

/// Add a history entry without reloading the app; a no-op on native.
pub fn push(path: &str) {
    #[cfg(target_arch = "wasm32")]
    history::push(path);
    #[cfg(not(target_arch = "wasm32"))]
    let _ = path;
}

/// Update the browser tab title; native window titles are owned by iced.
pub fn set_title(title: &str) {
    #[cfg(target_arch = "wasm32")]
    if let Some(document) = web_sys::window().and_then(|window| window.document()) {
        document.set_title(title);
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = title;
}

/// Observe Back/Forward navigation for the lifetime of this subscription.
pub fn subscription() -> Subscription<String> {
    #[cfg(target_arch = "wasm32")]
    return history::subscription();
    #[cfg(not(target_arch = "wasm32"))]
    Subscription::none()
}

#[cfg(test)]
mod tests {
    use super::{Route, Router};

    #[derive(Debug, PartialEq)]
    enum Page {
        Home,
        Guide,
    }

    impl Route for Page {
        fn name(&self) -> &str {
            match self {
                Self::Home => "home",
                Self::Guide => "guide",
            }
        }
        fn path(&self) -> &str {
            match self {
                Self::Home => "/components/accordion",
                Self::Guide => "/docs/introduction",
            }
        }
        fn title(&self) -> Option<&str> {
            match self {
                Self::Home => Some("Accordion"),
                Self::Guide => None,
            }
        }
        fn aliases(&self) -> &[&str] {
            match self {
                Self::Home => &["/"],
                Self::Guide => &[],
            }
        }
    }

    #[test]
    fn resolves_canonical_paths_aliases_and_trailing_slashes_with_original_metadata() {
        let router = Router::new([Page::Home, Page::Guide]);
        for route in router.routes() {
            let found = router.find(route.path()).unwrap();
            assert!(std::ptr::eq(found, route));
            assert_eq!(router.find(&format!("{}/", route.path())), Some(route));
        }
        assert_eq!(router.find("/"), Some(&Page::Home));
        assert_eq!(router.find("/").unwrap().title(), Some("Accordion"));
        assert_eq!(router.find("/docs/introduction").unwrap().title(), None);
        assert_eq!(router.find("/docs/introduction").unwrap().name(), "guide");
    }

    #[test]
    fn rejects_unknown_namespaces_names_and_extra_segments() {
        let router = Router::new([Page::Home, Page::Guide]);
        for path in [
            "",
            "docs/introduction",
            "/unknown",
            "/docs/accordion",
            "/components/introduction",
            "/components/missing/",
            "/docs/introduction/extra",
            "/missing.js",
            "/%3Cscript%3E",
        ] {
            assert_eq!(router.find(path), None, "{path}");
        }
        assert_eq!(Router::new(Vec::<Page>::new()).find("/"), None);
    }
}
