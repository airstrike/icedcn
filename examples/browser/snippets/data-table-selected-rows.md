### Show Selected Rows

The footer counts selected records in the current filter. To read the selected keys in the app, supply `.on_value_change(Message::TableChanged)` and inspect `value.selected`.
