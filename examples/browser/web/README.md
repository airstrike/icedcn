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

The WASM build bundles Twemoji Mozilla v0.7.0 as a color emoji fallback because
browser applications cannot load system fonts. Its COLR/CPAL version 0 font is
supported by the locked cosmic-text/Swash renderer on both WebGPU and the CPU
fallback. This is the unmodified [Mozilla release font](https://github.com/mozilla/twemoji-colr/releases/tag/v0.7.0),
with Twemoji artwork under CC BY 4.0 and font-generation code under Apache 2.0;
its attribution, upstream license notice, source URL and SHA-256 are recorded in
`../fonts/TwemojiMozilla-LICENSE.txt`. The build includes all bundled font
licenses at `/font-licenses.txt`.

The loader preserves Cmd/Ctrl+R and Cmd/Ctrl+Shift+R as native browser shortcuts,
preventing the canvas event handler from cancelling them. Right-clicks on the
canvas use the application's context menu; surrounding HTML retains its native
browser menu.
