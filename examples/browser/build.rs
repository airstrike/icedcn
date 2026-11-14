#[path = "src/page/heading.rs"]
mod heading;

use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo::rerun-if-changed=fonts/icons.toml");
    iced_lucide::build("fonts/icons.toml").expect("Build icon module");
    println!("cargo::rerun-if-changed=snippets");
    println!("cargo::rerun-if-changed=src/page/heading.rs");
    let output = PathBuf::from(env::var_os("OUT_DIR").ok_or("missing OUT_DIR")?);
    bundle_snippets(output)?;
    Ok(())
}

fn bundle_snippets(output: PathBuf) -> io::Result<()> {
    let mut files = fs::read_dir("snippets")?
        .map(|entry| entry.map(|file| file.path()))
        .collect::<io::Result<Vec<_>>>()?;
    files.sort();

    let mut bundle = fs::File::create(output.join("snippets.rs"))?;
    let mut headings = fs::File::create(output.join("headings.rs"))?;
    writeln!(headings, "pub(super) const HEADINGS: &[(&str, &str)] = &[")?;
    writeln!(
        bundle,
        "pub(super) fn snippet(path: &str) -> Option<&'static str> {{ match path {{"
    )?;
    for path in files {
        if path.extension().is_some_and(|ext| ext == "md") {
            let name = path
                .file_name()
                .and_then(|name| name.to_str())
                .ok_or(io::ErrorKind::InvalidData)?;
            let file = format!("snippets/{name}");
            let key = format!("examples/browser/{file}");
            let include = format!("/{file}");
            let source = fs::read_to_string(path)?;
            if let Some((title, _)) = heading::split(&source)
                && !title.is_empty()
            {
                writeln!(headings, "({key:?}, {title:?}),")?;
            }
            writeln!(
                bundle,
                "{key:?} => Some(include_str!(concat!(env!(\"CARGO_MANIFEST_DIR\"), {include:?}))),"
            )?;
        }
    }
    writeln!(headings, "];")?;
    writeln!(bundle, "_ => None, }} }}")
}
