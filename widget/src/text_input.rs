//! Text input — sweetened iced text_input vendored here so we can render
//! focus rings and emit `on_focus` / `on_blur` events.

mod caret;
pub mod cursor;
mod editor;
pub(crate) mod element;
mod history;
mod value;
pub mod widget;

pub use caret::Caret;
pub use cursor::Cursor;
pub use value::Value;
pub use widget::{Catalog, Ring, Status, Style, StyleFn, TextInput};
