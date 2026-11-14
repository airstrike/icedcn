use icedcn::widget::questionnaire::{Answer, Answers, Item, choice, item};

fn demo() -> Vec<Item> {
    vec![
        item("direction", "What should the agent build next?")
            .required(true)
            .description("Choose a direction or describe another task.")
            .choices([
                choice("tool-calls", "Tool call timeline")
                    .description("Show what the agent ran and what came back."),
                choice("approvals", "Approval checkpoints")
                    .description("Ask before sensitive or destructive actions."),
                choice("handoffs", "Sub-agent handoffs")
                    .description("Make delegated work and results easier to follow."),
            ])
            .input("Another agent feature", "Describe another feature…"),
        item("signals", "What should every progress update include?")
            .multiple(true)
            .description("Select all that apply, or skip this question.")
            .choices([
                choice("progress", "Progress"),
                choice("decisions", "Decisions"),
                choice("risks", "Risks"),
                choice("next-step", "Next step"),
            ]),
        item("timing", "When should work begin?")
            .required(true)
            .description("Choose when the agent should begin the work.")
            .choices([
                choice("now", "Start now"),
                choice("next-cycle", "Next development cycle"),
                choice("backlog", "Add it to the backlog"),
            ]),
    ]
}

fn multiple() -> Vec<Item> {
    vec![
        item("context", "What context should the agent inspect?")
            .required(true)
            .multiple(true)
            .description("Select every source that may affect the implementation.")
            .choices([
                choice("source", "Relevant source files"),
                choice("tests", "Existing tests"),
                choice("docs", "Architecture documentation"),
                choice("history", "Recent commit history"),
            ]),
    ]
}

fn freeform() -> Vec<Item> {
    vec![
        item("approach", "How should the agent approach this refactor?")
            .required(true)
            .description("Choose a strategy or write a more specific instruction.")
            .choices([
                choice("incremental", "Make the smallest safe change"),
                choice("module", "Refactor one module at a time"),
                choice("rewrite", "Replace the implementation completely"),
            ])
            .input("Another refactoring approach", "Describe another approach…"),
    ]
}

fn skip() -> Vec<Item> {
    vec![
        item("task", "What kind of change is this?")
            .required(true)
            .description("Choose the category that best describes the work.")
            .choices([
                choice("feature", "New feature"),
                choice("fix", "Bug fix"),
                choice("refactor", "Refactor"),
            ]),
        item("constraints", "Are there any implementation constraints?")
            .description("Answer if needed, or intentionally skip this question.")
            .choices([
                choice("no-dependencies", "Do not add dependencies"),
                choice("no-migrations", "Do not change the database"),
                choice("preserve-api", "Preserve the public API"),
            ])
            .input(
                "Another implementation constraint",
                "Describe another constraint…",
            ),
        item("review", "How should the work be reviewed?")
            .required(true)
            .description("Choose the checks the agent should complete before handoff.")
            .choices([
                choice("tests", "Run the test suite"),
                choice("diff", "Review the final diff"),
                choice("both", "Tests and diff review"),
            ]),
    ]
}

fn shortcuts() -> Vec<Item> {
    vec![
        item("action", "What should the agent do next?")
            .required(true)
            .description("Use the displayed shortcut or navigate with the keyboard.")
            .choices([
                choice("inspect", "Inspect the implementation"),
                choice("tests", "Run the relevant tests"),
                choice("patch", "Prepare the patch"),
            ]),
    ]
}

fn validation() -> Vec<Item> {
    vec![
        item("detail", "How much detail should the answer include?")
            .required(true)
            .description("Choose the response depth.")
            .choices([
                choice("summary", "Concise summary"),
                choice("complete", "Complete answer"),
            ]),
        item("audience", "Who will read the answer?")
            .required(true)
            .description("Public answers require complete context.")
            .choices([
                choice("team", "My team"),
                choice("public", "Public audience"),
            ]),
    ]
}

fn controlled() -> Vec<Item> {
    vec![
        item("scope", "What may the agent change?")
            .required(true)
            .description("The host stores the active checkpoint while Questionnaire navigates.")
            .choices([
                choice("component", "Only the target component"),
                choice("tests", "Component and related tests"),
                choice("feature", "The complete feature area"),
            ]),
        item("checks", "Which verification level should it use?")
            .required(true)
            .choices([
                choice("targeted", "Targeted tests"),
                choice("package", "Package tests and typecheck"),
                choice("full", "Full workspace verification"),
            ]),
        item("output", "What should the agent return when finished?")
            .required(true)
            .choices([
                choice("summary", "Concise summary"),
                choice("diff", "Summary with changed files"),
                choice("handoff", "Detailed implementation handoff"),
            ]),
    ]
}

