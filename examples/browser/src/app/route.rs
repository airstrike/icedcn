//! Browser destinations and their metadata for the generic path router.

use browser_routing::Router;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum Route {
    Home,
    Create,
    Page(usize),
    Examples,
    Receipts,
    Playground,
    NotFound(String),
}

pub(super) struct Definition {
    pub destination: Route,
    pub path: String,
    pub name: &'static str,
    pub title: Option<&'static str>,
}

impl browser_routing::Route for Definition {
    fn name(&self) -> &str {
        self.name
    }
    fn path(&self) -> &str {
        &self.path
    }
    fn title(&self) -> Option<&str> {
        self.title
    }
}

impl Route {
    pub(super) fn parse(path: &str, routes: &Router<Definition>) -> Self {
        routes
            .find(path)
            .map(|definition| definition.destination.clone())
            .unwrap_or_else(|| Self::NotFound(path.to_owned()))
    }

    pub(super) fn path<'a>(&'a self, routes: &'a Router<Definition>) -> Option<&'a str> {
        match self {
            Self::NotFound(path) => Some(path),
            _ => routes
                .routes()
                .find(|definition| &definition.destination == self)
                .map(|definition| definition.path.as_str()),
        }
    }
}
