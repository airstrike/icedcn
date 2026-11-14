### Label the trigger

Include the filename and an instruction such as “Open preview dialog” in the content of a keyboard-focusable trigger. Keep the remove and copy buttons in `.action(...)` so activating either performs only its own action.

The web trigger also accepts an `aria-label`. The native button API currently provides no equivalent screen-reader name.
