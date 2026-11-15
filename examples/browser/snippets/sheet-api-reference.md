## API Reference

Sheet uses the same trigger, close action, focus handling and controlled state as [Dialog](icedcn://component/dialog). Use `.default_open(true)` for an initially open sheet or pair `.open(value)` with `.on_open_change(..)`.

Left and right sheets fill the viewport height and use 75% of its width. Their width is capped at 384 pixels on viewports of at least 640 pixels. Top and bottom sheets fit their content. Set `.max_height_ratio(0.5)` to limit them to half the viewport.
