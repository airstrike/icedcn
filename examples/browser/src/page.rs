mod heading;
pub(crate) mod rhea;
mod sections;

use iced::Element;
use iced::Widget;
use std::cell::{Cell, OnceCell};
use std::collections::{BTreeSet, HashMap};
use std::path::Path;
#[cfg(not(target_arch = "wasm32"))]
use tokio::fs::read_to_string;

mod bundled {
    include!(concat!(env!("OUT_DIR"), "/headings.rs"));
    #[cfg(target_arch = "wasm32")]
    include!(concat!(env!("OUT_DIR"), "/snippets.rs"));
}

use iced::Alignment::*;
use iced::border;
use iced::mouse::Interaction;
use iced::widget::{Id, column, lazy, scrollable, stack, themer};
use iced::{Background, Fill, Font, Pixels, Subscription, Task};

use icedcn::Size::{Lg, Sm};
use icedcn::core::{Border, Radius};
use icedcn::FontFamily;
use icedcn::theme::container::{
    Style, code_block, code_block_panel, code_fade_overlay, transparent,
};
use icedcn::theme::tweak::Opaque;
use icedcn::widget::markdown;
use icedcn::widget::markdown::widget::Highlight;
use icedcn::widget::{button, rule, text};
use icedcn::widget::{container, toast};
use icedcn::{Slot, Theme, Weight, radius, theme};

use crate::component::code_block;
use crate::font;
use crate::icon;
use icedcn::widget::scroller;

/// Height for preview containers, in logical pixels. Per-example
/// cards stay at this fixed height; tall content (the typography
/// article) lives in a `Block::Inline` instead, which has no card chrome
/// and grows naturally.
const PREVIEW_HEIGHT: f32 = 288.0;

/// Padding around the live preview, in logical pixels.
const PREVIEW_PADDING: f32 = 24.0;

/// Collapsed code-snippet height when not expanded by the user (px).
const COLLAPSED_CODE_HEIGHT: f32 = 160.0;

/// Static metadata for a component page.
pub struct Meta {
    pub title: &'static str,
    pub description: &'static str,
}

