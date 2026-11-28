//! Composed chart annotations, shared by native hover and static tooltip anatomy.

use std::marker::PhantomData;

use hyozu::TooltipEntry;
use iced_core::Length::{Fill, Shrink};
use iced_core::alignment::Vertical;
use iced_core::{Color, Element, Font, Length, Pixels};
use iced_widget::Renderer;
use iced_widget::Widget;
use icedcn_core::theme::chart;
use icedcn_core::theme::tweak::Bg;
use icedcn_core::{Theme, radius, theme};

use crate::compose::{Compose, View};
use crate::{column, container, row, space, text};

pub use Indicator::{Dashed, Dot, Line};

/// The series swatch beside a tooltip row.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Indicator {
    #[default]
    Dot,
    Line,
    Dashed,
}

/// One presentation row. Native hover entries and `(name, value, color)` tuples convert here.
pub struct Entry {
    name: String,
    value: f64,
    color: Color,
}

impl From<&TooltipEntry> for Entry {
    fn from(entry: &TooltipEntry) -> Self {
        Self {
            name: entry.series_name.clone().unwrap_or_default(),
            value: entry.y,
            color: entry.color,
        }
    }
}

impl From<TooltipEntry> for Entry {
    fn from(entry: TooltipEntry) -> Self {
        Self {
            name: entry.series_name.unwrap_or_default(),
            value: entry.y,
            color: entry.color,
        }
    }
}

impl<Name: Into<String>, Value: Into<f64>, Fill: Into<Color>> From<(Name, Value, Fill)> for Entry {
    fn from((name, value, color): (Name, Value, Fill)) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            color: color.into(),
        }
    }
}

/// Compact tooltip content. Hyozu owns inspection and annotation positioning.
pub struct Tooltip<Message> {
    message: PhantomData<Message>,
    entries: Vec<Entry>,
    label: Option<String>,
    indicator: Indicator,
    hide_label: bool,
    hide_indicator: bool,
    width: Length,
    font: Font,
}

/// Compose native resolved entries, or standalone `(name, value, color)` rows.
pub fn tooltip<Message>(entries: impl IntoIterator<Item = impl Into<Entry>>) -> Tooltip<Message> {
    Tooltip {
        message: PhantomData,
        entries: entries.into_iter().map(Into::into).collect(),
        label: None,
        indicator: Dot,
        hide_label: false,
        hide_indicator: false,
        width: Shrink,
        font: Font::MONOSPACE,
    }
}

impl<Message> Tooltip<Message> {
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn indicator(mut self, indicator: impl Into<Indicator>) -> Self {
        self.indicator = indicator.into();
        self
    }

    pub fn hide_label(mut self, hide: bool) -> Self {
        self.hide_label = hide;
        self
    }

    pub fn hide_indicator(mut self, hide: bool) -> Self {
        self.hide_indicator = hide;
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Monospace font for values. Defaults to the renderer's generic monospace family.
    pub fn font(mut self, font: impl Into<Font>) -> Self {
        self.font = font.into();
        self
    }
}

impl<Message> Compose<Message> for Tooltip<Message> {
    fn compose<'a>(self: Box<Self>) -> Element<'a, Message, Theme, Renderer>
    where
        Self: 'a,
    {
        if self.entries.is_empty() {
            return space::horizontal().boxed();
        }
        let nested = self.entries.len() == 1 && self.indicator != Dot;
        let label = self.label.filter(|_| !self.hide_label);
        let heading = |label: String| text(label).size(12).medium().line_height(Pixels(16.0));
        // The content owns the surface's inner width. Preserve its minimum
        // during intrinsic measurement as well: 128px minus horizontal padding.
        let mut content = column([]).spacing(6).width(Fill.min(128.0 - 20.0));
        if !nested && let Some(label) = &label {
            content = content.push(heading(label.clone()).width(Fill).into());
        }
        for entry in self.entries {
            let mut names = column([]).spacing(6);
            if nested && let Some(label) = &label {
                names = names.push(heading(label.clone()).into());
            }
            names = names.push(
                text::muted(entry.name)
                    .size(12)
                    .line_height(Pixels(12.0))
                    .into(),
            );
            let swatch: Option<View<'a, Message>> = (!self.hide_indicator).then(|| {
                let fill = theme::container::Transparent
                    + Bg::from(icedcn_core::Color::from(entry.color))
                    + radius(2);
                match self.indicator {
                    Dot => container(space::horizontal())
                        .width(10)
                        .height(10)
                        .style(fill)
                        .into(),
                    Line => container(space::horizontal())
                        .width(4)
                        .height(Fill)
                        .style(fill)
                        .into(),
                    Dashed => column((0..if nested { 5 } else { 2 }).map(|_| {
                        container(space::horizontal())
                            .width(3)
                            .height(3)
                            .style(fill.clone())
                            .into()
                    }))
                    .spacing(3)
                    .into(),
                }
            });
            let values = row![
                names,
                space::horizontal(),
                text(number(entry.value))
                    .size(12)
                    .medium()
                    .font(self.font)
                    .line_height(Pixels(12.0)),
            ]
            .align_y(if nested {
                Vertical::Bottom
            } else {
                Vertical::Center
            })
            .width(Fill);
            let mut item = row![];
            if let Some(swatch) = swatch {
                item = item.push(swatch);
            }
            content = content.push(
                item.push(values.into())
                    .spacing(8)
                    .align_y(Vertical::Center)
                    .width(Fill)
                    .into(),
            );
        }
        let surface = container(content)
            .width(self.width)
            .min_width(128)
            .padding([6, 10])
            .style(chart::tooltip);
        if self.width == Shrink {
            surface.fit_content().into()
        } else {
            surface.into()
        }
    }
}

