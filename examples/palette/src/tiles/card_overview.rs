//! Card overview card.

use icedcn::Theme;

use super::api_gap;
use crate::{Message, State};

pub fn view(_state: &State) -> iced::Element<'static, Message, Theme> {
    api_gap::missing(
        "Card Overview",
        "card_overview.rs",
        &["ChartContainer", "BarChart", "XAxis", "ChartTooltip"],
    )
}
