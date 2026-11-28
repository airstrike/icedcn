use super::{Event, Table, Value};
use crate::compose::View;
use crate::icon::Icon;
use crate::scrollable::{Direction, Visibility, scrollbar};
use crate::table::Track;
use crate::{
    Size, button, checkbox, column, container, dropdown_menu, input, menu, row, scrollable, table,
    text,
};
use iced_core::Length::Fill;
use iced_core::alignment::Horizontal;
use iced_widget::Widget as _;
use icedcn_core::theme::container::Style;
use icedcn_core::{Border, Theme};

fn border(t: &Theme) -> Style {
    Style {
        border: Border {
            color: t.neutral.border,
            width: 1.,
            radius: 8.into(),
        },
        ..Style::default()
    }
}
impl<'a, T: 'a, K: Ord + Clone + 'static, M: Clone + 'static> Table<'a, T, K, M> {
    pub(super) fn body(
        &self,
        value: &Value<K>,
        filtered: &[usize],
        filtered_selected: usize,
        mobile: bool,
    ) -> View<'a, Event<K, M>> {
        let toggles: Vec<_> = self
            .columns
            .iter()
            .map(|c| (c.label, !value.hidden.contains(c.label)))
            .collect();
        let columns = dropdown_menu(
            |m| {
                m.trigger(
                    button!["Columns", Icon::Chevron.boxed()]
                        .style(button::Outline)
                        .has_popup(true),
                )
            },
            move |_| {
                column(toggles.iter().map(|&(label, shown)| {
                    menu::checkbox_item(label, shown, move |show| Event::Visible(label, show))
                        .into()
                }))
                .width(Fill)
                .into()
            },
        )
        .align(dropdown_menu::Align::End);
        let mut toolbar = row![];
        if let Some((placeholder, _)) = &self.filter {
            let field = container(
                input(*placeholder)
                    .text_size(if mobile { 16. } else { 14. })
                    .value(value.query.clone())
                    .on_input(Event::Filter)
                    .width(Fill),
            )
            .width(Fill)
            .max_width(384);
            toolbar = toolbar.push(container(field).width(Fill).into());
        } else {
            toolbar = toolbar.push(crate::space::horizontal().into());
        }
        toolbar = toolbar.push(columns.into()).width(Fill);

        let cols: Vec<_> = self
            .columns
            .iter()
            .filter(|c| !value.hidden.contains(c.label))
            .collect();
        let page = self.page(value, filtered);
        let checked = |i: usize| value.selected.contains(&(self.key)(&self.records[i]));
        let mut heads = Vec::new();
        if self.selectable {
            let count = page.iter().filter(|&&i| checked(i)).count();
            let selection = if count == 0 {
                checkbox::State::Unchecked
            } else if count == page.len() {
                checkbox::State::Checked
            } else {
                checkbox::State::Indeterminate
            };
            heads.push(
                table::head(
                    checkbox()
                        .checked(selection)
                        .on_checked_change(Event::PageSelection),
                )
                .padding([0., 0., 0., 8.])
                .height(39.),
            );
        }
        for col in &cols {
            let head = if col.compare.is_some() {
                table::head(
                    button![col.label, Icon::Sort.boxed()]
                        .style(button::Ghost)
                        .on_press(Event::Sort(col.label)),
                )
            } else {
                table::head(col.label)
            };
            heads.push(head.align_x(col.align).padding([0., 8.]).height(39.));
        }
        if self.actions.is_some() {
            heads.push(table::head("").height(39.));
        }
        let tracks = heads.len().max(1);
        let mut rows = Vec::new();
        for &i in page {
            let record = &self.records[i];
            let mut cells = Vec::new();
            if self.selectable {
                let key = (self.key)(record);
                cells.push(
                    table::cell(
                        checkbox()
                            .checked(checked(i))
                            .on_checked_change(move |s| Event::Select(key.clone(), s)),
                    )
                    .padding([8., 0., 8., 8.]),
                );
            }
            for col in &cols {
                cells.push(table::cell((col.cell)(record).map(Event::App)).align_x(col.align));
            }
            if let Some(render) = &self.actions {
                let render = render.clone();
                let actions = dropdown_menu(
                    |m| {
                        m.trigger(
                            button(Icon::More.boxed())
                                .square()
                                .size(Size::Xs)
                                .style(button::Ghost)
                                .has_popup(true),
                        )
                    },
                    move |_| render(record).map(Event::App),
                )
                .align(dropdown_menu::Align::End)
                .width(160);
                cells.push(table::cell(actions));
            }
            rows.push(table::row(cells).selected(self.selectable && checked(i)));
        }
        if rows.is_empty() {
            rows.push(table::row([table::cell("No results.")
                .span(tracks)
                .height(96)
                .align_x(Horizontal::Center)]));
        }
        let grid = table::grid(std::iter::repeat_n(Track::Auto, tracks))
            .header(table::header([table::row(heads)]))
            .body(table::body(rows));
        let overflow = Direction::Horizontal(scrollbar());
        let body = container(
            scrollable(grid)
                .direction(overflow)
                .visibility(Visibility::Scroll)
                .width(Fill),
        )
        .padding(1)
        .width(Fill)
        .clip(true)
        .style(border);
        let mut footer = row![];
        if self.selectable {
            footer = footer.push(
                text::muted(format!(
                    "{filtered_selected} of {} row(s) selected.",
                    filtered.len()
                ))
                .size(14.)
                .width(Fill)
                .into(),
            );
        } else {
            footer = footer.push(crate::space::horizontal().into());
        }
        for (label, target, disabled) in [
            ("Previous", value.page.saturating_sub(1), value.page == 0),
            (
                "Next",
                value.page.saturating_add(1),
                value.page >= filtered.len().saturating_sub(1) / self.page_size,
            ),
        ] {
            footer = footer.push(
                button(label)
                    .size(Size::Sm)
                    .style(button::Outline)
                    .disabled(disabled)
                    .on_press(Event::Page(target))
                    .into(),
            );
        }
        column![toolbar, body, footer.width(Fill).spacing(8)]
            .padding([16., 0.])
            .spacing(16)
            .width(Fill)
            .into()
    }
}