/// Composition primitive for a component page. A page is a `&'static [Block]`
/// rendered in order between the page title/description and the bottom of
/// the scroll area.
///
/// Four flavors:
///
/// - [`Block::Md`] is text-shaped content (Installation, Usage, API
///   Reference, Cursor, free-standing prose) loaded asynchronously from a
///   markdown file. The first `# `, `## `, or `### ` heading in the file
///   is extracted as the section title for the right-side TOC and
///   rendered with the depth-aware heading style.
/// - [`Block::Example`] is a live preview rendered by `Page::preview(id)`
///   wrapped in card chrome (288px-tall preview area + collapsible
///   code block underneath). Empty `title` skips the heading + TOC entry —
///   used for the page hero.
/// - [`Block::Inline`] is a live preview rendered into the page flow with
///   no card chrome. Used by typography for the joke-tax article.
/// - [`Block::Group`] wraps a section heading + nested children. The
///   heading's depth is derived from the group's nesting level: depth 0
///   uses `text::h2`, depth >= 1 uses `text::h3`. Children inherit
///   `depth + 1` for THEIR own headings, and the right-side TOC indents
///   them by 12px per level with a 0.85x alpha multiplier on each level
///   below the top.
pub enum Block {
    /// A pre-rendered markdown section. Path is relative to the repo root,
    /// loaded asynchronously via `page::State::init`.
    #[allow(dead_code)]
    Md(&'static str),

    /// A live interactive example with a `preview()` view, optional
    /// description, and a code snippet.
    Example {
        /// Stable id; `Page::preview(id)` dispatches by this string.
        id: &'static str,
        /// Heading rendered above the card; also drives the TOC anchor.
        /// `""` means no heading and no TOC entry — used for the page hero.
        title: &'static str,
        /// Inline description rendered between the heading and the preview card.
        description: Option<&'static str>,
        /// Path to a `.md` file containing the code snippet. Empty string
        /// means no code block under the preview.
        snippet: &'static str,
    },

    /// An interactive `preview(id)` rendered as page-level prose (no card
    /// chrome). For typography demos and similar inline content.
    #[allow(dead_code)]
    Inline { id: &'static str },

    /// A grouping section with an explicit heading and nested children.
    /// Replaces the auto-h2-injection trick the older renderer used to
    /// drop an "Examples" heading before the first non-hero example.
    /// Children render at the parent's indentation; only the heading
    /// level + TOC indent change with depth.
    #[allow(dead_code)]
    Group {
        /// Section heading rendered at the group's depth.
        title: &'static str,
        /// Nested blocks, rendered after the group heading.
        children: &'static [Block],
    },
}

impl Block {
    /// Title used for the right-side TOC. `None` skips the entry.
    fn toc_title<'a>(&'a self, state: &'a State) -> Option<&'a str> {
        match self {
            Block::Md(p) => state.title(p),
            Block::Example { title, .. } => {
                if title.is_empty() {
                    None
                } else {
                    Some(*title)
                }
            }
            Block::Group { title, .. } => {
                if title.is_empty() {
                    None
                } else {
                    Some(*title)
                }
            }
            Block::Inline { .. } => None,
        }
    }

    /// Path to the markdown source file loaded asynchronously, if any.
    /// For [`Block::Group`], children expose their own paths via the
    /// recursive walk in [`for_each_path`].
    fn path(&self) -> &'static str {
        match self {
            Block::Md(p) => p,
            Block::Example { snippet, .. } => snippet,
            Block::Inline { .. } => "",
            Block::Group { .. } => "",
        }
    }
}

/// Recursively visit every block and invoke `f(path)` for every non-empty
/// markdown source path — used by `State::init` to schedule snippet/markdown
/// loads across nested groups.
fn for_each_path(blocks: &[Block], f: &mut impl FnMut(&'static str)) {
    for b in blocks {
        let p = b.path();
        if !p.is_empty() {
            f(p);
        }
        if let Block::Group { children, .. } = b {
            for_each_path(children, f);
        }
    }
}

/// Flatten the recursive block tree into a depth-tagged visit order.
/// Each yielded entry corresponds to ONE addressable position in the
/// page (sensor + scroll target + TOC entry), keyed by a flat `usize`
/// index in visit order. Groups precede their children — a group's
/// flat index points at its heading, and each nested child gets its
/// own subsequent flat index.
fn flatten(blocks: &'static [Block]) -> Vec<(&'static Block, u8)> {
    fn walk(out: &mut Vec<(&'static Block, u8)>, blocks: &'static [Block], depth: u8) {
        for b in blocks {
            out.push((b, depth));
            if let Block::Group { children, .. } = b {
                walk(out, children, depth + 1);
            }
        }
    }
    let mut out = Vec::new();
    walk(&mut out, blocks, 0);
    out
}

/// Searchable TOC destinations, using the same flat keys as the scroll targets.
pub(crate) fn sections(page: &dyn Page) -> impl Iterator<Item = (usize, &str)> {
    flatten(page.blocks())
        .into_iter()
        .enumerate()
        .filter_map(move |(i, (block, _))| block.toc_title(page.state()).map(|title| (i, title)))
}

/// Messages produced by page infrastructure and component interactions.
#[derive(Debug, Clone)]
pub enum Message {
    /// Markdown bodies for one page, keyed by source path.
    Loaded(Vec<(&'static str, String)>),
    LinkClicked(markdown::Uri),
    ChooseFile,
    Notice(String, String),
    CodeBlock(code_block::Event),
    FileSelected(Option<String>),
    VisibleSections(BTreeSet<usize>),
    ScrollTo(usize),
    ScrollTranscript(Id, scroller::Target, scroller::Animation),
    ExpandCode(usize),
    CopyCode(usize),
    Noop,
}

/// Spacing and surfaces around an installed example.
pub struct Backdrop {
    /// Minimum preview height in logical pixels, before the code panel.
    pub min_height: f32,
    pub padding: iced::Padding,
    pub surface: fn(&Theme) -> theme::container::Style,
    pub border: fn(&Theme) -> theme::container::Style,
}

impl Default for Backdrop {
    fn default() -> Self {
        Self {
            min_height: PREVIEW_HEIGHT,
            padding: PREVIEW_PADDING.into(),
            surface: transparent,
            border: |theme| Style {
                border: Border {
                    width: 1.0,
                    color: theme.neutral.border,
                    radius: radius::XL2.into(),
                },
                snap: true,
                ..Style::default()
            },
        }
    }
}

/// Object-safe trait for a component page.
pub trait Page: Send {
    fn meta(&self) -> &'static Meta;
    fn blocks(&self) -> &'static [Block];
    fn page_index(&self) -> usize;
    fn state(&self) -> &State;
    fn scrollable_id(&self) -> Id;

    fn init(&self) -> Task<super::Message>;
    fn update(&mut self, message: Message) -> Task<super::Message>;
    fn preview(&self, id: &str) -> iced::Element<'_, super::Message, Theme>;

    /// Update the window width used by viewport-dependent example recipes.
    fn window_width(&mut self, _width: f32) {}

    fn backdrop(&self) -> Backdrop {
        Backdrop::default()
    }

    /// The preview frame for an individual example.
    fn example_backdrop(&self, _id: &str) -> Backdrop {
        self.backdrop()
    }

    /// Some upstream recipes ship a particular palette inside the preview.
    fn preview_theme(&self, _outer: &Theme) -> Option<Theme> {
        None
    }

    fn themed_preview(&self, id: &str, outer: &Theme) -> iced::Element<'_, super::Message, Theme> {
        let content = self.preview(id);
        match self.preview_theme(outer) {
            Some(theme) => themer(Some(theme), content)
                .text_color(|t| t.neutral.foreground.into())
                .boxed(),
            None => content,
        }
    }

    /// The installed documentation may constrain a recipe's width, height
    /// and alignment independently of the standalone component example.
    fn docs_preview(&self, id: &str, outer: &Theme) -> iced::Element<'_, super::Message, Theme> {
        self.themed_preview(id, outer)
    }

    /// Subscription for pages that drive animations or timers. Default:
    /// `Subscription::none()`. Only the currently active page's subscription
    /// is wired by `App`, so pages don't waste cycles when the user is on
    /// a different tab.
    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }

    /// Reset state that should NOT persist across navigation away from this
    /// page (e.g., expanded-code dismissal). Default: clear nothing —
    /// pages with `page::State` should override to call `state.reset_volatile()`.
    fn reset_volatile(&mut self) {}

    /// Called by `App` whenever the user navigates TO this page. Use it for
    /// "first-paint" triggers that iced's `sensor` can't reliably catch when
    /// the example is already in the viewport at mount time. Default: no-op.
    fn on_visible(&mut self) -> Task<super::Message> {
        Task::none()
    }
}

/// Render a page's body and optional "On This Page" nav. The `viewer`
/// owns the active code-theme `Map`; the caller (App) keeps it in
/// state and rebuilds it on theme toggle so the viewer reference here
/// outlives the returned Element.
pub fn view<'a>(
    page: &'a dyn Page,
    viewer: &'a SnippetViewer,
) -> (Slot<'a, super::Message>, Option<Slot<'a, super::Message>>) {
    view_with_toc(page, viewer, true)
}

pub fn view_with_toc<'a>(
    page: &'a dyn Page,
    viewer: &'a SnippetViewer,
    show_toc: bool,
) -> (Slot<'a, super::Message>, Option<Slot<'a, super::Message>>) {
    let body = render_blocks(page, viewer, show_toc);
    let nav = (show_toc && sections(page).nth(1).is_some()).then(|| toc(page));
    (body, nav)
}

