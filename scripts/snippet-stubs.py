"""Stubs for scripts/check-snippets.py.

Snippets show only what a reader writes, so some lean on names they
never define: the page's private helpers (PAGES, keyed by the page file
that shows the snippet), a local the surrounding view would have
(OV[key]['locals']), or another snippet's items (OV[key]['prelude'], with
{path:<snippet>#<block>} naming that snippet's module). A key is a
snippet file name and the index of its rust block.
"""
PAGES = {
'trace.rs': r'''
use icedcn::widget::trace::{self, Line, Status};
pub type Glyph = fn() -> Text<'static, Theme>;
pub fn line<M: 'static>(title: &'static str, glyph: Glyph) -> Line<'static, M> { todo!() }
pub fn result<M: Clone + 'static>(title: &'static str, glyph: Glyph, body: &'static str, status: Status, open: bool) -> View<'static, M> { todo!() }
pub fn source<M: Clone + 'static>(title: &'static str, href: &'static str) -> View<'static, M> { todo!() }
pub fn local<M: Clone + 'static>(title: &'static str, slug: &str, index: usize) -> View<'static, M> { todo!() }
pub fn file_diff<M: Clone + 'static>(path: &'static str, title: &'static str, stat: &'static str, desc: &'static str, src: &'static str, glyph: Glyph) -> View<'static, M> { todo!() }
pub const LOGIN_DIFF: &str = "";
pub const AUTH_DIFF: &str = "";
''',
'message.rs': r'''
pub fn frame<M>(body: impl Into<View<'static, M>>) -> View<'static, M> { todo!() }
pub fn portrait<M>(number: usize, fallback: &'static str) -> View<'static, M> { todo!() }
pub const INSTALL: &str = "";
''',
'drawer.rs': r'''
use icedcn::widget::drawer::Side;
pub const LOREM: &str = "";
pub fn panel<M>(label: &'static str, side: Side, fraction: f32) -> View<'static, M> { todo!() }
''',
'resizable.rs': r'''
pub fn panel<M>(label: &'static str) -> View<'static, M> { todo!() }
pub fn frame<M>(group: impl Into<View<'static, M>>, width: f32) -> View<'static, M> { todo!() }
''',
'scroll_area.rs': r'''
pub fn frame<'a, M>(content: impl Into<View<'a, M>>) -> View<'a, M> { todo!() }
''',
'context_menu.rs': r'''
pub fn with_icon<'a, M: 'a>(glyph: Text<'a, Theme>, label: &'a str) -> View<'a, M> { todo!() }
#[derive(Debug, Clone)]
pub enum Choice { Checked(usize, bool), Theme(&'static str), Bookmarks(bool), FullUrls(bool), Person(&'static str) }
#[derive(Clone, Copy)]
pub struct Preferences { pub checked: [bool; 3], pub person: &'static str, pub theme: &'static str }
''',
'menubar.rs': r'''
use std::collections::BTreeSet;
pub fn with_icon<M>(glyph: Text<'static, Theme>, label: &'static str) -> View<'static, M> { todo!() }
#[derive(Debug, Clone)]
pub enum Choice { Check(&'static str, bool), User(&'static str), Theme(&'static str) }
pub struct Preferences { pub checked: BTreeSet<&'static str>, pub user: &'static str, pub theme: &'static str }
''',
'avatar.rs': r'''
pub fn alex_handle() -> image::Handle { todo!() }
pub fn maxleiter_handle() -> image::Handle { todo!() }
pub fn evilrabbit_handle() -> image::Handle { todo!() }
''',
'toggle_group.rs': r'''
use icedcn::widget::toggle_group;
pub fn formatting<'a, M>() -> [toggle_group::Item<'a, M>; 3] { todo!() }
''',
}
for pg in ('empty.rs', 'item.rs'):
    PAGES[pg] = PAGES['avatar.rs']

PAGES['toggle_group.rs'] += "pub fn sides<'a, M>() -> [toggle_group::Item<'a, M>; 4] { todo!() }\n"
PAGES['context_menu.rs'] += "pub fn state() -> Preferences { todo!() }\n"
H = "let workspace: iced::widget::image::Handle = todo!(); let desk: iced::widget::image::Handle = todo!(); let office: iced::widget::image::Handle = todo!();"
OV = {
 'attachment-demo#0': {'locals': [H]},
 'attachment-image#0': {'locals': [H]},
 'attachment-group#0': {'locals': [H]},
 'code-block-actions#0': {'locals': ['let wrap: bool = todo!();']},
 'code-block-demo#0': {'locals': ['let wrap: bool = todo!();']},
 'code-block-actions-guide#0': {'locals': ['let wrap: bool = todo!(); let output: &str = todo!();']},
 'code-block-usage#1': {'locals': ['let output: &str = todo!();']},
 'code-block-syntax-guide#0': {'locals': ['let code: &str = todo!();']},
 'code-block-streaming#0': {'locals': ['let received: String = todo!(); let complete: bool = todo!();']},
 'code-block-header-guide#0': {'ret': "icedcn::widget::code_block::Header<'static, Message>"},
 'code-block-markdown#0': {'locals': ["let items: Vec<icedcn::widget::markdown::Item> = todo!(); let settings: icedcn::widget::markdown::Settings = todo!(); let reply = Reply { complete: true };"], 'ret': "iced::Element<'static, icedcn::widget::markdown::Uri, icedcn::Theme>"},
 'code-block-typeset#0': {'locals': ["let before: Vec<icedcn::widget::markdown::Item> = todo!(); let after: Vec<icedcn::widget::markdown::Item> = todo!(); let settings: icedcn::widget::markdown::Settings = todo!();"]},
 'context-menu-checkboxes#0': {'locals': ['let state = state();'], 'ret': "icedcn::View<'static, Choice>"},
 'context-menu-radio#0': {'locals': ['let state = state();'], 'ret': "icedcn::View<'static, Choice>"},
 'drawer-dialog#0': {'locals': ["let description: &str = todo!(); let form = |d: &icedcn::widget::dialog::Handle| -> icedcn::View<'static, Message> { todo!() };"]},
 'drawer-scrollable-content#0': {'locals': ["let label: &'static str = todo!(); let side: icedcn::widget::drawer::Side = todo!(); let fraction: f32 = todo!();"]},
 'drawer-demo#0': {'locals': ["let state: (Option<u32>,) = todo!();"]},
 'trace-demo#0': {'locals': ["let reason: &str = todo!();"]},
 'trace-thinking#0': {'locals': ["let reason: &str = todo!(); let running: bool = todo!();"]},
 'trace-link#0': {'locals': ["let index: usize = todo!();"]},
 'trace-steps#0': {'locals': ["let index: usize = todo!();"]},
 'trace-turn#0': {'locals': ["let index: usize = todo!();"]},
 'trace-open#0': {'locals': ["let open: bool = todo!();"], 'variants': {'Toggle': ['bool']}},
 'navigation-menu-demo#0': {'locals': ["let wide: bool = todo!();"]},
 'questionnaire-controlled#0': {'locals': ["let active: String = todo!();"]},
 'progress-usage#1': {'fields': {'value': 'f32'}},
}
for k in ('menubar-checkbox#0','menubar-demo#0','menubar-radio#0'):
    OV[k] = {'locals': ['let state: &Preferences = todo!();'], 'ret': "icedcn::View<'static, Choice>"}
