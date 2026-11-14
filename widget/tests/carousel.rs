//! Keyboard selection survives a view rebuild; a controlled carousel only
//! publishes a new snap. Autoplay runs on its own scheduled redraw.
use iced_core::keyboard::key::Named;
use iced_core::time::Instant;
use iced_core::widget::operation::{Operation, Outcome, focusable};
use iced_core::{Element, Event, Point, window};
use iced_test::simulator::{self, simulator};
use iced_widget::{Renderer, Space};
use icedcn_core::Theme;
use icedcn_widget::{carousel, container};
use std::time::Duration;

fn gallery(controlled: bool) -> Element<'static, usize, Theme, Renderer> {
    let mut c = carousel(5, |_| Space::new().width(100).height(100))
        .on_value_change(|i| i)
        .autoplay(Duration::from_secs(2));
    if controlled {
        c = c.value(0);
    }
    container(c).width(220).into()
}
#[test]
fn keyboard_clamps_and_respects_controlled_snap() {
    for (controlled, expected) in [(true, vec![1, 1]), (false, vec![1, 2])] {
        let mut ui = simulator(gallery(controlled));
        let mut focus = focusable::focus_next::<()>();
        ui.operate(&mut focus);
        if let Outcome::Chain(mut next) = focus.finish() {
            ui.operate(next.as_mut());
        }
        let _ = ui.simulate([simulator::press_key(Named::ArrowRight, None)]);
        let mut ui = ui.rebuild(gallery(controlled));
        let _ = ui.simulate([simulator::press_key(Named::ArrowRight, None)]);
        assert_eq!(ui.into_messages().collect::<Vec<_>>(), expected);
    }
}
#[test]
fn autoplay_uses_scheduled_redraw_and_pauses_on_hover() {
    let mut ui = simulator(gallery(false));
    ui.point_at(Point::new(300.0, 300.0));
    let now = Instant::now();
    let tick = |sec| {
        Event::Window(window::Event::RedrawRequested(
            now + Duration::from_secs(sec),
        ))
    };
    let _ = ui.simulate([tick(0), tick(2)]);
    ui.point_at(Point::new(60.0, 20.0));
    let _ = ui.simulate([tick(4), tick(6)]);
    assert_eq!(ui.into_messages().collect::<Vec<_>>(), [1]);
}

mod track {
    use std::any::Any;
    use std::time::Duration;

    use iced_core::Length::Fill;
    use iced_core::keyboard::key::Named;
    use iced_core::time::Instant;
    use iced_core::widget::operation::{Focusable, Outcome, focusable};
    use iced_core::widget::{Id, Meta, Operation, Tree, tree};
    use iced_core::{
        Alignment, Element, Event, Layout, Length, Point, Rectangle, Shell, Size, Widget, layout,
        mouse, renderer, touch, window,
    };
    use iced_test::simulator::{self, simulator};
    use iced_widget::{Renderer, Widget as _};
    use icedcn_core::{Orientation, Theme};
    use icedcn_widget::{Inherited, Slot, button, carousel, column};

    #[derive(Debug, Clone, PartialEq)]
    enum Message {
        Selected(usize),
        OtherSelected(usize),
        Clicked(usize, u32),
    }
    type Preview = iced_test::Simulator<'static, Message, Theme, Renderer>;

    fn preview(element: Element<'static, Message, Theme, Renderer>) -> Preview {
        simulator(element)
    }

    struct Leaf(usize);
    struct Memory {
        index: usize,
        clicks: u32,
    }
    impl Meta for Leaf {}
    impl Widget<Message, Theme, Renderer> for Leaf {
        fn tag(&self) -> tree::Tag {
            tree::Tag::of::<Memory>()
        }
        fn state(&self) -> tree::State {
            tree::State::new(Memory {
                index: self.0,
                clicks: 0,
            })
        }
        fn size(&self) -> Size<Length> {
            Size::new(Length::Fill, 100.into())
        }
        fn layout(&mut self, tree: &mut Tree, _: &Renderer, limits: &layout::Limits) {
            tree.size = limits.resolve(Length::Fill, 100, Size::ZERO);
        }
        fn operate(
            &mut self,
            tree: &mut Tree,
            layout: Layout,
            _: &Rectangle,
            _: &Renderer,
            operation: &mut dyn Operation,
        ) {
            operation.custom(None, layout.bounds(), tree.state.downcast_mut::<Memory>());
        }
        fn update(
            &mut self,
            tree: &mut Tree,
            event: &Event,
            layout: Layout,
            cursor: mouse::Cursor,
            _: &Renderer,
            shell: &mut Shell<'_, Message>,
            viewport: &Rectangle,
        ) {
            if matches!(
                event,
                Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            ) && cursor.is_over(layout.bounds())
                && cursor.is_over(*viewport)
            {
                let state = tree.state.downcast_mut::<Memory>();
                state.clicks += 1;
                shell.publish(Message::Clicked(self.0, state.clicks));
            }
        }
        fn draw(
            &self,
            _: &Tree,
            _: &mut Renderer,
            _: &Theme,
            _: &renderer::Style,
            _: Layout,
            _: mouse::Cursor,
            _: &Rectangle,
        ) {
        }
    }

