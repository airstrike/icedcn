# Component browser on the web

The web build runs the same Rust entry point and component registry as the native browser. Install the toolchain in the root rust-toolchain.toml, Python 3, a Clang build with the WebAssembly target, and Binaryen (wasm-opt). The wasm-bindgen CLI must match the wasm-bindgen version in Cargo.lock.

From the workspace root, run

```sh
rustup target add wasm32-unknown-unknown
cargo install --locked wasm-bindgen-cli --version 0.2.130
python3 scripts/build-browser-web.py
python3 -m http.server --directory target/browser-web 8080
```

Set WASM_BINDGEN to select a specific CLI executable. CC_wasm32_unknown_unknown can select a WebAssembly-capable Clang if the system default lacks that target.

To deploy the generated assets to icedcn.airstrike.fyi from an authenticated Cloudflare account, run

```sh
npx --yes wrangler@4.69.0 deploy --config examples/browser/wrangler.jsonc
```

The build also copies `web/og.png` to the deployment root. The loader's Open Graph and Twitter metadata reference this 1200 × 630 image at the canonical production URL. To regenerate it after a brand change, install Pillow and `rsvg-convert`, then run `python3 examples/browser/web/render-og.py` from the workspace root. It uses the existing logo and bundled Geist fonts. Regeneration is optional during normal builds.
