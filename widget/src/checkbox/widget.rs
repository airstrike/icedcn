//! Checkboxes can be used to let users make binary (or tri-state) choices.
//!
//! This is a sweetened version of `iced`'s [`checkbox`], vendored from
//! sweeten with these additions on top of sweeten's animated mark fade:
//!
//! - [`State::Indeterminate`] draws a horizontal stroke.
//! - `Style::ring` controls the halo behind the box.
//! - **Native input state** — focus operations, Space release activation,
//!   an expanded pointer target, and canceled-gesture handling. The focus
//!   ring follows keyboard modality; disabled controls leave focus traversal.
//!
//! [`checkbox`]: https://docs.iced.rs/iced/widget/checkbox/
use crate::animation::cubic_bezier;
use crate::core::alignment;
use crate::core::animation::Easing;
use crate::core::font;
use crate::core::keyboard::{self, Key, key::Named};
use crate::core::layout;
use crate::core::mouse;
use crate::core::renderer;
use crate::core::text;
use crate::core::text::paragraph;
use crate::core::time::Instant;
use crate::core::touch;
use crate::core::widget;
use crate::core::widget::operation::Focusable;
use crate::core::widget::tree::{self, Tree};
use crate::core::window;
use crate::core::{
    Animation, Background, Border, Color, Element, Em, Event, Font, Layout, Length, Pixels,
    Rectangle, Shell, Size, Theme, Widget,
};
use crate::operation::control::Control;
use crate::operation::focus;

use std::marker::PhantomData;
use std::time::Duration;

// Iced's widget crate re-exports `core::widget::text::*` at `crate::text`. We
// can't reuse that path (our `crate::text` is the icedcn typography helpers,
// and `use iced_widget::text` collides with `crate::core::text` already in
// scope above). A private re-exporting module gives the vendored draw + layout
// calls shallow `text_helpers::*` paths that satisfy the pre-commit deep-path
// / aliased-import checks. Mirrors `widget/src/switch/widget.rs`.
mod text_helpers {
    pub use iced_widget::text::{Format, Operand, Style, draw, layout};
}

/// A box that can be checked, unchecked, or indeterminate.
///
/// **Deviation from upstream sweeten/iced**: `state` is a [`State`] domain enum
/// (`Checked` / `Unchecked` / `Indeterminate`) instead of a `bool`. The
/// `From<bool> for State` impl preserves the bool-shaped call site:
/// `Checkbox::new(self.is_checked)` keeps working.
pub struct Checkbox<'a, Message, Theme = iced_widget::Theme, Renderer = iced_widget::Renderer>
where
    Renderer: text::Renderer,
    Theme: Catalog,
{
    state: State,
    is_controlled: bool,
    is_disabled: bool,
    on_toggle: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    label: Option<text::Fragment<'a>>,
    width: Length,
    size: f32,
    spacing: f32,
    text_size: Option<Pixels>,
    line_height: text::LineHeight,
    shaping: text::Shaping,
    wrapping: text::Wrapping,
    letter_spacing: Em,
    font_features: Vec<font::Feature>,
    font: Option<Font>,
    icon: Icon,
    check: Option<fn(&mut Renderer, Rectangle, Color, Rectangle)>,
    class: Theme::Class<'a>,
    last_status: Option<Status>,
    renderer: PhantomData<Renderer>,
}

