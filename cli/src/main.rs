//! Create standalone icedcn applications from bundled templates.

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, ensure};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "icedcn", version, about = "Create icedcn applications")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Create a standalone counter app in a new directory.
    New {
        /// Destination directory; its final component becomes the package name.
        path: PathBuf,
    },
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::New { path } => {
            scaffold(&path)?;
            println!("Created {}", path.display());
            println!("\nNext: change into that directory and run `cargo run`.");
            println!("Customize your app in src/main.rs and src/theme.rs.");
        }
    }
    Ok(())
}

fn scaffold(path: &Path) -> Result<()> {
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .context("the destination must end in a valid package name")?;
    ensure!(
        name.starts_with(|character: char| character.is_ascii_lowercase())
            && name.chars().all(|character| {
                character.is_ascii_lowercase()
                    || character.is_ascii_digit()
                    || matches!(character, '-' | '_')
            }),
        "package names must start with a lowercase letter and contain only lowercase letters, digits, hyphens, or underscores"
    );
    ensure!(
        !matches!(name, "test" | "build" | "deps" | "examples"),
        "{name:?} is reserved by Cargo; choose another package name"
    );

    // create_dir atomically refuses existing directories, files, and symlinks.
    fs::create_dir(path).with_context(|| {
        format!(
            "cannot create {}: the destination must not exist and its parent must exist",
            path.display()
        )
    })?;
    fs::create_dir(path.join("src")).context("cannot create the source directory")?;

    for (relative, content) in [
        (
            "Cargo.toml",
            include_str!("../templates/Cargo.toml.template"),
        ),
        (
            "rust-toolchain.toml",
            include_str!("../templates/rust-toolchain.toml"),
        ),
        ("src/main.rs", include_str!("../templates/main.rs")),
        ("src/theme.rs", include_str!("../templates/theme.rs")),
        (".gitignore", "/target\n"),
    ] {
        fs::write(path.join(relative), content.replace("__NAME__", name))
            .with_context(|| format!("cannot write {relative}"))?;
    }
    Ok(())
}
