//! The kill buffer of the macOS text system: Control+K cuts into it and
//! Control+Y inserts it, apart from the clipboard. Kills in a row add up.
use std::cell::RefCell;

#[derive(Debug, Default)]
struct Buffer {
    text: String,
    /// The last key press killed, so the next kill appends.
    open: bool,
}

thread_local! {
    static BUFFER: RefCell<Buffer> = RefCell::default();
}

/// Keep `text`, after the text of the kill just before, if any.
pub(crate) fn push(text: &str) {
    BUFFER.with_borrow_mut(|buffer| {
        if !buffer.open {
            buffer.text.clear();
        }
        buffer.text.push_str(text);
        buffer.open = true;
    });
}

/// The killed text, if any.
pub(crate) fn yank() -> Option<String> {
    BUFFER.with_borrow(|buffer| (!buffer.text.is_empty()).then(|| buffer.text.clone()))
}

/// End a run of kills: the next one starts the buffer again.
pub(crate) fn close() {
    BUFFER.with_borrow_mut(|buffer| buffer.open = false);
}