impl<'a, Message, Theme, Renderer> Checkbox<'a, Message, Theme, Renderer>
where
    Renderer: text::Renderer,
    Theme: Catalog,
{
    /// The default size of a [`Checkbox`].
    const DEFAULT_SIZE: f32 = 16.0;

    /// Creates a new controlled [`Checkbox`] showing `state`.
    ///
    /// Accepts a [`State`] (the domain enum with `Checked`, `Unchecked`, and
    /// `Indeterminate` variants) — or a `bool`, via the
    /// `From<bool> for State` impl (`true` → `Checked`, `false` → `Unchecked`).
    pub fn new(state: impl Into<State>) -> Self {
        Checkbox {
            state: state.into(),
            is_controlled: true,
            is_disabled: false,
            on_toggle: None,
            label: None,
            width: Length::Shrink,
            size: Self::DEFAULT_SIZE,
            spacing: Self::DEFAULT_SIZE / 2.0,
            text_size: None,
            line_height: text::LineHeight::default(),
            shaping: text::Shaping::default(),
            wrapping: text::Wrapping::default(),
            letter_spacing: Em::default(),
            font_features: Vec::new(),
            font: None,
            icon: Icon {
                font: Renderer::ICON_FONT,
                code_point: Renderer::CHECKMARK_ICON,
                size: None,
                line_height: text::LineHeight::default(),
                shaping: text::Shaping::Basic,
            },
            check: None,
            class: Theme::default(),
            last_status: None,
            renderer: PhantomData,
        }
    }

    /// Sets the label of the [`Checkbox`].
    pub fn label(mut self, label: impl text::IntoFragment<'a>) -> Self {
        self.label = Some(label.into_fragment());
        self
    }

    /// Makes the [`Checkbox`] uncontrolled: it starts in `state` and keeps
    /// its own value in the widget tree from then on.
    pub fn uncontrolled(mut self, state: impl Into<State>) -> Self {
        self.state = state.into();
        self.is_controlled = false;
        self
    }

    /// Sets whether the [`Checkbox`] is disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    /// Sets the function that will be called when the [`Checkbox`] is toggled.
    /// It will receive the new state of the [`Checkbox`] and must produce a
    /// `Message`.
    pub fn on_toggle<F>(mut self, f: F) -> Self
    where
        F: 'a + Fn(bool) -> Message,
    {
        self.on_toggle = Some(Box::new(f));
        self
    }

    /// Sets the function that will be called when the [`Checkbox`] is toggled,
    /// if `Some`.
    pub fn on_toggle_maybe<F>(mut self, f: Option<F>) -> Self
    where
        F: Fn(bool) -> Message + 'a,
    {
        self.on_toggle = f.map(|f| Box::new(f) as _);
        self
    }

    /// Sets the size of the [`Checkbox`].
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = size.into().0;
        self
    }

    /// Sets the width of the [`Checkbox`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the spacing between the [`Checkbox`] and the text.
    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.spacing = spacing.into().0;
        self
    }

    /// Sets the text size of the [`Checkbox`].
    pub fn text_size(mut self, text_size: impl Into<Pixels>) -> Self {
        self.text_size = Some(text_size.into());
        self
    }

    /// Sets the text [`text::LineHeight`] of the [`Checkbox`].
    pub fn line_height(mut self, line_height: impl Into<text::LineHeight>) -> Self {
        self.line_height = line_height.into();
        self
    }

    /// Sets the [`text::Shaping`] strategy of the [`Checkbox`].
    pub fn shaping(mut self, shaping: text::Shaping) -> Self {
        self.shaping = shaping;
        self
    }

    /// Sets the [`text::Wrapping`] strategy of the [`Checkbox`].
    pub fn wrapping(mut self, wrapping: text::Wrapping) -> Self {
        self.wrapping = wrapping;
        self
    }

    /// Sets the letter spacing of the [`Checkbox`].
    pub fn letter_spacing(mut self, letter_spacing: impl Into<Em>) -> Self {
        self.letter_spacing = letter_spacing.into();
        self
    }

    /// Adds a single font [`Feature`](font::Feature) to the [`Checkbox`].
    pub fn font_feature(mut self, feature: impl Into<font::Feature>) -> Self {
        self.font_features.push(feature.into());
        self
    }

    /// Sets the font features of the [`Checkbox`].
    pub fn font_features(mut self, features: Vec<font::Feature>) -> Self {
        self.font_features = features;
        self
    }

    /// Sets the [`Font`] of the text of the [`Checkbox`].
    pub fn font(mut self, font: impl Into<Font>) -> Self {
        self.font = Some(font.into());
        self
    }

    /// Sets the [`Icon`] of the [`Checkbox`].
    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = icon;
        self.check = None;
        self
    }

    // The public Nova composition supplies its vector mark without imposing
    // an SVG renderer bound on generic, custom-font Checkbox callers.
    pub(super) fn check(mut self, draw: fn(&mut Renderer, Rectangle, Color, Rectangle)) -> Self {
        self.check = Some(draw);
        self
    }

    /// Sets the style of the [`Checkbox`].
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
        self
    }

    /// Sets the style class of the [`Checkbox`].
    /// **Deviation from upstream sweeten**: stock sweeten gates `class` behind
    /// the `advanced` feature flag. We always expose it because our
    /// `.style(...)` bridge in `widget/src/checkbox.rs` needs to land a
    /// pre-baked `widget::StyleFn` (the closure that runs `from_core` on the
    /// core StyleFn's output). Without `class`, that bridge has nowhere to
    /// land. Mirrors the same deviation in `widget/src/switch/widget.rs`.
    #[must_use]
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }
}

