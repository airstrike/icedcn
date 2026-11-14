### Build a Row Menu

`.actions(|payment| ...)` receives the actual record, even after sorting or filtering. Return menu items from the closure. The demo publishes the ID with `menu::item("Copy payment ID").on_select(copy_id(payment.id.to_owned()))`. Handle that message by returning `iced::clipboard::write(id)` from the application update. Add your own messages to the customer and payment detail items to navigate.
