//! Editable content: iced's text editor laid out and painted like the listing.
//!
//! The editor keeps its [`Content`] in the tree, so it can be uncontrolled, and
//! reuses the textarea's [`sync`] so a stale controlled value never replaces an
//! edit. Syntax colors come from the listing's own tokens through a
//! [`Parser`](text::Parser) that serves one tokenized line at a time.
use std::collections::BTreeMap;
use std::iter;
use std::ops::Range;
use std::sync::Arc;

use iced_widget::Renderer;
use iced_widget::text_editor::{
    Action, Binding, Content, Cursor, Edit, KeyPress, Status, Style, TextEditor,
};
use icedcn_core::Theme;

use super::lines::{self, Gutter};
use super::listing;
use super::{LineStyle, line_style};
use crate::core::alignment::Vertical;
use crate::core::keyboard;
use crate::core::keyboard::Key;
use crate::core::keyboard::key::Named;
use crate::core::layout;
use crate::core::mouse;
use crate::core::renderer;
use crate::core::shell::Bus;
use crate::core::text::{self, Renderer as _, highlighter};
use crate::core::widget::tree::{self, Tree};
use crate::core::widget::{Id, Meta, Operation};
use crate::core::{
    Background, Code, Color, Event, InputMethod, Layout, Length, Point, Rectangle, Renderer as _,
    Shell, Size, Vector, Widget, clipboard, overlay, window,
};
use crate::edit::{self, Command, Intent, Platform};
use crate::operation::control::Editable;
use crate::operation::focus;
use crate::textarea::sync;

pub(super) struct Editor<'a, Message> {
    value: Option<String>,
    default_value: String,
    on_input: Option<Box<dyn Fn(String) -> Message + 'a>>,
    language: Option<String>,
    styles: BTreeMap<usize, LineStyle>,
    listing: listing::Layout,
    id: Option<Id>,
    disabled: bool,
    /// Fill the viewport below the last line.
    fill: bool,
}

impl<'a, Message> Editor<'a, Message> {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn new(
        value: Option<String>,
        default_value: String,
        on_input: Option<Box<dyn Fn(String) -> Message + 'a>>,
        language: Option<String>,
        styles: BTreeMap<usize, LineStyle>,
        listing: listing::Layout,
        id: Option<Id>,
        disabled: bool,
        fill: bool,
    ) -> Self {
        Self {
            value,
            default_value,
            on_input,
            language,
            styles,
            listing,
            id,
            disabled,
            fill,
        }
    }

    fn padding(&self) -> f32 {
        if self.listing.compact { 10. } else { 12. }
    }

    /// The width of the line numbers, as the listing sizes them.
    fn gutter(&self, lines: usize) -> Option<f32> {
        let first = self.listing.start_line?;
        let last = first.saturating_add(lines.saturating_sub(1));
        Some((last.to_string().len() as f32 * self.listing.size() * 0.61).max(24.))
    }

    fn editor<'b>(&'b self, state: &'b State) -> TextEditor<'b, Tokens, Action, Theme, Renderer> {
        let editor = TextEditor::new(&state.content)
            .font(self.listing.font)
            .size(self.listing.size())
            .line_height(text::LineHeight::Absolute(
                self.listing.line_height().into(),
            ))
            .wrapping(self.listing.wrapping())
            .padding(0)
            .height(Length::Shrink)
            .key_binding(move |press| binding(press, &state.content, state.overwrite))
            .caret_width(
                state
                    .block
                    .filter(|_| state.overwrite)
                    .map(|(_, width)| width),
            )
            .highlight_with::<Tokens>(
                Settings {
                    revision: state.revision,
                    rows: Arc::clone(&state.rows),
                },
                highlight,
            );
        let editor = if self.listing.wrap {
            editor.width(state.width)
        } else {
            editor
        };
        let editor = match &self.id {
            Some(id) => editor.id(id.clone()),
            None => editor,
        };
        if self.disabled {
            editor
        } else {
            editor.on_action(|action| action)
        }
    }

