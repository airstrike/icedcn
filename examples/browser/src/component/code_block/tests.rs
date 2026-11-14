//! State regressions that do not need a window or clipboard access.
use super::super::Event;
use super::*;

#[test]
fn wrap_and_expansion_are_independent_and_repeatable() {
    let mut page = State::new();
    for _ in 0..2 {
        let _ = page.update(Message::Wrap("demo"));
        assert_eq!(page.wraps["demo"], true);
        let _ = page.update(Message::Wrap("actions"));
        assert_eq!(page.wraps["actions"], false);
        let _ = page.update(Message::Expand("show-more"));
        assert_eq!(page.expanded["show-more"], true);
        assert!(!page.expanded.contains_key("show-more-scroll"));
        let _ = page.update(Message::Wrap("demo"));
        let _ = page.update(Message::Wrap("actions"));
        let _ = page.update(Message::Expand("show-more"));
        assert_eq!(page.wraps["demo"], false);
        assert_eq!(page.wraps["actions"], true);
        assert_eq!(page.expanded["show-more"], false);
    }
}

#[test]
fn repeated_copy_rejects_stale_reset_and_cancel_does_not_claim_save() {
    let mut page = State::new();
    let first = Instant::now();
    let second = first + Duration::from_millis(500);
    let _ = page.update(Message::Copied("demo", first));
    let _ = page.update(Message::Copied("demo", second));
    let _ = page.update(Message::ResetCopy("demo", first));
    assert_eq!(page.copied["demo"], second);
    let _ = page.update(Message::ResetCopy("demo", second));
    assert!(!page.copied.contains_key("demo"));
    let _ = page.update(Message::Saved("view.rs", Ok(false)));
    assert!(page.save_status.is_empty());
    let _ = page.update(Message::Saved("view.rs", Ok(true)));
    assert_eq!(page.save_status["view.rs"], "Saved view.rs");
}

#[test]
fn replay_tracks_all_five_blocks_and_markdown_fence_independently() {
    let mut page = State::new();
    assert!(page.streaming.is_empty());
    assert_eq!(STREAMS.len(), 5);
    assert_eq!(
        total_length("streaming"),
        STREAMS.iter().map(|(_, _, s)| s.len()).sum::<usize>()
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
fn full_sources_and_diff_decoration_survive_recipe_transforms() {
    let page = State::new();
    assert_eq!(page.logs.lines().count(), 40);
    assert!(page.logs.ends_with("[40] Indexed notebook in 45ms"));
    assert_eq!(
        code_block::diff_text("+added\n-removed\n unchanged"),
        "added\nremoved\nunchanged"
    );
    assert!(SHOW_MORE.lines().count() > 20);
    use code_block::ansi;
    let output = ansi::parse(ANSI);
    let copy = output.as_str();
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
fn completion_receipts_are_targeted_replay_safe_and_not_lost_in_a_batch() {
    let mut first = State::new();
    let mut second = State::new();
    let now = Instant::now();
    let a = Completion {
        target: first.target.clone(),
        serial: Id::unique(),
        message: Message::Copied("view", now),
    };
    let b = Completion {
        target: first.target.clone(),
        serial: Id::unique(),
        message: Message::Copied("search", now),
    };
    for completion in [&a, &b] {
        first.receive(completion);
        second.receive(completion);
    }
    assert_eq!(first.copied.len(), 2);
    assert!(second.copied.is_empty());
    first.advance(now + Duration::from_secs(2));
    assert!(first.copied.is_empty());
    first.receive(&a);
    assert!(
        first.copied.is_empty(),
        "a rebuild cannot replay an old receipt"
    );
    let Some(Event::Acknowledged(ids)) = first.update(Message::Acknowledge) else {
        panic!("acknowledgement expected");
    };
    assert_eq!(ids.len(), 2);
    assert!(first.acknowledgements.is_empty());
    let _ = first.update(Message::Wrap("demo"));
    assert!(first.wraps["demo"]);
    assert!(second.wraps.is_empty());
}
#[test]
fn chevron_rotates_reverses_continuously_and_settles() {
    let now = Instant::now();
    let mut turn = Turn::default();
    turn.retarget(1., now);
    assert_eq!(turn.value, 0.);
    turn.advance(now + Duration::from_millis(75));
    let midway = turn.value;
    assert!(midway > 0. && midway < 1.);
    turn.retarget(0., now + Duration::from_millis(75));
    assert_eq!(turn.value, midway);
    turn.advance(now + Duration::from_millis(150));
    assert!(turn.value > 0. && turn.value < midway);
    turn.advance(now + Duration::from_millis(225));
    assert_eq!(turn.value, 0.);
    assert!(turn.start.is_none());
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
        completions: Vec::new(),
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
    assert_eq!(STREAMS[1], ("Terminal", "sh", STREAMING_1));
    assert_eq!(STREAMS[4], (".cargo/config.toml", "toml", STREAMING_4));
}
