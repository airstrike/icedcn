//! Discover and load Google Fonts with the optional `fount` feature.
//!
//! Run [`init`] at startup, store its catalog and system families in [`Fount`],
//! then feed [`Fount::families`] or [`Fount::search`] to a picker. Choosing a
//! family returns [`load`], which registers every downloaded face with iced
//! before publishing one completion message. It does not change the app's font.
//!
//! The feature selects iced's Tokio executor on native platforms; web builds
//! keep iced's browser executor.
//!
//! Native applications refresh Google's catalog with a disk cache. Web apps
//! use the complete bundled catalog snapshot and CORS-safe TrueType URLs;
//! see [`google::snapshot`] for its provenance and refresh policy.

pub use ::fount::{Catalog, Error, Fount, google};
#[cfg(not(target_arch = "wasm32"))]
pub use ::fount::{Config, system};

use iced_core::font::Family;
use iced_runtime::{Task, font};

/// Results from font discovery and registration, mapped into the app's message.
#[derive(Debug, Clone)]
pub enum Message {
    /// Google Fonts families and their available weight and italic variants.
    CatalogLoaded(Result<Catalog, Error>),
    /// Families already registered with iced's font system.
    SystemFontsLoaded(Vec<Family>),
    /// All faces for a requested family have finished registration.
    Loaded(String, Result<(), Error>),
}

/// Discover the complete Google Fonts catalog and fonts already available to iced.
pub fn init() -> Task<Message> {
    Task::batch([
        Task::future(google::catalog(google::DEFAULT_CATALOG_MAX_AGE)).map(Message::CatalogLoaded),
        font::list().map(|result| Message::SystemFontsLoaded(result.unwrap_or_default())),
    ])
}

/// Download a family and register every face before reporting completion.
///
/// Pass the discovered catalog to request the family's real variants. Existing
/// fonts remain usable if download or registration fails. The application owns
/// its selected font and decides whether to apply a successfully loaded family.
pub fn load(name: impl Into<String>, catalog: Option<&Catalog>) -> Task<Message> {
    let name = name.into();
    let catalog = catalog.cloned();
    Task::future(async move {
        let result = google::load(&name, catalog.as_ref()).await;
        (name, result)
    })
    .then(|(name, result)| match result {
        Ok(faces) => Task::batch(faces.into_iter().map(font::load))
            .collect()
            .map(move |results| {
                let result = results
                    .into_iter()
                    .collect::<Result<Vec<_>, _>>()
                    .map(|_| ())
                    .map_err(|error| Error::Font(format!("{error:?}")));
                Message::Loaded(name.clone(), result)
            }),
        Err(error) => Task::done(Message::Loaded(name, Err(error))),
    })
}
