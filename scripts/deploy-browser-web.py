#!/usr/bin/env python3
"""Deploy a build-browser-web.py output: the wasm to R2, then the Worker and its assets."""
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]
WRANGLER = ["npx", "--yes", "wrangler@4.69.0"]
CONFIG = ROOT / "examples/browser/wrangler.jsonc"
BUCKET = "icedcn-wasm"


def main():
    output = ROOT / "target/browser-web"
    modules = sorted(output.glob("browser-*.wasm"))
    if len(modules) != 1:
        raise SystemExit(f"Expected one browser wasm in {output}; run scripts/build-browser-web.py.")
    wasm = modules[0]
    # Hashed names never change, so an upload must land before the pages that load it.
    subprocess.run([*WRANGLER, "r2", "object", "put", f"{BUCKET}/{wasm.name}", "--file", str(wasm),
                    "--content-type", "application/wasm", "--remote", "--config", str(CONFIG)],
                   cwd=ROOT, check=True)
    subprocess.run([*WRANGLER, "deploy", "--config", str(CONFIG)], cwd=ROOT, check=True)


if __name__ == "__main__":
    main()
