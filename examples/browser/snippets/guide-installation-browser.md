## Run the browser

Clone the repository and run the browser from its root:

```sh
git clone https://github.com/airstrike/icedcn.git
cd icedcn
cargo run --locked -p browser
```

The checkout's `rust-toolchain.toml` selects the Rust toolchain. `Cargo.lock` fixes the dependency revisions used by these examples. Keep both files when reproducing a build.

The browser uses local bundled examples and snippets. Its font chooser can fetch additional fonts, but the bundled font and component examples are available without selecting an online font.