/// Heading widget for a TOC-bearing section. Depth 0 → `h2`, depth >= 1
/// → `h3`. Centralized so render-side and any future depth-aware sizing
/// changes update in one place.
fn heading_for<'a>(
    depth: u8,
    title: impl Into<String>,
) -> iced::Element<'a, super::Message, Theme> {
    let title = title.into();
    if depth == 0 {
        text::h2(title).into()
    } else {
        text::h3(title).into()
    }
}

fn render_blocks<'a>(
    page: &'a dyn Page,
    viewer: &'a SnippetViewer,
    show_toc: bool,
) -> Slot<'a, super::Message> {
    let pi = page.page_index();
    let st = page.state();
    let mut items: Vec<iced::Element<'_, super::Message, Theme>> = Vec::new();
    let mut flat_idx: usize = 0;
    walk_blocks(
        page.blocks(),
        0,
        page,
        viewer,
        st,
        pi,
        &mut flat_idx,
        &mut items,
    );
    sections::Sections::new(items, pi, show_toc).boxed().into()
}

#[allow(clippy::too_many_arguments)]
fn walk_blocks<'a>(
    blocks: &'static [Block],
    depth: u8,
    page: &'a dyn Page,
    viewer: &'a SnippetViewer,
    st: &'a State,
    pi: usize,
    flat_idx: &mut usize,
    items: &mut Vec<iced::Element<'a, super::Message, Theme>>,
) {
    for block in blocks {
        let i = *flat_idx;
        *flat_idx += 1;
        let block_id = st.block_ids[i].clone();

        let body: iced::Element<'_, super::Message, Theme> = match block {
            Block::Md(path) => render_md(pi, depth, path, st, viewer),
            Block::Example {
                id,
                title,
                description,
                snippet,
            } => render_example(
                pi,
                i,
                depth,
                id,
                title,
                *description,
                snippet,
                st,
                viewer,
                page,
            ),
            Block::Inline { id } => page.themed_preview(id, &viewer.theme),
            Block::Group { title, .. } => heading_for(depth, *title),
        };

        items.push(container(body).id(block_id).width(Fill).into());

        if let Block::Group { children, .. } = block {
            walk_blocks(children, depth + 1, page, viewer, st, pi, flat_idx, items);
        }
    }
}

fn render_md<'a>(
    page_index: usize,
    depth: u8,
    path: &'static str,
    st: &'a State,
    viewer: &'a SnippetViewer,
) -> iced::Element<'a, super::Message, Theme> {
    let mut col = column![].spacing(16).width(Fill);
    if let Some(t) = st.title(path) {
        col = col.push(heading_for(depth, t.to_owned()));
    }
    match st.md_items.get(path) {
        Some(Some(items)) if !items.is_empty() => {
            col = col.push(snippet(page_index, items, viewer).compose());
        }
        Some(Some(_)) => {
            // Body is empty (heading-only file) — nothing to render below.
        }
        _ => {
            let loading: iced::Element<'_, super::Message, Theme> = text("Loading...").into();
            col = col.push(loading);
        }
    };
    col.boxed()
}

#[allow(clippy::too_many_arguments)]
fn render_example<'a>(
    page_index: usize,
    block_pos: usize,
    depth: u8,
    id: &'static str,
    title: &'static str,
    description: Option<&'static str>,
    snippet_path: &'static str,
    st: &'a State,
    viewer: &'a SnippetViewer,
    page: &'a dyn Page,
) -> iced::Element<'a, super::Message, Theme> {
    let backdrop = page.example_backdrop(id);
    let code: Option<iced::Element<'_, super::Message, Theme>> =
        (!snippet_path.is_empty()).then(|| {
            let body = st
                .md_items
                .get(snippet_path)
                .and_then(|opt| opt.as_ref())
                .map(|items| snippet(page_index, items, viewer))
                .unwrap_or_else(|| text("Missing snippet!").into());
            code_panel_with_border(
                page_index,
                block_pos,
                body,
                st.code_expanded.contains(&block_pos),
                (backdrop.border)(&viewer.theme).border,
            )
        });

    let description = description
        .and_then(|copy| st.descriptions().get(copy))
        .map(|items| snippet(page_index, items, viewer).compose());
    framed_example(
        depth,
        title,
        description,
        page.docs_preview(id, &viewer.theme),
        code,
        backdrop,
    )
}

