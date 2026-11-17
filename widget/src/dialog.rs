//! A modal panel displayed over a backdrop. Two closures receive a [`Handle`]. Use the first to
//! place the trigger and the second to build the panel and its close controls.
//!
//! ```ignore
//! use icedcn::widget::{button, column, dialog, field, input};
//!
//! dialog(
//!     |d| d.trigger(button("Open Dialog").style(button::Outline)),
//!     |_| {
//!         column![
//!             dialog::header("Edit profile", "Make changes to your profile here."),
//!             field(input("").default_value("Pedro Duarte")).label("Name"),
//!         ]
//!         .spacing(16)
//!         .into()
//!     },
//! )
//! .footer(|d| [
//!     d.close(button("Cancel").style(button::Outline)),
//!     button("Save changes").on_press(Message::Save).into(),
//! ])
//! // Controlled: .open(self.editing).on_open_change(Message::Editing)
//! ```
//!
//! Click outside the panel, press Escape, or activate a close control to dismiss it. Each
//! opening creates a fresh panel widget tree. Both closures use the application’s message type.

pub mod modal;

use std::rc::Rc;

use iced_core::Element;
use iced_core::Length::{Fill, Fit, Shrink};
use iced_core::alignment::Horizontal;
use iced_core::time::Instant;
use iced_widget::component::{self, Component};
use iced_widget::{Renderer, Widget};
use icedcn_core::Theme;
use icedcn_core::border;
use icedcn_core::theme;
use icedcn_core::theme::container::Style;
use icedcn_core::theme::{alert_dialog, dialog};

use crate::button::{self, button};
use crate::compose::{Compose, Inherited, Slot};
use crate::container::Container;
use crate::marker::{self, Press};
use crate::sidebar::viewport::Width;
use crate::{Size, column, row, scrollable, separator, text};

pub use modal::{Placement, Side, Signal};

/// A modal panel opened by its triggers.
pub struct Dialog<'a, Message> {
    trigger: Body<'a, Message>,
    content: Body<'a, Message>,
    scrollable_body: Option<Body<'a, Message>>,
    footer: Option<Footer<'a, Message>>,
    footer_alignment: Horizontal,
    open: Option<bool>,
    default_open: bool,
    on_open_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    placement: Placement,
    drawer_height: Option<f32>,
    pub(crate) sheet_height: f32,
    responsive: Option<f32>,
    compact: Option<(f32, f32)>,
    show_close_button: bool,
    dismiss_outside: bool,
    padding: u16,
    style: fn(&Theme) -> Style,
    margin: f32,
    top: Option<f32>,
    backdrop: bool,
    inherited: Inherited,
}

/// A closure that builds part of a [`Dialog`].
type Body<'a, Message> = Rc<dyn Fn(&Handle) -> Slot<'a, Message> + 'a>;
type Footer<'a, Message> = Rc<dyn Fn(&Handle, &Inherited) -> Slot<'a, Message> + 'a>;

/// What a [`Dialog`]'s closures receive: whether it is open, and the
/// wrappers for its triggers and close buttons.
#[derive(Debug, Clone, Copy)]
pub struct Handle {
    is_open: bool,
    mobile: bool,
}

/// The dialog's private events.
#[derive(Debug, Clone)]
pub enum Event<Message> {
    /// A trigger was pressed.
    Open,
    /// The panel was dismissed.
    Close,
    /// The closing animation reached its endpoint.
    Exited,
    /// The window crossed the breakpoint for a responsive dialog.
    Mobile(bool),
    /// The content published an app message.
    App(Message),
}

/// The open state of an uncontrolled dialog.
#[derive(Debug, Default)]
pub struct State {
    open: Option<bool>,
    shown: bool,
    mobile: bool,
    closing: Option<Instant>,
}

/// Create a closed, uncontrolled [`Dialog`]: `trigger` builds what sits in
/// place, `content` the panel shown while open.
pub fn dialog<'a, Message>(
    trigger: impl Fn(&Handle) -> Slot<'a, Message> + 'a,
    content: impl Fn(&Handle) -> Slot<'a, Message> + 'a,
) -> Dialog<'a, Message> {
    Dialog::new(trigger, content, Placement::Center { max_width: 384.0 })
}