fn resume() -> Vec<Item> {
    vec![
        item("change", "What kind of migration is this?")
            .required(true)
            .description("This answer was saved during the previous session.")
            .choices([
                choice("incremental", "Incremental migration"),
                choice("cutover", "Single cutover"),
            ]),
        item("verification", "How should the migration be verified?")
            .required(true)
            .multiple(true)
            .description("These checks were selected during the previous session.")
            .choices([
                choice("tests", "Run migration tests"),
                choice("typecheck", "Run the typecheck"),
                choice("manual", "Perform a manual smoke test"),
            ]),
        item("notes", "Anything else the agent should remember?")
            .description("This note was saved with the draft.")
            .input("Saved migration note", ""),
    ]
}

fn conditional() -> Vec<Item> {
    vec![
        item("runtime", "Where should the agent run?")
            .required(true)
            .description("Cloud runs add an environment question to this flow.")
            .choices([
                choice("local", "Local workspace"),
                choice("cloud", "Cloud workspace"),
            ]),
        item("environment", "Which cloud environment should it use?")
            .required(true)
            .choices([
                choice("preview", "Preview"),
                choice("staging", "Staging"),
                choice("isolated", "Isolated sandbox"),
            ])
            .enabled_when(|answers| answers.get("runtime").is_some_and(|a| a.contains("cloud"))),
        item("approval", "When should the agent request approval?")
            .required(true)
            .choices([
                choice("writes", "Before writing files"),
                choice("commands", "Before running commands"),
                choice("sensitive", "Only for sensitive actions"),
            ]),
    ]
}

fn navigation_state() -> Vec<Item> {
    vec![
        item("permission", "What may the agent modify?")
            .required(true)
            .description("Next is intentionally disabled until an answer is selected.")
            .choices([
                choice("files", "Project files"),
                choice("tests", "Project files and tests"),
                choice("config", "Files, tests, and configuration"),
            ]),
        item("verification", "What must pass before completion?")
            .required(true)
            .choices([
                choice("tests", "Tests"),
                choice("types", "Tests and types"),
                choice("all", "Tests, types, and visual QA"),
            ]),
    ]
}

fn progress() -> Vec<Item> {
    vec![
        item("scope", "How large is the change?")
            .required(true)
            .choices([
                choice("small", "Small patch"),
                choice("medium", "Feature-sized change"),
                choice("large", "Cross-package change"),
            ]),
        item("strategy", "How should commits be organized?")
            .required(true)
            .choices([
                choice("single", "Single commit"),
                choice("logical", "Logical commits"),
                choice("squash", "Squash before review"),
            ]),
        item("tests", "Which tests should run?")
            .required(true)
            .choices([
                choice("targeted", "Targeted tests"),
                choice("package", "Package suite"),
                choice("workspace", "Full workspace"),
            ]),
        item("delivery", "How should the work be delivered?")
            .required(true)
            .choices([
                choice("patch", "Patch only"),
                choice("commit", "Committed locally"),
                choice("branch", "Push a review branch"),
            ]),
    ]
}

fn animated() -> Vec<Item> {
    vec![
        item("task", "What should the agent do?")
            .required(true)
            .description("Choose the task for this run.")
            .choices([
                choice("implement", "Implement the requested change"),
                choice("debug", "Debug the current behavior"),
                choice("review", "Review the implementation"),
            ]),
        item("review", "How should the work be reviewed?")
            .required(true)
            .description("Select the verification depth.")
            .choices([
                choice("targeted", "Targeted checks"),
                choice("complete", "Complete test suite"),
                choice("manual", "Tests and manual QA"),
            ]),
        item("delivery", "How should the result be delivered?")
            .required(true)
            .description("Choose the final handoff format.")
            .choices([
                choice("summary", "Concise summary"),
                choice("diff", "Summary and changed files"),
                choice("handoff", "Detailed review handoff"),
            ]),
    ]
}

fn card() -> Vec<Item> {
    vec![
        item("task", "What should the agent work on?")
            .required(true)
            .description("Choose the task that should be handled next.")
            .choices([
                choice("fix", "Fix the failing tests"),
                choice("refactor", "Refactor the data layer"),
                choice("docs", "Update the integration guide"),
            ]),
        item("output", "What should the final handoff include?")
            .required(true)
            .description("Pick the level of detail needed for review.")
            .choices([
                choice("summary", "Summary only"),
                choice("files", "Summary and changed files"),
                choice("review", "Full review handoff"),
            ]),
    ]
}

fn dialog() -> Vec<Item> {
    vec![
        item("scope", "Which files are in scope?")
            .required(true)
            .description("Choose how broadly the agent can update the workspace.")
            .choices([
                choice("component", "Component only"),
                choice("feature", "Complete feature directory"),
                choice("workspace", "Any related workspace file"),
            ]),
        item("tests", "How much verification is needed?")
            .required(true)
            .description("Choose the checks the agent should run before handoff.")
            .choices([
                choice("targeted", "Targeted tests"),
                choice("package", "Package tests"),
                choice("full", "Full workspace verification"),
            ]),
    ]
}

