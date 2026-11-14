use iced::Widget;
use iced::widget::svg;
use icedcn::widget::{alert, text};
use icedcn::{Slot, Theme};

use crate::Message;

pub fn view<'a>(wrapping: text::Wrapping) -> Slot<'a, Message> {
    const ICON: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="M12 8v4M12 16h.01"/></svg>"#;
    alert()
        .description_wrapping(wrapping)
        .icon(
            svg(svg::Handle::from_memory(ICON.as_bytes()))
                .width(16)
                .height(16)
                .style(|theme: &Theme, _| svg::Style {
                    color: Some(theme.destructive.base.into()),
                })
                .boxed(),
        )
        .title("Payment failed")
        .description(
            "Your payment could not be processed. Please check your payment method and try again.",
        )
        .style(alert::Destructive)
        .max_width(448)
        .into()
}