/// Render example-card chrome: an optional depth-aware heading
/// (`h2`/`h3`) and muted description, then the bordered card — a preview at least
/// 288px tall over an optional collapsible code block. Tall previews grow
/// with their content. An empty `title` omits the heading.
pub fn example_card<'a>(
    depth: u8,
    title: &'a str,
    description: Option<&'a str>,
    preview: iced::Element<'a, super::Message, Theme>,
    code: Option<iced::Element<'a, super::Message, Theme>>,
) -> iced::Element<'a, super::Message, Theme> {
    let description = description.map(|d| text::p(d).style(theme::text::Muted).into());
    framed_example(
        depth,
        title,
        description,
        preview,
        code,
        Backdrop::default(),
    )
}

fn framed_example<'a>(
    depth: u8,
    title: &'a str,
    description: Option<iced::Element<'a, super::Message, Theme>>,
    preview: iced::Element<'a, super::Message, Theme>,
    code: Option<iced::Element<'a, super::Message, Theme>>,
    backdrop: Backdrop,
) -> iced::Element<'a, super::Message, Theme> {
    let preview_el: iced::Element<'_, super::Message, Theme> = container(preview)
        .center_x(Fill)
        .align_y(iced::Center)
        .min_height(backdrop.min_height)
        .padding(backdrop.padding)
        .style(backdrop.surface)
        .into();

    let divider: Option<iced::Element<'_, super::Message, Theme>> =
        code.as_ref().map(|_| rule::horizontal(1).into());

    let card_el: iced::Element<'_, super::Message, Theme> =
        container(column![preview_el, divider, code])
            .style(backdrop.border)
            .padding(1)
            .clip(true)
            .width(Fill)
            .into();

    // Hero: no h3, no description, just the card.
    if title.is_empty() {
        return card_el;
    }

    let mut col = column![].spacing(16).width(Fill);
    col = col.push(heading_for(depth, title));
    if let Some(d) = description {
        col = col.push(d);
    }
    col = col.push(card_el);
    col.boxed()
}

/// The collapsible code panel beneath a preview. Until expanded it is
/// clipped to [`COLLAPSED_CODE_HEIGHT`] under a fade overlay carrying
/// the "View Code" button; once expanded it shows the full snippet with
/// a top-right copy button.
pub fn code_panel<'a>(
    page_index: usize,
    block_pos: usize,
    body: Slot<'a, super::Message>,
    expanded: bool,
) -> iced::Element<'a, super::Message, Theme> {
    code_panel_with_border(
        page_index,
        block_pos,
        body,
        expanded,
        Border::default().rounded(radius::XL2).width(1),
    )
}

fn code_panel_with_border<'a>(
    page_index: usize,
    block_pos: usize,
    body: Slot<'a, super::Message>,
    expanded: bool,
    border: Border,
) -> iced::Element<'a, super::Message, Theme> {
    // The container clips to a rectangle. Fit the code surface and fade to the
    // frame's inner arc so neither paints over the border.
    let inner_radius = Radius {
        bottom_left: (border.radius.bottom_left - border.width).max(0.0),
        bottom_right: (border.radius.bottom_right - border.width).max(0.0),
        ..Radius::default()
    };
    let style = move |base: fn(&Theme) -> Style| {
        move |theme: &Theme| {
            let mut panel = base(theme);
            panel.border.radius = inner_radius;
            panel.snap = true;
            panel
        }
    };
    let inner = container(body)
        .padding(iced::Padding::default().top(12).bottom(12).left(16))
        .width(Fill)
        .style(style(code_block_panel));

    if expanded {
        let copy_btn: Slot<'_, super::Message> = button(icon::copy().size(14))
            .style(button::Ghost)
            .size(Sm)
            .square()
            .on_press(super::Message::Page(
                page_index,
                Message::CopyCode(block_pos),
            ))
            .into();

        let copy_corner = container(copy_btn)
            .align_x(End)
            .align_y(Start)
            .padding([8, 8])
            .width(Fill)
            .height(Fill);

        stack![Element::from(inner), Element::from(copy_corner)].boxed()
    } else {
        let truncated = inner.max_height(COLLAPSED_CODE_HEIGHT).clip(true);

        let view_code_btn: Slot<'_, super::Message> = button("View Code")
            .style(button::Outline + Opaque + radius(Lg))
            .on_press(super::Message::Page(
                page_index,
                Message::ExpandCode(block_pos),
            ))
            .into();

        let overlay = container(view_code_btn)
            .center(Fill)
            .style(style(code_fade_overlay))
            .height(COLLAPSED_CODE_HEIGHT);
        stack![Element::from(truncated), Element::from(overlay)].boxed()
    }
}

