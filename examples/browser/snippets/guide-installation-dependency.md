## Add it to an application

Install Rust with [rustup](https://rustup.rs/), then create an application beside an icedcn checkout:

```sh
git clone https://github.com/airstrike/icedcn.git
cargo new my-app --edition 2024
cp icedcn/rust-toolchain.toml my-app/rust-toolchain.toml
cp icedcn/Cargo.lock my-app/Cargo.lock
cd my-app
```

The toolchain file selects the nightly used by icedcn. The copied lockfile starts your application with the checkout's tested dependency revisions, including the iced, cosmic-text, and winit forks.

Replace the generated `[dependencies]` section in your application's `Cargo.toml` with:

```toml
[dependencies]
iced = "0.15.0-dev"
icedcn = { path = "../icedcn" }

[patch.crates-io]
iced = { git = "https://github.com/airstrike/iced.git", branch = "dev" }
iced_core = { git = "https://github.com/airstrike/iced.git", branch = "dev" }
iced_runtime = { git = "https://github.com/airstrike/iced.git", branch = "dev" }
iced_selector = { git = "https://github.com/airstrike/iced.git", branch = "dev" }
iced_test = { git = "https://github.com/airstrike/iced.git", branch = "dev" }
iced_widget = { git = "https://github.com/airstrike/iced.git", branch = "dev" }
iced_graphics = { git = "https://github.com/airstrike/iced.git", branch = "dev" }
iced_renderer = { git = "https://github.com/airstrike/iced.git", branch = "dev" }
iced_program = { git = "https://github.com/airstrike/iced.git", branch = "dev" }
iced_winit = { git = "https://github.com/airstrike/iced.git", branch = "dev" }
iced_wgpu = { git = "https://github.com/airstrike/iced.git", branch = "dev" }
iced_tiny_skia = { git = "https://github.com/airstrike/iced.git", branch = "dev" }
iced_futures = { git = "https://github.com/airstrike/iced.git", branch = "dev" }
iced_debug = { git = "https://github.com/airstrike/iced.git", branch = "dev" }
iced_highlighter = { git = "https://github.com/airstrike/iced.git", branch = "dev" }
cosmic-text = { git = "https://github.com/airstrike/cosmic-text", branch = "dev" }

[patch."https://github.com/hecrj/cosmic-text.git"]
cosmic-text = { git = "https://github.com/airstrike/cosmic-text", branch = "dev" }
```

Cargo only applies patches from the root manifest. If your app belongs to a Cargo workspace, put these patch tables in that workspace's root manifest and use its lockfile. The tables above match the current checkout's `Cargo.toml`; when updating icedcn, keep them in sync.

Keep the branch references in the patch tables: the lockfile pins the commits, and the matching references let iced and its dependencies share the same types.

The path dependency follows your local checkout. For CI or sharing, replace it with a git dependency pinned to the same icedcn commit. Start with the default features; see the [Cargo guide](icedcn://guide/cargo#feature-flags) for `chart`, `svg`, `markdown`, `highlighter`, `spinner`, and `sweeten`.
