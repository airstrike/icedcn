use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo::rerun-if-changed=fonts/icons.toml");
    iced_lucide::build("fonts/icons.toml").expect("Build icon module");
    if env::var("CARGO_CFG_TARGET_ARCH")? == "wasm32" {
        println!("cargo::rerun-if-changed=snippets");
        bundle_snippets(PathBuf::from(
            env::var_os("OUT_DIR").ok_or("missing OUT_DIR")?,
        ))?;
    }
    Ok(())
}

fn bundle_snippets(output: PathBuf) -> io::Result<()> {
    let mut files = fs::read_dir("snippets")?
        .map(|entry| entry.map(|file| file.path()))
        .collect::<io::Result<Vec<_>>>()?;
    files.sort();

    let mut bundle = fs::File::create(output.join("snippets.rs"))?;
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
            writeln!(
                bundle,
                "{key:?} => Some(include_str!(concat!(env!(\"CARGO_MANIFEST_DIR\"), {include:?}))),"
            )?;
        }
    }
    writeln!(bundle, "_ => None, }} }}")
}
