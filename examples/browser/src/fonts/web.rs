//! Bundle the default face and fetch other typefaces only when selected.
use iced::Task;
use iced::font::{self, Family};
use std::collections::BTreeMap;
use std::sync::LazyLock;

const INDEX: &str = include_str!("../../fonts/catalog.tsv");

static SOURCES: LazyLock<BTreeMap<&'static str, Vec<&'static str>>> = LazyLock::new(|| {
    INDEX
        .lines()
        .skip(1)
        .filter_map(|line| {
            let mut columns = line.split('\t');
            Some((columns.next()?, columns.collect()))
        })
        .collect()
});

#[derive(Clone, Debug)]
pub enum Message {
    SystemFontsLoaded(Vec<Family>),
    Loaded(String, Result<(), String>),
}

#[derive(Default)]
pub struct Fount {
    system: Vec<String>,
}

impl Fount {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_system_families(&mut self, names: Vec<String>) {
        self.system = names;
    }

    pub fn system_families(&self) -> &[String] {
        &self.system
    }

    pub fn families(&self) -> Vec<String> {
        SOURCES.keys().map(ToString::to_string).collect()
    }
}

pub fn init() -> Task<Message> {
    Task::done(Message::SystemFontsLoaded(vec![Family::Name("Geist")]))
}

pub fn load(name: String) -> Task<Message> {
    let Some(paths) = SOURCES.get(name.as_str()) else {
        return Task::done(Message::Loaded(name, Err("Unknown typeface".into())));
    };
    let base = INDEX
        .lines()
        .next()
        .and_then(|line| line.strip_prefix("# "))
        .unwrap_or_default();
    let client = reqwest::Client::new();
    let loaded = Task::future(async move {
        let mut fonts = Vec::with_capacity(paths.len());
        for path in paths {
            let response = client
                .get(format!("{base}{path}"))
                .send()
                .await?
                .error_for_status()?;
            fonts.push(response.bytes().await?.to_vec());
        }
        Ok::<_, reqwest::Error>(fonts)
    });
    loaded.then(move |result| match result {
        Ok(faces) => Task::batch(faces.into_iter().map(|face| {
            let name = name.clone();
            font::load(face)
                .map(move |r| Message::Loaded(name.clone(), r.map_err(|e| format!("{e:?}"))))
        })),
        Err(error) => Task::done(Message::Loaded(name.clone(), Err(error.to_string()))),
    })
}
