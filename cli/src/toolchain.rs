//! Manage the project's explicit Rust toolchain without changing rustup overrides.

use std::fs;
use std::io::{ErrorKind, Write};
use std::path::Path;

use anyhow::{Context, Result, bail, ensure};
use toml_edit::{DocumentMut, Item, Value};

/// The pinned toolchain shared by new projects and existing-project setup.
pub const TEMPLATE: &str = include_str!("../templates/rust-toolchain.toml");

/// Create or check a project toolchain, changing its channel only when requested.
pub fn configure(directory: &Path, update: bool) -> Result<()> {
    let metadata = fs::symlink_metadata(directory)
        .with_context(|| format!("cannot inspect project directory {}", directory.display()))?;
    ensure!(
        metadata.file_type().is_dir(),
        "{} must be an existing directory, not a symlink",
        directory.display()
    );
    ensure!(
        directory.join("Cargo.toml").is_file(),
        "{} does not contain Cargo.toml; run this command in a Rust project directory",
        directory.display()
    );

    let legacy = directory.join("rust-toolchain");
    match fs::symlink_metadata(&legacy) {
        Ok(_) => bail!(
            "{} conflicts with rust-toolchain.toml; migrate or remove the legacy file before running this command",
            legacy.display()
        ),
        Err(error) if error.kind() == ErrorKind::NotFound => {}
        Err(error) => return Err(error).context("cannot inspect the legacy rust-toolchain file"),
    }

    let template: DocumentMut = TEMPLATE
        .parse()
        .context("invalid bundled toolchain template")?;
    let required = template["toolchain"]["channel"]
        .as_str()
        .context("bundled toolchain template has no channel")?;
    let path = directory.join("rust-toolchain.toml");
    match fs::symlink_metadata(&path) {
        Ok(metadata) => ensure!(
            metadata.file_type().is_file(),
            "{} must be a regular file, not a directory or symlink",
            path.display()
        ),
        Err(error) if error.kind() == ErrorKind::NotFound => {
            let mut file = fs::File::create_new(&path)
                .with_context(|| format!("cannot create {}", path.display()))?;
            file.write_all(TEMPLATE.as_bytes())
                .with_context(|| format!("cannot write {}", path.display()))?;
            println!("Created {} with channel {required}", path.display());
            return Ok(());
        }
        Err(error) => return Err(error).context("cannot inspect rust-toolchain.toml"),
    }

    let source =
        fs::read_to_string(&path).with_context(|| format!("cannot read {}", path.display()))?;
    let mut document: DocumentMut = source
        .parse()
        .with_context(|| format!("invalid TOML in {}; file left unchanged", path.display()))?;
    let channel = document
        .get_mut("toolchain")
        .and_then(Item::as_table_like_mut)
        .and_then(|table| table.get_mut("channel"))
        .and_then(Item::as_value_mut)
        .context(
            "rust-toolchain.toml must contain a string toolchain.channel; file left unchanged",
        )?;
    let current = channel
        .as_str()
        .context("toolchain.channel must be a string; file left unchanged")?;
    if current == required {
        println!("{} already uses {required}; unchanged", path.display());
        return Ok(());
    }
    ensure!(
        update,
        "{} uses channel {current}, but icedcn requires {required}; rerun `icedcn toolchain --update` in that project directory to change only the channel; file left unchanged",
        path.display()
    );

    let decor = channel.decor().clone();
    *channel = Value::from(required);
    *channel.decor_mut() = decor;
    fs::write(&path, document.to_string())
        .with_context(|| format!("cannot update {}", path.display()))?;
    println!("Updated {} to channel {required}", path.display());
    Ok(())
}
