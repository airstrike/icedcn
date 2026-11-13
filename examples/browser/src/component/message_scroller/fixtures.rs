//! Native copies of the scripted conversations in the upstream examples.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Role {
    User,
    Assistant,
    Guest,
    Marker,
}
#[derive(Clone, Copy)]
pub(super) struct Turn {
    pub role: Role,
    pub speaker: &'static str,
    pub text: &'static str,
}
use Role::{Assistant, Guest, Marker, User};
pub(super) const DEMO: &[Turn] = &[
    Turn {
        role: User,
        speaker: "",
        text: "I'm building a chat for our app and the scroll behavior is driving me nuts. Every time the AI streams a reply, the whole thread jumps around.",
    },
    Turn {
        role: Assistant,
        speaker: "",
        text: "That's the classic streaming scroll problem. Wrap your message list in `MessageScroller` and turn on `autoScroll` — the viewport pins to the bottom as tokens arrive, so users always see the latest text land in place.\n\nThe important part: it only auto-scrolls while the reader is already at the bottom. The moment they scroll up to read something earlier, auto-scroll backs off and their position is preserved. You get smooth streaming without fighting the user's intent.",
    },
    Turn {
        role: User,
        speaker: "",
        text: "Okay, but when someone sends a new message the view still feels jarring — like the whole conversation reloads from the top.",
    },
    Turn {
        role: Assistant,
        speaker: "",
        text: "MessageScrollerItem fixes that with turn anchoring. Set `scrollAnchor` on the turn that should settle near the top instead of blindly snapping to the document bottom.\n\nIt also leaves a small peek of the previous exchange visible above the anchor, so context isn't lost. The reply starts in view without that disorienting jump you get from a plain overflow container.",
    },
    Turn {
        role: User,
        speaker: "",
        text: "And if they've scrolled up to re-read an older answer? I don't want to yank them back down.",
    },
    Turn {
        role: Assistant,
        speaker: "",
        text: "You won't. Auto-scroll only runs when the viewport is already pinned to the bottom, so scrolling up is a deliberate opt-out — their place in the thread stays put even as new tokens keep arriving below.\n\nWhen there is content they haven't seen yet, `MessageScrollerButton` appears at the bottom of the viewport. One tap jumps them back to the newest message and re-engages auto-scroll. Same pattern as Slack or iMessage: quiet when you're caught up, helpful when you're not.",
    },
    Turn {
        role: User,
        speaker: "",
        text: "Last one — does this work with assistive tech?",
    },
    Turn {
        role: Assistant,
        speaker: "",
        text: "`MessageScrollerContent` sets `role=\"log\"` and `aria-relevant=\"additions\"` by default, so screen readers announce new messages as they stream in.\n\nThe scroll button is a real `<button>` with an sr-only label, and it's removed from the tab order when you're already at the bottom — no ghost focus stops.",
    },
];
pub(super) const ANCHORING: &[Turn] = &[
    Turn {
        role: User,
        speaker: "",
        text: "Can you show me how anchoring behaves when a new prompt starts the turn?",
    },
    Turn {
        role: Assistant,
        speaker: "",
        text: "Append the user prompt first, then append the assistant response. With User selected, the prompt settles near the top and the assistant response fills in below it.",
    },
    Turn {
        role: User,
        speaker: "",
        text: "What changes when assistant messages are the anchor?",
    },
    Turn {
        role: Assistant,
        speaker: "",
        text: "Now each assistant response is the item `MessageScroller` keeps in view. This is useful when the reply is the moment you want readers to land on after each turn.",
    },
    Turn {
        role: User,
        speaker: "",
        text: "Can I switch roles and keep adding turns?",
    },
    Turn {
        role: Assistant,
        speaker: "",
        text: "Yes. The next appended message with the selected role becomes the anchor, so you can compare user and assistant anchoring without resetting the demo.",
    },
];
pub(super) const GROUP_CHAT: &[Turn] = &[
    Turn {
        role: User,
        speaker: "Grace",
        text: "@mary, the astrophage line keeps matching Venus energy output. Can you check my math?",
    },
    Turn {
        role: Assistant,
        speaker: "Mary (Agent)",
        text: "Yes. Confirmed. The curve points to a microorganism harvesting stellar energy and breeding near carbon dioxide. If @rocky agrees, this is the clue we need.",
    },
    Turn {
        role: User,
        speaker: "Grace",
        text: "ping @rocky",
    },
    Turn {
        role: Marker,
        speaker: "",
        text: "Rocky has joined the chat",
    },
    Turn {
        role: Guest,
        speaker: "Rocky",
        text: "Amaze. Astrophage eats light, makes heat, goes to carbon dioxide. Rocky has fuel model. Grace is smart.",
    },
];
pub(super) const PREVIOUS_CONTEXT: &[Turn] = DEMO;
pub(super) const STREAMING: &[Turn] = DEMO;
pub(super) const OPENING_POSITION: &[Turn] = &[
    Turn {
        role: User,
        speaker: "",
        text: "This is the first message the user sent in the conversation.",
    },
    Turn {
        role: Assistant,
        speaker: "",
        text: "Workspace creation rose 8%, but first invite completion only rose 2%.",
    },
    Turn {
        role: User,
        speaker: "",
        text: "This is the last message the user sent in the conversation.",
    },
    Turn {
        role: Assistant,
        speaker: "",
        text: "Start with the invite step. Teams are creating workspaces but waiting to add collaborators.\n\nRecommended follow-up:\n\n1. Compare invite drop-off by account size.\n2. Check whether users who skip invites still return within 24 hours.\n3. Review the empty-state copy on the first project screen.\n4. Segment activation by template, since template users may not need invites right away.\n\nIf that pattern holds, the next experiment should make collaboration useful earlier instead of prompting for invites harder.",
    },
];
pub(super) const LOAD_HISTORY: &[Turn] = &[
    Turn {
        role: User,
        speaker: "",
        text: "Can you summarize the incident channel?",
    },
    Turn {
        role: Assistant,
        speaker: "",
        text: "The first alert was a delayed export job. It started backing up around 09:42 UTC and triggered the warning once the retry queue crossed the threshold.\n\nNo customer-facing checkout paths were affected, but exports for larger workspaces were running about 12 minutes behind.",
    },
    Turn {
        role: User,
        speaker: "",
        text: "Was checkout affected?",
    },
    Turn {
        role: Assistant,
        speaker: "",
        text: "No checkout errors were reported. Payment authorization, order creation, and confirmation emails stayed inside their normal latency bands.\n\nThe only elevated metric was export queue depth, which maps to analytics downloads instead of checkout.",
    },
    Turn {
        role: User,
        speaker: "",
        text: "What changed in the last deploy?",
    },
    Turn {
        role: Assistant,
        speaker: "",
        text: "Only the export queue worker changed. The deploy moved large CSV jobs onto the shared retry policy, which made each failed attempt hold a worker slot longer than before.\n\nThe app deploy did not include checkout, pricing, or billing API changes.",
    },
    Turn {
        role: User,
        speaker: "",
        text: "Do we need to roll back?",
    },
    Turn {
        role: Assistant,
        speaker: "",
        text: "Not yet. Queue depth is recovering after we reduced retry concurrency, and the oldest pending job is now under five minutes old.\n\nKeep rollback ready if the queue starts climbing again, but the current trend points toward recovery.",
    },
    Turn {
        role: User,
        speaker: "",
        text: "Keep watching for customer-visible issues.",
    },
    Turn {
        role: Assistant,
        speaker: "",
        text: "I will watch the queue and support tags for another 15 minutes. I am tracking export failures, delayed download requests, and any support thread that mentions missing reports.\n\nIf those stay quiet through the next batch window, we can close this as an internal degradation.",
    },
];
pub(super) const ANIMATION: &[Turn] = &[
    Turn {
        role: User,
        speaker: "",
        text: "Can user messages pop in like iMessage without breaking anchoring?",
    },
    Turn {
        role: Assistant,
        speaker: "",
        text: "Yes. Animate the user row with transform and opacity, and let the assistant response stream normally below it.\n\nThat keeps the row measurement predictable while still giving the newly sent bubble a more tactile entrance.",
    },
    Turn {
        role: User,
        speaker: "",
        text: "What makes the animation feel more like iMessage?",
    },
    Turn {
        role: Assistant,
        speaker: "",
        text: "Use a quick spring from the trailing edge: a little scale, a small upward move, and no layout animation.\n\nThe bubble feels tactile, but the measured row stays predictable, so anchoring and auto-scroll do not have to fight a changing layout.",
    },
    Turn {
        role: User,
        speaker: "",
        text: "Can I switch between presets while testing the same thread?",
    },
    Turn {
        role: Assistant,
        speaker: "",
        text: "Yes. Keep the conversation in place while you change the preset, then send the next message to compare the new entrance against the same context.\n\nThat makes it easier to judge the difference between a subtle fade, a snappy pop, and a more dramatic 3D tilt without rebuilding the scenario each time.",
    },
];
pub(super) const COMMANDS: &[Turn] = &[
    Turn {
        role: User,
        speaker: "",
        text: "We're seeing activation dip after workspace creation. Can you help me find the likely step?",
    },
    Turn {
        role: Assistant,
        speaker: "",
        text: "The sharpest drop is between creating the workspace and inviting the first teammate.\n\nWorkspace creation is still healthy, but the invite step is where users pause. That suggests the product is asking for collaboration before the user has enough confidence in the workspace.",
    },
    Turn {
        role: User,
        speaker: "",
        text: "What should I compare before we change the onboarding flow?",
    },
    Turn {
        role: Assistant,
        speaker: "",
        text: "Compare three cohorts:\n\n1. Users who choose a template before inviting teammates.\n2. Users who start from a blank workspace.\n3. Users who skip invites and return within 24 hours.\n\nIf template users invite faster, the fix is probably better first-run guidance rather than a louder invite prompt.",
    },
    Turn {
        role: User,
        speaker: "",
        text: "Can you turn that into an experiment?",
    },
    Turn {
        role: Assistant,
        speaker: "",
        text: "Yes. Create a variant that shows a short checklist after workspace creation:\n\n- Pick a template.\n- Add one project detail.\n- Invite a teammate when the workspace has context.\n\nMeasure first invite completion, 24-hour return rate, and whether teams create a second project.",
    },
    Turn {
        role: User,
        speaker: "",
        text: "What's the risk if we delay the invite prompt?",
    },
    Turn {
        role: Assistant,
        speaker: "",
        text: "The main risk is reducing team creation for accounts that already know who they want to invite.\n\nTo protect that path, keep the invite action visible in the header and only change the primary empty-state guidance. That gives confident teams a direct route without forcing uncertain users through the invite step too early.",
    },
];
pub(super) const VISIBILITY: &[Turn] = &[
    Turn {
        role: User,
        speaker: "",
        text: "Review the incident handoff and tell me what to read first.",
    },
    Turn {
        role: Assistant,
        speaker: "",
        text: "Start with the summary and the impact section. The regression affected the upload queue, but the recovery path completed for every queued job.",
    },
    Turn {
        role: User,
        speaker: "",
        text: "What was the customer impact?",
    },
    Turn {
        role: Assistant,
        speaker: "",
        text: "Impact was limited to delayed processing.\n\nNo records were dropped, and the reconciliation worker confirmed each retry batch. Support saw confusion from two customers, but there were no checkout or billing errors.",
    },
    Turn {
        role: User,
        speaker: "",
        text: "What actions are open?",
    },
    Turn {
        role: Assistant,
        speaker: "",
        text: "Keep the retry window enabled until the next deploy, then add a queue-depth alert as the long-term fix.\n\nThe alert should fire on sustained queue growth, not a single short spike.",
    },
    Turn {
        role: User,
        speaker: "",
        text: "Give me the follow-up checklist.",
    },
    Turn {
        role: Assistant,
        speaker: "",
        text: "After that, compare the queue recovery graph with the deploy timeline so the handoff shows exactly when processing returned to baseline. That makes it easier for support and engineering to answer the same customer questions without re-reading the whole incident thread.\n\nI would also add a short owner note beside each follow-up item. The checklist is small, but ownership keeps the retry-window decision, alert tuning, and support macro from drifting into separate follow-up conversations.\n\nKeep the retry window enabled until the next deploy, then add a queue-depth alert as the long-term fix.\n\nThe alert should fire on sustained queue growth, not a single short spike.",
    },
];
pub(super) const SCROLLABLE: &[Turn] = &[
    Turn {
        role: User,
        speaker: "",
        text: "Review scroll checkpoint 1.",
    },
    Turn {
        role: Assistant,
        speaker: "",
        text: "Checkpoint 2 is synced. The scrollable hook updates as the viewport moves.\n\nWhen the reader is at the first message, the footer should only point them down. Once they move into the middle of the transcript, it should explain that both directions are available.\n\nAt the latest message, the footer should switch again and only point them back up.",
    },
    Turn {
        role: User,
        speaker: "",
        text: "Review scroll checkpoint 3.",
    },
    Turn {
        role: Assistant,
        speaker: "",
        text: "Checkpoint 4 is synced. The scrollable hook updates as the viewport moves.\n\nWhen the reader is at the first message, the footer should only point them down. Once they move into the middle of the transcript, it should explain that both directions are available.\n\nAt the latest message, the footer should switch again and only point them back up.",
    },
    Turn {
        role: User,
        speaker: "",
        text: "Review scroll checkpoint 5.",
    },
    Turn {
        role: Assistant,
        speaker: "",
        text: "Checkpoint 6 is synced. The scrollable hook updates as the viewport moves.\n\nWhen the reader is at the first message, the footer should only point them down. Once they move into the middle of the transcript, it should explain that both directions are available.\n\nAt the latest message, the footer should switch again and only point them back up.",
    },
    Turn {
        role: User,
        speaker: "",
        text: "Review scroll checkpoint 7.",
    },
    Turn {
        role: Assistant,
        speaker: "",
        text: "Checkpoint 8 is synced. The scrollable hook updates as the viewport moves.\n\nWhen the reader is at the first message, the footer should only point them down. Once they move into the middle of the transcript, it should explain that both directions are available.\n\nAt the latest message, the footer should switch again and only point them back up.",
    },
    Turn {
        role: User,
        speaker: "",
        text: "Review scroll checkpoint 9.",
    },
    Turn {
        role: Assistant,
        speaker: "",
        text: "Checkpoint 10 is synced. The scrollable hook updates as the viewport moves.\n\nWhen the reader is at the first message, the footer should only point them down. Once they move into the middle of the transcript, it should explain that both directions are available.\n\nAt the latest message, the footer should switch again and only point them back up.",
    },
    Turn {
        role: User,
        speaker: "",
        text: "Review scroll checkpoint 11.",
    },
    Turn {
        role: Assistant,
        speaker: "",
        text: "Checkpoint 12 is synced. The scrollable hook updates as the viewport moves.\n\nWhen the reader is at the first message, the footer should only point them down. Once they move into the middle of the transcript, it should explain that both directions are available.\n\nAt the latest message, the footer should switch again and only point them back up.",
    },
];
