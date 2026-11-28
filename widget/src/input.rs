//! A single-line text input with a styled focus ring.
//!
//! # Example
//!
//! ```ignore
//! use icedcn::widget::input;
//!
//! // Uncontrolled: the input keeps what the user types.
//! input("Name").default_value("Pedro Duarte")
//!
//! // Controlled: the app owns the value.
//! input("Email").value(&self.email).on_input(Message::EmailChanged)
//! ```

use crate::Size;
use crate::button::group::Join;
use crate::compose::{Compose, Inherited, View};
use crate::text_input;
use crate::text_input::element::from_core;
use iced_core::text::{Fragment, IntoFragment, LineHeight};
use iced_core::widget::Id;
use iced_core::{Length, Padding, alignment};
use iced_widget::Renderer;
use iced_widget::Widget;
use icedcn_core::Theme;
use icedcn_core::theme;

/// A single-line text field. [`default_value`](Self::default_value) initializes text kept by
/// the widget. Use [`value`](Self::value) and [`on_input`](Self::on_input) to manage the text
/// in the application. A controlled field without an input handler is read-only.
pub struct Input<'a, Message> {
    id: Option<Id>,
    join: Option<Join>,
    placeholder: Fragment<'a>,
    value: Option<Fragment<'a>>,
    default_value: Fragment<'a>,
    secure: bool,
    disabled: bool,
    size: Option<Size>,
    text_size: Option<f32>,
    padding: Option<Padding>,
    group_padding: Option<Padding>,
    alignment: alignment::Horizontal,
    width: Length,
    on_input: Option<Box<dyn Fn(String) -> Message + 'a>>,
    on_submit: Option<Message>,
    on_focus: Option<Message>,
    on_blur: Option<Message>,
    style: Option<theme::text_input::StyleFn<'a, Theme>>,
}

/// Create an empty [`Input`] with the given placeholder.
///
/// Both the placeholder and the [`value`](Input::value) accept owned or
/// borrowed text, so an input can be built from state a view does not
/// outlive (an iced `Component`'s view, for one).
pub fn input<'a, Message: Clone + 'a>(placeholder: impl IntoFragment<'a>) -> Input<'a, Message> {
    Input {
        id: None,
        join: None,
        placeholder: placeholder.into_fragment(),
        value: None,
        default_value: Fragment::Borrowed(""),
        secure: false,
        disabled: false,
        size: None,
        text_size: None,
        padding: None,
        group_padding: None,
        alignment: alignment::Horizontal::Left,
        width: Length::Shrink,
        on_input: None,
        on_submit: None,
        on_focus: None,
        on_blur: None,
        style: None,
    }
}

impl<'a, Message: Clone + 'a> Input<'a, Message> {
    /// Set the widget ID used by focus operations.
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set the space around the text, overriding the size and group defaults.
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = Some(padding.into());
        self
    }

    /// Set the horizontal alignment of the text.
    pub fn align_x(mut self, alignment: impl Into<alignment::Horizontal>) -> Self {
        self.alignment = alignment.into();
        self
    }

    /// Control the input: it always shows `value`, and edits only publish
    /// [`on_input`](Self::on_input). Owned or borrowed.
    pub fn value(mut self, value: impl IntoFragment<'a>) -> Self {
        self.value = Some(value.into_fragment());
        self
    }

    /// The initial value of an uncontrolled input. Later rebuilds keep what
    /// the user typed.
    pub fn default_value(mut self, value: impl IntoFragment<'a>) -> Self {
        self.default_value = value.into_fragment();
        self
    }

    /// Mask the displayed characters when entering a password.
    pub fn secure(mut self, secure: bool) -> Self {
        self.secure = secure;
        self
    }

    /// Disable the input: not focusable, not editable, muted.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set the size variant.
    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.size = Some(size.into());
        self
    }

    /// Set the field width. It defaults to `Length::Shrink`. Use `Length::Fill` to take the
    /// available width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Set the message published with the new value on every edit.
    pub fn on_input(mut self, on_input: impl Fn(String) -> Message + 'a) -> Self {
        self.on_input = Some(Box::new(on_input));
        self
    }

    /// Set the message to emit when the user presses Enter.
    pub fn on_submit(mut self, message: Message) -> Self {
        self.on_submit = Some(message);
        self
    }

    /// Set the message to emit when the input gains focus.
    pub fn on_focus(mut self, message: Message) -> Self {
        self.on_focus = Some(message);
        self
    }

    /// Set the message to emit when the input loses focus.
    pub fn on_blur(mut self, message: Message) -> Self {
        self.on_blur = Some(message);
        self
    }

    /// Override the input's style. Accepts our OKLCH-native `Default` variant
    /// or any `Spec` (e.g. `Default + Bg(...) + focus(Ring(...))`). Ring is
    /// preserved across the crate boundary by the bridge in
    /// [`text_input::element`](crate::text_input::element).
    pub fn style(mut self, style: impl Into<theme::text_input::StyleFn<'a, Theme>>) -> Self {
        self.style = Some(style.into());
        self
    }

    /// Set the text size independently of the control's height.
    pub fn text_size(mut self, size: f32) -> Self {
        self.text_size = Some(size);
        self
    }

    pub(crate) fn group_padding(mut self, padding: Padding) -> Self {
        self.group_padding = Some(padding);
        self
    }

    /// Input height — delegates to `Size::height` so size variants stay in
    /// sync with buttons. Nova `h-8` (32px) is `Size::Default`.
    fn height(&self) -> f32 {
        let height = self.size.unwrap_or_default().height();
        if let Some(padding) = self.padding.or(self.group_padding) {
            let line = LineHeight::default().to_absolute(self.font_size());
            return height.max(line.0 + padding.top + padding.bottom);
        }
        height
    }

    /// Text size based on size variant.
    /// Default/Sm: 14px (text-sm), Lg: 16px (text-base)
    fn font_size(&self) -> f32 {
        self.text_size
            .unwrap_or(match self.size.unwrap_or_default() {
                Size::Lg => 16.0,
                _ => 14.0,
            })
    }

    /// CSS padding plus the one-pixel border, which iced paints inside the bounds.
    fn resolved_padding(&self) -> Padding {
        self.padding.or(self.group_padding).unwrap_or(Padding {
            top: 5.0,
            bottom: 5.0,
            left: 11.0,
            right: 11.0,
        })
    }
}