/// Internal state for the animated [`Checkbox`].
///
/// **Deviation from upstream sweeten**: sweeten names this `State<P>` and
/// uses `is_checked: bool` as the public driver. Our public driver is the
/// [`State`] domain enum (Checked / Unchecked / Indeterminate), so the
/// internal widget-state struct is renamed to [`Internal<P>`] to avoid the
/// name collision. The animation tracks `state.is_marked()` (the boolean
/// projection that maps Indeterminate to "marked = true"), so the bg/border
/// fade fires identically when toggling between any two states.
struct Internal<P: text::Paragraph> {
    paragraph: paragraph::Plain<P>,
    /// The value an uncontrolled checkbox keeps across rebuilds; a
    /// controlled one mirrors its prop here.
    value: State,
    control: Control,
    animation: Animation<bool>,
    now: Option<Instant>,
    last_is_marked: bool,
    /// Whether the most recent left-button / finger press landed inside
    /// the checkbox. Toggling fires on release, gated by this flag plus
    /// a fresh "still inside on release" bounds check — pressing
    /// outside and dragging in (or pressing in and dragging out before
    /// release) must not toggle.
    is_pressed: bool,
    is_focused: bool,
    indicator: focus::Visible,
    ring: focus::Ring,
    space_pressed: bool,
}

impl<P: text::Paragraph> Focusable for Internal<P> {
    fn is_focused(&self) -> bool {
        self.is_focused
    }

    fn focus(&mut self) {
        self.is_focused = true;
        self.indicator.0 = true;
    }

    fn unfocus(&mut self) {
        self.is_focused = false;
        self.space_pressed = false;
    }
}

impl<Message, Theme, Renderer> Checkbox<'_, Message, Theme, Renderer>
where
    Renderer: text::Renderer,
    Theme: Catalog,
{
    /// Flip the checkbox as a click does: an uncontrolled checkbox stores
    /// the new value, and the change message is published either way.
    ///
    /// Activating an indeterminate checkbox checks it.
    fn toggle(
        &mut self,
        internal: &mut Internal<Renderer::Paragraph>,
        shell: &mut Shell<'_, Message>,
    ) {
        let next = match self.state {
            State::Checked => false,
            State::Unchecked | State::Indeterminate => true,
        };

        if !self.is_controlled {
            self.state = State::from(next);
            internal.value = self.state;
            shell.request_redraw();
        }

        if let Some(on_toggle) = &self.on_toggle {
            shell.publish((on_toggle)(next));
        }

        shell.capture_event();
    }
}

