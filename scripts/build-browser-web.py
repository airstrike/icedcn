#!/usr/bin/env python3
"""Build the component browser into target/browser-web for static hosting."""
import hashlib
import json
import os
import re
from pathlib import Path
import shutil
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]
TARGET = "wasm32-unknown-unknown"


def patched_source(build, package, crate, file, original, replacement, patch=None):
    manifest = Path(package["manifest_path"])
    source = manifest.parent
    revision = package["source"].rsplit("#", 1)[1]
    inputs = Path(__file__).read_bytes() + replacement.encode()
    if patch:
        inputs += patch.read_bytes()
    patch_hash = hashlib.sha256(inputs).hexdigest()[:8]
    patched = build / "web-dependencies" / f'{package["name"]}-{revision}-{patch_hash}'
    if not patched.exists():
        patched.parent.mkdir(parents=True, exist_ok=True)
        with tempfile.TemporaryDirectory(dir=build) as tmp:
            staging = Path(tmp) / "checkout"
            shutil.copytree(source, staging, ignore=shutil.ignore_patterns(".git", "target"))
            if patch:
                subprocess.run(["patch", "-s", "-p1", "-i", str(patch)], cwd=staging, check=True)
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
            path = staging / file
            code = path.read_text()
            if code.count(original) != 1:
                raise SystemExit(f"Review the web build patch for {package['name']} in {file}")
            path.write_text(code.replace(original, replacement))
            staging.rename(patched)
    return patched


def main():
    os.chdir(ROOT)
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
        patch=ROOT / "examples/browser/web/clipboard.patch",
    )
    subprocess.run([
        "cargo", "--config", "paths = " + json.dumps([str(sweeten), str(winit)]),
        "build", "--locked", "--release", "--target", TARGET, "-p", "browser", "--bin", "browser",
    ], env=env, check=True)
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
        (output / js_name).write_text(js.replace("browser_bg.wasm", wasm_name))
        template = (ROOT / "examples/browser/web/index.html").read_text()
        (output / "index.html").write_text(template.replace("BROWSER_MODULE", js_name))
        # Only generated modules in this dedicated output tree are stale.
        for pattern in ("browser-*.wasm", "browser-*.js"):
            for old in output.glob(pattern):
                if old.name not in (wasm_name, js_name):
                    old.unlink()
    print(output)


if __name__ == "__main__":
    main()