pub(super) fn items(id: &str) -> Vec<Item> {
    match id {
        "demo" => demo(),
        "multiple" => multiple(),
        "freeform" => freeform(),
        "skip" => skip(),
        "shortcuts" => shortcuts(),
        "validation" => validation(),
        "controlled" => controlled(),
        "resume" => resume(),
        "conditional" => conditional(),
        "navigation-state" => navigation_state(),
        "progress" => progress(),
        "animated" => animated(),
        "card" => card(),
        "dialog" => dialog(),
        _ => unreachable!("unknown questionnaire {id}"),
    }
}
pub(super) fn submit(id: &str) -> &'static str {
    match id {
        "demo" => "Save plan",
        "multiple" => "Share context",
        "freeform" => "Use this approach",
        "skip" => "Submit brief",
        "shortcuts" => "Confirm action",
        "validation" => "Validate answers",
        "controlled" => "Save workflow",
        "resume" => "Update draft",
        "conditional" => "Save execution plan",
        "navigation-state" => "Save permissions",
        "progress" => "Finish plan",
        "animated" => "Save workflow",
        "card" => "Create task",
        "dialog" => "Send answer",
        _ => unreachable!("unknown questionnaire {id}"),
    }
}
pub(super) fn notice(id: &str) -> &'static str {
    match id {
        "demo" => "Agent plan saved",
        "multiple" => "Context selected",
        "freeform" => "Approach selected",
        "skip" => "Agent brief submitted",
        "shortcuts" => "Next action selected",
        "validation" => "Agent response configured",
        "controlled" => "Agent workflow configured",
        "resume" => "Draft updated",
        "conditional" => "Execution plan saved",
        "navigation-state" => "Permissions saved",
        "progress" => "Pull request plan ready",
        "animated" => "Agent workflow saved",
        "card" => "Agent task created",
        "dialog" => "Clarification sent",
        _ => unreachable!("unknown questionnaire {id}"),
    }
}
pub(super) fn saved(id: &str) -> Answers {
    let mut answers = Answers::new();
    match id {
        "resume" => {
            answers
                .entry("change".into())
                .or_default()
                .choices
                .push("incremental".into());
            answers
                .entry("verification".into())
                .or_default()
                .choices
                .push("tests".into());
            answers
                .entry("verification".into())
                .or_default()
                .choices
                .push("typecheck".into());
            answers.insert(
                "notes".into(),
                Answer {
                    text: "Keep the existing public API stable.".into(),
                    ..Answer::default()
                },
            );
        }
        "conditional" => {
            answers
                .entry("runtime".into())
                .or_default()
                .choices
                .push("local".into());
        }
        _ => {}
    }
    answers
}

/// Format the submitted values in questionnaire order, independently of the map's key order.
pub(super) fn summary(id: &str, answers: &Answers, shortcuts: Option<&str>) -> String {
    let fields: &[(&str, &str)] = match id {
        "animated" => &[
            ("task", "Task"),
            ("review", "Review"),
            ("delivery", "Delivery"),
        ],
        "card" => &[("task", "Task"), ("output", "Handoff")],
        "conditional" => &[
            ("runtime", "Runtime"),
            ("environment", "Environment"),
            ("approval", "Approval"),
        ],
        "controlled" => &[
            ("scope", "Scope"),
            ("checks", "Verification"),
            ("output", "Output"),
        ],
        "demo" => &[
            ("direction", "Direction"),
            ("signals", "Progress signals"),
            ("timing", "Timing"),
        ],
        "dialog" => &[("scope", "Scope"), ("tests", "Verification")],
        "freeform" => &[("approach", "Approach")],
        "multiple" => &[("context", "Context")],
        "navigation-state" => &[
            ("permission", "Permission"),
            ("verification", "Verification"),
        ],
        "progress" => &[
            ("scope", "Scope"),
            ("strategy", "Commits"),
            ("tests", "Tests"),
            ("delivery", "Delivery"),
        ],
        "resume" => &[
            ("change", "Migration"),
            ("verification", "Verification"),
            ("notes", "Notes"),
        ],
        "shortcuts" => &[("action", "Action"), ("shortcuts", "Shortcuts")],
        "skip" => &[
            ("task", "Task"),
            ("constraints", "Constraints"),
            ("review", "Review"),
        ],
        "validation" => &[("detail", "Detail"), ("audience", "Audience")],
        _ => &[],
    };
    fields
        .iter()
        .map(|&(name, label)| {
            let display = if name == "shortcuts" {
                shortcuts.unwrap_or("letters").to_owned()
            } else if let Some(answer) = answers.get(name) {
                if name == "constraints" && answer.skipped {
                    "Skipped".into()
                } else {
                    let mut values = answer.choices.clone();
                    if !answer.text.trim().is_empty() {
                        values.push(answer.text.clone())
                    }
                    if values.is_empty() {
                        "None".into()
                    } else {
                        values.join(", ")
                    }
                }
            } else if name == "environment" {
                "Not applicable".into()
            } else {
                "None".into()
            };
            format!("{label}: {display}")
        })
        .collect::<Vec<_>>()
        .join(" · ")
}