impl<'a, Message: Clone + 'a> From<Input<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(input: Input<'a, Message>) -> Self {
        let height = input.height();
        let text_size = input.font_size();
        let padding = input.join.map_or_else(
            || input.resolved_padding(),
            |join| join.padding(input.resolved_padding(), 1.),
        );

        let ti: text_input::TextInput<'a, Message, Theme, Renderer> = match &input.value {
            Some(value) => text_input::TextInput::new(&input.placeholder, value),
            None => text_input::TextInput::new(&input.placeholder, "")
                .uncontrolled(&input.default_value),
        };
        let mut ti = ti
            .secure(input.secure)
            .disabled(input.disabled)
            .padding(padding)
            .align_x(input.alignment)
            .size(text_size)
            .width(input.width)
            .height(height)
            .join(input.join);

        if let Some(id) = input.id {
            ti = ti.id(id);
        }

        if let Some(style) = input.style {
            // Wrap the core StyleFn so it produces widget::Style (with ring).
            // Status types match across crates (vendored re-exports core's),
            // so no Status conversion is needed here.
            let bridge: text_input::StyleFn<'a, Theme> =
                Box::new(move |theme, status| from_core(style(theme, status)));
            ti = ti.class(bridge);
        }

        if let Some(on_input) = input.on_input {
            ti = ti.on_input(on_input);
        }
        if let Some(on_submit) = input.on_submit {
            ti = ti.on_submit(on_submit);
        }
        if let Some(on_focus) = input.on_focus {
            ti = ti.on_focus(on_focus);
        }
        if let Some(on_blur) = input.on_blur {
            ti = ti.on_blur(on_blur);
        }

        ti.boxed()
    }
}

impl<'a, Message: Clone + 'a> From<Input<'a, Message>> for View<'a, Message> {
    fn from(widget: Input<'a, Message>) -> Self {
        View::new(widget)
    }
}

impl<'a, Message: Clone + 'a> Compose<Message> for Input<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.join = self.join.or(from.join());
        self.size = self.size.or(from.size());
        self.disabled |= from.is_disabled();
    }

    fn compose<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        (*self).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced_core::widget::operation::focusable;
    use iced_core::{Alignment, Element};
    use iced_test::simulator::simulator;

    #[test]
    fn composed_inputs_can_be_focused_by_id() {
        let first = Id::unique();
        let second = Id::unique();
        let fields: Element<'_, String, Theme, Renderer> = crate::column![
            input("Quantity")
                .id(first.clone())
                .padding(5)
                .align_x(Alignment::Center)
                .on_input(|value| value),
            input("Price")
                .id(second.clone())
                .padding([5, 8])
                .align_x(Alignment::End)
                .on_input(|value| value),
        ]
        .into();
        let mut ui = simulator(fields);
        ui.operate(&mut focusable::focus(first));
        ui.typewrite("2");
        ui.operate(&mut focusable::focus(second));
        ui.typewrite("5");
        assert_eq!(ui.into_messages().collect::<Vec<_>>(), ["2", "5"]);
    }
}
