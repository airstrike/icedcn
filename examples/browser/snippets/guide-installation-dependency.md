## Create an application

Install Rust with [rustup](https://rustup.rs/), then install the icedcn CLI:

```sh
cargo install --git https://github.com/airstrike/icedcn --branch master icedcn_cli --locked
icedcn new my-app
cd my-app
cargo run
```

The CLI creates a standalone application with its dependencies and Rust toolchain configured. Cargo downloads icedcn from GitHub; you do not need a local checkout. Commit your application's generated `Cargo.lock`.

## Manual setup

Install Rust with [rustup](https://rustup.rs/), then create an application:

```sh
cargo new my-app --edition 2024
cd my-app
rustup override set nightly-2026-07-11
```

This selects the nightly currently used by icedcn. Cargo downloads the library from GitHub; no local icedcn checkout is needed. Commit the generated `Cargo.lock` to keep dependency revisions reproducible.

Replace the generated `[dependencies]` section in your application's `Cargo.toml` with:

```toml
[dependencies]
iced = "0.15.0-dev"
icedcn = { git = "https://github.com/airstrike/icedcn", branch = "master" }

[patch.crates-io]
iced = { git = "https://github.com/airstrike/iced.git", branch = "dev" }
iced_core = { git = "https://github.com/airstrike/iced.git", branch = "dev" }
iced_widget = { git = "https://github.com/airstrike/iced.git", branch = "dev" }
iced_runtime = { git = "https://github.com/airstrike/iced.git", branch = "dev" }

[patch."https://github.com/hecrj/cosmic-text.git"]
cosmic-text = { git = "https://github.com/airstrike/cosmic-text", branch = "dev" }
```

Cargo only applies patches from the root manifest. If your app belongs to a Cargo workspace, put these patch tables in that workspace's root manifest and use its lockfile. These patches select the forks currently required by icedcn.

Keep the branch references in the patch tables: the lockfile pins the commits, and the matching references let iced and its dependencies share the same types.

Start with the default features; see the [Cargo guide](icedcn://guide/cargo#feature-flags) for `chart`, `svg`, `markdown`, `highlighter`, `spinner`, and `sweeten`.