impl<Message, Theme, Renderer> widget::Meta for Checkbox<'_, Message, Theme, Renderer>
where
    Renderer: text::Renderer,
    Theme: Catalog,
{
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Checkbox<'_, Message, Theme, Renderer>
where
    Renderer: text::Renderer,
    Theme: Catalog,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<Internal<Renderer::Paragraph>>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(Internal::<Renderer::Paragraph> {
            paragraph: paragraph::Plain::default(),
            value: self.state,
            control: Control::new(self.state.is_marked()),
            animation: Animation::new(self.state.is_marked())
                .duration(Duration::from_millis(150))
                .easing(Easing::Custom(|t| cubic_bezier(0.4, 0.0, 0.2, 1.0, t))),
            now: None,
            last_is_marked: self.state.is_marked(),
            is_pressed: false,
            is_focused: false,
            indicator: focus::Visible::default(),
            ring: focus::Ring::default(),
            space_pressed: false,
        })
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: Length::Shrink,
        }
    }

    fn diff(&mut self, tree: &mut Tree) {
        // The children of the tree are the box and the label; they only
        // carry their geometry, so no state is needed.
        tree.children.resize_with(2, Tree::empty);

        let internal = tree.state.downcast_mut::<Internal<Renderer::Paragraph>>();

        internal.ring.delegated = false;
        if self.is_controlled {
            internal.value = self.state;
        } else {
            self.state = internal.value;
        }
        if self.is_disabled {
            internal.unfocus();
            internal.is_pressed = false;
        }
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        let limits = limits.width(self.width);

        let checkbox = Size::new(self.size, self.size);
        let spacing = if self.label.is_some() {
            self.spacing
        } else {
            0.0
        };

        let label = if let Some(label) = self.label.as_deref() {
            let state = tree.state.downcast_mut::<Internal<Renderer::Paragraph>>();

            text_helpers::layout(
                &mut state.paragraph,
                renderer,
                &limits.shrink(Size::new(checkbox.width + spacing, 0.0)),
                label,
                text_helpers::Format {
                    width: self.width,
                    height: Length::Shrink,
                    line_height: Some(self.line_height),
                    size: self.text_size,
                    font: self.font,
                    align_x: text::Alignment::Default,
                    align_y: alignment::Vertical::Top,
                    shaping: self.shaping,
                    wrapping: self.wrapping,
                    ellipsis: text::Ellipsis::None,
                    letter_spacing: self.letter_spacing,
                    font_features: self.font_features.clone(),
                    font_variations: Vec::new(),
                    weight: None,
                },
            )
        } else {
            Size::ZERO
        };

        layout::next_to_each_other(tree, checkbox, label, spacing);
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let internal = tree.state.downcast_mut::<Internal<Renderer::Paragraph>>();
        let was_focused = (internal.is_focused, internal.indicator.0);
        // Nova's pseudo-element enlarges the target without changing layout.
        let is_over =
            cursor.is_over(layout.bounds().expand([8.0, 12.0])) && cursor.is_over(*viewport);

        if internal.control.take_activation() && !self.is_disabled {
            internal.is_focused = true;
            internal.indicator.0 = false;
            self.toggle(internal, shell);
        }

        if self.is_disabled {
            internal.unfocus();
            internal.is_pressed = false;
        } else {
            match event {
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                | Event::Touch(touch::Event::FingerPressed { .. }) => {
                    internal.space_pressed = false;
                    internal.is_pressed = is_over && !shell.is_event_captured();
                    internal.is_focused = internal.is_pressed;
                    internal.indicator.0 = false;
                    if internal.is_pressed {
                        shell.capture_event();
                    }
                }
                Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
                | Event::Touch(touch::Event::FingerLifted { .. }) => {
                    if std::mem::take(&mut internal.is_pressed) {
                        if is_over && !shell.is_event_captured() {
                            self.toggle(internal, shell);
                        }
                        shell.capture_event();
                    }
                }
                Event::Touch(touch::Event::FingerLost { .. }) => {
                    internal.is_pressed = false;
                }
                Event::Keyboard(keyboard::Event::KeyPressed { key, .. })
                    if internal.is_focused && !shell.is_event_captured() =>
                {
                    internal.indicator.0 = true;
                    if matches!(key, Key::Named(Named::Space)) {
                        internal.space_pressed = true;
                        shell.capture_event();
                    } else if matches!(key, Key::Named(Named::Enter)) {
                        // Space activates a focused checkbox.
                        shell.capture_event();
                    }
                }
                Event::Keyboard(keyboard::Event::KeyReleased {
                    key: Key::Named(Named::Space),
                    ..
                }) => {
                    if std::mem::take(&mut internal.space_pressed)
                        && internal.is_focused
                        && !shell.is_event_captured()
                    {
                        self.toggle(internal, shell);
                    }
                }
                Event::Window(window::Event::Unfocused) => {
                    internal.unfocus();
                    internal.is_pressed = false;
                }
                _ => {}
            }
        }

        if was_focused != (internal.is_focused, internal.indicator.0) {
            shell.request_redraw();
        }
        if let Event::Window(window::Event::RedrawRequested(now)) = event {
            internal.now = Some(*now);
            if internal.animation.is_animating(*now) {
                shell.request_redraw();
            }
        }
        let is_checked = self.state.is_marked();
        if is_checked != internal.last_is_marked {
            internal.last_is_marked = is_checked;
            // Input can follow a long idle with no redraw. Start at this event,
            // not the timestamp of the last rendered frame.
            let now = match event {
                Event::Window(window::Event::RedrawRequested(now)) => *now,
                _ => Instant::now(),
            };
            internal.now = Some(now);
            internal.animation.go_mut(is_checked, now);
            shell.request_redraw();
        }
        let status = if self.is_disabled {
            Status::Disabled { is_checked }
        } else if is_over {
            Status::Hovered { is_checked }
        } else {
            Status::Active { is_checked }
        };
        if self.last_status != Some(status) {
            shell.request_redraw();
        }
        self.last_status = Some(status);
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        if self.is_disabled
            && cursor.is_over(layout.bounds().expand([8.0, 12.0]))
            && cursor.is_over(*viewport)
        {
            mouse::Interaction::NotAllowed
        } else {
            mouse::Interaction::default()
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        defaults: &renderer::Style,
        layout: Layout,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let internal = tree.state.downcast_ref::<Internal<Renderer::Paragraph>>();

        let mut children = layout.iter(&tree.children);

        let current_status = self.last_status.unwrap_or(if self.is_disabled {
            Status::Disabled {
                is_checked: self.state.is_marked(),
            }
        } else {
            Status::Active {
                is_checked: self.state.is_marked(),
            }
        });

        // Fade the background and border together. The indicator changes as soon as the value
        // changes.
        let style = match internal.now {
            Some(now) if internal.animation.is_animating(now) => {
                let off = theme.style(&self.class, current_status.with_checked(false));
                let on = theme.style(&self.class, current_status.with_checked(true));

                // Only `Background::Color` is interpolable here; fall back
                // to transparent for gradients so the draw code stays
                // simple.
                let bg_color = |bg: &Background| match bg {
                    Background::Color(c) => *c,
                    _ => Color::TRANSPARENT,
                };

                let background = Background::Color(internal.animation.interpolate(
                    bg_color(&off.background),
                    bg_color(&on.background),
                    now,
                ));

                // Snap border width and radius — interpolating them looks
                // jittery — but interpolate the color along with the fill.
                let target_border = if internal.animation.value() {
                    on.border
                } else {
                    off.border
                };
                let border = Border {
                    color: internal
                        .animation
                        .interpolate(off.border.color, on.border.color, now),
                    ..target_border
                };

                // Snap to the on/off target for everything not interpolated
                // (icon_color, ring, text_color). `transition-none` on the
                // indicator means the icon changes immediately with the target.
                let target = if internal.animation.value() { on } else { off };
                Style {
                    background,
                    border,
                    ..target
                }
            }
            _ => theme.style(&self.class, current_status),
        };

        let style = if internal.is_focused && internal.indicator.0 && !self.is_disabled {
            if internal.ring.delegated {
                theme.delegated(style, self.state.is_marked())
            } else {
                theme.focused(style)
            }
        } else {
            style
        };

        {
            let (checkbox_layout, _) = children.next().expect("checkbox box");
            let bounds = checkbox_layout.bounds();

            // Ring: paint a halo *outside* the box bounds, behind the main
            // fill, when `style.ring` is `Some`. Mirrors the text_input + switch
            // painters (see `widget/src/text_input/widget.rs` lines 503-516,
            // `widget/src/switch/widget.rs` ring block in `draw`).
            // `border.width = ring.width` paints inward from the expanded
            // bounds, so the visible halo wraps around the box.
            if let Some(ring) = style.ring {
                let r = ring.width;
                let mut radius = style.border.radius;
                radius.top_left += r;
                radius.top_right += r;
                radius.bottom_right += r;
                radius.bottom_left += r;
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: Rectangle {
                            x: bounds.x - r,
                            y: bounds.y - r,
                            width: bounds.width + r * 2.0,
                            height: bounds.height + r * 2.0,
                        },
                        border: Border {
                            radius,
                            width: r,
                            color: ring.color,
                        },
                        ..renderer::Quad::default()
                    },
                    Background::Color(Color::TRANSPARENT),
                );
            }

            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border: style.border,
                    ..renderer::Quad::default()
                },
                style.background,
            );

            let Icon {
                font,
                code_point,
                size,
                line_height,
                shaping,
            } = &self.icon;
            let size = size.unwrap_or(Pixels(bounds.height * 0.875));

            match self.state {
                State::Checked if self.check.is_some() => {
                    if let Some(draw) = self.check {
                        draw(renderer, bounds, style.icon_color, *viewport);
                    }
                }
                State::Checked => {
                    renderer.fill_text(
                        text::Text {
                            content: code_point.to_string(),
                            font: *font,
                            size,
                            line_height: *line_height,
                            bounds: bounds.size(),
                            align_x: text::Alignment::Center,
                            align_y: alignment::Vertical::Center,
                            shaping: *shaping,
                            wrapping: text::Wrapping::default(),
                            ellipsis: text::Ellipsis::default(),
                            letter_spacing: Em::default(),
                            font_features: Vec::new(),
                            font_variations: Vec::new(),
                            weight: None,
                            hint_factor: None,
                        },
                        bounds.center(),
                        style.icon_color,
                        *viewport,
                    );
                }
                State::Indeterminate => {
                    // Draw a centered dash for the indeterminate state.
                    let stroke_w = bounds.width * 0.5;
                    let stroke_h = (bounds.height * 0.1).max(1.0);
                    renderer.fill_quad(
                        renderer::Quad {
                            bounds: Rectangle {
                                x: bounds.center_x() - stroke_w / 2.0,
                                y: bounds.center_y() - stroke_h / 2.0,
                                width: stroke_w,
                                height: stroke_h,
                            },
                            ..renderer::Quad::default()
                        },
                        Background::Color(style.icon_color),
                    );
                }
                State::Unchecked => {}
            }
        }

        if self.label.is_none() {
            return;
        }

        {
            let (label_layout, _) = children.next().expect("checkbox child");

            text_helpers::draw(
                renderer,
                defaults,
                label_layout.bounds(),
                internal.paragraph.raw(),
                text_helpers::Style {
                    color: style.text_color,
                    selection: None,
                },
                theme.selection(),
                viewport,
            );
        }
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        _viewport: &Rectangle,
        _renderer: &Renderer,
        operation: &mut dyn widget::Operation,
    ) {
        let internal = tree.state.downcast_mut::<Internal<Renderer::Paragraph>>();

        if !self.is_disabled {
            operation.focusable(None, layout.bounds(), internal);
            operation.custom(None, layout.bounds(), &mut internal.indicator);
        }
        internal.ring.visible = internal.is_focused && internal.indicator.0 && !self.is_disabled;
        internal.ring.disabled = self.is_disabled;
        operation.custom(None, layout.bounds(), &mut internal.ring);
        internal.control.set_checked(self.state.is_marked());
        operation.custom(None, layout.bounds(), &mut internal.control);

        if self.label.is_none() {
            return;
        }

        let paragraph = &mut internal.paragraph;

        let mut children = layout.iter(&tree.children);
        let (layout, _) = children.next().expect("checkbox child");

        operation.text(
            None,
            layout.bounds(),
            &mut text_helpers::Operand {
                paragraph,
                layout,
                selectable: true,
            },
        );
    }
}

