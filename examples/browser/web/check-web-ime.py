#!/usr/bin/env python3
"""Compile just the patched DOM input bridge and exercise it in Chromium/WebKit.

Requires the matching wasm-bindgen CLI, and Playwright with its browsers. Set
WASM_BINDGEN and PLAYWRIGHT_MODULE to select existing installations. This builds
no browser app, GPU backend, or native window; it cannot prove iOS keyboard UI.
"""
from functools import partial
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
from threading import Thread

ROOT = Path(__file__).resolve().parents[3]
WEB = Path(__file__).resolve().parent


def run(command, **kwargs):
    subprocess.run(command, check=True, **kwargs)


def main():
    metadata = json.loads(subprocess.check_output([
        "cargo", "metadata", "--format-version", "1", "--locked", "--offline",
        "--filter-platform", "wasm32-unknown-unknown",
    ], cwd=ROOT))
    packages = {package["name"]: package for package in metadata["packages"]}
    bindgen = os.environ.get("WASM_BINDGEN", "wasm-bindgen")
    expected = packages["wasm-bindgen"]["version"]
    installed = subprocess.check_output([bindgen, "--version"], text=True).split()[-1]
    if installed != expected:
        raise SystemExit(f"Set WASM_BINDGEN to wasm-bindgen {expected} (found {installed}).")
    with tempfile.TemporaryDirectory(prefix="icedcn-web-ime-") as tmp:
        tmp = Path(tmp)
        patched = tmp / "winit"
        shutil.copytree(Path(packages["iced_winit"]["manifest_path"]).parent, patched)
        for patch in ["clipboard.patch", "ime.patch"]:
            run(["patch", "-s", "-p1", "-i", str(WEB / patch)], cwd=patched)
        source = tmp / "src"
        source.mkdir()
        shutil.copyfile(patched / "src/web_input.rs", source / "web_input.rs")
        shutil.copyfile(WEB / "tests/ime.rs", source / "lib.rs")
        core = str(Path(packages["iced_core"]["manifest_path"]).parent)
        features = ["Document", "Window", "HtmlCanvasElement", "HtmlInputElement", "CssStyleDeclaration", "FocusEvent", "FocusOptions", "InputEvent", "CompositionEvent", "KeyboardEvent", "KeyboardEventInit", "DomRect"]
        (tmp / "Cargo.toml").write_text(f'''[package]
name = "ime-harness"
version = "0.1.0"
edition = "2024"
[lib]
crate-type = ["cdylib"]
[dependencies]
iced_core = {{ path = {json.dumps(core)} }}
wasm-bindgen = "={expected}"
web-sys = {{ version = "={packages['web-sys']['version']}", features = {json.dumps(features)} }}
''')
        shutil.copyfile(ROOT / "Cargo.lock", tmp / "Cargo.lock")
        env = os.environ.copy()
        target = Path(metadata["target_directory"]) / "web-ime-test"
        env["CARGO_TARGET_DIR"] = str(target)
        run(["cargo", "build", "--offline", "--manifest-path", str(tmp / "Cargo.toml"), "--target", "wasm32-unknown-unknown"], env=env, cwd=ROOT)
        site = tmp / "site"
        run([bindgen, "--target", "web", "--out-dir", str(site), str(target / "wasm32-unknown-unknown/debug/ime_harness.wasm")])
        shutil.copyfile(WEB / "tests/ime.html", site / "index.html")
        server = ThreadingHTTPServer(("127.0.0.1", 0), partial(SimpleHTTPRequestHandler, directory=str(site)))
        Thread(target=server.serve_forever, daemon=True).start()
        env["BASE_URL"] = f"http://127.0.0.1:{server.server_port}"
        try:
            run(["node", str(WEB / "tests/ime.cjs")], env=env)
        finally:
            server.shutdown()
            server.server_close()


if __name__ == "__main__":
    main()
