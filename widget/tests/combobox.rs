//! Combobox's composed controls, popup geometry and retained state.
use iced_core::Length::{Fill, Shrink};
use iced_core::keyboard::key::Named;
use iced_core::time::Instant;
use iced_core::widget::operation::{Focusable, Outcome, TextInput, focusable};
use iced_core::widget::{Id, Operation};
use iced_core::{Element, Event, Point, Rectangle, Size, window};
use iced_test::simulator::{self, simulator};
use iced_widget::{Renderer, Space};
use icedcn_core::Theme;
use icedcn_widget::{Inherited, View, button, combobox, container};

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Value(String),
    Values(Vec<String>),
    Child,
}
type Simulator = iced_test::Simulator<'static, Message, Theme, Renderer>;
fn frame(ui: &mut Simulator) {
    ui.simulate([Event::Window(
        window::Event::RedrawRequested(Instant::now()),
    )]);
}
fn click(ui: &mut Simulator, x: f32, y: f32) {
    ui.point_at(Point::new(x, y));
    ui.simulate(simulator::click());
    frame(ui);
}

#[test]
fn combobox_clear_reselect_and_empty_filter_publish_only_real_values() {
    let field: Element<'_, Message, Theme, Renderer> = combobox(["Next.js", "Remix"])
        .width(256)
        .default_value("Next.js")
        .clearable(true)
        .auto_highlight(true)
        .on_value_change(Message::Value)
        .into();
    let mut ui = simulator(field);
    click(&mut ui, 236., 16.);
    ui.typewrite("Rem");
    ui.tap_key(Named::Enter);
    frame(&mut ui);
    click(&mut ui, 236., 16.);
    ui.typewrite("zzzz");
    ui.tap_key(Named::Enter);
    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [
            Message::Value("".into()),
            Message::Value("Remix".into()),
            Message::Value("".into())
        ]
    );
}

#[test]
fn combobox_inherited_disabled_blocks_inline_popup_and_chips() {
    for mode in 0..3 {
        let mut field: View<'static, Message> = combobox(["Next.js", "Remix"])
            .width(256)
            .popup(mode == 1)
            .multiple(mode == 2)
            .default_values(["Next.js"])
            .multiple(mode == 2)
            .on_value_change(Message::Value)
            .on_values_change(Message::Values)
            .into();
        field.inherit(&Inherited::default().with_disabled(true));
        let mut ui = simulator(field.compose());
        click(&mut ui, 50., 16.);
        ui.typewrite("Rem");
        ui.tap_key(Named::ArrowDown);
        ui.tap_key(Named::Enter);
        click(&mut ui, 50., 46.);
        assert!(ui.into_messages().next().is_none());
    }
}

#[test]
fn combobox_multiple_backspace_publishes_complete_selection_and_preserves_controlled_value() {
    let view = |controlled| -> Element<'static, Message, Theme, Renderer> {
        let field = combobox(["Next.js", "Remix", "Astro"])
            .width(320)
            .default_values(["Next.js", "Remix"])
            .on_values_change(Message::Values);
        if controlled {
            field.values(["Next.js", "Remix"]).into()
        } else {
            field.into()
        }
    };
    for controlled in [false, true] {
        let mut ui = simulator(view(controlled));
        let mut inputs = Inputs::default();
        ui.operate(&mut inputs);
        let input = inputs.0[0].center();
        click(&mut ui, input.x, input.y);
        ui.tap_key(Named::Backspace);
        frame(&mut ui);
        let mut ui = ui.rebuild(view(controlled));
        ui.tap_key(Named::Backspace);
        let expected = if controlled {
            vec!["Next.js".into()]
        } else {
            vec![]
        };
        assert_eq!(
            ui.into_messages().collect::<Vec<_>>(),
            [
                Message::Values(vec!["Next.js".into()]),
                Message::Values(expected)
            ]
        );
    }
}

#[test]
fn combobox_custom_trigger_forwards_its_handler_and_opens_search() {
    let field: Element<'_, Message, Theme, Renderer> = combobox(["Apple", "Apricot", "Pear"])
        .trigger(|_| button("Pick").width(256).on_press(Message::Child))
        .auto_highlight(true)
        .on_value_change(Message::Value)
        .into();
    let mut ui = simulator(field);
    click(&mut ui, 20., 16.);
    ui.typewrite("Ap");
    ui.tap_key(Named::ArrowDown);
    ui.tap_key(Named::Enter);
    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [Message::Child, Message::Value("Apricot".into())]
    );
}

