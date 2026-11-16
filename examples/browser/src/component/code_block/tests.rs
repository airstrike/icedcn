//! State regressions that do not need a window or clipboard access.
use super::super::Event;
use super::*;

#[test]
fn wrap_is_independent_across_examples_and_repeatable() {
    let mut page = State::new();
    for _ in 0..2 {
        let _ = page.update(Message::Wrap("demo"));
        assert!(page.wrap("demo"));
        let _ = page.update(Message::Wrap("actions"));
        assert!(!page.wrap("actions"));
        let _ = page.update(Message::Wrap("demo"));
        let _ = page.update(Message::Wrap("actions"));
        assert!(!page.wrap("demo"));
        assert!(page.wrap("actions"));
    }
}

#[test]
fn save_asks_the_page_to_write_the_file() {
    let mut page = State::new();
    let Some(Event::Save { name, value }) = page.update(Message::Save("view.rs", DEMO)) else {
        panic!("save effect expected");
    };
    assert_eq!((name, value), ("view.rs", DEMO));
}

#[test]
fn replay_tracks_all_five_blocks_and_markdown_fence_independently() {
    let mut page = State::new();
    assert!(page.streaming.is_empty());
    assert_eq!(STREAMS.len(), 5);
    assert_eq!(
        total_length("streaming"),
        STREAMS
            .iter()
            .map(|stream| stream.code.len())
            .sum::<usize>()
    );
    let _ = page.update(Message::Replay("streaming"));
    let _ = page.update(Message::Replay("markdown"));
    assert_eq!(page.streaming.len(), 2);
    page.streaming
        .insert("streaming", Instant::now() - Duration::from_secs(120));
    let _ = page.update(Message::Tick(Instant::now()));
    assert!(!page.streaming.contains_key("streaming"));
    assert!(page.streaming.contains_key("markdown"));
    let opening = MARKDOWN.find("```rust\n").expect("opening fence") + "```rust\n".len();
    let (before, after) = markdown_parts(opening + 12);
    assert_eq!(before.len(), 1);
    assert!(after.is_empty());
    let (before, after) = markdown_parts(usize::MAX);
    assert_eq!(before.len(), 1);
    assert_eq!(after.len(), 1);
    page.streaming
        .insert("markdown", Instant::now() - Duration::from_secs(120));
    let _ = page.update(Message::Tick(Instant::now()));
    assert!(page.streaming.is_empty());
    let _ = page.update(Message::Replay("streaming"));
    assert!(page.streaming.contains_key("streaming"));
}

#[test]
fn full_sources_and_line_states_survive_recipe_transforms() {
    let page = State::new();
    assert_eq!(page.logs.lines().count(), 40);
    assert!(page.logs.ends_with("[40] Indexed notebook in 45ms"));
    let styles: Vec<_> = DIFF.lines().map(diff_style).collect();
    assert_eq!(
        styles,
        [
            LineStyle::Default,
            LineStyle::Removed,
            LineStyle::Added,
            LineStyle::Added,
            LineStyle::Default,
        ]
    );
    assert!(SHOW_MORE.lines().count() > 20);
    let copy = ansi::strip(ANSI);
    assert_eq!(copy.lines().count(), 8);
    assert!(!copy.contains('\x1b'));
    assert!(copy.ends_with("test result: FAILED. 2 passed; 1 failed; 0 ignored"));
}

#[test]
fn markdown_uses_word_plus_whitespace_deltas() {
    assert_eq!(markdown_prefix(0), 0);
    assert_eq!(&MARKDOWN[..markdown_prefix(1)], "Normalize ");
    assert_eq!(&MARKDOWN[..markdown_prefix(2)], "Normalize the ");
    assert_eq!(markdown_prefix(usize::MAX), MARKDOWN.len());
}

#[test]
fn markdown_and_typeset_prose_remain_paragraphs() {
    for items in [
        &State::new().typeset_before,
        &State::new().typeset_after,
        &State::new().markdown_before,
        &State::new().markdown_after,
    ] {
        assert!(
            items
                .iter()
                .all(|item| matches!(item, markdown::Item::Paragraph(_)))
        );
    }
}

#[test]
fn local_timer_requests_the_next_frame_after_a_published_tick() {
    let mut state = State::new();
    let now = Instant::now();
    state.last_tick = now - Duration::from_millis(20);
    state
        .streaming
        .insert("streaming", now - Duration::from_millis(20));
    let component = Demo {
        id: "streaming".into(),
        dark: false,
        page: 0,
    };
    let action = component.listen(
        &state,
        &iced::Event::Window(window::Event::RedrawRequested(now)),
        iced::Rectangle::default(),
        mouse::Cursor::Unavailable,
    );
    let (event, redraw, _) = action.into_inner();
    assert!(matches!(event, Some(Message::Tick(at)) if at == now));
    assert!(matches!(redraw, window::RedrawRequest::At(at) if at > now));
}

#[test]
fn streaming_lists_keep_their_source_language() {
    assert_eq!((STREAMS[1].title, STREAMS[1].language), (None, "sh"));
    assert_eq!(
        (STREAMS[4].title, STREAMS[4].language),
        (Some(".cargo/config.toml"), "toml")
    );
}
