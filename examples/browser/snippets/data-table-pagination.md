### Pagination State

Give another table its own `.page_size(...)`. To drive navigation externally, pass `.value(value.clone()).on_value_change(Message::TableChanged)` and store the complete new value in update. The zero-based `value.page` indexes the filtered and sorted records.