impl<'a, Message: 'a> From<Tooltip<Message>> for View<'a, Message> {
    fn from(tooltip: Tooltip<Message>) -> Self {
        Self::new(tooltip)
    }
}

impl<'a, Message: 'a> From<Tooltip<Message>> for Element<'a, Message, Theme, Renderer> {
    fn from(tooltip: Tooltip<Message>) -> Self {
        Box::new(tooltip).compose()
    }
}

fn number(value: f64) -> String {
    let value = value.to_string();
    let (whole, fraction) = value.split_once('.').unwrap_or((&value, ""));
    let (sign, whole) = whole
        .strip_prefix('-')
        .map_or(("", whole), |whole| ("-", whole));
    let mut formatted = String::with_capacity(value.len() + whole.len() / 3);
    formatted.push_str(sign);
    for (index, digit) in whole.chars().enumerate() {
        if index > 0 && (whole.len() - index).is_multiple_of(3) && digit.is_ascii_digit() {
            formatted.push(',');
        }
        formatted.push(digit);
    }
    if !fraction.is_empty() {
        formatted.push('.');
        formatted.push_str(fraction);
    }
    formatted
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced_core::widget::{Id, Operation};
    use iced_core::{Rectangle, color};
    use iced_test::simulator::simulator;

    #[test]
    fn values_group_digits_without_counting_the_sign() {
        for (value, expected) in [
            (186.0, "186"),
            (12486.0, "12,486"),
            (-100.0, "-100"),
            (-1234.5, "-1,234.5"),
            (1234567.0, "1,234,567"),
        ] {
            assert_eq!(number(value), expected);
        }
    }

    #[test]
    fn line_indicator_stretches_only_to_its_label() {
        #[derive(Default)]
        struct Bounds(Vec<Rectangle>);
        impl Operation for Bounds {
            fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
                visit(self);
            }
            fn container(&mut self, _: Option<&Id>, bounds: Rectangle, _: &Rectangle) {
                self.0.push(bounds);
            }
        }
        let content: Element<'_, (), Theme, Renderer> =
            tooltip([("Desktop", 12486, color!(0x155dfb))])
                .label("Page Views")
                .indicator(Line)
                .width(144)
                .into();
        let mut ui = simulator(content);
        let mut bounds = Bounds::default();
        ui.operate(&mut bounds);
        let outer = bounds.0.first().expect("tooltip surface");
        assert_eq!(outer.width, 144.0);
        // The null renderer gives text zero height; compare allocation instead of font pixels.
        assert!(outer.height <= 60.0, "{outer:?}");
        let indicator = bounds
            .0
            .iter()
            .find(|bounds| bounds.width == 4.0)
            .expect("line swatch");
        assert_eq!(indicator.height, outer.height - 12.0, "{indicator:?}");
    }
}
