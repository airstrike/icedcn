use crate::icon;
use icedcn::Slot;
use icedcn::widget::{alert, text};

pub(super) fn view<M: 'static>(wrapping: text::Wrapping) -> Slot<'static, M> {
    alert()
        .icon(icon::circle_alert().size(16).line_height(1.0))
        .title("Payment failed")
        .description(
            "Your payment could not be processed. Please check your payment method and try again.",
        )
        .description_wrapping(wrapping)
        .style(alert::Destructive)
        .max_width(448)
        .into()
}
