use iced::theme::Mode;
use icedcn::{Design, Theme};

/// Customize this application's design for the current system preference.
pub fn create(mode: Mode) -> Theme {
    Design::from(mode).with_name("__NAME__").into()
}
