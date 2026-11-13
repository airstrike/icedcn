```rust
struct Outline;
impl canvas::Program<Message, Theme> for Outline {
    type State = ();
    fn draw(&self, _: &(), renderer: &iced::Renderer, theme: &Theme, bounds: Rectangle, _: Cursor) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let path = canvas::Path::rounded_rectangle(Point::new(0.5, 0.5), iced::Size::new((bounds.width-1.0).max(0.0), (bounds.height-1.0).max(0.0)), 12.0.into());
        let stroke = canvas::Stroke { line_dash: canvas::LineDash { segments: &[4.0, 3.0], offset: 0 }, ..canvas::Stroke::default().with_color(theme.neutral.border.into()).with_width(1.0) };
        frame.stroke(&path, stroke);
        vec![frame.into_geometry()]
    }
}

stack![Element::from(empty::empty().media(empty::media(icon::cloud()).style(empty::Icon)).title("Cloud Storage Empty").description("Upload files to your cloud storage to access them anywhere.").content(button("Upload Files").style(button::Outline).size(Size::Sm))), canvas(Outline).width(Fill).height(Fill)]
```
