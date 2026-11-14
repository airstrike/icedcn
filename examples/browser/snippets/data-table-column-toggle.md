### Column Toggle

The Columns menu derives its entries from each table’s columns. Start with `let mut value = data_table::Value::default();`, insert column names into `value.hidden` and pass the configured value to `.default_value(value)` to choose the starting view.
