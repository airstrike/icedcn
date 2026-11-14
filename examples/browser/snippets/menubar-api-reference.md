# API Reference

`menubar(entries)` accepts menus created with `menubar::menu(label, content)`. Use `.width(..)` on a menu to size its panel or on the bar to size its frame. Control the expanded menu with `.value(Option<usize>)` and `.on_value_change(..)`.

[Read the menubar implementation](https://github.com/airstrike/icedxdy/blob/HEAD/widget/src/menubar.rs).
