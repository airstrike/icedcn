### Format the Amount and Email

Return any icedcn cell content from the renderer. The demo uses `text::small(format!("${:.2}", p.amount)).medium()` for two decimal places and `.align_x(Right)` on the Amount column. Lowercasing the email changes its display without modifying the record.
