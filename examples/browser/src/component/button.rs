// RTL is omitted under the project component policy.
pub(crate) mod default;
mod destructive;
mod ghost;
mod icon;
mod icons;
mod link;
mod outline;
mod rounded;
mod secondary;
mod size;
mod spinner;
mod with_icon;

use iced::Task;

use icedxdy::widget::{button, row};
use icedxdy::{Slot, Theme};

use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Button",
    description: "Displays a button or a component that looks like a button.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/button-demo.md",
    },
    page::Block::Md("examples/browser/snippets/button-installation.md"),
    page::Block::Md("examples/browser/snippets/button-usage.md"),
    page::Block::Md("examples/browser/snippets/button-cursor.md"),
    page::Block::Example {
        id: "size",
        title: "Size",
        description: Some(
            "Use the `size` prop to change the size of the button. In Rust, use `.size(...)`.",
        ),
        snippet: "examples/browser/snippets/button-size.md",
    },
    page::Block::Example {
        id: "default",
        title: "Default",
        description: None,
        snippet: "examples/browser/snippets/button-default.md",
    },
    page::Block::Example {
        id: "outline",
        title: "Outline",
        description: None,
        snippet: "examples/browser/snippets/button-outline.md",
    },
    page::Block::Example {
        id: "secondary",
        title: "Secondary",
        description: None,
        snippet: "examples/browser/snippets/button-secondary.md",
    },
    page::Block::Example {
        id: "ghost",
        title: "Ghost",
        description: None,
        snippet: "examples/browser/snippets/button-ghost.md",
    },
    page::Block::Example {
        id: "destructive",
        title: "Destructive",
        description: None,
        snippet: "examples/browser/snippets/button-destructive.md",
    },
    page::Block::Example {
        id: "link",
        title: "Link",
        description: None,
        snippet: "examples/browser/snippets/button-link.md",
    },
    page::Block::Example {
        id: "icon",
        title: "Icon",
        description: None,
        snippet: "examples/browser/snippets/button-icon.md",
    },
    page::Block::Example {
        id: "with-icon",
        title: "With Icon",
        description: Some(
            "Remember to add the `data-icon=\"inline-start\"` or `data-icon=\"inline-end\"` attribute to the icon for the correct spacing. In Rust, use `.icon_start()` or `.icon_end()`.",
        ),
        snippet: "examples/browser/snippets/button-with-icon.md",
    },
    page::Block::Example {
        id: "rounded",
        title: "Rounded",
        description: Some(
            "Use the `rounded-full` class to make the button rounded. In Rust, use `.rounded()`.",
        ),
        snippet: "examples/browser/snippets/button-rounded.md",
    },
    page::Block::Example {
        id: "spinner",
        title: "Spinner",
        description: Some(
            "Render a `<Spinner />` component inside the button to show a loading state. Remember to add the `data-icon=\"inline-start\"` or `data-icon=\"inline-end\"` attribute to the spinner for the correct spacing. In Rust, use `.icon_start()` or `.icon_end()`.",
        ),
        snippet: "examples/browser/snippets/button-spinner.md",
    },
    page::Block::Example {
        id: "button-group",
        title: "Button Group",
        description: Some(
            "To create a button group, use the `ButtonGroup` component. See the [Button Group](https://ui.shadcn.com/docs/components/radix/button-group) documentation for more details.",
        ),
        snippet: "examples/browser/snippets/button-mail-actions.md",
    },
    page::Block::Example {
        id: "aschild",
        title: "As Child",
        description: Some(
            "You can use the `asChild` prop on `<Button />` to make another component look like a button. Here's an example of a link that looks like a button. In Rust, `.anchor()` provides link styling and `on_press` publishes the navigation message.",
        ),
        snippet: "examples/browser/snippets/button-aschild.md",
    },
    page::Block::Group {
        title: "API Reference",
        children: &[page::Block::Md(
            "examples/browser/snippets/button-api-reference.md",
        )],
    },
];

pub struct ButtonPage {
    state: page::State,
    wide: bool,
}

impl ButtonPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
            wide: true,
        })
    }
}

fn demo() -> Slot<'static, crate::Message> {
    // Verbatim shadcn `button-demo.tsx`:
    //   <Button variant="outline">Button</Button>
    //   <Button variant="outline" size="icon"><ArrowUpIcon /></Button>
    row![
        button("Button").style(button::Outline),
        button(icons::arrow_up(16.0))
            .square()
            .style(button::Outline),
    ]
    .spacing(8)
    .into()
}

fn view(id: &str, wide: bool) -> Slot<'static, crate::Message> {
    match id {
        "size" => size::view(wide),
        "default" => default::view(),
        "outline" => outline::view(),
        "secondary" => secondary::view(),
        "ghost" => ghost::view(),
        "destructive" => destructive::view(),
        "link" => link::view(),
        "icon" => icon::view(),
        "with-icon" => with_icon::view(),
        "rounded" => rounded::view(),
        "spinner" => spinner::view(),
        "button-group" => super::button_group::demo(),

        "aschild" => button("Login")
            .anchor()
            .on_press(crate::Message::OpenUrl("https://ui.shadcn.com/login"))
            .into(),

        _ => unreachable!("unknown button preview id: {id}"),
    }
}

impl page::Page for ButtonPage {
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
    fn scrollable_id(&self) -> iced::widget::Id {
        self.state.scrollable_id.clone()
    }

    fn init(&self) -> Task<crate::Message> {
        self.state.init()
    }

    fn update(&mut self, event: page::Message) -> Task<crate::Message> {
        self.state.update(&event)
    }

    fn reset_volatile(&mut self) {
        self.state.reset_volatile();
    }

    fn window_width(&mut self, width: f32) {
        self.wide = width >= 640.0;
    }

    fn preview(&self, id: &str) -> iced::Element<'_, crate::Message, Theme> {
        if id == "demo" {
            return demo().into();
        }
        view(id, self.wide).into()
    }
}
