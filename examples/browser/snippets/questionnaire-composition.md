## Composition

```text
Questionnaire
├── Progress
├── Item
│   ├── Title and description
│   ├── Choices (radio or checkbox)
│   ├── Freeform input
│   └── Validation error
└── Actions
    ├── Previous
    ├── Skip
    ├── Next
    └── Submit
```

Questionnaire owns its ordered questions, active item, answers, validation and navigation. The containing Card or Dialog owns cancellation, dismissal, persistence and transport.
