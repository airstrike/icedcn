//! Hyozu charts, using icedcn's theme and composition system.
//!
//! Build [`Data`] in your application's model, then borrow it with [`chart`].
//! The data, marks, interaction, layout and rendering are Hyozu's native API.
//! Colors resolve from the active [`Theme`], including its `chart` tokens;
//! leave marks uncolored or use Hyozu's semantic colors to follow theme changes.
//! Explicit mark colors and palettes remain overrides. For custom typography,
//! pass `.design(&theme)` (or an owned `Cow`) so layout can read its fonts too.

pub use hyozu;
pub use hyozu::chart::{Chart, chart, donut, hover, transparent};
pub use hyozu::{Data, axis, bar, bars, data, legend};

pub mod tooltip;
pub use tooltip::{Tooltip, tooltip};

use iced_core::{Element, Widget};
use iced_widget::Renderer;
use icedcn_core::Theme;

use crate::compose::{Compose, View};

impl<'a, Message: 'a, Design> Compose<Message> for Chart<'a, Message, Design, Theme>
where
    Design: hyozu::Design + Clone + 'a,
{
    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        (*self)._boxed()
    }
}

impl<'a, Message: 'a, Design> From<Chart<'a, Message, Design, Theme>> for View<'a, Message>
where
    Design: hyozu::Design + Clone + 'a,
{
    fn from(chart: Chart<'a, Message, Design, Theme>) -> Self {
        Self::new(chart)
    }
}
