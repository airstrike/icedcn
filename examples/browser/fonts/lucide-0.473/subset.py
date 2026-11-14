"""Rebuild the browser's historical Lucide glyph offline with fonttools==4.63.0.

Obtain font/lucide.ttf from the lucide-static@0.473.0 npm package, then run
    python3 subset.py /path/to/lucide.ttf
The message-circle-dashed SVG in this release is identical to lucide-react
0.474.0. Lucide redesigned its contours in 0.536.0.
"""
import argparse
import hashlib
from pathlib import Path

from fontTools import subset
from fontTools.ttLib import TTFont

SOURCE_SHA256 = "9f7dea115765813d093dce70f36712ebc94582f529019172aecac2bd72a8e3c9"
GLYPH = "message-circle-dashed"
FAMILY = "Lucide 0.473"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("source", type=Path)
    source = parser.parse_args().source
    if hashlib.sha256(source.read_bytes()).hexdigest() != SOURCE_SHA256:
        parser.error("expected the original lucide-static@0.473.0 font/lucide.ttf")

    with TTFont(source, recalcTimestamp=False) as font:
        options = subset.Options()
        options.name_IDs = ["*"]
        options.name_legacy = True
        options.name_languages = ["*"]
        subsetter = subset.Subsetter(options=options)
        subsetter.populate(glyphs=[GLYPH])
        subsetter.subset(font)

        # Preserve the viewBox's leading margin after recalculating bounds.
        glyph = font["glyf"][GLYPH]
        glyph.recalcBounds(font["glyf"])
        advance, _ = font["hmtx"][GLYPH]
        font["hmtx"][GLYPH] = advance, glyph.xMin
        for table in font["cmap"].tables:
            if table.isUnicode():
                table.cmap = {0xE563: GLYPH}  # iced_lucide's published codepoint

        for record in font["name"].names:
            if record.nameID in (1, 3, 4, 6, 16):
                name = "Lucide-0.473" if record.nameID == 6 else FAMILY
                record.string = name.encode(record.getEncoding())
        font.save(Path(__file__).with_name("message-circle-dashed.ttf"))


if __name__ == "__main__":
    main()