    /// The caret's line, padded on every side, in the editor's own coordinates.
    fn caret(&self, state: &State, renderer: &Renderer, left: f32) -> Rectangle {
        let position = state.content.cursor().position;
        let line_height = self.listing.line_height();
        let top: f32 = state.heights.iter().take(position.line).sum();
        let height = state
            .heights
            .get(position.line)
            .copied()
            .unwrap_or(line_height);
        let x = if self.listing.wrap {
            0.
        } else {
            let text = state
                .content
                .line(position.line)
                .map_or(String::new(), |line| {
                    line.text[..line.text.floor_char_boundary(position.index)].to_owned()
                });
            // A trailing space adds no width to a measured line, so measure up
            // to a sentinel and take the sentinel away.
            let width = |content: &str| {
                crate::text::min_bounds::<Renderer>(text::Text {
                    content,
                    bounds: Size::INFINITE,
                    size: self.listing.size().into(),
                    line_height: text::LineHeight::Absolute(line_height.into()),
                    font: self.listing.font,
                    align_x: text::Alignment::Default,
                    align_y: Vertical::Top,
                    shaping: text::Shaping::Advanced,
                    wrapping: text::Wrapping::None,
                    ellipsis: text::Ellipsis::None,
                    letter_spacing: Default::default(),
                    font_features: Vec::new(),
                    font_variations: Vec::new(),
                    weight: None,
                    hint_factor: renderer::Renderer::hint_factor(renderer),
                })
                .width
            };
            width(&format!("{text}x")) - width("x")
        };
        let padding = self.padding();
        Rectangle {
            x: left + x - padding,
            y: top - padding,
            width: padding * 2.,
            height: height + padding * 2.,
        }
    }

    fn retokenize(&self, state: &mut State) {
        state.rows = tokens(&state.content.text(), self.language.as_deref());
        state.revision = state.revision.wrapping_add(1);
        state.measured = None;
    }

    /// Whether edits change the text: enabled, and uncontrolled or listening.
    fn is_editable(&self) -> bool {
        !self.disabled && (self.value.is_none() || self.on_input.is_some())
    }

    fn available(&self, state: &State) -> edit::Available {
        let selected = state.content.selection().is_some();

        edit::Available {
            undo: Some(self.is_editable()),
            redo: None,
            cut: selected && self.is_editable(),
            copy: selected,
            paste: self.is_editable(),
            select_all: !state.content.is_empty(),
        }
    }

    /// Run `command`: the clipboard through the shell, and the edit it makes
    /// as the editor's own action.
    fn command(
        &self,
        state: &State,
        command: Command,
        shell: &mut Shell<'_, Message>,
    ) -> Option<Action> {
        let editable = self.is_editable();

        match command {
            Command::Copy | Command::Cut => {
                let text = state.content.selection()?;
                if command == Command::Cut && !editable {
                    return None;
                }
                shell.write_clipboard(clipboard::Content::Text(text));
                (command == Command::Cut).then_some(Action::Edit(Edit::Backspace))
            }
            // The focused editor pastes the text it reads.
            Command::Paste => {
                if editable {
                    shell.read_clipboard(clipboard::Kind::Text);
                }
                None
            }
            Command::SelectAll => Some(Action::SelectAll),
            Command::Undo => editable.then_some(Action::Edit(Edit::Undo)),
            Command::Redo => editable.then_some(Action::Edit(Edit::Redo)),
        }
    }

    /// Restart the caret's blink after a key press the widget ran itself.
    fn blink(&self, tree: &mut Tree, layout: Layout, renderer: &Renderer, viewport: &Rectangle) {
        let Tree {
            state, children, ..
        } = tree;
        let state = state.downcast_mut::<State>();
        let (child_layout, child_tree) = layout
            .iter_mut(children)
            .next()
            .expect("the editor is the only child");
        self.editor(state).operate(
            child_tree,
            child_layout,
            viewport,
            renderer,
            &mut edit::Blink,
        );
    }

    /// Perform the editor's actions on the content and publish an edit.
    fn perform(
        &self,
        state: &mut State,
        actions: impl IntoIterator<Item = Action>,
        shell: &mut Shell<'_, Message>,
    ) {
        let mut edited = false;
        for action in actions {
            if action.is_edit() && self.value.is_some() && self.on_input.is_none() {
                continue;
            }
            // An undo or redo with nothing to restore changes nothing.
            let before = matches!(action, Action::Edit(Edit::Undo | Edit::Redo))
                .then(|| state.content.text());
            let is_edit = action.is_edit();
            if !matches!(action, Action::Scroll { .. }) && !state.moved {
                state.moved = true;
                shell.invalidate_layout();
            }
            match action {
                Action::Edit(Edit::Indent) => indent(&mut state.content, false),
                Action::Edit(Edit::Unindent) => indent(&mut state.content, true),
                action => state.content.perform(action),
            }
            edited |= is_edit && before.is_none_or(|before| before != state.content.text());
        }
        if edited {
            self.retokenize(state);
            if let Some(on_input) = &self.on_input {
                shell.publish(on_input(state.content.text()));
            }
            shell.invalidate_layout();
            shell.request_redraw();
        }
    }
}

