### Column Definitions

Each `data_table::column(label, render)` pairs a unique display name with a function that turns a record into cell content. For example, `data_table::column("Status", |p: &Payment| text::small(p.status))` displays the status field.
