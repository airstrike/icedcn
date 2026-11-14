## Composition

The date trigger anchors a Popover containing the Calendar. The editable picker places its calendar button at the end of an Input Group.

```text
date_picker()
├── trigger button
└── popover content
    └── calendar
```

A single date can close the popover on selection. A range stays open while you adjust its endpoints.