impl<'a, Message, Theme, Renderer> From<Checkbox<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a + Catalog,
    Renderer: 'a + text::Renderer,
{
    fn from(
        checkbox: Checkbox<'a, Message, Theme, Renderer>,
    ) -> Element<'a, Message, Theme, Renderer> {
        checkbox._boxed()
    }
}

/// The icon in a [`Checkbox`].
#[derive(Debug, Clone, PartialEq)]
pub struct Icon {
    /// Font that will be used to display the `code_point`,
    pub font: Font,
    /// The unicode code point that will be used as the icon.
    pub code_point: char,
    /// Font size of the content.
    pub size: Option<Pixels>,
    /// The line height of the icon.
    pub line_height: text::LineHeight,
    /// The shaping strategy of the icon.
    pub shaping: text::Shaping,
}

/// Selection state of a [`Checkbox`]. Boolean values convert to checked and unchecked states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// Checked. Renders the check glyph.
    Checked,
    /// Unchecked. Renders an empty box.
    Unchecked,
    /// Indeterminate, drawn with a horizontal stroke.
    Indeterminate,
}

impl State {
    /// Whether the box renders ANY mark (check OR indeterminate stroke).
    /// Used for catalog `Status` dispatch — nova styles `data-state=indeterminate`
    /// and `data-state=checked` identically (both show the primary surface).
    pub fn is_marked(self) -> bool {
        matches!(self, State::Checked | State::Indeterminate)
    }
}

