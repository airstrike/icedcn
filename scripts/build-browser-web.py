#!/usr/bin/env python3
"""Build the component browser, public docs, and share cards for static hosting."""
import hashlib
import json
import os
import re
from pathlib import Path
import shutil
import subprocess
import runpy
import tempfile

ROOT = Path(__file__).resolve().parents[1]
TARGET = "wasm32-unknown-unknown"


def patched_source(build, package, crate, file, original=None, replacement="", patch=None):
    manifest = Path(package["manifest_path"])
    source = manifest.parent
    revision = package["source"].partition("#")[2] or package["version"]
    inputs = Path(__file__).read_bytes() + replacement.encode()
    patches = [patch] if isinstance(patch, Path) else (patch or [])
    for item in patches:
        inputs += item.read_bytes()
    patch_hash = hashlib.sha256(inputs).hexdigest()[:8]
    patched = build / "web-dependencies" / f'{package["name"]}-{revision}-{patch_hash}'
    if not patched.exists():
        patched.parent.mkdir(parents=True, exist_ok=True)
        with tempfile.TemporaryDirectory(dir=build) as tmp:
            staging = Path(tmp) / "checkout"
            shutil.copytree(source, staging, ignore=shutil.ignore_patterns(".git", "target"))
            for item in patches:
                subprocess.run(["patch", "-s", "-p1", "-i", str(item)], cwd=staging, check=True)
            if crate != ".":
                # Lift just this member into its own workspace so Cargo resolves
                # the other iced crates from the application's existing patches.
                upstream = (source.parent / "Cargo.toml").read_text()
                shared = []
                for name in ("package", "dependencies", "lints.rust", "lints.clippy", "lints.rustdoc"):
                    section = re.search(rf"(?ms)^\[workspace\.{name}\].*?(?=^\[|\Z)", upstream)
                    shared.append(re.sub(r', path = "[^"]+"', "", section.group()))
                with (staging / "Cargo.toml").open("a") as output:
                    output.write("\n[workspace]\n" + "\n".join(shared))
            if original is not None:
                path = staging / file
                code = path.read_text()
                if code.count(original) != 1:
                    raise SystemExit(f"Review the web build patch for {package['name']} in {file}")
                path.write_text(code.replace(original, replacement))
            staging.rename(patched)
    return patched