/// Tab indents and Shift+Tab outdents while editing; Enter keeps the indentation
/// before the caret. Other keys keep the editor's own bindings.
fn binding(
    press: KeyPress,
    content: &Content<Renderer>,
    overwrite: bool,
) -> Option<Binding<Action>> {
    if !press.is_focused {
        return None;
    }
    match press.key.as_ref() {
        Key::Named(Named::Tab)
            if press.modifiers.control() || press.modifiers.alt() || press.modifiers.logo() =>
        {
            None
        }
        Key::Named(Named::Tab) if press.modifiers.shift() => {
            Some(Binding::Custom(Action::Edit(Edit::Unindent)))
        }
        Key::Named(Named::Tab) => Some(Binding::Custom(Action::Edit(Edit::Indent))),
        Key::Named(Named::Enter) if edit::binding::<Action>(press.clone()).is_none() => None,
        Key::Named(Named::Enter) => {
            let cursor = content.cursor().position;
            let indent: String = content
                .line(cursor.line)
                .map(|line| {
                    line.text[..line.text.floor_char_boundary(cursor.index)]
                        .chars()
                        .take_while(|c| *c == ' ' || *c == '\t')
                        .collect()
                })
                .unwrap_or_default();
            Some(Binding::Sequence(
                iter::once(Binding::Enter)
                    .chain(indent.chars().map(Binding::Insert))
                    .collect(),
            ))
        }
        _ => edit::typing(press, overwrite.then_some(content)),
    }
}

/// The width of one indentation level, in spaces.
const INDENT: usize = 4;

/// Indent at the caret, or shift the selected lines one level either way.
/// The editor's own indent uses its tab width of eight.
fn indent(content: &mut Content<Renderer>, outdent: bool) {
    let cursor = content.cursor();
    let caret = cursor.position;
    if !outdent && cursor.selection.is_none() {
        let column = content.line(caret.line).map_or(0, |line| {
            line.text[..line.text.floor_char_boundary(caret.index)]
                .chars()
                .count()
        });
        for _ in 0..INDENT - column % INDENT {
            content.perform(Action::Edit(Edit::Insert(' ')));
        }
        return;
    }
    let other = cursor
        .selection
        .map_or(caret.line, |position| position.line);
    let lines = caret.line.min(other)..=caret.line.max(other);
    let mut shifts = Vec::new();
    let mut source = String::new();
    for (index, line) in content.lines().enumerate() {
        let mut text = line.text.into_owned();
        let shift = if !lines.contains(&index) {
            0
        } else if outdent {
            let spaces = text.chars().take(INDENT).take_while(|c| *c == ' ').count();
            text.drain(..spaces);
            -(spaces as isize)
        } else if text.is_empty() && lines.start() != lines.end() {
            0
        } else {
            text.insert_str(0, &" ".repeat(INDENT));
            INDENT as isize
        };
        shifts.push(shift);
        source.push_str(&text);
        source.push_str(line.ending.as_str());
    }
    if shifts.iter().all(|shift| *shift == 0) {
        return;
    }
    *content = Content::with_text(&source);
    let mut cursor = cursor;
    for position in iter::once(&mut cursor.position).chain(cursor.selection.iter_mut()) {
        let shift = shifts.get(position.line).copied().unwrap_or(0);
        position.index = position.index.saturating_add_signed(shift);
    }
    content.move_to(cursor);
}

fn highlight(code: Code, theme: &Theme) -> highlighter::Style {
    #[cfg(feature = "highlighter")]
    {
        highlighter::Style {
            color: Some(super::syntax::color(code, theme.neutral.is_dark).into()),
            style: None,
        }
    }
    #[cfg(not(feature = "highlighter"))]
    {
        let _ = (code, theme);
        highlighter::Style::default()
    }
}

/// One source line and its tokens, as byte ranges into the line.
pub(super) struct Row {
    text: String,
    spans: Vec<(Range<usize>, Code)>,
}