impl<'a, Message> Dialog<'a, Message> {
    pub(crate) fn new(
        trigger: impl Fn(&Handle) -> Slot<'a, Message> + 'a,
        content: impl Fn(&Handle) -> Slot<'a, Message> + 'a,
        placement: Placement,
    ) -> Self {
        Self {
            trigger: Rc::new(trigger),
            content: Rc::new(content),
            scrollable_body: None,
            footer: None,
            footer_alignment: Horizontal::Right,
            open: None,
            default_open: false,
            on_open_change: None,
            placement,
            drawer_height: None,
            sheet_height: 1.,
            responsive: None,
            compact: None,
            show_close_button: true,
            dismiss_outside: true,
            padding: 16,
            style: dialog::content,
            margin: 16.,
            top: None,
            backdrop: true,
            inherited: Inherited::default(),
        }
    }

    /// Control the dialog: it is open exactly when `open` is, and its
    /// triggers and close buttons only publish
    /// [`on_open_change`](Self::on_open_change).
    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    /// Whether an uncontrolled dialog starts open.
    pub fn default_open(mut self, open: bool) -> Self {
        self.default_open = open;
        self
    }

    /// Set the message published with the new open state when the dialog
    /// is opened or dismissed.
    pub fn on_open_change(mut self, on_change: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_open_change = Some(Box::new(on_change));
        self
    }

    /// Show or hide the panel’s corner close button. It is visible by default.
    pub fn show_close_button(mut self, show: bool) -> Self {
        self.show_close_button = show;
        self
    }

    /// Set the maximum width of a centered panel, 384 pixels by default. Sheets use the extent
    /// of their window edge.
    pub fn max_width(mut self, max_width: f32) -> Self {
        if let Placement::Center { .. } = self.placement {
            self.placement = Placement::Center { max_width };
        }
        self
    }

    /// Place scrolling content below the header, capped at half the window height.
    pub fn scrollable_body(mut self, body: impl Fn(&Handle) -> Slot<'a, Message> + 'a) -> Self {
        self.scrollable_body = Some(Rc::new(body));
        self
    }

    /// Place actions in a footer that stays visible while the body scrolls.
    /// Below 640 pixels, actions fill the panel in reverse visual order.
    pub fn footer<I>(mut self, actions: impl Fn(&Handle) -> I + 'a) -> Self
    where
        I: IntoIterator<Item = Slot<'a, Message>>,
        Message: 'a,
    {
        self.footer = Some(Rc::new(move |handle, inherited| {
            let actions = actions(handle);
            if handle.mobile {
                let cells = actions.into_iter().map(|mut action| {
                    action.inherit(inherited);
                    iced_widget::grid([action.compose()])
                        .columns(1)
                        .height(Shrink)
                        .boxed()
                        .into()
                });
                column(cells).width(Fill).reverse().spacing(8).into()
            } else {
                row::Row::with_children(actions).spacing(8).into()
            }
        }));
        self
    }

    /// Align footer actions horizontally on desktop. The default is right.
    pub fn footer_align_x(mut self, alignment: Horizontal) -> Self {
        self.footer_alignment = alignment;
        self
    }

    /// Keep the panel open on a press outside it (an alert dialog).
    pub(crate) fn dismiss_outside(mut self, dismiss: bool) -> Self {
        self.dismiss_outside = dismiss;
        self
    }

    /// Position a dialog's top edge at a fraction of the viewport height.
    /// The panel is clamped inside the window margins; the default is centered.
    pub fn top(mut self, fraction: f32) -> Self {
        self.top = Some(fraction.clamp(0.0, 1.0));
        self
    }

    /// Set the centered panel's padding (`p-4` by default; an alert
    /// dialog's sections pad themselves).
    pub fn padding(mut self, padding: u16) -> Self {
        self.padding = padding;
        self
    }

    /// Paint the centered dialog panel.
    pub fn style(mut self, style: fn(&Theme) -> Style) -> Self {
        self.style = style;
        self
    }

    /// Minimum clearance between a centered panel and the window edge.
    pub fn margin(mut self, margin: f32) -> Self {
        self.margin = margin.max(0.);
        self
    }

    /// Dim the background while open. Input remains modal in either case.
    pub fn backdrop(mut self, visible: bool) -> Self {
        self.backdrop = visible;
        self
    }

    pub(crate) fn drawer(mut self, fraction: f32) -> Self {
        self.drawer_height = Some(fraction.clamp(0.1, 1.0));
        self
    }
    /// Limit a top or bottom drawer to this fraction of the window height.
    pub fn height_fraction(mut self, fraction: f32) -> Self {
        if self.drawer_height.is_some() {
            self.drawer_height = Some(fraction.clamp(0.1, 1.0));
        }
        self
    }
    /// Render a centered dialog at desktop width and this drawer below 768px.
    /// Both presentations share the same controlled or uncontrolled open state.
    pub fn desktop_dialog(mut self, max_width: f32) -> Self {
        self.responsive = Some(max_width);
        self
    }
    /// Pin the panel to the selected window edge.
    pub fn side(mut self, side: Side) -> Self {
        self.placement = Placement::Side(side);
        self
    }

    /// Use a narrower centered panel below a window breakpoint.
    pub(crate) fn compact(mut self, max_width: f32, breakpoint: f32) -> Self {
        self.compact = Some((max_width, breakpoint));
        self
    }

    fn is_open(&self, state: &State) -> bool {
        self.open.or(state.open).unwrap_or(self.default_open)
    }
}

impl Handle {
    /// Whether a responsive drawer is using its narrow-screen presentation.
    pub fn is_mobile(&self) -> bool {
        self.mobile
    }
    /// Whether the dialog is open.
    pub fn is_open(&self) -> bool {
        self.is_open
    }

