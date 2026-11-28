use crate::icon;
use crate::page::{rhea, theme_blue};
use crate::{Message, page};
use iced::widget::Id;
use iced::widget::component::{Component, component};
use iced::widget::image;
use iced::{ContentFit, Fill, Widget};
use iced::{Element, Task};
use icedcn::Size;
use icedcn::widget::attachment::State;
use icedcn::widget::{attachment, button, column, container, dialog, spinner, tooltip};
use icedcn::{Theme, View};
use page::Block;
use std::collections::BTreeSet;
use std::sync::LazyLock;

pub static META: page::Meta = page::Meta {
    title: "Attachment",
    description: "Displays a file or image attachment with media, metadata, upload state, and actions.",
};
pub static BLOCKS: &[Block] = &[
    Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/attachment-demo.md",
    },
    Block::Md("examples/browser/snippets/attachment-introduction.md"),
    Block::Md("examples/browser/snippets/attachment-usage.md"),
    Block::Md("examples/browser/snippets/attachment-composition.md"),
    Block::Md("examples/browser/snippets/attachment-features.md"),
    Block::Example {
        id: "image",
        title: "Image",
        description: Some(
            "Use `.media(image(handle))` for an image preview. Set `.vertical(true)` to stack the image above the filename and metadata.",
        ),
        snippet: "examples/browser/snippets/attachment-image.md",
    },
    Block::Example {
        id: "states",
        title: "States",
        description: Some(
            "Set `.state(...)` to reflect the upload lifecycle. `Uploading` and `Processing` shimmer the title, and `Error` switches to a destructive treatment.",
        ),
        snippet: "examples/browser/snippets/attachment-states.md",
    },
    Block::Example {
        id: "sizes",
        title: "Sizes",
        description: Some(
            "Use `.size(...)` to switch between `Size::Default`, `Size::Sm`, and `Size::Xs`.",
        ),
        snippet: "examples/browser/snippets/attachment-sizes.md",
    },
    Block::Example {
        id: "group",
        title: "Group",
        description: Some(
            "Wrap attachments in `attachment::group` to lay them out in a horizontally scrollable, snapping row with an edge fade.",
        ),
        snippet: "examples/browser/snippets/attachment-group.md",
    },
    Block::Example {
        id: "trigger",
        title: "Trigger",
        description: Some(
            "Add `.trigger(...)` to make the whole card open a link or dialog. It fills the card behind the actions, so the actions stay clickable.",
        ),
        snippet: "examples/browser/snippets/attachment-trigger.md",
    },
    Block::Md("examples/browser/snippets/attachment-trigger-composition.md"),
    Block::Group {
        title: "Accessibility",
        children: &[
            Block::Md("examples/browser/snippets/attachment-accessibility.md"),
            Block::Md("examples/browser/snippets/attachment-label-actions.md"),
            Block::Md("examples/browser/snippets/attachment-label-trigger.md"),
            Block::Md("examples/browser/snippets/attachment-keyboard-scrolling.md"),
            Block::Md("examples/browser/snippets/attachment-meaning.md"),
        ],
    },
    Block::Group {
        title: "API Reference",
        children: &[
            Block::Md("examples/browser/snippets/attachment-api-reference.md"),
            Block::Md("examples/browser/snippets/attachment-api-media.md"),
            Block::Md("examples/browser/snippets/attachment-api-content.md"),
            Block::Md("examples/browser/snippets/attachment-api-title.md"),
            Block::Md("examples/browser/snippets/attachment-api-description.md"),
            Block::Md("examples/browser/snippets/attachment-api-actions.md"),
            Block::Md("examples/browser/snippets/attachment-api-action.md"),
            Block::Md("examples/browser/snippets/attachment-api-trigger.md"),
            Block::Md("examples/browser/snippets/attachment-api-group.md"),
        ],
    },
];
pub struct AttachmentPage {
    state: page::State,
    wide: bool,
}
impl AttachmentPage {
    pub fn new(index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(index, BLOCKS),
            wide: true,
        })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Example {
    Demo,
    Image,
    States,
    Sizes,
    Group,
    Trigger,
}
struct Files(Example);
#[derive(Default)]
struct FilesState {
    removed: BTreeSet<&'static str>,
    retried: bool,
}
#[derive(Debug, Clone)]
enum Edit {
    Remove(&'static str),
    Retry,
    Open(&'static str),
    Copy,
}

const PHOTOS: [(&str, &str, &str); 3] = [
    (
        "workspace.png",
        "PNG · 820 KB",
        "https://images.unsplash.com/photo-1497366754035-f200968a6e72?w=900&auto=format&fit=crop&q=80",
    ),
    (
        "desk-reference.jpg",
        "JPG · 1.1 MB",
        "https://images.unsplash.com/photo-1497215728101-856f4ea42174?w=900&auto=format&fit=crop&q=80",
    ),
    (
        "office-reference.jpg",
        "JPG · 940 KB",
        "https://images.unsplash.com/photo-1497366811353-6870744d04b2?w=900&auto=format&fit=crop&q=80",
    ),
];
fn picture(index: usize, side: u32) -> View<'static, Edit> {
    static HANDLES: LazyLock<[image::Handle; 3]> = LazyLock::new(|| {
        [
            image::Handle::from_bytes(
                include_bytes!("../../assets/attachments/workspace.jpg").as_slice(),
            ),
            image::Handle::from_bytes(
                include_bytes!("../../assets/attachments/desk.jpg").as_slice(),
            ),
            image::Handle::from_bytes(
                include_bytes!("../../assets/attachments/office.jpg").as_slice(),
            ),
        ]
    });
    image(HANDLES[index].clone())
        .width(side)
        .height(side)
        .content_fit(ContentFit::Cover)
        .border_radius(10)
        .boxed()
        .into()
}
fn action(name: &'static str, cancel: bool) -> View<'static, Edit> {
    tooltip(
        attachment::action(icon::x().size(12).selectable(false))
            .style(rhea::button(button::Ghost))
            .on_press(Edit::Remove(name)),
        if cancel {
            "Cancel upload".into()
        } else {
            format!("Remove {name}")
        },
    )
    .into()
}
fn canvas(theme: &Theme) -> icedcn::Color {
    if theme.neutral.is_dark {
        theme.neutral.base
    } else {
        icedcn::core::oklch(0.98, 0., 0.)
    }
}
fn photos(state: &FilesState, interactive: bool) -> View<'static, Edit> {
    attachment::group(
        PHOTOS
            .into_iter()
            .enumerate()
            .filter(|(_, p)| !state.removed.contains(p.0))
            .map(|(i, (name, meta, url))| {
                let mut card = attachment(name)
                    .media(picture(i, 102))
                    .description(meta)
                    .vertical(true);
                if interactive {
                    card = card.action(action(name, false)).trigger(move |body| {
                        attachment::trigger(body).on_press(Edit::Open(url)).into()
                    });
                }
                card.into()
            }),
    )
    .fade_to(canvas)
    .into()
}
fn status_card(
    name: &'static str,
    meta: &'static str,
    state: State,
    media: View<'static, Edit>,
) -> View<'static, Edit> {
    let mut card = attachment(name).description(meta).state(state).media(media);
    if state == State::Error {
        card = card.action(tooltip(
            attachment::action(icon::refresh_cw().size(12).selectable(false))
                .style(rhea::button(button::Ghost))
                .on_press(Edit::Retry),
            "Retry upload",
        ));
    }
    card.action(action(name, state == State::Uploading)).into()
}
fn states_list(state: &FilesState) -> Vec<View<'static, Edit>> {
    use State::*;
    let (s, description, glyph) = if state.retried {
        (
            Done,
            "Uploaded",
            icon::check().size(16).selectable(false).into(),
        )
    } else {
        (
            Error,
            "Upload failed. Try again.",
            icon::file_warning().size(16).selectable(false).into(),
        )
    };
    [
        (
            "selected-file.pdf",
            "Ready to upload",
            Idle,
            icon::clock().size(16).selectable(false).into(),
        ),
        (
            "design-system.zip",
            "Uploading · 64%",
            Uploading,
            spinner().size(16.).into(),
        ),
        (
            "market-research.pdf",
            "Processing document",
            Processing,
            icon::file_text().size(16).selectable(false).into(),
        ),
        ("financial-model.xlsx", description, s, glyph),
        (
            "uploaded-report.pdf",
            "Uploaded · 1.8 MB",
            Done,
            icon::check().size(16).selectable(false).into(),
        ),
    ]
    .into_iter()
    .filter(|(name, ..)| !state.removed.contains(name))
    .map(|(name, meta, s, glyph)| status_card(name, meta, s, glyph))
    .collect()
}
fn tiles(state: &FilesState) -> View<'static, Edit> {
    let items = [
        (
            "briefing-notes.pdf",
            "PDF · 1.4 MB",
            icon::file_text().size(16).selectable(false).into(),
        ),
        ("workspace.png", "PNG · 820 KB", picture(0, 40)),
        (
            "customers.csv",
            "CSV · 18 KB",
            icon::table().size(16).selectable(false).into(),
        ),
        (
            "renderer.rs",
            "Rust · 12 KB",
            icon::file_code().size(16).selectable(false).into(),
        ),
    ];
    attachment::group(
        items
            .into_iter()
            .filter(|(name, ..)| !state.removed.contains(name))
            .map(|(name, meta, media): (_, _, View<'static, Edit>)| {
                attachment(name)
                    .media(media)
                    .description(meta)
                    .width(256)
                    .action(action(name, false))
                    .into()
            }),
    )
    .fade_to(canvas)
    .into()
}
fn preview_dialog() -> View<'static, Edit> {
    let name = "research-summary.pdf";
    dialog(
        move |d| {
            let opener = *d;
            attachment(name)
                .description("Open preview dialog")
                .media(icon::file_search().size(16).selectable(false))
                .trigger(move |body| opener.trigger(attachment::trigger(body)))
                .action(tooltip(
                    attachment::action(icon::copy().size(12).selectable(false))
                        .style(rhea::button(button::Ghost))
                        .on_press(Edit::Copy),
                    "Copy link",
                ))
                .action(action(name, false))
                .into()
        },
        move |_| dialog::header(name, "The attachment trigger fills the card and opens the dialog, while the actions stay independently clickable above it."),
    )
    .max_width(448.)
    .padding(rhea::DIALOG_PADDING)
    .style(rhea::dialog)
    .into()
}
impl<'a> Component<'a, Message, Theme> for Files {
    type State = FilesState;
    type Event = Edit;
    fn update(&self, state: &mut FilesState, event: Edit, _: &iced::Renderer) -> Option<Message> {
        match event {
            Edit::Remove(name) => {
                state.removed.insert(name);
            }
            Edit::Retry => state.retried = true,
            Edit::Open(url) => return Some(Message::OpenUrl(url)),
            Edit::Copy => {
                return Some(Message::CopyText(
                    "icedcn://component/attachment#trigger".into(),
                ));
            }
        }
        None
    }
    fn view(&self, state: &FilesState) -> impl Widget<Edit, Theme> + 'a {
        let mut list = Vec::new();
        match self.0 {
            Example::Demo => {
                list.push(photos(state, false));
                if !state.removed.contains("sales-dashboard.pdf") {
                    list.push(status_card(
                        "sales-dashboard.pdf",
                        "Uploading · 64%",
                        State::Uploading,
                        spinner().size(16.).into(),
                    ));
                }
                if !state.removed.contains("message_renderer.rs") {
                    list.push(status_card(
                        "message_renderer.rs",
                        "Rust · 12 KB",
                        State::Done,
                        icon::file_code().size(16).selectable(false).into(),
                    ));
                }
            }
            Example::Image => list.push(photos(state, true)),
            Example::States => list = states_list(state),
            Example::Group => list.push(tiles(state)),
            Example::Trigger => {
                if !state.removed.contains("research-summary.pdf") {
                    list.push(preview_dialog());
                }
            }
            Example::Sizes => {
                for (name, size) in [
                    ("Default attachment", Size::Default),
                    ("Small attachment", Size::Sm),
                    ("Extra small attachment", Size::Xs),
                ] {
                    let mut a = attachment(name)
                        .media(
                            icon::file_text()
                                .size(if size == Size::Xs { 14 } else { 16 })
                                .selectable(false),
                        )
                        .size(size);
                    if size != Size::Xs {
                        a = a.description("PDF · 2.4 MB");
                    }
                    list.push(a.into());
                }
            }
        }
        let gap = if self.0 == Example::States { 8 } else { 12 };
        Element::from(
            container(column(list).spacing(gap).width(Fill))
                .width(Fill)
                .max_width(384)
                .padding([48, 0]),
        )
    }
}

