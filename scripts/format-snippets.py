#!/usr/bin/env python3
"""Format Rust documentation fences with the same 100-column rustfmt policy."""

import argparse
from pathlib import Path
import re
import subprocess
import sys
import textwrap


FENCE = re.compile(r"(?m)^(```rust[^\n]*\n)(.*?)^(```)", re.DOTALL)
RAW_STRING = re.compile(r'\b(?:b|c)?r(#{0,255})".*?"\1', re.DOTALL)


def format_rust(source, rustfmt):
    command = [rustfmt, "--edition", "2024", "--config", "max_width=100", "--emit", "stdout"]
    result = subprocess.run(command, input=source, text=True, capture_output=True)
    if result.returncode == 0:
        return result.stdout
    # Many examples intentionally show an expression rather than a whole program.
    # Supply the wrapper's indentation up front: rustfmt can leave unfamiliar
    # macro bodies untouched, so blindly dedenting an unindented input drifts.
    # Protect raw strings while indenting/dedenting their surrounding syntax.
    literals = []

    def protect(match):
        literals.append(match[0])
        return f"__ICEDCN_RAW_LITERAL_{len(literals) - 1}__"

    protected = RAW_STRING.sub(protect, source)
    result = subprocess.run(
        command,
        input="fn main() {\n" + textwrap.indent(protected, "    ") + "\n}\n",
        text=True,
        capture_output=True,
    )
    if result.returncode:
        raise ValueError(result.stderr.strip())
    lines = result.stdout.splitlines()[1:-1]
    formatted = "\n".join(line[4:] if line.startswith("    ") else line for line in lines) + "\n"
    for index, literal in enumerate(literals):
        formatted = formatted.replace(f"__ICEDCN_RAW_LITERAL_{index}__", literal)
    return formatted


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    parser.add_argument("paths", nargs="*", type=Path)
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[1]
    paths = args.paths or sorted((root / "examples/browser/snippets").glob("*.md"))
    rustfmt = subprocess.check_output(["rustup", "which", "rustfmt"], text=True).strip()
    changed = 0
    failures = 0
    for path in paths:
        source = path.read_text()
        try:
            formatted = FENCE.sub(
                lambda match: match[1] + format_rust(match[2], rustfmt) + match[3], source
            )
        except ValueError as error:
            print(f"{path}: {error}", file=sys.stderr)
            failures += 1
            continue
        if source != formatted:
            changed += 1
            if args.check:
                print(path)
            else:
                path.write_text(formatted)
    print(f"{changed} snippets {'need formatting' if args.check else 'formatted'}; {failures} errors")
    return 1 if failures or (args.check and changed) else 0


if __name__ == "__main__":
    sys.exit(main())