    /// Pressing the child opens the dialog and runs the child’s own handlers.
    pub fn trigger<'a, Message: 'a>(
        &self,
        child: impl Into<Slot<'a, Message>>,
    ) -> Slot<'a, Message> {
        Slot::new(Marked {
            child: child.into(),
            signal: Signal::Open,
        })
    }

    /// Pressing the child closes the dialog and runs the child’s own handlers.
    pub fn close<'a, Message: 'a>(&self, child: impl Into<Slot<'a, Message>>) -> Slot<'a, Message> {
        Slot::new(Marked {
            child: child.into(),
            signal: Signal::Close,
        })
    }
}

/// A `DialogHeader`: a `DialogTitle` over a `DialogDescription`.
pub fn header<'a, Message: 'a>(title: &'a str, description: &'a str) -> Slot<'a, Message> {
    column![
        text::Text::new(title)
            .size(text::size::BASE)
            .line_height(text::leading::NONE)
            .medium()
            .style(theme::text::Foreground),
        text::muted(description),
    ]
    // Native heading metrics extend below its 16px CSS line box.
    .spacing(4)
    .into()
}

/// A `DialogFooter`: its actions in a row, aligned to the end.
pub fn footer<'a, Message: 'a>(
    actions: impl IntoIterator<Item = Slot<'a, Message>>,
) -> Slot<'a, Message> {
    Container::new(row::Row::with_children(actions).spacing(8))
        .width(Fill)
        .align_x(Horizontal::Right)
        .into()
}

#[cfg(feature = "svg")]
fn close_icon<'a, Message: 'a>() -> Slot<'a, Message> {
    use iced_widget::svg::{self, Handle};
    use std::sync::LazyLock;
    const X: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"
        stroke="black" stroke-width="2" stroke-linecap="round"><path d="M18 6 6 18M6 6l12 12"/></svg>"#;
    static ICON: LazyLock<Handle> = LazyLock::new(|| Handle::from_memory(X));
    iced_widget::svg(ICON.clone())
        .width(16)
        .height(16)
        .style(|theme: &Theme, _| svg::Style {
            color: Some(theme.popover.foreground.into()),
        })
        .boxed()
        .into()
}

#[cfg(not(feature = "svg"))]
fn close_icon<'a, Message: 'a>() -> Slot<'a, Message> {
    text::Text::new("×").size(text::size::BASE).into()
}