#[test]
fn combobox_end_reveals_last_option_in_a_scrolling_popup() {
    let last = Id::unique();
    let mark = last.clone();
    let field: Element<'_, Message, Theme, Renderer> =
        combobox((0..30).map(|i| format!("Item {i}")))
            .width(256)
            .auto_highlight(true)
            .format_item(move |item| {
                let slot = container(Space::new().width(80).height(20));
                if item.value == "Item 29" {
                    slot.id(mark.clone())
                } else {
                    slot
                }
            })
            .on_value_change(Message::Value)
            .into();
    let mut ui = simulator(field);
    click(&mut ui, 30., 16.);
    ui.tap_key(Named::End);
    frame(&mut ui);
    frame(&mut ui);
    let target = ui.find(last).expect("last row remains mounted");
    let visible = target
        .visible_bounds()
        .expect("highlight is scrolled into view");
    assert!(visible.height >= 19.0, "{visible:?}");
    ui.tap_key(Named::Enter);
    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [Message::Value("Item 29".into())]
    );
}

#[test]
fn combobox_content_width_and_offset_are_independent_of_trigger() {
    let row = Id::unique();
    let mark = row.clone();
    let field: Element<'_, Message, Theme, Renderer> = combobox(["One"])
        .width(256)
        .content_width(240)
        .align_offset(-28.0)
        .format_item(move |_| {
            container(Space::new().width(Fill).height(20))
                .width(Fill)
                .id(mark.clone())
        })
        .into();
    let field = container(field).padding(50);
    let mut ui = simulator(Element::from(field));
    click(&mut ui, 80., 66.);
    let bounds = ui.find(row).expect("option content").bounds();
    // Popup starts x=22; list padding=4, item left padding=6.
    assert!((bounds.x - 32.0).abs() < 1.0, "{bounds:?}");
    assert!(bounds.width > 170.0 && bounds.width < 210.0, "{bounds:?}");
}

#[test]
fn combobox_multiple_chips_wrap_within_narrow_parent() {
    let id = Id::unique();
    let field: Element<'_, Message, Theme, Renderer> = container(
        combobox(["One", "Two", "Three", "Four"])
            .default_values(["One", "Two", "Three", "Four"])
            .width(90),
    )
    .id(id.clone())
    .into();
    let mut ui = Simulator::with_size(Default::default(), Size::new(140., 200.), field);
    frame(&mut ui);
    let bounds = ui.find(id).expect("field").bounds();
    assert!(bounds.height > 32.0, "chips must wrap: {bounds:?}");
    assert!(bounds.width <= 90.0, "{bounds:?}");
}

#[test]
fn combobox_resolves_default_and_bounded_widths_from_the_final_mode() {
    let bounds = |field: combobox::Combobox<'static, Message>| {
        let id = Id::unique();
        let mut ui = simulator(Element::from(container(field).id(id.clone())));
        ui.find(id).expect("field").bounds().width
    };
    let field = || combobox(["One"]);
    let intrinsic = bounds(field());
    assert_eq!(bounds(field().popup(true)), 256.0);
    assert_eq!(bounds(field().popup(true).popup(false)), intrinsic);
    assert_eq!(bounds(field().width(Shrink.max(300))), intrinsic);
    assert_eq!(bounds(field().width(Shrink.max(100))), 100.0);
    let trigger = |_: &str| button("One").width(70);
    assert_eq!(bounds(field().width(Shrink).trigger(trigger)), 70.0);
    assert_eq!(bounds(field().trigger(trigger).width(Shrink)), 70.0);
}

#[test]
fn combobox_chip_label_is_passive_and_x_removes_only_that_chip() {
    let field: Element<'_, Message, Theme, Renderer> = combobox(["One", "Two"])
        .width(320)
        .default_values(["One", "Two"])
        .on_values_change(Message::Values)
        .into();
    let mut ui = simulator(field);
    click(&mut ui, 6., 14.);
    assert!(ui.drain().next().is_none(), "a chip label is passive");
    let mut buttons = Buttons::default();
    ui.operate(&mut buttons);
    let remove = buttons.0[0].center();
    click(&mut ui, remove.x, remove.y);
    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [Message::Values(vec!["Two".into()])]
    );
}

