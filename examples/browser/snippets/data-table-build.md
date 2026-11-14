### Build the Table

`data_table(records, |p| p.id.to_owned(), columns)` returns a table that owns its interaction state. It measures each visible column across the header and body, and scrolls horizontally when the records need more width.
