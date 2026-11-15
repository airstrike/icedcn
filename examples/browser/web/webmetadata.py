#!/usr/bin/env python3
"""Build public HTML, Markdown, icons, and share images from the native registry.

May also refresh an existing WASM build without recompiling it:
  python webmetadata.py --routes routes.json --output target/browser-web
"""
import argparse
from html import escape
import json
from pathlib import Path
import re
import runpy
import shutil
import xml.etree.ElementTree as ET

ROOT = Path(__file__).resolve().parents[3]
WEB = Path(__file__).resolve().parent
ORIGIN = "https://icedcn.airstrike.fyi"
SNIPPETS = (ROOT / "examples/browser/snippets").resolve()


def markdown_path(path):
    return "/index.md" if path == "/" else path + ".md"


def metadata(route, image_path, missing=False):
    title = "icedcn — Components for iced" if route["path"] == "/" else route["title"] + " — icedcn"
    description = route["description"]
    image = ORIGIN + image_path
    alt = route["title"] + " — icedcn components for iced, with an illustrated interface."
    tags = [f"<title>{escape(title)}</title>"]
    def meta(key, value, attribute="name"):
        tags.append(f'<meta {attribute}="{key}" content="{escape(value, quote=True)}">')
    meta("description", description)
    meta("og:type", "website", "property")
    meta("og:site_name", "icedcn", "property")
    for prefix, attr in [("og", "property"), ("twitter", "name")]:
        for key, value in [("title", title), ("description", description), ("image", image), ("image:alt", alt)]:
            meta(prefix + ":" + key, value, attr)
    for key, value in [("type", "image/png"), ("width", "1200"), ("height", "630")]:
        meta("og:image:" + key, value, "property")
    meta("twitter:card", "summary_large_image")
    if missing:
        meta("robots", "noindex")
    else:
        url = ORIGIN + route["path"]
        tags.append(f'<link rel="canonical" href="{escape(url, quote=True)}">')
        meta("og:url", url, "property")
        tags.append(f'<link rel="alternate" type="text/markdown" href="{ORIGIN}{markdown_path(route["path"])}" title="Markdown documentation">')
    tags.append(f'<link rel="describedby" type="text/plain" href="{ORIGIN}/llms.txt" title="Documentation index">')
    return "\n  ".join(tags)


def source(path):
    resolved = (ROOT / path).resolve()
    if not resolved.is_relative_to(SNIPPETS) or resolved.suffix != ".md":
        raise ValueError(f"Public documentation must come from snippets/: {path}")
    return resolved.read_text().strip()


def prose_links(markdown):
    # Preserve copyable code exactly; rewrite only Markdown prose links.
    def rewrite(match):
        namespace = "components" if match[1] == "component" else "docs"
        return f"]({ORIGIN}/{namespace}/{match[2]})"
    parts = re.split(r"(^```.*?^```[^\n]*$)", markdown, flags=re.M | re.S)
    return "".join(part if part.startswith("```") else
                   re.sub(r"\]\(icedcn://(component|guide)/([^\s)]+)\)", rewrite, part)
                   for part in parts)


def block_markdown(blocks, depth=2):
    parts = []
    for block in blocks:
        kind = block["kind"]
        if kind == "markdown":
            parts.append(source(block["path"]))
        elif kind == "example":
            if block["title"]:
                parts.append("#" * min(depth, 6) + " " + block["title"])
            if block["description"]:
                parts.append(block["description"])
            if block["snippet"]:
                parts.append(source(block["snippet"]))
        elif kind == "group":
            if block["title"]:
                parts.append("#" * min(depth, 6) + " " + block["title"])
            parts.extend(block_markdown(block["children"], depth + 1))
        else:
            raise ValueError(f"Unsupported exported block: {kind}")
    return parts


def documentation(route):
    parts = ["# " + route["title"], route["description"], "Interactive page: " + ORIGIN + route["path"]]
    if route["blocks"]:
        parts.extend(block_markdown(route["blocks"]))
    else:
        name = "home" if route["path"] == "/" else route["path"].strip("/").replace("/", "-")
        parts.append(source(f"examples/browser/snippets/web-{name}.md"))
    return prose_links("\n\n".join(parts)) + "\n"


