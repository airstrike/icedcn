#!/usr/bin/env python3
"""Deterministic share cards and icons from the bundled logo and Geist fonts.

Requires Pillow and rsvg-convert. Called by webmetadata.py during web builds.
"""
from functools import lru_cache
from io import BytesIO
from pathlib import Path
import subprocess

from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[3]
SCALE = 2
WIDTH, HEIGHT = 1200, 630


@lru_cache(maxsize=None)
def font(size, weight="Regular", mono=False):
    filename = "GeistMono-Regular.ttf" if mono else "Geist.ttf"
    face = ImageFont.truetype(ROOT / "examples/browser/fonts" / filename, size * SCALE)
    if not mono:
        face.set_variation_by_name(weight)
    return face


@lru_cache(maxsize=None)
def logo(size):
    png = subprocess.check_output([
        "rsvg-convert", "--width", str(size), "--height", str(size),
        str(ROOT / "assets/logo.svg"),
    ])
    return Image.open(BytesIO(png)).convert("RGBA")


def icons(output):
    (output / "favicon.svg").write_bytes((ROOT / "assets/logo.svg").read_bytes())
    logo(32).save(output / "favicon.png")
    logo(48).save(output / "favicon.ico", sizes=[(16, 16), (32, 32), (48, 48)])
    apple = Image.new("RGB", (180, 180), "#fafafa")
    apple.paste(logo(152), (14, 14), logo(152))
    apple.save(output / "apple-touch-icon.png")


def render(output, title, description, path):
    image = Image.new("RGB", (WIDTH * SCALE, HEIGHT * SCALE), "#fafafa")
    draw = ImageDraw.Draw(image)

    def box(bounds, fill, radius=12, outline=None):
        draw.rounded_rectangle(tuple(v * SCALE for v in bounds), radius=radius * SCALE,
                               fill=fill, outline=outline, width=SCALE)

    def text(x, y, value, size=20, color="#71717a", weight="Regular", mono=False):
        draw.text((x * SCALE, y * SCALE), value, font=font(size, weight, mono), fill=color)

    def line(points, color="#e4e4e7", width=1):
        draw.line(tuple(v * SCALE for v in points), fill=color, width=width * SCALE)

    def wrapped(value, x, y, size, max_width, max_lines=3, color="#71717a", weight="Regular"):
        words = value.split()
        lines = []
        while words and len(lines) < max_lines:
            current = words.pop(0)
            while words and draw.textlength(current + " " + words[0], font=font(size, weight)) <= max_width * SCALE:
                current += " " + words.pop(0)
            lines.append(current)
        if words:
            while draw.textlength(lines[-1] + "…", font=font(size, weight)) > max_width * SCALE:
                lines[-1] = lines[-1][:-1]
            lines[-1] += "…"
        for index, value in enumerate(lines):
            text(x, y + index * size * 1.18, value, size, color, weight)
        return len(lines) * size * 1.18

    # An editorial grid gives the neutral palette a little depth without a gradient.
    for x in range(780, WIDTH, 24):
        for y in range(0, HEIGHT, 24):
            draw.ellipse((x * SCALE, y * SCALE, x * SCALE + 2, y * SCALE + 2), fill="#d4d4d8")
    image.paste(logo(62 * SCALE), (55 * SCALE, 46 * SCALE), logo(62 * SCALE))
    text(127, 53, "icedcn", 40, "#18181b", "SemiBold")
    category = ("THEME BUILDER" if path == "/create" else "DOCUMENTATION" if path.startswith("/docs/")
                else "COMPONENTS" if path.startswith("/components/") else "EXAMPLES" if path.startswith("/examples")
                else "RUST UI LIBRARY")
    text(64, 170, category, 16, mono=True)
    heading = "Components\nfor iced." if path == "/" else title
    y = 207
    if "\n" in heading:
        for value in heading.splitlines():
            text(60, y, value, 68, "#18181b", "SemiBold")
            y += 76
    else:
        y += wrapped(heading, 60, y, 64, 650, 2, "#18181b", "SemiBold")
    wrapped(description, 64, max(385, y + 20), 23, 625, 3)
    line((64, 546, 1136, 546))
    text(64, 574, "icedcn.airstrike.fyi", 17, mono=True)
    footer = path if path != "/" else "Native interfaces. Written in Rust."
    footer_width = draw.textlength(footer, font=font(15, mono=True)) / SCALE
    text(1136 - footer_width, 577, footer, 15, mono=True)

    # Keep motifs illustrative: they complement the page title, not fake a screenshot.
    box((768, 145, 1136, 490), "#e4e4e7", 22)
    box((760, 137, 1128, 482), "#ffffff", 22, "#d4d4d8")
    if path == "/create" or path.endswith("/theming"):
        text(790, 166, "Make it yours.", 24, "#18181b", "Medium")
        for i, color in enumerate(["#18181b", "#f97316", "#8b5cf6", "#0d9488", "#2563eb"]):
            box((790 + i * 61, 220, 839 + i * 61, 285), color, 9)
        box((790, 317, 1098, 367), "#18181b", 9)
        text(862, 327, "Your next app", 20, "#fafafa", "Medium")
        line((800, 412, 1088, 412), "#e4e4e7", 5)
        line((800, 412, 976, 412), "#8b5cf6", 5)
        box((967, 402, 987, 422), "#ffffff", 10, "#8b5cf6")
    elif path == "/examples/receipts":
        text(791, 166, "Receipt", 25, "#18181b", "Medium")
        text(791, 208, "ORDER #0042", 14, mono=True)
        for y, label, amount in [(266, "Components", "12"), (310, "Possibilities", "∞")]:
            text(791, y, label, 18)
            text(1058, y, amount, 18, "#18181b", mono=True)
        line((791, 355, 1095, 355))
        text(791, 382, "Made with icedcn", 18, "#18181b", "Medium")
        box((791, 426, 895, 452), "#dcfce7", 13)
        text(814, 429, "Ready", 14, "#166534", "Medium")
    elif path.startswith("/docs/"):
        text(791, 166, "A little Rust.", 24, "#18181b", "Medium")
        box((786, 221, 1102, 364), "#18181b", 10)
        for y, value, color in [(242, 'column![', "#c4b5fd"), (276, '  text("Hello"),', "#fafafa"), (310, '  button("Build"),', "#fafafa"), (339, ']', "#c4b5fd")]:
            text(805, y, value, 17, color, mono=True)
        text(791, 402, "A native interface.", 22, "#71717a")
    else:
        text(791, 166, "Build something.", 24, "#18181b", "Medium")
        box((790, 226, 1098, 277), "#ffffff", 9, "#d4d4d8")
        text(807, 240, "Your next idea", 18)
        box((790, 297, 978, 347), "#18181b", 9)
        text(830, 310, "Get started", 19, "#fafafa", "Medium")
        box((998, 306, 1098, 338), "#e4e4e7", 16)
        box((1069, 309, 1095, 335), "#ffffff", 13)
        box((790, 390, 813, 413), "#18181b", 5)
        line((796, 401, 801, 406, 808, 397), "#ffffff", 2)
        text(827, 389, "Ready when you are", 18, "#52525b")
        for i, height in enumerate([14, 25, 21, 37, 30, 47]):
            box((990 + i * 17, 458 - height, 1000 + i * 17, 458), "#a1a1aa" if i < 5 else "#18181b", 3)

    image.resize((WIDTH, HEIGHT), Image.Resampling.LANCZOS).save(output, optimize=True)
