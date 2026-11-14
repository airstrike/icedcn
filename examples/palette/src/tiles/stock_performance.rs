//! Stock performance card.

use icedxdy::Theme;

use super::api_gap;
use crate::{Message, State};

pub fn view(_state: &State) -> iced::Element<'static, Message, Theme> {
    api_gap::missing(
        "Stock Performance",
        "stock_performance.rs",
        &["Combobox", "ChartContainer", "AreaChart"],
    )
}
