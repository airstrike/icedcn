### Compare Records

Add `.sort_by(|a, b| a.amount.total_cmp(&b.amount))` to Amount for numerical sorting. A comparator receives the original records, so display formatting never affects the ordering.