fn demo_example() -> View<'static, Message> {
    component(Files(Example::Demo)).boxed().into()
}

fn image_example() -> View<'static, Message> {
    component(Files(Example::Image)).boxed().into()
}

fn states_example() -> View<'static, Message> {
    component(Files(Example::States)).boxed().into()
}

fn sizes_example() -> View<'static, Message> {
    component(Files(Example::Sizes)).boxed().into()
}

fn group_example() -> View<'static, Message> {
    component(Files(Example::Group)).boxed().into()
}

fn trigger_example() -> View<'static, Message> {
    component(Files(Example::Trigger)).boxed().into()
}

impl page::Page for AttachmentPage {
    fn page_index(&self) -> usize {
        self.state.page_index
    }
    fn meta(&self) -> &'static page::Meta {
        &META
    }
    fn blocks(&self) -> &'static [page::Block] {
        BLOCKS
    }
    fn state(&self) -> &page::State {
        &self.state
    }
    fn scrollable_id(&self) -> Id {
        self.state.scrollable_id.clone()
    }
    fn init(&self) -> Task<Message> {
        self.state.init()
    }
    fn update(&mut self, event: page::Message) -> Task<Message> {
        self.state.update(&event)
    }
    fn reset_volatile(&mut self) {
        self.state.reset_volatile();
    }
    fn window_width(&mut self, width: f32) {
        self.wide = width >= 768.;
    }
    fn backdrop(&self) -> page::Backdrop {
        page::Backdrop {
            min_height: 0.,
            padding: iced::Padding::from([40, if self.wide { 40 } else { 32 }]),
            ..theme_blue::backdrop()
        }
    }

    fn preview_theme(&self, outer: &Theme) -> Option<Theme> {
        Some(theme_blue::theme(outer))
    }

    fn preview(&self, id: &str) -> Element<'_, Message, Theme> {
        match id {
            "demo" => demo_example().into(),
            "image" => image_example().into(),
            "states" => states_example().into(),
            "sizes" => sizes_example().into(),
            "group" => group_example().into(),
            "trigger" => trigger_example().into(),
            _ => unreachable!("unknown attachment preview {id}"),
        }
    }
}