/// Right-side "On This Page" navigation.
///
/// Visible headings brighten toward the center of the viewport. The active
/// heading uses the foreground color and medium weight.
fn toc<'a>(page: &'a dyn Page) -> Slot<'a, super::Message> {
    /// Keep off-screen links legible against both documentation surfaces.
    const TOC_ALPHA_FLOOR: f32 = 0.9;

    let pi = page.page_index();
    let st = page.state();

    // Walk the block tree in flat-visit order, capturing each TOC-bearing
    // entry's flat index, depth, and title.
    let entries: Vec<(usize, u8, String)> = flatten(page.blocks())
        .into_iter()
        .enumerate()
        .filter_map(|(flat, (b, depth))| b.toc_title(st).map(|t| (flat, depth, t.to_owned())))
        .collect();

    let visible_in_toc: Vec<usize> = st
        .visible
        .iter()
        .copied()
        .filter(|i| entries.iter().any(|(j, _, _)| j == i))
        .collect();

    let toc_count = visible_in_toc.len();

    // Rank-distance fade. Center of visible set → 1.0, edges → floor.
    // Off-visible entries → floor flat. Empty-visible → floor flat.
    // Closure is `Fn` (read-only) so each map call below can re-invoke it.
    let alpha_for = |i: usize| -> f32 {
        if toc_count == 0 || !visible_in_toc.contains(&i) {
            return TOC_ALPHA_FLOOR;
        }
        let Some(rank) = visible_in_toc.iter().position(|&v| v == i) else {
            return TOC_ALPHA_FLOOR;
        };
        let center = (toc_count - 1) as f32 / 2.0;
        let distance = (rank as f32 - center).abs() / center.max(1.0);
        1.0 - (1.0 - TOC_ALPHA_FLOOR) * distance
    };

    // The "active" TOC entry mirrors docs-toc: whichever entry currently
    // sits at rank-center of the sensor-visible set is the one the
    // reader is reading. Empty-visible-set pages have no active entry.
    let active_index: Option<usize> = if toc_count == 0 {
        None
    } else {
        let center = (toc_count - 1) / 2;
        visible_in_toc.get(center).copied()
    };

    use iced_widget::button::StyleFn;
    use theme::button::{Status, Style};
    let nav_items = entries.into_iter().map(|(i, depth, title)| {
        let indent = 8.0 + f32::from(depth) * 12.0;
        let is_active = Some(i) == active_index;
        let alpha = alpha_for(i);
        let toc_style: StyleFn<'_, Theme> = Box::new(move |theme, status| {
            let p = theme;
            // Active entry promotes to `foreground` at full opacity (no
            // alpha math). Inactive entries stay at `muted_foreground` and
            // pick up the rank-distance alpha so they recede toward the
            // edges of the reader's viewport but never drop below the
            // legibility floor. Hover/press promote to `foreground` for
            // affordance. No backgrounds at any state.
            let base = match status {
                Status::Active if is_active => p.neutral.foreground,
                Status::Active => p.muted.foreground.alpha(alpha),
                Status::Hovered | Status::Pressed | Status::Disabled => p.neutral.foreground,
            };
            Style {
                background: None,
                text_color: base,
                ..Style::default()
            }
            .into()
        });
        let weight = if is_active {
            Weight::Medium
        } else {
            Weight::Normal
        };
        let entry: Slot<'_, super::Message> = button(text(title).width(Fill))
            .size(Sm)
            .width(Fill)
            .weight(weight)
            .translate(false)
            .style(toc_style)
            .inset([1, 0])
            .interaction(Interaction::Pointer)
            .on_press(super::Message::Page(pi, Message::ScrollTo(i)))
            .into();
        let row: iced::Element<'_, super::Message, Theme> = container(entry)
            .width(Fill)
            .padding([0.0, 8.0, 0.0, indent])
            .into();
        row
    });

    let header = container(text::muted("On This Page").selectable(false)).padding([0, 8]);

    let items = scrollable(column(nav_items).width(Fill).spacing(2))
        .width(Fill)
        .height(Fill);
    column![Element::from(header)]
        .push(items.boxed())
        .spacing(16)
        .padding([24, 8])
        .width(220)
        .height(Fill)
        .into()
}

pub fn snippet<'a>(
    page_index: usize,
    items: &'a Vec<markdown::Item>,
    viewer: &'a SnippetViewer,
) -> Slot<'a, super::Message> {
    let mut settings = markdown::Settings {
        code_size: Pixels(13.0),
        line_height: markdown::LineHeight::Relative(1.75),
        ..markdown::Settings::with_style(viewer.style)
    };
    // `Settings::with_style` auto-derives inline_code_size as text_size *
    // 0.875 (= 14 for body 16). Geist Mono's cap-height reads ~1px taller
    // than Inter Tight at the same nominal size, so 14 still looks bigger
    // than body. Pin inline at 13 to land visually-balanced with the
    // surrounding prose.
    settings.style.inline_code_size = Pixels(13.0);
    markdown::view_with(items, settings, viewer)
        .map(move |url| super::Message::Page(page_index, Message::LinkClicked(url)))
        .boxed()
        .into()
}

