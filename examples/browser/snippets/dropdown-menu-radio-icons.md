```rust
use crate::icon;
use iced::widget::Text;
use icedcn::widget::{button, dropdown_menu, menu, menu_group};
use icedcn::{Slot, Theme};

pub fn view(selected: &'static str) -> Slot<'static, &'static str> {
    let caption = |glyph: Text<'static, Theme>, label: &'static str| {
        menu::caption(glyph.size(16).line_height(1.0), label)
    };
    dropdown_menu(
        |m| m.trigger(button("Payment Method").style(button::Outline)),
        move |_| menu_group![
            menu::label("Select Payment Method"),
            menu::radio_item(caption(icon::credit_card(), "Credit Card"), "card", Some(selected), |v| v),
            menu::radio_item(caption(icon::wallet(), "PayPal"), "paypal", Some(selected), |v| v),
            menu::radio_item(caption(icon::building_2(), "Bank Transfer"), "bank", Some(selected), |v| v),
        ],
    ).width(224).into()
}
```

Initialize the application value to `"card"`. Keep the value this menu publishes when an item is selected.