impl From<bool> for State {
    fn from(b: bool) -> Self {
        if b { State::Checked } else { State::Unchecked }
    }
}

/// The possible status of a [`Checkbox`].
///
/// **Deviation from upstream sweeten/iced**: `Status` is re-exported from
/// `icedcn_core` so the whole crate (theme + widget + wrapper) speaks one
/// canonical type. Variants match upstream 1:1 so the rest of this file's
/// pattern matches keep working.
pub use icedcn_core::theme::checkbox::Status;

trait StatusExt {
    fn with_checked(self, is_checked: bool) -> Self;
}

impl StatusExt for Status {
    fn with_checked(self, is_checked: bool) -> Self {
        match self {
            Status::Active { .. } => Status::Active { is_checked },
            Status::Hovered { .. } => Status::Hovered { is_checked },
            Status::Disabled { .. } => Status::Disabled { is_checked },
        }
    }
}

/// A ring of `width` pixels painted behind the checkbox.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ring {
    /// Halo color (already alpha-resolved by the catalog).
    pub color: Color,
    /// Halo thickness in pixels.
    pub width: f32,
}

/// The style of a checkbox.
#[derive(Debug, Clone, PartialEq)]
pub struct Style {
    /// The [`Background`] of the checkbox.
    pub background: Background,
    /// The icon [`Color`] of the checkbox.
    pub icon_color: Color,
    /// The [`Border`] of the checkbox.
    pub border: Border,
    /// The text [`Color`] of the checkbox.
    pub text_color: Option<Color>,
    /// Optional ring painted around the checkbox.
    pub ring: Option<Ring>,
}

