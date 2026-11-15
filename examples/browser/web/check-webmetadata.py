#!/usr/bin/env python3
"""Verify the built site's crawler responses against its exported route registry."""
import argparse
from html.parser import HTMLParser
import json
from pathlib import Path
import re
import struct
from urllib.error import HTTPError
from urllib.request import Request, urlopen
import xml.etree.ElementTree as ET

ORIGIN = "https://icedcn.airstrike.fyi"


class Head(HTMLParser):
    def __init__(self, html):
        super().__init__()
        self.meta = {}
        self.links = {}
        self.scripts = []
        self.feed(html)

    def handle_starttag(self, tag, attrs):
        attrs = dict(attrs)
        if tag == "meta":
            key = attrs.get("name", attrs.get("property"))
            if key:
                assert key not in self.meta, f"Duplicate metadata: {key}"
                self.meta[key] = attrs.get("content")
        if tag == "link":
            self.links.setdefault(attrs.get("rel"), []).append(attrs)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--base-url", default="http://localhost:8787")
    parser.add_argument("--routes", type=Path, required=True)
    args = parser.parse_args()
    routes = json.loads(args.routes.read_text())

    def fetch(path, method="GET"):
        try:
            response = urlopen(Request(args.base_url.rstrip("/") + path, method=method), timeout=30)
        except HTTPError as error:
            response = error
        with response:
            return response.status, response.headers, response.read()

    def png(path, dimensions):
        status, headers, body = fetch(path)
        assert status == 200, (path, status)
        assert headers.get_content_type() == "image/png", (path, headers)
        assert body[:8] == b"\x89PNG\r\n\x1a\n", path
        assert struct.unpack(">II", body[16:24]) == dimensions, path

    root_html = None
    for route in routes:
        path = route["path"]
        status, headers, body = fetch(path + "?preview=1")
        assert status == 200 and headers.get_content_type() == "text/html", (path, status)
        html = body.decode()
        assert "BROWSER_" not in html, path
        head = Head(html)
        title = "icedcn — Components for iced" if path == "/" else route["title"] + " — icedcn"
        assert head.meta["og:title"] == title, path
        assert head.meta["twitter:title"] == title, path
        assert head.meta["description"] == route["description"], path
        assert head.meta["og:description"] == route["description"], path
        assert head.meta["twitter:description"] == route["description"], path
        assert head.meta["og:url"] == ORIGIN + path, path
        assert head.links["canonical"][0]["href"] == ORIGIN + path, path
        assert head.meta["twitter:image"] == head.meta["og:image"], path
        png(head.meta["og:image"].removeprefix(ORIGIN), (1200, 630))
        md_path = "/index.md" if path == "/" else path + ".md"
        assert head.links["alternate"][0]["href"] == ORIGIN + md_path, path
        assert head.links["describedby"][0]["href"] == ORIGIN + "/llms.txt", path
        status, _, md = fetch(md_path)
        assert status == 200 and md.startswith(("# " + route["title"]).encode()), md_path
        assert len(md) > len(route["description"]) + 150, md_path
        assert not re.search(rb"\]\(icedcn://", md), md_path
        if path == "/components/button":
            assert b"```rust" in md and b".style(" in md, "Missing public Rust examples"
        if path == "/docs/installation":
            assert b"[patch.crates-io]" in md, "Missing actual installation instructions"
        if path == "/":
            root_html = html
    for path in ["/not-a-page", "/components/not-a-widget", "/docs/unknown", "/%3Cscript%3Ealert(1)%3C%2Fscript%3E", "/%252Funknown", "/missing.js", "/missing.wasm"]:
        status, _, body = fetch(path)
        assert status == 404, (path, status)
        head = Head(body.decode())
        assert head.meta["robots"] == "noindex", path
        assert head.meta["og:title"] == "Page not found — icedcn", path
        assert "canonical" not in head.links and "og:url" not in head.meta, path
    for path, content_type in [("/favicon.svg", "image/svg+xml"), ("/favicon.ico", "image/vnd.microsoft.icon")]:
        status, headers, body = fetch(path)
        assert status == 200 and headers.get_content_type() == content_type, (path, headers)
        assert body and b"<!doctype html>" not in body, path
    png("/favicon.png", (32, 32))
    png("/apple-touch-icon.png", (180, 180))
    png("/og.png", (1200, 630))
    for extension, expected_type, magic in [("js", "text/javascript", None), ("wasm", "application/wasm", b"\0asm")]:
        asset = re.search(r"/(browser-[a-f0-9]+\." + extension + ")", root_html)[0]
        status, headers, body = fetch(asset)
        assert status == 200 and headers.get_content_type() == expected_type, (asset, headers)
        if magic:
            assert body.startswith(magic), asset
        else:
            assert b"<!doctype html>" not in body and b"export" in body, asset
    status, _, body = fetch("/llms.txt")
    assert status == 200 and body.startswith(b"# icedcn")
    assert body.count(b"](") == len(routes)
    status, _, body = fetch("/sitemap.xml")
    assert status == 200
    sitemap = ET.fromstring(body)
    locations = {el.text for el in sitemap.iter() if el.tag.endswith("loc")}
    assert locations == {ORIGIN + r["path"] for r in routes}
    status, _, body = fetch("/robots.txt")
    assert status == 200 and b"Allow: /" in body and b"Sitemap: " + ORIGIN.encode() + b"/sitemap.xml" in body
    status, _, body = fetch("/.webmetadata.json")
    assert status == 404, "Private generation manifest must not be served"
    # Wrangler resolves these to canonical route HTML and keeps route-specific metadata.
    for path in ["/create/", "/create.html", "/components/button/", "/components/button.html"]:
        status, _, body = fetch(path)
        assert status == 200 and Head(body.decode()).meta["og:title"] != "icedcn — Components for iced", path
    print(f"Passed: {len(routes)} route metadata, images, Markdown, discovery files, icons, unknown paths, and static assets.")


if __name__ == "__main__":
    main()
