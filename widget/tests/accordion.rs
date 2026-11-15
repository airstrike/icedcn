//! Public positional/keyed selection and composition contracts.
use iced_core::Element;
use iced_core::widget::Id;
use iced_test::simulator::simulator;
use iced_widget::{Renderer, Space};
use icedcn_core::Theme;
use icedcn_widget::accordion::Item;
use icedcn_widget::{Slot, accordion, button, container, input};

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Single(Option<usize>),
    Multiple(Vec<usize>),
    Keyed(Option<Record>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Record {
    Shipping,
    Returns,
}

fn item(id: &'static str) -> Item<'static, Message> {
    accordion::item(
        container(Space::new().width(100).height(20)).id(Id::new(id)),
        container(input(id)).id(Id::from(format!("body-{id}"))),
    )
}

fn mount(
    view: impl Into<Element<'static, Message, Theme, Renderer>>,
) -> iced_test::Simulator<'static, Message, Theme, Renderer> {
    simulator(view.into())
}

#[test]
fn literal_function_push_extend_and_empty_forms_compose() {
    let _: Slot<'_, Message> = icedcn_widget::accordion![].into();
    let _: Slot<'_, Message> = icedcn_widget::accordion![
        ("Tuple", "Body"),
        accordion::item(button("Custom"), input("Editable")).disabled(true),
    ]
    .default_value(0)
    .into();
    let _: Slot<'_, Message> = accordion([(button("Trigger"), "Body")])
        .push(("Second", input("Details")))
        .extend([("Third", "Body")])
        .into();
    let _: Slot<'_, Message> = accordion::keyed([(1usize, ("First", "Body"))])
        .push(9, ("Second", "Body"))
        .extend([(5, ("Third", "Body"))])
        .default_value(9)
        .into();
}

#[test]
fn uncontrolled_positions_keep_selection_and_publish_optional_indices() {
    let view = || {
        accordion([item("a"), item("b")])
            .default_value(0)
            .on_value_change(Message::Single)
    };
    let mut ui = mount(view());
    assert!(ui.find(Id::new("body-a")).is_ok());
    assert!(ui.find(Id::new("body-b")).is_err());
    ui.click(Id::new("b")).expect("second header");
    ui = ui.rebuild(Element::from(view()));
    assert!(ui.find(Id::new("body-a")).is_err());
    assert!(ui.find(Id::new("body-b")).is_ok());
    ui.click(Id::new("b")).expect("open header");
    assert!(ui.find(Id::new("body-b")).is_err());
    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [Message::Single(Some(1)), Message::Single(None),]
    );
}

#[test]
fn controlled_single_and_noncollapsible_values_keep_the_application_selection() {
    let mut ui = mount(
        accordion([item("a"), item("b")])
            .value(Some(0))
            .collapsible(false)
            .on_value_change(Message::Single),
    );
    ui.click(Id::new("a")).expect("open header");
    ui.click(Id::new("b")).expect("other header");
    assert!(ui.find(Id::new("body-a")).is_ok());
    assert!(ui.find(Id::new("body-b")).is_err());
    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [Message::Single(Some(1))]
    );
}

#[test]
fn multiple_values_drop_unknown_and_repeated_positions() {
    let mut ui = mount(
        accordion([item("a"), item("b")])
            .multiple()
            .value([0, 0, 19])
            .on_value_change(Message::Multiple),
    );
    ui.click(Id::new("b")).expect("second header");
    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [Message::Multiple(vec![0, 1])]
    );
}

#[test]
fn enum_keys_are_preserved_in_controlled_callbacks() {
    let mut ui = mount(
        accordion::keyed([(Record::Shipping, item("a")), (Record::Returns, item("b"))])
            .value(Some(Record::Shipping))
            .on_value_change(Message::Keyed),
    );
    ui.click(Id::new("b")).expect("returns header");
    ui.click(Id::new("a")).expect("shipping header");
    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [Message::Keyed(Some(Record::Returns)), Message::Keyed(None),]
    );
}

#[test]
fn changing_multiple_to_single_reconciles_the_uncontrolled_open_set() {
    let mut ui = mount(
        accordion([item("a"), item("b")])
            .multiple()
            .default_value([0, 1]),
    );
    assert!(ui.find(Id::new("body-b")).is_ok());
    ui = ui.rebuild(Element::from(accordion([item("a"), item("b")])));
    assert!(ui.find(Id::new("body-a")).is_ok());
    assert!(ui.find(Id::new("body-b")).is_err());
}

#[test]
fn a_disabled_ancestor_reaches_the_trigger_and_body() {
    use icedcn_widget::Inherited;

    let mut slot: Slot<'_, Message> = accordion([item("a")])
        .default_value(0)
        .on_value_change(Message::Single)
        .into();
    slot.inherit(&Inherited::default().with_disabled(true));
    let mut ui = mount(slot.compose());
    ui.click(Id::new("a"))
        .expect("disabled header stays mounted");
    assert!(ui.find(Id::new("body-a")).is_ok());
    assert_eq!(ui.into_messages().count(), 0);
}

#[test]
fn uncontrolled_multiple_selection_publishes_and_retains_the_complete_set() {
    let view = || {
        accordion([item("a"), item("b")])
            .multiple()
            .default_value([0])
            .on_value_change(Message::Multiple)
    };
    let mut ui = mount(view());
    ui.click(Id::new("b")).expect("second header");
    ui = ui.rebuild(Element::from(view()));
    assert!(ui.find(Id::new("body-a")).is_ok());
    assert!(ui.find(Id::new("body-b")).is_ok());
    ui.click(Id::new("a")).expect("first header");
    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [Message::Multiple(vec![0, 1]), Message::Multiple(vec![1]),]
    );
}