fn tokens(source: &str, language: Option<&str>) -> Arc<[Row]> {
    #[cfg(feature = "highlighter")]
    if let Some(language) = language
        && !language.eq_ignore_ascii_case("ansi")
    {
        return super::syntax::tokenize(source, language)
            .into_iter()
            .map(|line| {
                let mut text = String::new();
                let spans = line
                    .into_iter()
                    .map(|(piece, code)| {
                        let start = text.len();
                        text.push_str(&piece);
                        (start..text.len(), code)
                    })
                    .collect();
                Row { text, spans }
            })
            .collect();
    }
    let _ = (source, language);
    Arc::from([])
}

/// The tokens the editor highlights with. A new revision restarts the parser.
#[derive(Clone)]
pub(super) struct Settings {
    revision: u64,
    rows: Arc<[Row]>,
}

impl PartialEq for Settings {
    fn eq(&self, other: &Self) -> bool {
        self.revision == other.revision
    }
}

/// Serves the tokenized rows line by line. A line whose text no longer matches its
/// row (the edit is newer than the tokens) stays plain until the next revision.
pub(super) struct Tokens {
    rows: Arc<[Row]>,
    line: usize,
}

impl text::Parser for Tokens {
    type Settings = Settings;
    type Output = Code;
    type Iterator<'a> = std::vec::IntoIter<(Range<usize>, Code)>;

    fn new(settings: &Settings) -> Self {
        Self {
            rows: Arc::clone(&settings.rows),
            line: 0,
        }
    }

    fn update(&mut self, settings: &Settings) {
        self.rows = Arc::clone(&settings.rows);
        self.line = 0;
    }

    fn change_line(&mut self, line: usize) {
        // A new revision may already have rewound to the first line.
        self.line = self.line.min(line);
    }

    fn parse_line(&mut self, line: &str) -> Self::Iterator<'_> {
        let spans = match self.rows.get(self.line) {
            Some(row) if row.text == line => row.spans.clone(),
            _ => Vec::new(),
        };
        self.line += 1;
        spans.into_iter()
    }

    fn current_line(&self) -> usize {
        self.line
    }
}

pub(super) struct State {
    content: Content<Renderer>,
    /// The controlled value the content last synced to; see [`sync`].
    synced: Option<String>,
    language: Option<String>,
    rows: Arc<[Row]>,
    revision: u64,
    /// The editor's text width from the last layout.
    width: f32,
    /// Each line's height, from the last layout.
    heights: Vec<f32>,
    /// The revision and width `heights` were measured at.
    measured: Option<(u64, f32)>,
    pub(super) focused: bool,
    /// The caret moved, so the next layout reveals it.
    moved: bool,
    /// The last key pressed was Escape, so Tab moves the focus on.
    escaped: bool,
    /// What the scrollables around must show, once the caret is laid out.
    show: Option<edit::Reveal>,
    /// Typing replaces the character after the caret.
    overwrite: bool,
    /// The width of the block caret of overwrite mode, and where it was
    /// measured.
    block: Option<(text::Position, f32)>,
    reveal: Reveal,
    gutter: Gutter,
    /// Below the caret, relative to the editor: where Shift+F10 opens the menu.
    caret: Point,
    menu: edit::Menu,
    autoscroll: edit::Autoscroll,
}

/// The caret to scroll into view, once, handed to the code block's viewport
/// through [`Operation::custom`].
#[derive(Default)]
pub(super) struct Reveal(Option<Rectangle>);

impl Reveal {
    pub(super) fn take(&mut self) -> bool {
        self.0.take().is_some()
    }
}

impl<Message> Meta for Editor<'_, Message> {}

/// An editable code block running the intents of key presses on its content.
struct Editing<'a, 'b, 'c, Message> {
    editor: &'a Editor<'b, Message>,
    state: &'a mut State,
    shell: &'a mut Shell<'c, Message>,
    page: i32,
}

impl<Message> edit::Target for Editing<'_, '_, '_, Message> {
    fn reveal(&mut self, reveal: edit::Reveal) {
        // The code block shows every line: the scrollables around scroll.
        self.state.show = Some(reveal);
        self.shell.request_redraw();
    }

    fn overwrite(&mut self) {
        self.state.overwrite = !self.state.overwrite;
        self.state.block = None;
        self.shell.request_redraw();
    }

    fn content(&self) -> &Content<Renderer> {
        &self.state.content
    }

    fn editable(&self) -> bool {
        self.editor.is_editable()
    }

    fn perform(&mut self, action: Action) {
        self.editor.perform(self.state, Some(action), self.shell);
        self.shell.request_redraw();
    }

    fn move_to(&mut self, cursor: Cursor) {
        self.state.content.move_to(cursor);
        self.state.moved = true;
        self.shell.invalidate_layout();
        self.shell.request_redraw();
    }

    fn page(&self) -> i32 {
        self.page
    }
}

