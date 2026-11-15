use iced::theme::Mode;
use iced::{Task, font};
use icedcn::core::design::{Accent, Base};
use icedcn::{Design, Theme};

pub fn theme(_mode: Mode) -> Theme {
    Design::from(Mode::Light)
        .with_name("Slate Violet")
        .with_base(Base::Slate)
        .with_accent(Accent::Violet)
        .with_chart(Accent::Green)
        .with_font_sans("A \"Quoted\" Font")
        .with_menu_accent()
        .into()
}

// Pass the system preference or your app's chosen mode to theme(mode).
// Load the selected font, then return or batch font_setup() from boot/update.
pub fn font_setup<Message>() -> Task<Message> {
    font::set_defaults("A \"Quoted\" Font".into(), 14.0)
}