def main():
    os.chdir(ROOT)
    # Fail before the expensive native/WASM builds if the image tools are absent.
    runpy.run_path(str(ROOT / "examples/browser/web/render-og.py"))
    if not shutil.which("rsvg-convert"):
        raise SystemExit("Install librsvg (rsvg-convert) to generate web share images.")
    # Use the application's own registry; no mirrored route list or Rust source scraping.
    routes = json.loads(subprocess.check_output([
        "cargo", "run", "--quiet", "--locked", "-p", "browser", "--bin", "browser",
        "--", "--export-web-routes",
    ]))
    metadata = json.loads(subprocess.check_output([
        "cargo", "metadata", "--format-version", "1", "--locked",
        "--filter-platform", TARGET,
    ]))
    packages = {p["name"]: p for p in metadata["packages"]}
    build = Path(metadata["target_directory"])
    bindgen = os.environ.get("WASM_BINDGEN", "wasm-bindgen")
    expected = packages["wasm-bindgen"]["version"]
    installed = subprocess.check_output([bindgen, "--version"], text=True).split()[-1]
    if installed != expected:
        raise SystemExit(f"Install wasm-bindgen-cli {expected} or set WASM_BINDGEN to its executable.")

    # The parser's C grammar needs the same freestanding headers as its Rust runtime.
    headers = Path(packages["tree-sitter-language"]["manifest_path"]).parent / "wasm/include"
    env = os.environ.copy()
    env["CC_SHELL_ESCAPED_FLAGS"] = "1"
    env["CFLAGS_wasm32_unknown_unknown"] = f'-I"{headers}" -DTREE_SITTER_REUSE_ALLOCATOR ' + env.get("CFLAGS_wasm32_unknown_unknown", "")

    # Keep the locked native checkouts intact. The web build needs Sweeten's
    # menu to use iced's portable clock. Winit can also report an initial
    # 0x0 canvas until ResizeObserver runs, which the CPU compositor rejects.
    sweeten = patched_source(
        build, packages["sweeten"], ".", "src/widget/overlay/menu.rs",
        "use std::time::{Duration, Instant};", "use crate::core::time::{Duration, Instant};",
    )
    configure = "compositor.configure_surface(&mut surface, surface_size.width, surface_size.height);"
    winit = patched_source(
        build, packages["iced_winit"], "winit", "src/window.rs",
        configure,
        f"if surface_size.width > 0 && surface_size.height > 0 {{\n            {configure}\n        }}",
        patch=ROOT / "examples/browser/web/ime.patch",
    )
    # Probe adapter/device compatibility on a disposable canvas. A failed WebGPU
    # attempt must leave the application's canvas free to acquire a 2D context.
    probe = "wgpu::SurfaceTarget::Window(Box::new(compatible_window))"
    disposable = """{
                use wgpu::web_sys::{self, HtmlCanvasElement, wasm_bindgen::JsCast};
                let _ = compatible_window;
                let canvas = web_sys::window()
                    .and_then(|window| window.document())
                    .and_then(|doc| doc.create_element("canvas").ok())
                    .and_then(|element| element.dyn_into::<HtmlCanvasElement>().ok())
                    .ok_or(Error::IncompatibleSurface)?;
                wgpu::SurfaceTarget::Canvas(canvas)
            }"""
    gpu = patched_source(build, packages["iced_wgpu"], "wgpu", "src/window/compositor.rs",
                         probe, disposable)
    # Safari rejects repeated copies from one mapped upload buffer at submit
    # (gfx-rs/wgpu#10460). Recheck the workaround whenever the locked belt changes.
    wgpu = packages["wgpu"]
    belt = "src/util/belt.rs"
    source = Path(wgpu["manifest_path"]).parent / belt
    if (wgpu["version"] != "30.0.1"
            or hashlib.sha256(source.read_bytes()).hexdigest()
            != "6b5869754341042480cf43e65d83e3006d3b9a506165bd5f2394c8c26d3f0be0"):
        raise SystemExit("Review the WebGPU staging-belt patch for this wgpu source.")
    uploads = patched_source(build, wgpu, ".", belt,
                             patch=ROOT / "examples/browser/web/staging-belt.patch")
    overrides = [str(path) for path in (sweeten, winit, gpu, uploads)]
    result = subprocess.run([
        "cargo", "--config", "paths = " + json.dumps(overrides),
        "build", "--locked", "--release", "--target", TARGET, "-p", "browser", "--bin", "browser",
        "--message-format=json-render-diagnostics",
    ], env=env, stdout=subprocess.PIPE, text=True, check=True)
    # Cargo can ignore an unused paths override. Check the resolved artifact
    # on fresh and incremental builds before creating any deployable files.
    artifacts = [json.loads(line) for line in result.stdout.splitlines()]
    for name, patched in [("wgpu", uploads), ("iced_winit", winit)]:
        if not any(item.get("reason") == "compiler-artifact"
                   and Path(item["manifest_path"]).resolve() == (patched / "Cargo.toml").resolve()
                   for item in artifacts):
            raise SystemExit(f"The browser build did not use the patched {name} crate.")
    output = build / "browser-web"
    output.mkdir(exist_ok=True)
    with tempfile.TemporaryDirectory(dir=build) as tmp:
        tmp = Path(tmp)
        subprocess.run([
            bindgen, "--target", "web", "--no-typescript", "--out-dir", tmp,
            build / TARGET / "release/browser.wasm",
        ], check=True)
        wasm = tmp / "browser_bg.wasm"
        optimized = tmp / "optimized.wasm"
        subprocess.run(["wasm-opt", "-Oz", wasm, "-o", optimized], check=True)
        js = (tmp / "browser.js").read_text()
        digest = hashlib.sha256(optimized.read_bytes() + js.encode()).hexdigest()[:16]
        wasm_name, js_name = f"browser-{digest}.wasm", f"browser-{digest}.js"
        shutil.copyfile(optimized, output / wasm_name)
        (output / js_name).write_text(js.replace("browser_bg.wasm", f"/{wasm_name}"))
        template = (ROOT / "examples/browser/web/index.html").read_text()
        html = (template.replace("BROWSER_MODULE", js_name)
                .replace("BROWSER_WASM", wasm_name)
                .replace("BROWSER_BYTES", str(optimized.stat().st_size))
                .replace("BROWSER_LOGO", (ROOT / "assets/logo.svg").read_text().strip()))
        webmetadata = runpy.run_path(str(ROOT / "examples/browser/web/webmetadata.py"))
        webmetadata["build"](routes, output, html)
        (output / "font-licenses.txt").write_text("\n\n".join(
            (ROOT / path).read_text()
            for path in (
                "examples/browser/fonts/Geist-OFL.txt",
                "examples/browser/fonts/GeistMono-OFL.txt",
                "widget/fonts/InterSymbols-OFL.txt",
                "examples/browser/fonts/TwemojiMozilla-LICENSE.txt",
            )
        ))
        shutil.copyfile(ROOT / "assets/Chromium-LICENSE", output / "third-party-licenses.txt")
        # The wasm outgrows Workers Static Assets; scripts/deploy-browser-web puts it in R2.
        (output / ".assetsignore").write_text("browser-*.wasm\n")
        # Only generated modules in this dedicated output tree are stale.
        for pattern in ("browser-*.wasm", "browser-*.js"):
            for old in output.glob(pattern):
                if old.name not in (wasm_name, js_name):
                    old.unlink()
    print(output)


if __name__ == "__main__":
    main()
