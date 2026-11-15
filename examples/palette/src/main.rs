use palette::{State, icon};

fn main() -> iced::Result {
    iced::application(State::new, State::update, State::view)
        .title("icedcn • palette demo")
        .window_size([1600.0, 1000.0])
        .theme(State::theme)
        .fonts([icon::FONT])
        .run()
}
