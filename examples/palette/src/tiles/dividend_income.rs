//! Dividend income card.

use icedxdy::Theme;

use super::api_gap;
use crate::{Message, State};

pub fn view(_state: &State) -> iced::Element<'static, Message, Theme> {
    api_gap::missing(
        "Q2 Dividend Income",
        "dividend_income.rs",
        &["ChartContainer", "BarChart"],
    )
}
