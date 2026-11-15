# Application examples

Run `cargo run -p browser` from the workspace and choose **Examples** in the header. The same applications run in the browser's WebAssembly build.

## Receipts

A remake of [iced_receipts](https://github.com/airstrike/iced_receipts) using icedcn cards, fields, inputs, selects, buttons, badges, and tables. Open **Receipts** to try two sample sales or create your own.

- Search the sales list and open a receipt.
- Create a sale with named items, quantities, prices, and individual tax groups.
- Add a service charge and gratuity; the summary updates while editing.
- Save to return to the receipt, or cancel to preserve the original sale.
- Tab and Shift+Tab move between controls. Escape cancels an edit or returns from receipt details to the list.

Data stays in memory for the current session. The fixed demo tax rates match the original example: food/other 8%, alcohol 10%, and non-taxable 0%.

`receipts.rs` owns the model, messages, and screen transitions. `receipts/receipt.rs` owns receipt data, decimal validation, and totals in integer cents. `receipts/view.rs` borrows that state to render the list, details, and editor. Edits use a separate draft: saving commits it; cancelling discards it. No global state or platform-specific storage is required.

Run the workflow and calculation tests with `cargo test -p browser examples::receipts`. Header navigation is covered by `cargo test -p browser app::header::tests`.