def build(routes, output, shell=None):
    output.mkdir(parents=True, exist_ok=True)
    renderer = runpy.run_path(str(WEB / "render-og.py"))
    renderer["icons"](output)
    if shell is None:
        # Reuse only the loader's generated module names/byte count from an existing build.
        previous = (output / "index.html").read_text()
        shell = re.sub(r"<!-- WEB_METADATA_START -->.*?<!-- WEB_METADATA_END -->", "BROWSER_METADATA", previous, flags=re.S)
        shell = re.sub(r"<!-- WEB_DOCUMENTATION_START -->.*?<!-- WEB_DOCUMENTATION_END -->", "BROWSER_DOCUMENTATION", shell, flags=re.S)
    if shell.count("BROWSER_METADATA") != 1 or shell.count("BROWSER_DOCUMENTATION") != 1:
        raise ValueError("Expected one metadata and one documentation placeholder in the web shell")
    expected = set()
    cards = output / "og"
    cards.mkdir(exist_ok=True)
    for route in routes:
        path = route["path"]
        if not re.fullmatch(r"/|(?:/[a-z0-9]+(?:-[a-z0-9]+)*)+", path):
            raise ValueError(f"Invalid public route: {path}")
        stem = "index" if path == "/" else path.lstrip("/")
        target = output / (stem + ".html")
        target.parent.mkdir(parents=True, exist_ok=True)
        card_name = stem.replace("/", "-") + ".png"
        card = cards / card_name
        renderer["render"](card, route["title"], route["description"], path)
        md = documentation(route)
        # A no-JavaScript reader gets the same public docs that agents can fetch as Markdown.
        readable = '<noscript><article class="documentation"><pre>' + escape(md) + '</pre></article></noscript>'
        html = shell.replace("BROWSER_METADATA", "<!-- WEB_METADATA_START -->\n  " + metadata(route, "/og/" + card_name) + "\n  <!-- WEB_METADATA_END -->")
        html = html.replace("BROWSER_DOCUMENTATION", "<!-- WEB_DOCUMENTATION_START -->" + readable + "<!-- WEB_DOCUMENTATION_END -->")
        target.write_text(html)
        document = output / markdown_path(path).lstrip("/")
        document.write_text(md)
        expected.update([target, document, card])
    shutil.copyfile(cards / "index.png", output / "og.png")
    expected.add(output / "og.png")
    missing = {"path": "/404", "title": "Page not found", "description": "This page could not be found. Explore icedcn components, documentation, and examples."}
    renderer["render"](cards / "404.png", missing["title"], missing["description"], "/404")
    (output / "404.html").write_text(shell.replace("BROWSER_METADATA", metadata(missing, "/og/404.png", missing=True)).replace("BROWSER_DOCUMENTATION", ""))
    expected.update([output / "404.html", cards / "404.png"])
    # Remove only outputs recorded by this generator, including routes that were retired.
    manifest = output / ".webmetadata.json"
    resolved_expected = {p.resolve() for p in expected}
    if manifest.exists():
        for relative in json.loads(manifest.read_text()):
            old = (output / relative).resolve()
            if old.is_relative_to(output.resolve()) and old not in resolved_expected and old.is_file():
                old.unlink()
    manifest.write_text(json.dumps(sorted(str(p.relative_to(output)) for p in expected)))
    index = ["# icedcn", "> Native components for iced, with composable Rust APIs and copyable examples.",
             "This index links to the public documentation exported from the component browser. Markdown pages include usage, examples, and API guidance; interactive previews remain on the website."]
    for title, predicate in [("Start here", lambda p: p.startswith("/docs/")), ("Build and explore", lambda p: not p.startswith(("/docs/", "/components/"))), ("Components", lambda p: p.startswith("/components/"))]:
        index.append("## " + title)
        index.append("\n".join(f'- [{r["title"]}]({ORIGIN}{markdown_path(r["path"])}): {r["description"]}' for r in routes if predicate(r["path"])))
    (output / "llms.txt").write_text("\n\n".join(index) + "\n")
    ET.register_namespace("", "http://www.sitemaps.org/schemas/sitemap/0.9")
    sitemap = ET.Element("{http://www.sitemaps.org/schemas/sitemap/0.9}urlset")
    for route in routes:
        entry = ET.SubElement(sitemap, "url")
        ET.SubElement(entry, "loc").text = ORIGIN + route["path"]
    ET.ElementTree(sitemap).write(output / "sitemap.xml", encoding="utf-8", xml_declaration=True)
    (output / "robots.txt").write_text(f"User-agent: *\nAllow: /\n\nSitemap: {ORIGIN}/sitemap.xml\n")
    (output / ".assetsignore").write_text(".webmetadata.json\n.assetsignore\n")
    print(f"Generated metadata, share cards, and Markdown for {len(routes)} routes.")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--routes", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--shell", type=Path)
    args = parser.parse_args()
    build(json.loads(args.routes.read_text()), args.output, args.shell.read_text() if args.shell else None)
