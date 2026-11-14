//! A streaming fixture schedules its next chunk only while its preview is visible.
use super::demo::Event;
use iced::time::Instant;
use iced::{Fill, Length, Rectangle, Renderer, Size, window};
use iced_widget::core::widget::{Meta, Tree, tree};
use iced_widget::core::{Layout, Shell, Widget, layout, mouse, renderer};
use icedxdy::Theme;
use std::time::Duration;

pub(super) struct Tick;
impl Meta for Tick {}
impl Widget<Event, Theme, Renderer> for Tick {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<Option<Instant>>()
    }
    fn state(&self) -> tree::State {
        tree::State::new(None::<Instant>)
    }
    fn size(&self) -> Size<Length> {
        Size::new(Fill, Fill)
    }
    fn layout(&mut self, tree: &mut Tree, _: &Renderer, limits: &layout::Limits) {
        tree.size = limits.resolve(Fill, Fill, Size::ZERO);
    }
    fn update(
        &mut self,
        tree: &mut Tree,
        event: &iced::Event,
        layout: Layout,
        _: mouse::Cursor,
        _: &Renderer,
        shell: &mut Shell<'_, Event>,
        viewport: &Rectangle,
    ) {
        if let iced::Event::Window(window::Event::RedrawRequested(now)) = event
            && layout.bounds().intersects(viewport)
        {
            let next = tree
                .state
                .downcast_mut::<Option<Instant>>()
                .get_or_insert(*now);
            if *now >= *next {
                shell.publish(Event::Tick);
                *next = *now + Duration::from_millis(24);
            }
            shell.request_redraw_at(*next);
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
