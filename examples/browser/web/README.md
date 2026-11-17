# Component browser on the web

The web build runs the same Rust entry point and component registry as the native browser. Install the toolchain in the root rust-toolchain.toml, Python 3, a Clang build with the WebAssembly target, Binaryen (wasm-opt), Pillow 12, and librsvg (rsvg-convert). The wasm-bindgen CLI must match the wasm-bindgen version in Cargo.lock.

From the workspace root, run

```sh
rustup target add wasm32-unknown-unknown
cargo install --locked wasm-bindgen-cli --version 0.2.130
uv run --with pillow==12.0.0 python scripts/build-browser-web.py
python3 -m http.server --directory target/browser-web 8080
```

Set WASM_BINDGEN to select a specific CLI executable. CC_wasm32_unknown_unknown can select a WebAssembly-capable Clang if the system default lacks that target.

To deploy the generated assets to icedcn.airstrike.fyi from an authenticated Cloudflare account, run

```sh
rawr run scripts/deploy-browser-web
```

The deploy is a small Rust crate run by [rawr](https://github.com/airstrike/rawr); add `-- --dry-run` to print the wrangler commands instead. It also builds with Cargo (`cargo run --manifest-path scripts/deploy-browser-web/Cargo.toml`). Its process and filesystem calls live in the native `src/sys.rs`, since rawr's interpreter doesn't run `std::process` or those `std::fs` calls yet (rawr #2082, #646).

Workers Static Assets serves files of up to 25 MiB, and the browser's wasm is larger since the Playground example embeds the rawr engine. The build writes an `.assetsignore` that leaves the wasm out of the assets; the deploy uploads it to the `icedcn-wasm` R2 bucket, then deploys. `web/worker.js` runs only for `/browser-*.wasm` and serves it from R2 with an immutable cache; every other path stays a static asset. To test locally, put the wasm in the local bucket before `wrangler dev`:

```sh
npx --yes wrangler@4.69.0 r2 object put icedcn-wasm/<browser-hash>.wasm --file target/browser-web/<browser-hash>.wasm --content-type application/wasm --local --config examples/browser/wrangler.jsonc
```

The build exports route titles, descriptions, and documentation blocks from the
native application's registry using `--export-web-routes`; it does not open a
window or run startup tasks. It then creates one HTML page and one 1200 × 630
PNG share card per route, plus SVG/PNG/ICO favicons and an Apple touch icon. Cards
use the existing logo, bundled Geist fonts, and deterministic interface motifs.
There is no request-time image generation; the only Worker code serves the wasm. `uv` is optional if
Pillow is already installed in the Python environment used to run the build.

The same registry produces full Markdown documentation (`/components/button.md`,
`/docs/installation.md`, and `/index.md`), a grouped `/llms.txt` index, `/sitemap.xml`,
and `/robots.txt`. Markdown comes only from public browser snippets; code blocks
stay intact, while prose links using `icedcn://` become public web links. HTML
advertises its Markdown counterpart and the index. Without JavaScript, the
HTML page shows the same documentation as readable text. `llms.txt` is a discovery
convention, not a guarantee that a crawler will use the site.

To refresh metadata and artwork around an existing WASM build, without rebuilding
WASM, first export the current registry and then use the postprocessor:

```sh
cargo run --quiet --locked -p browser --bin browser -- --export-web-routes > /tmp/icedcn-routes.json
uv run --with pillow==12.0.0 python examples/browser/web/webmetadata.py --routes /tmp/icedcn-routes.json --output target/browser-web
```

The postprocessor expects a build made by the current template. The regular build
is the source of truth and supplies the freshly generated shell directly.

The WASM build bundles Twemoji Mozilla v0.7.0 as a color emoji fallback because
browser applications cannot load system fonts. Its COLR/CPAL version 0 font is
supported by the locked cosmic-text/Swash renderer on both WebGPU and the CPU
fallback. This is the unmodified [Mozilla release font](https://github.com/mozilla/twemoji-colr/releases/tag/v0.7.0),
with Twemoji artwork under CC BY 4.0 and font-generation code under Apache 2.0;
its attribution, upstream license notice, source URL and SHA-256 are recorded in
`../fonts/TwemojiMozilla-LICENSE.txt`.

For the same reason icedcn bundles Inter Symbols, a subset of [Inter](https://github.com/rsms/inter)
4.001 (SIL OFL 1.1) with the key and interface symbols Geist lacks (⌘ ⌥ ⌃ ⌄ ⌫ ⏎
✓ ✗ and others), renamed so it never stands in for a full Inter. Its widgets
load it into iced's font system, so cosmic-text falls back to it like to any
loaded font; its license is `widget/fonts/InterSymbols-OFL.txt`. The build
includes all bundled font licenses at `/font-licenses.txt`.

The loader preserves Cmd/Ctrl+R and Cmd/Ctrl+Shift+R as native browser shortcuts,
preventing the canvas event handler from cancelling them. Right-clicks on the
canvas use the application's context menu; surrounding HTML retains its native
browser menu.

Browser navigation uses `/` for Home, `/create` for the theme customizer,
`/components/<slug>`, `/docs/<slug>`, `/examples`, `/examples/receipts`, and `/examples/playground`.
Reload and browser Back/Forward restore the selected page; editing state inside
examples and Create remains in memory. Create uses the palette example library
and keeps its theme separate from the documentation theme. Unknown paths show a not-found page and retain the requested URL; trailing
slashes are accepted.

The Workers asset configuration serves the generated HTML with canonical paths
without trailing slashes. Known pages redirect trailing slash and `.html` forms
to those paths. Unknown paths return the generated shell with HTTP 404 and
`noindex`; the application still renders its not-found screen and preserves the
requested URL. Missing assets also return 404. JS, WASM, images, Markdown, and
other existing files remain ordinary static assets.

Use Wrangler's local server to test these rules; Python's static server above
does not provide Cloudflare's extensionless routing or custom 404 handling:

```sh
npx --yes wrangler@4.69.0 dev --config examples/browser/wrangler.jsonc --local
python3 examples/browser/web/check-webmetadata.py --base-url http://localhost:8787 --routes /tmp/icedcn-routes.json
```

The app-local `routing` crate owns pathname resolution and browser history.
`Router<R>` accepts application route types implementing `Route`: a name,
canonical path, optional title, and optional aliases. The exported application
registry supplies the static build's paths, titles, and page documentation.

Static routing behavior follows Cloudflare's [HTML handling](https://developers.cloudflare.com/workers/static-assets/routing/advanced/html-handling/)
and [custom 404 pages](https://developers.cloudflare.com/workers/static-assets/routing/static-site-generation/).
The Markdown index follows the [llms.txt proposal](https://llmstxt.org/).

## Mobile keyboard and web input

The locked winit web backend leaves its IME methods unimplemented. `ime.patch`
adds a window-owned DOM input to iced_winit, driven by iced's existing
`InputMethod` request. It forwards text/composition through IME events and
navigation/selection shortcuts through the canvas keyboard handler. Secure fields
use a password input; the DOM input is an ephemeral buffer, while iced retains
the actual field text and selection.

On web input gestures, the runtime settles queued application messages and an
IME-collecting redraw before returning to the original DOM handler. This lets a
newly opened command dialog request the keyboard within its opening tap. Ordinary
animation frames cannot activate the DOM input. The bridge does not inspect
application routes, labels, or pointer coordinates to decide whether to open.
Native builds do not include this patch.

This addresses the missing backend described in [winit's virtual keyboard issue](https://github.com/rust-windowing/winit/issues/3938)
and the gesture requirement described by [WebKit](https://bugs.webkit.org/show_bug.cgi?id=195884).

To exercise the actual Rust DOM bridge without building the browser or GPU
backend, install Playwright with Chromium and WebKit and run:

```sh
python3 examples/browser/web/check-web-ime.py
```

Set `WASM_BINDGEN` to the CLI matching Cargo.lock and `PLAYWRIGHT_MODULE` if
Playwright is installed outside Node's normal lookup paths. Optional
`CHROMIUM_EXECUTABLE` and `WEBKIT_EXECUTABLE` select existing browser binaries.
The harness checks gesture gating, text insertion, deletion, composition,
shortcut modifiers, secure input, window focus transfer, and cleanup. Desktop
WebKit can exercise these DOM behaviors but cannot display a physical iOS
software keyboard.
