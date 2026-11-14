Use negative margins with `-mx-(--card-spacing)` to make content go edge to edge while keeping it aligned with the card inset. When the edge-to-edge content sits above a footer, use `-mb-(--card-spacing)` on `CardContent` to remove the section gap.

The Rust composition places the scrollable body directly between the padded header and footer. The body owns its text inset and its 192px scroll viewport.
