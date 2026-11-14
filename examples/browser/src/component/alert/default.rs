use iced::Widget;
use iced::widget::svg;
use icedxdy::widget::alert;
use icedxdy::{Slot, Theme};

use crate::Message;

pub fn view<'a>() -> Slot<'a, Message> {
    // shadcn `alert-basic.tsx` caps the alert at `max-w-md` (448px).
    alert()
        .icon(
svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="m9 12 2 2 4-4"/></svg>"#.as_bytes()))
                .width(16)
                .height(16)
                .style(|theme: &Theme, _| svg::Style { color: Some(theme.card.foreground.into()) })
                .boxed(),
)
        .title("Account updated successfully")
        .description(
            "Your profile information has been saved. Changes will be reflected immediately.",
        )
        .max_width(448)
        .into()
}
