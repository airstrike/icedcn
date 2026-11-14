//! A trace presents the steps in an agent's work. Lines can be static, links,
//! disclosure triggers, or anchors for a sheet or dialog.
use crate::button::widget::{Button, StyleFn};
use crate::collapsible::{self, Collapsible};
use crate::compose::{Compose, Inherited, Slot};
use crate::{column, container, row, status_shimmer, text};
use iced_core::{Alignment, Element, Length::Fill};
use iced_widget::{Renderer, Space, Widget};
use icedcn_core::Theme;
use icedcn_core::theme::button::ghost;
use icedcn_core::theme::container::Style;
use icedcn_core::theme::text::{Destructive, Muted};
use std::borrow::Cow;

/// Stack trace lines and nested disclosures at Nova's 8px gap.
pub fn trace<'a, M: 'a>(
    children: impl IntoIterator<Item = Slot<'a, M>>,
) -> crate::column::Column<'a, M> {
    column::Column::with_children(children)
        .width(Fill)
        .spacing(8)
}
/// Presentation of a single trace entry.
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub enum Status {
    #[default]
    Complete,
    Running,
    Error,
    Pending,
}
/// A trace line with optional leading/trailing icons and secondary text.
pub struct Line<'a, M> {
    title: Cow<'a, str>,
    icon: Option<Slot<'a, M>>,
    trailing: Option<Slot<'a, M>>,
    description: Option<Cow<'a, str>>,
    status: Status,
}
/// Create a plain non-interactive line.
pub fn line<'a, M>(title: impl Into<Cow<'a, str>>) -> Line<'a, M> {
    Line {
        title: title.into(),
        icon: None,
        trailing: None,
        description: None,
        status: Status::Complete,
    }
}
impl<'a, M: 'a> Line<'a, M> {
    /// Leading indicator.
    pub fn icon(mut self, icon: impl Into<Slot<'a, M>>) -> Self {
        self.icon = Some(icon.into());
        self
    }
    /// Trailing link or navigation glyph.
    pub fn trailing(mut self, icon: impl Into<Slot<'a, M>>) -> Self {
        self.trailing = Some(icon.into());
        self
    }
    /// Secondary text such as duration or change count.
    pub fn description(mut self, detail: impl Into<Cow<'a, str>>) -> Self {
        self.description = Some(detail.into());
        self
    }
    /// Set the semantic status. Running titles shimmer.
    pub fn status(mut self, status: Status) -> Self {
        self.status = status;
        self
    }
}
impl<'a, M: 'a> Compose<M> for Line<'a, M> {
    fn inherit(&mut self, props: &Inherited) {
        if let Some(icon) = &mut self.icon {
            icon.inherit(props);
        }
        if let Some(trailing) = &mut self.trailing {
            trailing.inherit(props);
        }
    }
    fn compose<'b>(self: Box<Self>) -> Element<'b, M, Theme, Renderer>
    where
        Self: 'b,
    {
        let status = self.status;
        let mut row = row::Row::new().spacing(8).align_y(Alignment::Center);
        if let Some(icon) = self.icon {
            row = row.push(container(icon).center(16).into());
        }
        let title = match status {
            Status::Running => status_shimmer::shimmer(self.title.into_owned()),
            Status::Error => text(self.title).size(14).style(Destructive).into(),
            Status::Complete => text(self.title).size(14).style(Muted).into(),
            Status::Pending => {
                let color: iced_widget::text::StyleFn<'a, Theme> =
                    Box::new(|t| iced_widget::text::Style {
                        color: Some(t.muted.foreground.alpha(0.6).into()),
                        ..Default::default()
                    });
                text(self.title).size(14).style(color).into()
            }
        };
        row = row.push(title);
        if let Some(desc) = self.description {
            row = row.push(text::muted(desc).size(12).into());
        }
        if let Some(trailing) = self.trailing {
            row = row.push(container(trailing).center(16).into());
        }
        Slot::from(container(row).style(move |theme: &Theme| Style {
            text_color: Some(match status {
                Status::Error => theme.destructive.base,
                Status::Pending => theme.muted.foreground.alpha(0.6),
                _ => theme.muted.foreground,
            }),
            ..Style::default()
        }))
        .compose()
    }
}
impl<'a, M: 'a> From<Line<'a, M>> for Slot<'a, M> {
    fn from(line: Line<'a, M>) -> Self {
        Slot::new(line)
    }
}
/// Give a trace line the hit target and keyboard behavior of a button.
pub fn action<'a, M: Clone + 'a>(
    content: impl Into<Slot<'a, M>>,
    message: Option<M>,
) -> Slot<'a, M> {
    let style: StyleFn<'a, Theme> = Box::new(|theme: &Theme, status| {
        let style: iced_widget::button::Style = ghost(theme, status.into()).into();
        style.into()
    });
    Button::new(content.into().compose())
        .padding(0)
        .translate(false)
        .style(style)
        .on_press_maybe(message)
        .boxed()
        .into()
}
/// A disclosure with the same controlled/uncontrolled open API as Collapsible.
pub fn disclosure<'a, M: Clone + 'static>(
    summary: impl Fn() -> Line<'a, M> + 'a,
    details: impl Fn() -> Slot<'a, M> + 'a,
) -> Collapsible<'a, M> {
    collapsible::collapsible(move |c| {
        let arrow = if c.is_open() { "⌄" } else { "›" };
        let summary = c.trigger(action(
            row![summary(), text(arrow).size(14)]
                .align_y(Alignment::Center)
                .spacing(4),
            None,
        ));
        let content = c.content(
            container(row![
                container(Space::new())
                    .width(1)
                    .height(Fill)
                    .style(|t: &Theme| Style {
                        background: Some(t.neutral.border.into()),
                        ..Style::default()
                    }),
                container(details()).padding([0, 0, 0, 16])
            ])
            .padding([8, 4, 4, 7]),
        );
        column![summary, content].into()
    })
}
