//! Exercise the public component with an inventory unrelated to the payments demo.
use data_table::{Order, Value};
use iced::{Element, Point, Size};
use iced_test::core::Settings;
use iced_test::{Simulator, simulator};
use icedcn::Theme;
use icedcn::widget::{data_table, menu, text};

type Inventory = (u8, &'static str);
const STOCK: &[Inventory] = &[(7, "Pear"), (4, "Plum"), (8, "Peach"), (2, "Apple")];

#[derive(Clone, Debug, PartialEq)]
enum Message {
    Change(Value<u8>),
    Inspect(u8),
}

fn inventory(rows: &[Inventory], controlled: Option<Value<u8>>) -> Element<'_, Message, Theme> {
    let mut table = data_table(
        rows,
        |r| r.0,
        [
            data_table::column("Name", |r: &Inventory| text::small(r.1))
                .sort_by(|a, b| a.1.cmp(b.1)),
            data_table::column("SKU", |r: &Inventory| text::small(r.0.to_string())),
        ],
    )
    .selectable(true)
    .filter("Find fruit", |r, q| r.1.starts_with(q))
    .page_size(2)
    .actions(|r| {
        menu::item("Inspect")
            .on_select(Message::Inspect(r.0))
            .into()
    })
    .on_value_change(Message::Change);
    if let Some(value) = controlled {
        table = table.value(value);
    }
    table.into()
}
type Ui<'a> = Simulator<'a, Message, Theme, iced::Renderer>;
fn open() -> Ui<'static> {
    Simulator::with_size(
        Settings::default(),
        Size::new(600., 500.),
        inventory(STOCK, None),
    )
}
fn changed(ui: &mut Ui<'_>) -> Value<u8> {
    match ui.drain().last() {
        Some(Message::Change(value)) => value,
        other => panic!("expected table change, received {other:?}"),
    }
}
#[test]
fn inventory_filter_sort_visibility_selection_and_pages_share_stable_keys() {
    let mut ui = open();
    ui.click("Next").unwrap();
    assert_eq!(changed(&mut ui).page, 1);
    ui.find("Peach").unwrap();
    ui.find("Apple").unwrap();
    let heading = ui.find("Name").unwrap().bounds();
    ui.point_at(Point::new(16., heading.center_y()));
    ui.simulate(simulator::click());
    let selected = changed(&mut ui).selected;
    assert_eq!(selected.into_iter().collect::<Vec<_>>(), vec![2, 8]);
    ui.find("2 of 4 row(s) selected.").unwrap();
    ui.click("Name").unwrap();
    assert_eq!(changed(&mut ui).sort, Some(("Name", Order::Ascending)));
    assert!(ui.find("Apple").unwrap().bounds().y < ui.find("Peach").unwrap().bounds().y);
    ui.click("Name").unwrap();
    assert_eq!(changed(&mut ui).sort, Some(("Name", Order::Descending)));
    assert!(ui.find("Plum").unwrap().bounds().y < ui.find("Pear").unwrap().bounds().y);
    ui.click("Columns").unwrap();
    ui.draw(&Theme::light());
    // The open menu sits directly beneath the trigger, above the header cells.
    let trigger = ui.find("Columns").unwrap().bounds();
    ui.point_at(Point::new(trigger.center_x(), trigger.center_y() + 64.));
    ui.simulate(simulator::click());
    assert!(changed(&mut ui).hidden.contains("SKU"));
    let mut ui = ui.rebuild(inventory(STOCK, None));
    ui.point_at(Point::new(20., 32.));
    ui.simulate(simulator::click());
    ui.typewrite("Ap");
    let value = changed(&mut ui);
    assert_eq!(value.query, "Ap");
    assert_eq!(value.page, 0);
    ui.find("1 of 1 row(s) selected.").unwrap();
    ui.typewrite("z");
    ui.find("No results.").unwrap();
}

#[test]
fn controlled_state_and_row_changes_preserve_identity_and_clamp_pages() {
    let controlled = Value {
        page: 1,
        selected: [2].into_iter().collect(),
        ..Value::default()
    };
    let mut ui = Simulator::with_size(
        Settings::default(),
        Size::new(600., 500.),
        inventory(STOCK, Some(controlled.clone())),
    );
    ui.find("Apple").unwrap();
    ui.click("Previous").unwrap();
    assert_eq!(changed(&mut ui).page, 0);
    ui.find("Apple").unwrap();
    let mut ui = ui.rebuild(inventory(&[(2, "Apple")], Some(controlled)));
    ui.find("Apple").unwrap();
    ui.find("1 of 1 row(s) selected.").unwrap();
}

#[test]
fn payments_actions_publish_the_filtered_records_id() {
    use browser::component::data_table::DataTablePage;
    let page = DataTablePage::new(0);
    let mut ui = Simulator::with_size(
        Settings::default(),
        Size::new(574., 500.),
        page.preview("demo"),
    );
    ui.point_at(Point::new(20., 32.));
    ui.simulate(simulator::click());
    ui.typewrite("ken99");
    let row = ui.find("ken99@example.com").unwrap().bounds();
    ui.find("0 of 1 row(s) selected.").unwrap();
    // The row action follows the right-aligned currency cell.
    let amount = ui.find("$316.00").unwrap().bounds();
    ui.point_at(Point::new(amount.x + amount.width + 28., row.center_y()));
    ui.simulate(simulator::click());
    ui.draw(&Theme::light());
    ui.click("Copy payment ID").unwrap();
    assert!(
        ui.drain()
            .any(|msg| matches!(msg, browser::Message::CopyText(id) if id == "m5gr84i9"))
    );
}

#[test]
fn narrow_preview_scrolls_to_the_amounts_without_overlapping_the_email() {
    use browser::component::data_table::DataTablePage;
    use iced::mouse::ScrollDelta;

    let page = DataTablePage::new(0);
    let mut ui = Simulator::with_size(
        Settings::default(),
        Size::new(308., 500.),
        page.preview("demo"),
    );
    let left = ui.find("ken99@example.com").unwrap().bounds();
    let total = ui.find("$316.00").unwrap().bounds();
    assert!(left.x + left.width < total.x);
    assert!(total.x + total.width > 308.);
    ui.point_at(left.center());
    ui.scroll(ScrollDelta::Pixels { x: -400., y: 0. });
    ui.draw(&Theme::light());
    let total = ui.find("$316.00").unwrap().visible_bounds().unwrap();
    assert!(total.width > 40. && total.x + total.width <= 308.);
}