#[test]
fn combobox_chip_arrow_focus_and_delete_remove_the_focused_chip() {
    let field: Element<'_, Message, Theme, Renderer> = combobox(["One", "Two", "Three"])
        .width(320)
        .default_values(["One", "Two", "Three"])
        .on_values_change(Message::Values)
        .into();
    let mut ui = simulator(field);
    let mut inputs = Inputs::default();
    ui.operate(&mut inputs);
    let input = inputs.0[0].center();
    click(&mut ui, input.x, input.y);
    ui.tap_key(Named::ArrowLeft);
    ui.tap_key(Named::ArrowLeft);
    ui.tap_key(Named::Delete);
    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [Message::Values(vec!["One".into(), "Three".into()])]
    );
}

#[derive(Default)]
struct Inputs(Vec<Rectangle>);

impl Operation for Inputs {
    fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
        visit(self);
    }

    fn text_input(&mut self, _: Option<&Id>, bounds: Rectangle, _: &mut dyn TextInput) {
        self.0.push(bounds);
    }
}

#[test]
fn chip_input_fills_remaining_line_and_keeps_editing_through_resize() {
    let field: Element<'_, Message, Theme, Renderer> = combobox(["One", "Two", "Three"])
        .default_values(["One"])
        .auto_highlight(true)
        .width(Fill)
        .on_values_change(Message::Values)
        .into();
    let mut ui = Simulator::with_size(Default::default(), Size::new(320., 400.), field);
    let mut inputs = Inputs::default();
    ui.operate(&mut inputs);
    let room = inputs.0[0];
    assert!(room.width > 225., "{room:?}");
    click(&mut ui, 294., 16.);
    ui.typewrite("Tw");
    let mut ui = ui.resize(Size::new(90., 400.));
    ui.tap_key(Named::Enter);
    frame(&mut ui);
    ui.tap_key(Named::Backspace);
    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [
            Message::Values(vec!["One".into(), "Two".into()]),
            Message::Values(vec!["One".into()]),
        ]
    );
}

#[test]
fn popup_opens_from_arrow_and_search_spans_the_panel() {
    let field: Element<'_, Message, Theme, Renderer> = combobox(["One", "Two"])
        .popup(true)
        .auto_highlight(true)
        .on_value_change(Message::Value)
        .into();
    let mut ui = simulator(field);
    let mut next = focusable::focus_next::<()>();
    ui.operate(&mut next);
    if let Outcome::Chain(mut step) = next.finish() {
        ui.operate(step.as_mut());
    }
    ui.tap_key(Named::ArrowDown);
    frame(&mut ui);
    let mut inputs = Inputs::default();
    ui.operate(&mut inputs);
    assert!(
        inputs
            .0
            .iter()
            .any(|bounds| (bounds.width - 276.).abs() < 1.),
        "{:?}",
        inputs.0
    );
    ui.typewrite("Tw");
    ui.tap_key(Named::Enter);
    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [Message::Value("Two".into())]
    );
}

#[test]
fn clicking_the_focused_input_reopens_after_escape() {
    let option = Id::unique();
    let view = || -> Element<'static, Message, Theme, Renderer> {
        let mark = option.clone();
        combobox(["One"])
            .format_item(move |_| container(Space::new().width(40).height(16)).id(mark.clone()))
            .into()
    };
    let mut ui = simulator(view());
    for key in [Named::Escape, Named::Tab] {
        click(&mut ui, 28., 16.);
        assert!(ui.find(option.clone()).is_ok());
        ui.tap_key(key);
        // The runtime rebuilds after a component's overlay invalidates its view.
        ui = ui.rebuild(view());
        frame(&mut ui);
        assert!(ui.find(option.clone()).is_err());
    }
    click(&mut ui, 28., 16.);
    assert!(ui.find(option.clone()).is_ok());
    click(&mut ui, 400., 200.);
    let mut ui = ui.rebuild(view());
    assert!(ui.find(option).is_err());
}

#[test]
fn filter_matches_unaccented_queries_and_publishes_the_original_value() {
    let field: Element<'_, Message, Theme, Renderer> = combobox(["New York", "São Paulo"])
        .auto_highlight(true)
        .on_value_change(Message::Value)
        .into();
    let mut ui = simulator(field);
    click(&mut ui, 28., 16.);
    ui.typewrite("SAO");
    ui.tap_key(Named::Enter);
    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [Message::Value("São Paulo".into())]
    );
}

#[derive(Default)]
struct Buttons(Vec<Rectangle>);

impl Operation for Buttons {
    fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
        visit(self);
    }

    fn focusable(&mut self, _: Option<&Id>, bounds: Rectangle, _: &mut dyn Focusable) {
        self.0.push(bounds);
    }
}