/// Build a [`markdown::Style`] whose inline-code chip pulls from the
/// active [`Theme`]. Mirrors `--color-code` / `--color-code-foreground`
/// pair: light-mode chip is a near-white bg with body fg; dark-mode chip
/// is a dark gray bg with a desaturated fg. Both flavors hand off via the
/// same field and stay in sync with the page's `code_block` container.
fn inline_style_from_theme(theme: &Theme, body: Font) -> markdown::Style {
    markdown::Style {
        font: body,
        inline_code_color: theme.neutral.foreground.into(),
        inline_code_highlight: Highlight {
            background: Background::Color(theme.muted.base.into()),
            border: border::rounded(4),
        },
        inline_code_padding: iced::Padding::default().left(2).right(2),
        inline_code_font: font::GEIST_MONO_NAME.into(),
        // Baseline matching body=16; `markdown::Settings::with_text_size`
        // overwrites this to `text_size * 0.875` when the snippet renders,
        // so this only matters if a caller wires up `Settings` manually.
        inline_code_size: Pixels(14.0),
        code_block_font: font::GEIST_MONO_NAME.into(),
        // Snippet bodies rarely contain links, but when they do the
        // primary base reads as a "clickable" accent in either theme
        // (dark text on light, light text on dark).
        link_color: theme.primary.base.into(),
    }
}

/// Markdown viewer for code-snippet rendering. Holds an owned
/// [`markdown::highlighter::Map`] derived from the current app theme
/// (or supplied directly by the app) plus a [`markdown::Style`] whose
/// inline-code chip is tinted to the active [`Theme`]. Both fields
/// are baked at construction time and the App rebuilds the viewer
/// whenever the theme toggles, so view-time rendering stays a pure
/// read.
///
/// Overrides `code_block` to skip iced's default container chrome —
/// bg + radius + padding come from the page-level container, the
/// markdown widget emits a selectable highlighted paragraph inside a horizontal
/// scrollable.
pub struct SnippetViewer {
    theme: Theme,
    code_theme_id: Id,
    map: markdown::highlighter::Map,
    /// Palette-derived markdown style. Inline-code fields (color +
    /// highlight bg) are driven by `palette.code`; everything else
    /// is the icedcn default. Private because only [`snippet`] in
    /// this module reads it; the App swaps the whole viewer on theme
    /// toggle rather than mutating in place.
    style: markdown::Style,
}

impl SnippetViewer {
    /// Build a viewer with an explicit code-theme map (typically a
    /// VS Code theme parsed by `Map::from_vscode_theme`) and a
    /// palette-derived [`markdown::Style`]. App-level construction site
    /// recomputes both whenever the user toggles light/dark, mirroring
    /// the existing `Map` rebuild path.
    pub fn new(map: markdown::highlighter::Map, theme: &Theme) -> Self {
        let mut viewer = Self {
            map,
            code_theme_id: Id::unique(),
            theme: theme.clone(),
            style: inline_style_from_theme(theme, font::BODY.into()),
        };
        viewer.set_font(font::BODY.into());
        viewer
    }

    pub(crate) fn set_font(&mut self, body: Font) {
        self.style.font = body;
        self.theme = Theme::new(self.theme.design().clone().with_font_sans(FontFamily {
            regular: body,
            medium: body.weight(Weight::Medium.into()),
            semibold: body.weight(Weight::Semibold.into()),
            bold: body.weight(Weight::Bold.into()),
        }));
    }

    pub(crate) fn theme(&self) -> &Theme {
        &self.theme
    }

    pub(crate) fn set_theme(&mut self, map: markdown::highlighter::Map, theme: Theme) {
        let theme = Theme::new(
            theme
                .design()
                .clone()
                .with_font_sans(self.theme.font_sans.clone()),
        );
        self.style = inline_style_from_theme(&theme, self.style.font);
        self.theme = theme;
        self.map = map;
        self.code_theme_id = Id::unique();
    }
}

impl<'a> markdown::Viewer<'a, markdown::Uri, Theme, iced::Renderer> for SnippetViewer {
    fn on_link_click(url: markdown::Uri) -> markdown::Uri {
        url
    }

    fn code_theme(&'a self) -> Option<&'a dyn markdown::highlighter::Theme> {
        Some(&self.map)
    }

    fn code_block(
        &'a self,
        settings: markdown::Settings,
        language: Option<&'a str>,
        code: &'a str,
        lines: &'a [markdown::Text],
    ) -> iced::Element<'a, markdown::Uri, Theme, iced::Renderer> {
        use icedcn::widget::scrollable;

        // Span construction clones every highlighted token. Reuse the selectable
        // paragraph while only the viewport or the active heading has changed.
        let key = (
            code,
            language,
            &self.code_theme_id,
            settings.style.code_block_font,
            settings.code_size.0.to_bits(),
        );
        let body = lazy(key, move |_| {
            markdown::code(settings, lines, Some(&self.map)).on_link_click(Self::on_link_click)
        })
        .boxed();
        let scroll = scrollable::scrollable(body).horizontal().spacing(2);

        container(scroll)
            .padding([12, 16])
            .width(Fill)
            .style(code_block)
            .into()
    }
}

