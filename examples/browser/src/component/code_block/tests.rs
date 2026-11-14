//! State regressions that do not need a window or clipboard access.
use super::*;
use tokio::runtime::Runtime;

#[test]
fn wrap_and_expansion_are_independent_and_repeatable() {
    let mut page = CodeBlockPage::at(0);
    for _ in 0..2 {
        let _ = page.update_demo(Event::Wrap("demo"));
        assert_eq!(page.wraps["demo"], true);
        let _ = page.update_demo(Event::Wrap("actions"));
        assert_eq!(page.wraps["actions"], false);
        let _ = page.update_demo(Event::Expand("show-more"));
        assert_eq!(page.expanded["show-more"], true);
        assert!(!page.expanded.contains_key("show-more-scroll"));
        let _ = page.update_demo(Event::Wrap("demo"));
        let _ = page.update_demo(Event::Wrap("actions"));
        let _ = page.update_demo(Event::Expand("show-more"));
        assert_eq!(page.wraps["demo"], false);
        assert_eq!(page.wraps["actions"], true);
        assert_eq!(page.expanded["show-more"], false);
    }
}

#[test]
fn repeated_copy_rejects_stale_reset_and_cancel_does_not_claim_save() {
    let runtime = Runtime::new().expect("test runtime");
    let _guard = runtime.enter();
    let mut page = CodeBlockPage::at(0);
    let first = Instant::now();
    let second = first + Duration::from_millis(500);
    let _ = page.update_demo(Event::Copied("demo", first));
    let _ = page.update_demo(Event::Copied("demo", second));
    let _ = page.update_demo(Event::ResetCopy("demo", first));
    assert_eq!(page.copied["demo"], second);
    let _ = page.update_demo(Event::ResetCopy("demo", second));
    assert!(!page.copied.contains_key("demo"));
    let _ = page.update_demo(Event::Saved("route.ts", Ok(false)));
    assert!(page.save_status.is_empty());
    let _ = page.update_demo(Event::Saved("route.ts", Ok(true)));
    assert_eq!(page.save_status["route.ts"], "Saved route.ts");
}

#[test]
fn replay_tracks_all_five_blocks_and_markdown_fence_independently() {
    let mut page = CodeBlockPage::at(0);
    assert!(page.streaming.is_empty());
    assert_eq!(STREAMS.len(), 5);
    assert_eq!(
        total_length("streaming"),
        STREAMS.iter().map(|(_, _, s)| s.len()).sum::<usize>()
    );
    let _ = page.update_demo(Event::Replay("streaming"));
    let _ = page.update_demo(Event::Replay("markdown"));
    assert_eq!(page.streaming.len(), 2);
    page.streaming
        .insert("streaming", Instant::now() - Duration::from_secs(120));
    let _ = page.update_demo(Event::Tick);
    assert!(!page.streaming.contains_key("streaming"));
    assert!(page.streaming.contains_key("markdown"));
    let opening = MARKDOWN.find("```tsx\n").expect("opening fence") + 7;
    let (before, after) = markdown_parts(opening + 12);
    assert_eq!(before.len(), 1);
    assert!(after.is_empty());
    let (before, after) = markdown_parts(usize::MAX);
    assert_eq!(before.len(), 1);
    assert_eq!(after.len(), 1);
    page.streaming
        .insert("markdown", Instant::now() - Duration::from_secs(120));
    let _ = page.update_demo(Event::Tick);
    assert!(page.streaming.is_empty());
    let _ = page.update_demo(Event::Replay("streaming"));
    assert!(page.streaming.contains_key("streaming"));
}

#[test]
fn full_sources_and_diff_decoration_survive_recipe_transforms() {
    let page = CodeBlockPage::at(0);
    assert_eq!(page.logs.lines().count(), 40);
    assert!(page.logs.ends_with("[40] GET /api/chat 200 in 45ms"));
    assert_eq!(diff_style("+added"), LineStyle::Added);
    assert_eq!(diff_style("-removed"), LineStyle::Removed);
    assert_eq!(diff_style(" unchanged"), LineStyle::Default);
    assert!(SHOW_MORE.lines().count() > 20);
    let (_, copy) = highlight::ansi(false);
    assert_eq!(copy.lines().count(), 8);
    assert!(!copy.contains('\x1b'));
    assert!(copy.ends_with("Tests  1 failed | 13 passed (14)"));
}

#[test]
fn markdown_uses_word_plus_whitespace_deltas() {
    assert_eq!(markdown_prefix(0), 0);
    assert_eq!(&MARKDOWN[..markdown_prefix(1)], "Wrap ");
    assert_eq!(&MARKDOWN[..markdown_prefix(2)], "Wrap the ");
    assert_eq!(markdown_prefix(usize::MAX), MARKDOWN.len());
}
