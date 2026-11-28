use icedcn::widget::{alert, button};
use icedcn::{Size, View};

pub(super) fn view<'a, M: Clone + 'a>(enable: Option<M>) -> View<'a, M> {
    alert()
        .title("Dark mode is now available")
        .description("Enable it under your profile settings to get started.")
        .action(button("Enable").size(Size::Xs).on_press_maybe(enable))
        .max_width(448)
        .into()
}
