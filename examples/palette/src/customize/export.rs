use iced::{Font, Task, font};
use icedcn::core::design::{Accent, Base};
use icedcn::{Design, Theme};

pub fn theme() -> Theme {
    Design::dark()
        .with_name("Slate Violet Dark")
        .with_base(Base::Slate)
        .with_accent(Accent::Violet)
        .with_chart(Accent::Green)
        .with_font_sans(Font::from("A \"Quoted\" Font"))
        .with_menu_accent()
        .into()
}

// Load the selected font, then return or batch font_setup() from boot/update.
pub fn font_setup<Message>() -> Task<Message> {
    font::set_defaults(theme().font_sans.regular, 14.0)
}