/// Common state that each page owns via composition.
///
/// Sensor / scroll / code-expansion bookkeeping uses a FLAT index space
/// keyed by `flatten(blocks)` visit order. Top-level blocks AND every
/// nested child of a `Block::Group` each get their own flat index, their
/// own widget [`Id`], and their own sensor — clicking a TOC entry for a
/// nested example scrolls to the example, not its parent group, and
/// expanding a child example's code block doesn't ripple to siblings.
pub struct State {
    pub page_index: usize,
    load: Cell<Load>,
    descriptions: OnceCell<HashMap<&'static str, Vec<markdown::Item>>>,
    /// Parsed markdown items keyed by path. For `Block::Example`, this is
    /// the code-snippet body. For `Block::Md`, this is the body with the
    /// first heading peeled off (rendered separately as a depth-aware heading).
    pub md_items: HashMap<&'static str, Option<Vec<markdown::Item>>>,
    /// Raw markdown source keyed by path — kept for the copy button.
    pub md_raw: HashMap<&'static str, String>,
    /// Extracted first-heading title for `Block::Md` paths. `None` if not
    /// loaded yet; `Some("")` if the file had no heading.
    pub md_titles: HashMap<&'static str, Option<String>>,
    /// Sections in the last mounted table of contents, indexed by flat block position.
    pub visible: BTreeSet<usize>,
    /// Per-flat-block widget ids for `scroll_into_view` and sensors.
    /// `block_ids.len() == flatten(blocks).len()`.
    pub block_ids: Vec<Id>,
    pub scrollable_id: Id,
    /// FLAT block positions whose code block is expanded.
    pub code_expanded: BTreeSet<usize>,
    blocks: &'static [Block],
}

#[derive(Clone, Copy)]
enum Load {
    Unvisited,
    Loading { section: Option<usize> },
    Ready,
}

impl State {
    pub fn new(page_index: usize, blocks: &'static [Block]) -> Self {
        let flat_count = flatten(blocks).len();
        Self {
            page_index,
            load: Cell::new(Load::Unvisited),
            descriptions: OnceCell::new(),
            md_items: HashMap::new(),
            md_raw: HashMap::new(),
            md_titles: HashMap::new(),
            visible: BTreeSet::new(),
            block_ids: (0..flat_count).map(|_| Id::unique()).collect(),
            scrollable_id: Id::unique(),
            code_expanded: BTreeSet::new(),
            blocks,
        }
    }