impl<Message> Widget<Message, Theme, Renderer> for Editor<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        let text = self.value.as_deref().unwrap_or(&self.default_value);
        tree::State::new(State {
            content: Content::with_text(text),
            synced: self.value.clone(),
            language: self.language.clone(),
            rows: tokens(text, self.language.as_deref()),
            revision: 0,
            width: 0.,
            heights: Vec::new(),
            measured: None,
            focused: false,
            moved: false,
            escaped: false,
            show: None,
            overwrite: false,
            block: None,
            reveal: Reveal::default(),
            gutter: Gutter::default(),
            caret: Point::ORIGIN,
            menu: edit::Menu::default(),
            autoscroll: edit::Autoscroll::default(),
        })
    }

    fn diff(&mut self, tree: &mut Tree) {
        let Tree {
            state, children, ..
        } = tree;
        let state = state.downcast_mut::<State>();
        let replaced = sync(&mut state.content, &mut state.synced, self.value.as_ref());
        if replaced || state.language != self.language {
            state.language.clone_from(&self.language);
            self.retokenize(state);
        }
        if self.disabled && state.focused {
            children.clear();
            state.focused = false;
        }
        if self.disabled {
            state.menu = edit::Menu::default();
        }
        let mut editor = self.editor(state);
        if children.is_empty() {
            children.push(Tree::new(&editor));
        }
        children[0].diff(&mut editor);
    }

    fn size(&self) -> Size<Length> {
        Size::new(
            Length::Fill,
            if self.fill {
                Length::Fill
            } else {
                Length::Shrink
            },
        )
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        let Tree {
            state,
            children,
            size,
            ..
        } = tree;
        let state = state.downcast_mut::<State>();
        let padding = self.padding();
        let lines = state.content.line_count().max(1);
        let left = padding + self.gutter(lines).map_or(0., |gutter| gutter + 12.);
        let width = limits.max.width;
        state.width = (width - left - padding).max(0.);
        let line_height = self.listing.line_height();
        if !self.listing.wrap {
            state.heights = vec![line_height; lines];
        } else if state.measured != Some((state.revision, state.width))
            || state.heights.len() != lines
        {
            state.heights = state
                .content
                .lines()
                .map(|line| {
                    crate::text::min_bounds::<Renderer>(text::Text {
                        content: line.text.as_ref(),
                        bounds: Size::new(state.width, f32::INFINITY),
                        size: self.listing.size().into(),
                        line_height: text::LineHeight::Absolute(line_height.into()),
                        font: self.listing.font,
                        align_x: text::Alignment::Default,
                        align_y: Vertical::Top,
                        shaping: text::Shaping::Advanced,
                        wrapping: self.listing.wrapping(),
                        ellipsis: text::Ellipsis::None,
                        letter_spacing: Default::default(),
                        font_features: Vec::new(),
                        font_variations: Vec::new(),
                        weight: None,
                        hint_factor: renderer::Renderer::hint_factor(renderer),
                    })
                    .height
                    .max(line_height)
                })
                .collect();
            state.measured = Some((state.revision, state.width));
        }
        // Every line shows: the editor's buffer must not stop at the viewport, or
        // it leaves the lines below unshaped. Unwrapped, it is as wide as its
        // longest line and the code block scrolls it like the listing.
        // The editor fills the viewport at least, so a click right of the text
        // lands in it.
        let editor_limits = layout::Limits::with_flags(
            Size::ZERO,
            Size::new(state.width, f32::INFINITY),
            Size::new(false, false),
            Size::new(!self.listing.wrap, false),
        );
        self.editor(state)
            .layout(&mut children[0], renderer, &editor_limits);
        state.width = children[0].size.width;
        let width = if self.listing.wrap {
            width
        } else {
            left + state.width + padding
        };
        children[0].translation = Vector::new(left, 0.);
        state.gutter.width = if self.listing.start_line.is_some() {
            left
        } else {
            0.
        };
        let rows = children[0].size.height.max(state.heights.iter().sum());
        let height = if self.fill {
            limits.resolve_height(Length::Fill, rows)
        } else {
            rows
        };
        *size = Size::new(width, height);
        if std::mem::take(&mut state.moved) {
            state.reveal = Reveal(Some(self.caret(state, renderer, left)));
        }
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        // The editor shows every line, so the wheel scrolls the page around it.
        if matches!(event, Event::Mouse(mouse::Event::WheelScrolled { .. })) {
            return;
        }
        let bounds = layout.bounds();
        let state = tree.state.downcast_mut::<State>();
        if let Some(command) = state.menu.take() {
            let action = self.command(state, command, shell);
            self.perform(state, action, shell);
            shell.capture_event();
            self.blink(tree, layout, renderer, viewport);
            return;
        }
        if matches!(
            event,
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
        ) && state.focused
        {
            edit::share(shell, || state.content.selection());
        }
        match event {
            // A middle press pastes the primary selection where it lands, on Linux.
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Middle))
                if !self.disabled
                    && edit::has_primary()
                    && cursor
                        .position_over(bounds)
                        .is_some_and(|position| viewport.contains(position)) =>
            {
                for event in [
                    Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)),
                    Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)),
                ] {
                    self.update(tree, &event, layout, cursor, renderer, shell, viewport);
                }
                if self.is_editable() {
                    shell.read_clipboard(clipboard::Kind::Primary);
                }
                shell.capture_event();
                return;
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Right)) if !self.disabled => {
                let press = Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left));
                let Some(position) = cursor
                    .position_over(bounds)
                    .filter(|position| viewport.contains(*position))
                else {
                    // A press elsewhere takes the focus away, as a left press does.
                    if state.focused {
                        let cursor = mouse::Cursor::Unavailable;
                        self.update(tree, &press, layout, cursor, renderer, shell, viewport);
                    }
                    return;
                };
                // Focus and put the caret under the pointer, unless the press
                // lands inside the selection.
                let before = state.content.cursor();
                let release = Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left));
                for event in [press, release] {
                    self.update(tree, &event, layout, cursor, renderer, shell, viewport);
                }
                let state = tree.state.downcast_mut::<State>();
                if edit::is_inside(before, state.content.cursor().position) {
                    state.content.move_to(before);
                }
                state
                    .menu
                    .open(position - Vector::new(bounds.x, bounds.y), false, shell);
                shell.capture_event();
                return;
            }
            Event::Keyboard(keyboard::Event::KeyPressed {
                key,
                physical_key,
                modifiers,
                ..
            }) if state.focused && !self.disabled => {
                // Tab indents; after Escape, it leaves for the next field.
                let escaped =
                    std::mem::replace(&mut state.escaped, *key == Key::Named(Named::Escape));
                if escaped && *key == Key::Named(Named::Tab) {
                    return;
                }
                if edit::opens_menu(key, *modifiers) {
                    state.menu.open(state.caret, true, shell);
                    shell.capture_event();
                    return;
                }
                let platform = Platform::current();
                match edit::resolve(key, *physical_key, *modifiers, platform, true) {
                    Some(Intent::Command(command)) => {
                        let action = self.command(state, command, shell);
                        self.perform(state, action, shell);
                        if command == Command::SelectAll {
                            edit::share(shell, || state.content.selection());
                        }
                        shell.capture_event();
                        self.blink(tree, layout, renderer, viewport);
                        return;
                    }
                    Some(intent) => {
                        let selecting = matches!(intent, Intent::Select(_));
                        let visible = viewport.intersection(&bounds).unwrap_or(bounds);
                        let page = (visible.height / self.listing.line_height()).max(1.0) as i32;
                        let mut editing = Editing {
                            editor: self,
                            state,
                            shell,
                            page,
                        };
                        if edit::apply(&mut editing, intent, platform) {
                            shell.capture_event();
                        }
                        if selecting {
                            edit::share(shell, || state.content.selection());
                        }
                        self.blink(tree, layout, renderer, viewport);
                        return;
                    }
                    None => {}
                }
            }
            _ => {}
        }
        let mut bus = Bus::new();
        {
            let Tree {
                state, children, ..
            } = &mut *tree;
            let state = state.downcast_mut::<State>();
            let cursor = if self.disabled {
                cursor
            } else {
                state.autoscroll.track(event, cursor, bounds, viewport)
            };
            // A press on a line number starts at that line's code.
            let cursor = match cursor.position() {
                Some(position) if state.gutter.is_pressed(event, cursor, layout.bounds()) => {
                    mouse::Cursor::Available(lines::line_start(
                        layout.bounds(),
                        state.gutter,
                        position.y,
                    ))
                }
                // Below the last line, a press lands at the end of the text.
                Some(position)
                    if self.fill
                        && matches!(event, Event::Mouse(mouse::Event::ButtonPressed(_)))
                        && cursor.is_over(layout.bounds())
                        && position.y >= layout.bounds().y + text_end(children).y =>
                {
                    let end = text_end(children) - Vector::new(1., self.listing.line_height() / 2.);
                    mouse::Cursor::Available(layout.position() + Vector::new(end.x, end.y))
                }
                _ => cursor,
            };
            let (child_layout, child_tree) = layout
                .iter_mut(children)
                .next()
                .expect("the editor has one child");
            let mut local = shell.local(&mut bus);
            self.editor(state).update(
                child_tree,
                event,
                child_layout,
                cursor,
                renderer,
                &mut local,
                viewport,
            );
            let focused = focus::count(|op| {
                self.editor(state)
                    .operate(child_tree, child_layout, viewport, renderer, op)
            })
            .focused
            .is_some();
            if state.focused != focused {
                state.focused = focused;
                shell.request_redraw();
            }
            if local.is_event_captured() {
                shell.capture_event();
            }
            if let InputMethod::Enabled { cursor: caret, .. } = local.input_method() {
                state.caret = Point::new(caret.x - bounds.x, caret.y + caret.height - bounds.y);
            }
            // Typing shows the caret, as the keys run here do.
            if matches!(event, Event::Keyboard(keyboard::Event::KeyPressed { .. }))
                && local.is_event_captured()
            {
                state.show = Some(edit::Reveal::Caret);
            }
            if matches!(event, Event::Window(window::Event::RedrawRequested(_)))
                && let Some(reveal) = state.show.take()
            {
                let line = self.listing.line_height();
                let caret = Rectangle::new(
                    Point::new(bounds.x + state.caret.x, bounds.y + state.caret.y - line),
                    Size::new(1.0, line),
                );
                if let Some(area) = edit::area(reveal, bounds, *viewport, caret) {
                    shell.request_reveal(area);
                }
            }
            if state.overwrite && state.focused {
                let position = state.content.cursor().position;
                if state.block.is_none_or(|(at, _)| at != position) {
                    let width = edit::block_width(
                        &state.content,
                        self.listing.font,
                        self.listing.size(),
                        renderer,
                    );
                    state.block = Some((position, width));
                }
            }
            shell.request_redraw_at(local.redraw_request());
            shell.invalidate(local.invalidation());
            shell.request_input_method(local.input_method());
            shell.clipboard_mut().merge(local.clipboard_mut());
        }

        let state = tree.state.downcast_mut::<State>();
        self.perform(state, bus.drain().map(|(action, _)| action), shell);

        let visible = viewport.intersection(&bounds).unwrap_or(bounds);
        if let Some(pointer) = state.autoscroll.tick(event, visible, viewport, shell) {
            let drag = Event::Mouse(mouse::Event::CursorMoved { position: pointer });
            let cursor = mouse::Cursor::Available(pointer);
            self.update(tree, &drag, layout, cursor, renderer, shell, viewport);
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State>();
        let bounds = layout.bounds();
        let padding = self.padding();
        let size = self.listing.size();
        let line_height = self.listing.line_height();
        let gutter = self.gutter(state.heights.len());
        let pinned = bounds.x + state.gutter.offset;
        let muted: Color = theme.muted.foreground.into();
        let hint_factor = renderer::Renderer::hint_factor(renderer);
        let mut y = bounds.y;
        for (index, height) in state.heights.iter().enumerate() {
            let row = Rectangle {
                x: bounds.x,
                y,
                width: bounds.width,
                height: *height,
            };
            y += height;
            if row.intersection(viewport).is_none() {
                continue;
            }
            let variant = self.styles.get(&(index + 1)).copied().unwrap_or_default();
            if let Some(background) = line_style(theme, variant).background {
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: row,
                        ..Default::default()
                    },
                    background,
                );
            }
            let label = |content: String, x: f32, align_x, color| {
                (
                    text::Text {
                        content,
                        bounds: Size::new(gutter.unwrap_or(size), line_height),
                        size: size.into(),
                        line_height: text::LineHeight::Absolute(line_height.into()),
                        font: self.listing.font,
                        align_x,
                        align_y: Vertical::Top,
                        shaping: text::Shaping::Advanced,
                        wrapping: text::Wrapping::None,
                        ellipsis: text::Ellipsis::None,
                        letter_spacing: Default::default(),
                        font_features: Vec::new(),
                        font_variations: Vec::new(),
                        weight: None,
                        hint_factor,
                    },
                    Point::new(x, row.y),
                    color,
                )
            };
            if let (Some(first), Some(gutter)) = (self.listing.start_line, gutter) {
                let (number, position, color) = label(
                    first.saturating_add(index).to_string(),
                    pinned + padding + gutter,
                    text::Alignment::Right,
                    muted,
                );
                renderer.fill_text(number, position, color, *viewport);
            }
            let glyph = match variant {
                LineStyle::Added => Some(("+", muted)),
                LineStyle::Removed => Some(("−", theme.destructive.base.into())),
                _ => None,
            };
            if let Some((glyph, color)) = glyph {
                let (mark, position, color) = label(
                    glyph.to_owned(),
                    pinned + 4.,
                    text::Alignment::Default,
                    color,
                );
                renderer.fill_text(mark, position, color, *viewport);
            }
        }

        let (child_layout, child_tree) = layout
            .iter(&tree.children)
            .next()
            .expect("the editor has one child");
        let value = style.text_color;
        let selection = theme.selection();
        let surface = move |_: &Theme, _: Status| Style {
            background: Background::Color(Color::TRANSPARENT),
            border: Default::default(),
            placeholder: muted,
            value,
            selection,
        };
        // A long unwrapped line scrolls inside the editor, never over the gutter.
        let clip = if state.gutter.width > 0. {
            Some(state.gutter.code(bounds, *viewport))
        } else {
            child_layout.bounds().expand(1.).intersection(viewport)
        };
        let Some(clip) = clip.filter(|clip| clip.width > 0.) else {
            return;
        };
        renderer.with_layer(clip, |renderer| {
            self.editor(state).style(surface).draw(
                child_tree,
                renderer,
                theme,
                style,
                child_layout,
                cursor,
                viewport,
            );
        });
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let state = tree.state.downcast_ref::<State>();
        if state.gutter.width > 0. && cursor.is_over(state.gutter.strip(layout.bounds())) {
            return mouse::Interaction::default();
        }
        let (child_layout, child_tree) = layout
            .iter(&tree.children)
            .next()
            .expect("the editor has one child");
        self.editor(state)
            .mouse_interaction(child_tree, child_layout, cursor, viewport, renderer)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        let Tree {
            state, children, ..
        } = tree;
        let state = state.downcast_mut::<State>();
        operation.custom(None, layout.bounds(), &mut state.gutter);
        if self.disabled {
            return;
        }
        let (child_layout, child_tree) = layout
            .iter_mut(children)
            .next()
            .expect("the editor has one child");
        operation.custom(None, layout.bounds(), &mut Editable);
        if let Some(caret) = state.reveal.0 {
            let caret = caret + Vector::new(layout.position().x, layout.position().y);
            operation.custom(None, caret, &mut state.reveal);
        }
        self.editor(state)
            .operate(child_tree, child_layout, viewport, renderer, operation);
        state.focused = focus::count(|op| {
            self.editor(state)
                .operate(child_tree, child_layout, viewport, renderer, op)
        })
        .focused
        .is_some();
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout,
        renderer: &Renderer,
        _viewport: &Rectangle,
        translation: Vector,
        window: Size,
    ) -> Vec<overlay::Element<'b, Message, Theme, Renderer>> {
        let state = tree.state.downcast_mut::<State>();
        // Reading the selection copies it: only an open menu needs it.
        if !state.menu.is_open() {
            return Vec::new();
        }
        let available = self.available(state);

        state
            .menu
            .overlay(available, layout.bounds(), translation, renderer, window)
            .into_iter()
            .collect()
    }
}

/// The bottom right corner of the editor's text, from the editor's corner.
fn text_end(children: &[Tree]) -> Point {
    children.first().map_or(Point::ORIGIN, |child| {
        Point::new(
            child.translation.x + child.size.width,
            child.translation.y + child.size.height,
        )
    })
}

/// Whether an editor in `tree` holds the focus.
pub(super) fn focused(tree: &Tree) -> bool {
    tree.tag == tree::Tag::of::<State>() && tree.state.downcast_ref::<State>().focused
}