/// The theme catalog of a [`Checkbox`].
pub trait Catalog: Sized {
    /// The item class of the [`Catalog`].
    type Class<'a>;

    /// The default class produced by the [`Catalog`].
    fn default<'a>() -> Self::Class<'a>;

    /// The [`Style`] of a class with the given status.
    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;

    /// Add keyboard focus styling. Existing invalid rings take precedence.
    fn focused(&self, mut style: Style) -> Style {
        if style.ring.is_none() {
            let color = self.selection();
            style.border.color = color;
            style.ring = Some(Ring {
                color: color.scale_alpha(0.5),
                width: 3.0,
            });
        }
        style
    }

    /// Restore the control edge when its Field card paints the focus ring.
    fn delegated(&self, mut style: Style, _is_checked: bool) -> Style {
        style.ring = None;
        style
    }

    /// The global selection [`Color`].
    fn selection(&self) -> Color;
}

/// A styling function for a [`Checkbox`].
///
/// This is just a boxed closure: `Fn(&Theme, Status) -> Style`.
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

/// Convert from iced's stock `checkbox::Style` into the vendored widget's
/// `Style`. Lossless in this direction (iced has no `ring` field, so we set
/// `ring: None`). Used by `Catalog for iced_widget::Theme` to satisfy callers
/// that pick stock iced::Theme instead of `icedcn_core::Theme`.
impl From<iced_widget::checkbox::Style> for Style {
    fn from(s: iced_widget::checkbox::Style) -> Self {
        Style {
            background: s.background,
            icon_color: s.icon_color,
            border: s.border,
            text_color: s.text_color,
            ring: None,
        }
    }
}