    #[derive(Default, Debug)]
    struct Snapshot {
        leaves: Vec<(usize, u32, Rectangle)>,
        controls: Vec<(Rectangle, bool)>,
    }
    impl Operation for Snapshot {
        fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
            visit(self);
        }
        fn custom(&mut self, _: Option<&Id>, bounds: Rectangle, state: &mut dyn Any) {
            if let Some(state) = state.downcast_mut::<Memory>() {
                self.leaves.push((state.index, state.clicks, bounds));
            }
        }
        fn focusable(&mut self, _: Option<&Id>, bounds: Rectangle, state: &mut dyn Focusable) {
            self.controls.push((bounds, state.is_focused()));
        }
    }
    fn snapshot(ui: &mut Preview) -> Snapshot {
        let mut result = Snapshot::default();
        ui.operate(&mut result);
        result
    }
    fn click(ui: &mut Preview, x: f32, y: f32) {
        ui.point_at(Point::new(x, y));
        ui.simulate(simulator::click());
    }
    fn tick(ui: &mut Preview, at: Instant) {
        ui.simulate([Event::Window(window::Event::RedrawRequested(at))]);
    }
    fn settle(ui: &mut Preview, clock: &mut Instant) {
        for _ in 0..180 {
            *clock += Duration::from_secs_f32(1.0 / 60.0);
            tick(ui, *clock);
        }
    }
    fn next_focus(ui: &mut Preview) {
        let mut operation = focusable::focus_next::<()>();
        ui.operate(&mut operation);
        if let Outcome::Chain(mut next) = operation.finish() {
            ui.operate(next.as_mut());
        }
    }
    fn gallery() -> icedcn_widget::carousel::Carousel<'static, Message> {
        carousel(5, |index| Leaf(index).boxed())
            .width(320)
            .on_value_change(Message::Selected)
    }

    #[test]
    fn all_slides_stay_mounted_and_child_state_follows_the_slide() {
        let mut ui = preview(gallery().into());
        let mut clock = Instant::now();
        assert_eq!(snapshot(&mut ui).leaves.len(), 5);
        click(&mut ui, 100.0, 50.0);
        assert_eq!(ui.drain().collect::<Vec<_>>(), [Message::Clicked(0, 1)]);
        click(&mut ui, 402.0, 50.0);
        settle(&mut ui, &mut clock);
        assert_eq!(ui.drain().collect::<Vec<_>>(), [Message::Selected(1)]);
        click(&mut ui, 100.0, 50.0);
        assert_eq!(ui.drain().collect::<Vec<_>>(), [Message::Clicked(1, 1)]);
        click(&mut ui, 14.0, 50.0);
        settle(&mut ui, &mut clock);
        click(&mut ui, 100.0, 50.0);
        assert_eq!(
            ui.drain().collect::<Vec<_>>(),
            [Message::Selected(0), Message::Clicked(0, 2)]
        );
        let saved = snapshot(&mut ui);
        assert_eq!(
            saved
                .leaves
                .iter()
                .map(|(index, _, _)| *index)
                .collect::<Vec<_>>(),
            [0, 1, 2, 3, 4]
        );
        assert_eq!(saved.leaves[1].1, 1);
    }

    #[test]
    fn motion_uses_source_fixed_steps_and_repeated_navigation_retargets_without_jumping() {
        let mut ui = preview(gallery().into());
        let now = Instant::now();
        tick(&mut ui, now - Duration::from_secs(20));
        click(&mut ui, 402.0, 50.0);
        assert_eq!(snapshot(&mut ui).leaves[0].2.x, 48.0);
        tick(&mut ui, now);
        let first = snapshot(&mut ui).leaves[0].2.x;
        assert!((first - (48.0 - 336.0 * 0.0272)).abs() < 0.01, "{first}");
        tick(&mut ui, now + Duration::from_secs_f32(1.0 / 60.0));
        let second = snapshot(&mut ui).leaves[0].2.x;
        assert!(
            (second - (48.0 - 336.0 * 0.07215616)).abs() < 0.01,
            "{second}"
        );
        click(&mut ui, 402.0, 50.0);
        assert!((snapshot(&mut ui).leaves[0].2.x - second).abs() < 0.01);
        let mut clock = now + Duration::from_secs(1);
        settle(&mut ui, &mut clock);
        assert!((snapshot(&mut ui).leaves[2].2.x - 48.0).abs() < 0.01);
        assert_eq!(
            ui.drain().collect::<Vec<_>>(),
            [Message::Selected(1), Message::Selected(2)]
        );
    }

    #[test]
    fn drag_moves_input_geometry_and_controlled_rejection_settles_back() {
        for controlled in [false, true] {
            let view = if controlled {
                gallery().value(0)
            } else {
                gallery()
            };
            let mut ui = preview(view.into());
            ui.point_at(Point::new(220.0, 50.0));
            ui.simulate([Event::Mouse(mouse::Event::ButtonPressed(
                mouse::Button::Left,
            ))]);
            ui.point_at(Point::new(120.0, 50.0));
            ui.simulate([Event::Mouse(mouse::Event::CursorMoved {
                position: Point::new(120.0, 50.0),
            })]);
            assert!((snapshot(&mut ui).leaves[0].2.x + 52.0).abs() < 0.01);
            ui.simulate([Event::Mouse(mouse::Event::ButtonReleased(
                mouse::Button::Left,
            ))]);
            assert_eq!(
                ui.drain().collect::<Vec<_>>(),
                [Message::Selected(1)],
                "drag release must not click slide content"
            );
            let mut clock = Instant::now();
            settle(&mut ui, &mut clock);
            let displayed = usize::from(!controlled);
            assert!((snapshot(&mut ui).leaves[displayed].2.x - 48.0).abs() < 0.01);
        }
    }

    #[test]
    fn endpoints_keyboard_activation_and_focus_are_scoped() {
        let mut ui = preview(gallery().into());
        next_focus(&mut ui);
        assert_eq!(
            snapshot(&mut ui)
                .controls
                .iter()
                .filter(|(_, focused)| *focused)
                .count(),
            1
        );
        ui.tap_key(Named::Space);
        ui.tap_key(Named::ArrowRight);
        ui.tap_key(Named::Enter);
        assert_eq!(
            ui.drain().collect::<Vec<_>>(),
            [
                Message::Selected(1),
                Message::Selected(2),
                Message::Selected(3)
            ]
        );
        ui.tap_key(Named::ArrowRight);
        ui.tap_key(Named::ArrowRight);
        click(&mut ui, 402.0, 50.0);
        assert_eq!(ui.drain().collect::<Vec<_>>(), [Message::Selected(4)]);
        click(&mut ui, 900.0, 400.0);
        ui.tap_key(Named::ArrowLeft);
        assert!(ui.drain().next().is_none());
        click(&mut ui, 14.0, 50.0);
        assert_eq!(ui.drain().collect::<Vec<_>>(), [Message::Selected(3)]);
    }

    #[test]
    fn snapping_alignment_looping_and_vertical_cells_follow_source_geometry() {
        assert_eq!(gallery().visible(2).align(Alignment::Start).snap_count(), 4);
        assert_eq!(gallery().visible(2).snap_count(), 5);
        assert_eq!(gallery().visible(3).snap_count(), 3);
        assert_eq!(gallery().visible(2).looping(true).snap_count(), 5);
        assert_eq!(gallery().visible(5).looping(true).snap_count(), 1);
        let mut ui = preview(
            gallery()
                .orientation(Orientation::Vertical)
                .visible(2)
                .spacing(4.0)
                .height(266.0)
                .align(Alignment::Start)
                .into(),
        );
        let view = snapshot(&mut ui);
        assert_eq!(
            view.leaves[0].2,
            Rectangle {
                x: 0.0,
                y: 48.0,
                width: 320.0,
                height: 100.0
            }
        );
        assert_eq!(view.leaves[1].2.y, 183.0);
        click(&mut ui, 160.0, 348.0);
        assert_eq!(ui.drain().collect::<Vec<_>>(), [Message::Selected(1)]);
        let mut looping = preview(gallery().looping(true).default_value(4).into());
        click(&mut looping, 402.0, 50.0);
        let mut clock = Instant::now();
        settle(&mut looping, &mut clock);
        assert_eq!(looping.drain().collect::<Vec<_>>(), [Message::Selected(0)]);
        assert!((snapshot(&mut looping).leaves[0].2.x - 48.0).abs() < 0.01);
        click(&mut looping, 14.0, 50.0);
        settle(&mut looping, &mut clock);
        assert_eq!(looping.drain().collect::<Vec<_>>(), [Message::Selected(4)]);
        assert!((snapshot(&mut looping).leaves[4].2.x - 48.0).abs() < 0.01);
    }

    #[test]
    fn hover_modes_and_manual_navigation_have_distinct_timer_semantics() {
        for permanent in [false, true] {
            let mut ui = preview(
                gallery()
                    .autoplay(Duration::from_secs(2))
                    .stop_on_hover(permanent)
                    .into(),
            );
            ui.point_at(Point::new(900.0, 400.0));
            let now = Instant::now();
            tick(&mut ui, now);
            tick(&mut ui, now + Duration::from_secs(2));
            assert_eq!(ui.drain().collect::<Vec<_>>(), [Message::Selected(1)]);
            ui.point_at(Point::new(200.0, 50.0));
            tick(&mut ui, now + Duration::from_secs(4));
            ui.point_at(Point::new(900.0, 400.0));
            tick(&mut ui, now + Duration::from_secs(5));
            tick(&mut ui, now + Duration::from_secs(7));
            assert_eq!(
                ui.drain().collect::<Vec<_>>(),
                if permanent {
                    vec![]
                } else {
                    vec![Message::Selected(2)]
                }
            );
        }
        let mut ui = preview(
            gallery()
                .autoplay(Duration::from_secs(2))
                .default_value(4)
                .into(),
        );
        ui.point_at(Point::new(900.0, 400.0));
        let now = Instant::now();
        tick(&mut ui, now);
        tick(&mut ui, now + Duration::from_secs(2));
        assert_eq!(ui.drain().collect::<Vec<_>>(), [Message::Selected(0)]);
        next_focus(&mut ui);
        ui.tap_key(Named::Enter);
        tick(&mut ui, now + Duration::from_secs(4));
        assert_eq!(
            ui.drain().collect::<Vec<_>>(),
            [Message::Selected(1), Message::Selected(2)]
        );
        click(&mut ui, 100.0, 50.0);
        ui.drain().for_each(drop);
        ui.point_at(Point::new(900.0, 400.0));
        tick(&mut ui, now + Duration::from_secs(8));
        tick(&mut ui, now + Duration::from_secs(12));
        assert!(ui.drain().next().is_none());
    }

    #[test]
    fn inherited_disable_and_explicit_disable_block_navigation_and_timers() {
        for explicit in [false, true] {
            let mut slot: Slot<'_, Message> = gallery()
                .autoplay(Duration::from_secs(2))
                .disabled(explicit)
                .into();
            if !explicit {
                slot.inherit(&Inherited::default().with_disabled(true));
            }
            let mut ui = preview(slot.compose());
            click(&mut ui, 402.0, 50.0);
            next_focus(&mut ui);
            ui.tap_key(Named::ArrowRight);
            ui.tap_key(Named::Enter);
            let id = touch::Finger(1);
            ui.simulate([
                Event::Touch(touch::Event::FingerPressed {
                    id,
                    position: Point::new(220.0, 50.0),
                }),
                Event::Touch(touch::Event::FingerMoved {
                    id,
                    position: Point::new(120.0, 50.0),
                }),
                Event::Touch(touch::Event::FingerLifted {
                    id,
                    position: Point::new(120.0, 50.0),
                }),
            ]);
            ui.point_at(Point::new(900.0, 400.0));
            let now = Instant::now();
            tick(&mut ui, now);
            tick(&mut ui, now + Duration::from_secs(10));
            assert!(ui.drain().next().is_none());
            assert_eq!(snapshot(&mut ui).leaves[0].2.x, 48.0);
        }
    }

    #[test]
    fn vertical_touch_drag_cancellation_and_source_keyboard_axes() {
        let mut ui = preview(
            gallery()
                .orientation(Orientation::Vertical)
                .visible(2)
                .spacing(4.0)
                .height(266.0)
                .align(Alignment::Start)
                .into(),
        );
        let id = touch::Finger(1);
        ui.simulate([
            Event::Touch(touch::Event::FingerPressed {
                id,
                position: Point::new(160.0, 200.0),
            }),
            Event::Touch(touch::Event::FingerMoved {
                id,
                position: Point::new(160.0, 100.0),
            }),
        ]);
        assert!((snapshot(&mut ui).leaves[0].2.y + 52.0).abs() < 0.01);
        ui.simulate([Event::Touch(touch::Event::FingerLost {
            id,
            position: Point::new(160.0, 100.0),
        })]);
        let mut clock = Instant::now();
        settle(&mut ui, &mut clock);
        assert_eq!(snapshot(&mut ui).leaves[0].2.y, 48.0);
        assert!(ui.drain().next().is_none());
        ui.simulate([
            Event::Touch(touch::Event::FingerPressed {
                id,
                position: Point::new(160.0, 200.0),
            }),
            Event::Touch(touch::Event::FingerMoved {
                id,
                position: Point::new(160.0, 100.0),
            }),
            Event::Touch(touch::Event::FingerLifted {
                id,
                position: Point::new(160.0, 100.0),
            }),
        ]);
        settle(&mut ui, &mut clock);
        assert!((snapshot(&mut ui).leaves[1].2.y - 48.0).abs() < 0.01);
        assert_eq!(ui.drain().collect::<Vec<_>>(), [Message::Selected(1)]);
        next_focus(&mut ui);
        ui.tap_key(Named::ArrowDown);
        ui.tap_key(Named::ArrowUp);
        assert!(ui.drain().next().is_none());
        ui.tap_key(Named::ArrowRight);
        ui.tap_key(Named::ArrowLeft);
        assert_eq!(
            ui.drain().collect::<Vec<_>>(),
            [Message::Selected(2), Message::Selected(1)]
        );
    }

    #[test]
    fn resizing_preserves_then_clamps_selection_without_reseeding_or_swapping_children() {
        let mut ui = preview(gallery().default_value(4).into());
        click(&mut ui, 100.0, 50.0);
        assert_eq!(ui.drain().collect::<Vec<_>>(), [Message::Clicked(4, 1)]);
        let mut ui = ui.rebuild(Element::from(gallery().width(192).default_value(0)));
        let before = snapshot(&mut ui);
        assert_eq!(before.leaves[4].1, 1);
        assert_eq!(before.leaves[4].2.x, 48.0);
        let mut ui = ui.rebuild(Element::from(gallery().visible(3).align(Alignment::Start)));
        let mut clock = Instant::now();
        settle(&mut ui, &mut clock);
        let after = snapshot(&mut ui);
        assert!((after.leaves[2].2.x - 48.0).abs() < 0.01);
        assert_eq!(after.leaves[4].1, 1);
        assert!(ui.drain().next().is_none(), "resize is not user selection");
    }

    #[test]
    fn descendant_focus_reveals_its_slide_and_keyboard_does_not_reach_another_carousel() {
        let mut ui = preview(
            column![
                gallery(),
                carousel(5, |index| Leaf(index).boxed())
                    .width(320)
                    .on_value_change(Message::OtherSelected),
            ]
            .spacing(0)
            .into(),
        );
        next_focus(&mut ui);
        ui.tap_key(Named::ArrowRight);
        assert_eq!(ui.drain().collect::<Vec<_>>(), [Message::Selected(1)]);
        next_focus(&mut ui);
        ui.tap_key(Named::ArrowRight);
        assert_eq!(ui.drain().collect::<Vec<_>>(), [Message::OtherSelected(1)]);

        let mut ui = preview(
            carousel(3, |index| {
                button(Leaf(index).boxed())
                    .width(Fill)
                    .height(100)
                    .padding(0)
            })
            .width(320)
            .on_value_change(Message::Selected)
            .into(),
        );
        next_focus(&mut ui);
        next_focus(&mut ui);
        let mut clock = Instant::now();
        settle(&mut ui, &mut clock);
        assert_eq!(ui.drain().collect::<Vec<_>>(), [Message::Selected(1)]);
        let observed = snapshot(&mut ui);
        let focused = observed
            .controls
            .iter()
            .find(|(_, focused)| *focused)
            .unwrap();
        assert!((focused.0.x - 48.0).abs() < 0.01, "{observed:?}");

        ui.tap_key(Named::ArrowRight);
        settle(&mut ui, &mut clock);
        assert_eq!(ui.drain().collect::<Vec<_>>(), [Message::Selected(2)]);
        let observed = snapshot(&mut ui);
        assert!(
            (observed.controls[2].0.x - 48.0).abs() < 0.01,
            "{observed:?}"
        );
    }
}