ROOT_EXTRA = ['#[derive(Debug, Clone)] pub enum Message {}', 'mod payments { pub use {path:data-table-demo#0}::*; }']

import glob as _g, os as _o
Q = 'icedcn::widget::questionnaire::'
for _f in _g.glob(ROOT + '/examples/browser/snippets/questionnaire-*.md'):
    _k = _o.path.basename(_f)[:-3] + '#0'
    OV.setdefault(_k, {}).setdefault('Event', {}).update({'Submit': [Q + 'Answers'], 'Navigate': ['String']})
OV['questionnaire-progress#0'].setdefault('locals', []).append(f"let render_progress = |_: {Q}Progress| -> icedcn::View<'static, {Q}Event> {{ todo!() }};")
OV['questionnaire-resume#0'].setdefault('locals', []).append(f"let saved: {Q}Answers = todo!();")
OV['questionnaire-shortcuts#0'].setdefault('locals', []).append(f"let shortcuts: {Q}Shortcuts = todo!();")
OV['questionnaire-validation#0'].setdefault('locals', []).append(f"let validate_answers = |_: &{Q}Answers| -> Option<{Q}Invalid> {{ todo!() }};")
PAGES['navigation_menu.rs'] = "pub fn doc_link<M>(label: &'static str, description: &'static str, url: &'static str) -> View<'static, M> { todo!() }\n"
for _k in ('attachment-group#0', 'attachment-image#0', 'attachment-states#0', 'attachment-trigger#0'):
    OV.setdefault(_k, {}).setdefault('variants', {})['Remove'] = ["&'static str"]
OV['code-block-api-highlighting#0'] = {'skip': True}
OV['guide-installation-icons#0'] = {'skip': True}
OV['guide-installation-icons#1'] = {'skip': True}
OV['code-block-typeset#0']['ret'] = None
OV['code-block-typeset#0']['msg'] = 'icedcn::widget::markdown::Uri'
OV['drawer-demo#0']['msg'] = 'u32'
OV['message-actions#0'] = {'prelude': ['#[derive(Debug, Clone)] pub enum Action { Copy, Vote(bool) }', 'pub struct Votes { pub vote: Option<bool> }'], 'locals': ['let state: Votes = todo!();'], 'msg': 'Action'}
OV['navigation-menu-link-component#0'] = {'ret': "icedcn::widget::menubar::Entry<'static, Message>"}
OV['progress-controlled#0'] = {'ret': "iced::Element<'static, Message, icedcn::Theme>"}
OV['progress-demo#0'] = {'state_impl': 'fn hero_value(&self) -> f32 { todo!() }'}
OV['skeleton-text#0'] = {'prelude': ['use {path:skeleton-card#0}::line;']}
OV['input-file#0'] = {'prelude': ['use crate::rfd;']}
OV['empty-input-group#0'] = {}
ROOT_EXTRA += ['''mod rfd {
    pub struct AsyncFileDialog;
    pub struct FileHandle;
    impl FileHandle { pub fn file_name(&self) -> String { todo!() } }
    impl AsyncFileDialog {
        pub fn new() -> Self { todo!() }
        pub async fn pick_file(self) -> Option<FileHandle> { todo!() }
    }
}''']
for _k in ('context-menu-checkboxes#0', 'context-menu-radio#0', 'menubar-checkbox#0', 'menubar-demo#0', 'menubar-radio#0', 'progress-controlled#0'):
    OV[_k]['into'] = True
OV['attachment-image#0']['variants']['Open'] = ["&'static str"]
OV['skeleton-text#0'] = {'prelude': ['use {path:skeleton-card#0}::{Message, line};'], 'no_message': True}

for _k in ('code-block-show-more#0', 'code-block-show-more-scroll#0'):
    OV.setdefault(_k, {}).setdefault('prelude', []).append('use {path:code-block-demo#0}::Message;')
for _k in ('basic', 'destructive', 'groups', 'icons', 'shortcuts', 'submenu', 'checkboxes', 'radio'):
    OV.setdefault(f'context-menu-{_k}#0', {}).setdefault('prelude', []).append('use {path:context-menu-demo#0}::area;')
