#!/usr/bin/env python3
"""Render the social preview from the project's logo and bundled Geist fonts.

Requires Pillow and rsvg-convert. The normal web build copies the rendered PNG;
it does not need these image-generation dependencies.
"""

from io import BytesIO
from pathlib import Path
import subprocess

from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[3]
SCALE = 2
WIDTH, HEIGHT = 1200, 630


def font(size, weight="Regular", mono=False):
    filename = "GeistMono-Regular.ttf" if mono else "Geist.ttf"
    face = ImageFont.truetype(ROOT / "examples/browser/fonts" / filename, size * SCALE)
    if not mono:
        face.set_variation_by_name(weight)
    return face


def main():
    image = Image.new("RGB", (WIDTH * SCALE, HEIGHT * SCALE), "#0a0a0a")
    draw = ImageDraw.Draw(image)
    draw.rounded_rectangle(
        (32 * SCALE, 32 * SCALE, 1168 * SCALE, 598 * SCALE),
        radius=24 * SCALE, outline="#282828", width=SCALE,
    )
    draw.text((88 * SCALE, 90 * SCALE), "RUST UI LIBRARY", font=font(18, mono=True), fill="#a3a3a3")
    draw.text((81 * SCALE, 190 * SCALE), "icedcn", font=font(132, "Medium"), fill="#fafafa")
    draw.text((88 * SCALE, 354 * SCALE), "Components for iced.", font=font(38), fill="#b8b8b8")
    logo = subprocess.check_output([
        "rsvg-convert", "--width", str(280 * SCALE), "--height", str(280 * SCALE),
        str(ROOT / "assets/logo.svg"),
    ])
    image.paste(Image.open(BytesIO(logo)), (820 * SCALE, 184 * SCALE), Image.open(BytesIO(logo)))
    draw.line((88 * SCALE, 492 * SCALE, 1112 * SCALE, 492 * SCALE), fill="#282828", width=SCALE)
    draw.text((88 * SCALE, 524 * SCALE), "icedcn.airstrike.fyi", font=font(20, mono=True), fill="#a3a3a3")
    image = image.resize((WIDTH, HEIGHT), Image.Resampling.LANCZOS)
    image.save(Path(__file__).with_name("og.png"), optimize=True)


if __name__ == "__main__":
    main()