/// A child that records a [`Signal`] when pressed.
pub(crate) struct Marked<'a, Message> {
    pub(crate) child: Slot<'a, Message>,
    pub(crate) signal: Signal,
}

impl<'a, Message: 'a> Compose<Message> for Marked<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.child.inherit(from);
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        marker::marker(self.child.compose(), self.signal, Press::Captured).into()
    }
}

impl State {
    fn transition(&mut self, open: bool) {
        if self.shown != open {
            self.shown = open;
            self.closing = (!open).then(Instant::now);
        }
    }
}
impl<'a, Message: Clone + 'static> Component<'a, Message, Theme, Renderer> for Dialog<'a, Message> {
    type State = State;
    type Event = Event<Message>;

    fn diff(&mut self, state: &mut State) {
        if self.open.is_none() && state.open.is_none() {
            state.open = Some(self.default_open);
        }
        state.transition(self.is_open(state));
    }

    fn update(
        &self,
        state: &mut State,
        event: Event<Message>,
        _renderer: &Renderer,
    ) -> Option<Message> {
        let next = match event {
            Event::App(message) => return Some(message),
            Event::Mobile(mobile) => {
                state.mobile = mobile;
                return None;
            }
            Event::Exited => {
                if !self.is_open(state) {
                    state.closing = None;
                }
                return None;
            }
            Event::Open => true,
            Event::Close => false,
        };

        if next == self.is_open(state) {
            return None;
        }

        if self.open.is_none() {
            state.open = Some(next);
            state.transition(next);
        }

        self.on_open_change
            .as_ref()
            .map(|on_change| on_change(next))
    }

    fn view(&self, state: &State) -> impl Widget<Event<Message>, Theme, Renderer> + 'a {
        let desktop = self.responsive.filter(|_| !state.mobile);
        let compact = self
            .compact
            .filter(|_| state.mobile)
            .map(|(width, _)| width);
        let placement = compact
            .or(desktop)
            .map_or(self.placement, |max_width| Placement::Center { max_width });
        let drawer_height = self.drawer_height.filter(|_| desktop.is_none());
        let handle = Handle {
            is_open: self.is_open(state),
            mobile: state.mobile,
        };
        let mut base = (self.trigger)(&handle);
        base.inherit(&self.inherited);
        let base = marker::owner(
            base.compose().map(Event::App).boxed(),
            |signal| match signal {
                Signal::Open => Event::Open,
                Signal::Close => Event::Close,
            },
        );