/// Stock-iced fallback Catalog impl — for callers who pick
/// `iced_core::Theme` (== `iced_widget::Theme`) instead of
/// `icedcn_core::Theme`. Routes through iced's stock checkbox default and
/// lifts the result through `From<iced_widget::checkbox::Style> for Style`.
/// Ring stays `None`.
impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }

    fn selection(&self) -> Color {
        <Theme as iced_widget::checkbox::Catalog>::selection(self)
    }
}

/// The default style of a stock-iced themed checkbox. Lifts through iced's
/// own default checkbox resolver and converts to our `Style`.
pub fn default(theme: &Theme, status: Status) -> Style {
    let iced: iced_widget::checkbox::Style = <Theme as iced_widget::checkbox::Catalog>::style(
        theme,
        &<Theme as iced_widget::checkbox::Catalog>::default(),
        status,
    );
    iced.into()
}

#[cfg(test)]
mod tests {
    use super::{Checkbox, Internal};
    use iced_core::shell::{Bus, Waker};
    use iced_core::time::{Duration, Instant};
    use iced_core::widget::Tree;
    use iced_core::{Event, Layout, Point, Rectangle, Shell, Size, Widget, mouse, window};
    use icedcn_core::Theme;

    fn step(checkbox: &mut Checkbox<'_, (), Theme, ()>, tree: &mut Tree, event: Event) {
        let mut messages = Bus::new();
        let mut shell = Shell::new(&window::Headless, Waker::noop(), &mut messages);
        checkbox.update(
            tree,
            &event,
            Layout::new(Size::new(16.0, 16.0)),
            mouse::Cursor::Available(Point::new(8.0, 8.0)),
            &(),
            &mut shell,
            &Rectangle::with_size(Size::new(100.0, 100.0)),
        );
    }

    #[test]
    fn color_transition_starts_after_idle_instead_of_using_old_frame_time() {
        let mut checkbox = Checkbox::new(false).uncontrolled(false);
        let mut tree = Tree::new(&checkbox);
        let idle = Instant::now() - Duration::from_secs(10);
        step(
            &mut checkbox,
            &mut tree,
            Event::Window(window::Event::RedrawRequested(idle)),
        );
        let input = Instant::now();
        step(
            &mut checkbox,
            &mut tree,
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)),
        );
        step(
            &mut checkbox,
            &mut tree,
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)),
        );
        let state = tree.state.downcast_ref::<Internal<()>>();
        let start = state.now.expect("input starts a transition");
        assert!(start >= input);
        assert!(state.animation.is_animating(start));
        assert!(state.animation.interpolate(0.0, 1.0, start) < 0.001);
        let halfway = start + Duration::from_millis(75);
        assert!((state.animation.interpolate(0.0, 1.0, halfway) - 0.7756_f32).abs() < 0.01);
        assert!(
            !state
                .animation
                .is_animating(start + Duration::from_millis(151))
        );
    }

    #[test]
    fn controlled_rebuild_reverses_color_transition_without_a_jump() {
        let mut checkbox = Checkbox::new(false);
        let mut tree = Tree::new(&checkbox);
        let start = Instant::now();
        checkbox = Checkbox::new(true);
        tree.diff(&mut checkbox);
        step(
            &mut checkbox,
            &mut tree,
            Event::Window(window::Event::RedrawRequested(start)),
        );
        let halfway = start + Duration::from_millis(75);
        let before = tree
            .state
            .downcast_ref::<Internal<()>>()
            .animation
            .interpolate(0.0, 1.0, halfway);
        checkbox = Checkbox::new(false);
        tree.diff(&mut checkbox);
        step(
            &mut checkbox,
            &mut tree,
            Event::Window(window::Event::RedrawRequested(halfway)),
        );
        let state = tree.state.downcast_ref::<Internal<()>>();
        assert!((state.animation.interpolate(0.0, 1.0, halfway) - before).abs() < 0.001);
        assert!(!state.animation.value());
        assert!(
            state
                .animation
                .interpolate(0.0, 1.0, halfway + Duration::from_millis(151))
                < 0.001
        );
    }
}