    fn descriptions(&self) -> &HashMap<&'static str, Vec<markdown::Item>> {
        self.descriptions.get_or_init(|| {
            flatten(self.blocks)
                .into_iter()
                .filter_map(|(block, _)| match block {
                    Block::Example {
                        description: Some(copy),
                        ..
                    } => Some((*copy, markdown::parse(copy).collect())),
                    _ => None,
                })
                .collect()
        })
    }

    fn title(&self, path: &str) -> Option<&str> {
        if let Some(loaded) = self.md_titles.get(path) {
            return loaded.as_deref().filter(|s| !s.is_empty());
        }
        let i = bundled::HEADINGS
            .binary_search_by_key(&path, |&(key, _)| key)
            .ok()?;
        Some(bundled::HEADINGS[i].1)
    }

    pub(crate) fn unvisited(&self) -> bool {
        matches!(self.load.get(), Load::Unvisited)
    }

    /// Collapse the code panels and forget any pending section jump when leaving.
    pub fn reset_volatile(&mut self) {
        self.code_expanded.clear();
        if matches!(self.load.get(), Load::Loading { .. }) {
            self.load.set(Load::Loading { section: None });
        }
    }

    pub(crate) fn section(&self, fragment: &str) -> Option<usize> {
        if fragment.is_empty() {
            return Some(0);
        }
        flatten(self.blocks).iter().position(|(block, _)| {
            if block
                .toc_title(self)
                .is_some_and(|title| slug(title) == fragment)
            {
                return true;
            }
            match block {
                Block::Example { id, .. } => *id == fragment,
                Block::Md(path) => Path::new(path)
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .is_some_and(|stem| {
                        stem.strip_suffix(fragment)
                            .is_some_and(|p| p.ends_with('-'))
                    }),
                _ => false,
            }
        })
    }

    pub fn init(&self) -> Task<super::Message> {
        if !self.unvisited() {
            return Task::none();
        }
        self.load.set(Load::Loading { section: None });
        self.descriptions();
        let page = self.page_index;
        let mut paths = Vec::new();
        for_each_path(self.blocks, &mut |path| paths.push(path));
        paths.sort_unstable();
        paths.dedup();
        Task::perform(
            async move {
                let mut sources = Vec::with_capacity(paths.len());
                for path in paths {
                    if let Ok(source) = load_snippet(path).await {
                        sources.push((path, source));
                    }
                }
                sources
            },
            move |sources| super::Message::Page(page, Message::Loaded(sources)),
        )
    }

    pub fn update(&mut self, event: &Message) -> Task<super::Message> {
        use icedcn::widget::operation::scroll;

        match event {
            Message::Loaded(sources) => {
                let articles: BTreeSet<_> = flatten(self.blocks)
                    .into_iter()
                    .filter_map(|(block, _)| {
                        if let Block::Md(path) = block {
                            Some(*path)
                        } else {
                            None
                        }
                    })
                    .collect();
                for (path, content) in sources {
                    let path = *path;
                    let mut body = content.as_str();
                    if articles.contains(path) {
                        let title = heading::split(content).map(|(title, remainder)| {
                            body = remainder;
                            title.to_owned()
                        });
                        self.md_titles.insert(path, title);
                    }
                    self.md_raw.insert(path, content.clone());
                    self.md_items
                        .insert(path, Some(markdown::parse(body).collect()));
                }
                match self.load.replace(Load::Ready) {
                    Load::Loading { section: Some(i) } => {
                        Task::done(super::Message::Page(self.page_index, Message::ScrollTo(i)))
                    }
                    _ => Task::none(),
                }
            }
            Message::ScrollTranscript(id, target, animation) => {
                scroller::scroll_to(id.clone(), *target, *animation)
            }
            Message::Notice(title, detail) => {
                toast::show(toast::toast::<super::Message>(title).description(detail))
            }
            Message::ChooseFile => {
                let page = self.page_index;
                Task::perform(
                    async {
                        rfd::AsyncFileDialog::new()
                            .pick_file()
                            .await
                            .map(|file| file.file_name())
                    },
                    move |file| super::Message::Page(page, Message::FileSelected(file)),
                )
            }
            Message::FileSelected(_) | Message::CodeBlock(_) => Task::none(),
            Message::LinkClicked(url) => {
                if let Some(fragment) = url.strip_prefix('#') {
                    return self
                        .section(fragment)
                        .map_or_else(Task::none, |index| self.update(&Message::ScrollTo(index)));
                }
                open_link(url)
            }
            Message::VisibleSections(indices) => {
                self.visible.clone_from(indices);
                Task::none()
            }
            Message::ScrollTo(idx) if matches!(self.load.get(), Load::Loading { .. }) => {
                self.load.set(Load::Loading {
                    section: Some(*idx),
                });
                Task::none()
            }
            Message::ScrollTo(idx) => self
                .block_ids
                .get(*idx)
                .cloned()
                .map(|id| scroll::scroll_into_view(self.scrollable_id.clone(), id))
                .unwrap_or_else(Task::none),
            Message::ExpandCode(idx) => {
                self.code_expanded.insert(*idx);
                Task::none()
            }
            Message::CopyCode(idx) => {
                let code = flatten(self.blocks)
                    .get(*idx)
                    .and_then(|(b, _)| match b {
                        Block::Example { snippet, .. } if !snippet.is_empty() => Some(*snippet),
                        Block::Example { .. }
                        | Block::Md(_)
                        | Block::Inline { .. }
                        | Block::Group { .. } => None,
                    })
                    .and_then(|p| self.md_raw.get(p))
                    .map(|raw| extract_code_from_markdown(raw))
                    .unwrap_or_default();
                let pi = self.page_index;
                iced::clipboard::write(code).map(move |_| super::Message::Page(pi, Message::Noop))
            }
            Message::Noop => Task::none(),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
async fn load_snippet(path: &str) -> Result<String, String> {
    let name = path
        .strip_prefix("examples/browser/")
        .ok_or_else(|| format!("Unknown snippet: {path}"))?;
    let file = Path::new(env!("CARGO_MANIFEST_DIR")).join(name);
    read_to_string(file)
        .await
        .map_err(|error| error.to_string())
}

#[cfg(target_arch = "wasm32")]
async fn load_snippet(path: &str) -> Result<String, String> {
    bundled::snippet(path)
        .map(str::to_owned)
        .ok_or_else(|| format!("Unknown snippet: {path}"))
}

pub(crate) fn slug(title: &str) -> String {
    title.to_lowercase().replace(' ', "-")
}

/// Route documentation links to installed pages and external URLs to the system handler.
pub fn open_link(url: &str) -> Task<super::Message> {
    if let Some(target) = url
        .strip_prefix("icedcn://component/")
        .or_else(|| url.strip_prefix("icedcn://guide/"))
    {
        return Task::done(super::Message::OpenComponent(target.to_owned()));
    }
    if url.starts_with("https://") || url.starts_with("http://") || url.starts_with("mailto:") {
        #[cfg(target_arch = "wasm32")]
        if let Some(window) = web_sys::window() {
            let _ =
                window.open_with_url_and_target_and_features(url, "_blank", "noopener,noreferrer");
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let url = url.to_owned();
            return Task::perform(async move { open::that_detached(url) }, |result| {
                if let Err(error) = result {
                    eprintln!("Could not open link: {error}");
                }
            })
            .discard();
        }
    }
    Task::none()
}

/// Extract the contents of the FIRST fenced code block from a markdown
/// source — strips the opening ```lang fence and the closing ```. Falls
/// back to the full input if no fence is present.
pub fn extract_code_from_markdown(raw: &str) -> String {
    let mut inside = false;
    let mut out: Vec<&str> = Vec::new();
    for line in raw.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") {
            if inside {
                break;
            }
            inside = true;
            continue;
        }
        if inside {
            out.push(line);
        }
    }
    if out.is_empty() {
        raw.to_string()
    } else {
        out.join("\n")
    }
}
