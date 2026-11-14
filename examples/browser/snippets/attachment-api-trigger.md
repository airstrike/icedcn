### AttachmentTrigger

Supply a full-card target with `.trigger(|target| attachment::trigger(target).on_press(Message::Open).into())`. To open a dialog, put `attachment::trigger(target)` inside `d.trigger(...)`. Padding and border are part of the target; action buttons keep independent focus and hit targets.

| Builder | Type | Default |
| --- | --- | --- |
| `.trigger(...)` | `FnOnce(Slot) -> Slot` | No trigger |
| `attachment::trigger(target).on_press(...)` | Application message | No message |
| `attachment::trigger(target).disabled(...)` | `bool` | `false`; also inherits a disabled ancestor |