        let panel = (handle.is_open || state.closing.is_some()).then(|| {
            let padding = match placement {
                Placement::Center { .. } => self.padding,
                Placement::Side(_) => 0,
            };
            let inherited = self.inherited;
            let body = Rc::clone(&self.content);
            let scrolling = self.scrollable_body.clone();
            let footer = self.footer.clone();
            let alignment = self.footer_alignment;
            let sections = move |window_height: f32| {
                let mut body = body(&handle);
                if let Some(scrolling) = &scrolling {
                    let scroll = scrollable(scrolling(&handle))
                        .width(Fill)
                        .height(Shrink.max(window_height * 0.5))
                        .class(theme::scrollable::Hidden);
                    body = column![body, scroll].width(Fill).spacing(16).into();
                }
                let body_height = if matches!(placement, Placement::Side(Side::Left | Side::Right))
                    && drawer_height.is_none()
                {
                    Fit
                } else {
                    Shrink
                };
                body = Container::new(body)
                    .width(Fill)
                    .height(body_height)
                    .padding(padding)
                    .into();
                if let Some(footer) = &footer {
                    let band = Container::new(footer(&handle, &inherited))
                        .width(Fill)
                        .align_x(alignment)
                        .padding(16)
                        .style(alert_dialog::footer);
                    body = column![body, separator(), band].width(Fill).into();
                }
                body.inherit(&inherited);
                body
            };
            let content = if self.scrollable_body.is_some() {
                // Modal layout gives the panel the window height less its two margins.
                let margin = self.margin;
                iced_widget::responsive(move |size| sections(size.height + 2. * margin).compose())
                    .height(Shrink)
                    .boxed()
                    .into()
            } else {
                sections(0.)
            };

            let content: Slot<'a, Message> = if self.show_close_button || desktop.is_some() {
                // nova `absolute top-2 right-2` (sheet: `top-3 right-3`).
                let inset = match placement {
                    Placement::Center { .. } => 8,
                    Placement::Side(_) => 12,
                };
                let close = handle.close(
                    button(close_icon())
                        .size(Size::Sm)
                        .square()
                        .style(button::Ghost),
                );

                Slot::from(
                    iced_widget::stack![
                        content.compose(),
                        iced_widget::container(close.compose())
                            .width(Fill)
                            .align_right(Fill)
                            .padding(inset),
                    ]
                    .boxed(),
                )
            } else {
                content
            };

            let content: Slot<'a, Message> =
                if drawer_height.is_some() && placement == Placement::Side(Side::Bottom) {
                    let handle = Container::new(iced_widget::Space::new())
                        .width(100)
                        .height(4)
                        .style(|theme: &Theme| Style {
                            background: Some(theme.muted.base.into()),
                            border: border::rounded(2),
                            ..Style::default()
                        });
                    column![
                        Container::new(handle).center_x(Fill).padding([16, 0, 0, 0]),
                        content
                    ]
                    .into()
                } else {
                    content
                };
            let style = move |theme: &Theme| {
                let mut style = dialog::sheet(theme);
                if drawer_height.is_some() {
                    style.border.width = 1.0;
                    style.border.color = theme.neutral.border;
                    let [top_left, top_right, bottom_right, bottom_left] = match placement {
                        Placement::Side(Side::Bottom) => [12., 12., 0., 0.],
                        Placement::Side(Side::Top) => [0., 0., 12., 12.],
                        Placement::Side(Side::Right) => [12., 0., 0., 12.],
                        Placement::Side(Side::Left) => [0., 12., 12., 0.],
                        Placement::Center { .. } => [12.; 4],
                    };
                    style.border.radius = icedcn_core::Radius {
                        top_left,
                        top_right,
                        bottom_right,
                        bottom_left,
                    };
                }
                style
            };
            let panel = match placement {
                Placement::Center { .. } => Container::new(content).width(Fill).style(self.style),
                Placement::Side(side) if drawer_height.is_none() => {
                    crate::sheet::panel(content, side)
                }
                // A left or right sheet spans the window's height; a top or
                // bottom one takes its content's (`h-auto`).
                Placement::Side(Side::Left | Side::Right) => Container::new(content)
                    .width(Fill)
                    .height(Fill)
                    .style(style),
                Placement::Side(Side::Top | Side::Bottom) => Container::new(content)
                    .width(Fill)
                    .height(Shrink)
                    .style(style),
            };

            Slot::from(panel).compose().map(Event::App).boxed()
        });

        let modal = modal::Modal::new(base.into(), panel, placement, Event::Close)
            .dismiss_outside(self.dismiss_outside)
            .margin(self.margin)
            .top(self.top)
            .backdrop(self.backdrop)
            .animation(state.closing, || Event::Exited)
            .side_height(drawer_height, self.sheet_height);
        if self.responsive.is_some() || self.compact.is_some() || self.footer.is_some() {
            Width {
                content: modal.boxed(),
                mobile: state.mobile,
                enabled: true,
                breakpoint: self.compact.map_or(
                    if self.responsive.is_some() {
                        768.
                    } else {
                        640.
                    },
                    |(_, breakpoint)| breakpoint,
                ),
                change: Event::Mobile,
            }
            .boxed()
        } else {
            modal.boxed()
        }
    }
}

impl<'a, Message: Clone + 'static> Compose<Message> for Dialog<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.inherited = self.inherited.or(*from);
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        component::component(*self).boxed()
    }
}

impl<'a, Message: Clone + 'static> From<Dialog<'a, Message>> for Slot<'a, Message> {
    fn from(dialog: Dialog<'a, Message>) -> Self {
        Slot::new(dialog)
    }
}

impl<'a, Message: Clone + 'static> From<Dialog<'a, Message>>
    for Element<'a, Message, Theme, Renderer>
{
    fn from(dialog: Dialog<'a, Message>) -> Self {
        component::component(dialog).boxed()
    }
}
