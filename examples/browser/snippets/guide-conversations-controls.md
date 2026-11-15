## Compose the rest of a turn

[Attachment](icedcn://component/attachment) displays files, [Code Block](icedcn://component/code-block) presents code and terminal output, and [Trace](icedcn://component/trace) displays structured progress. Keep an action's message specific: opening a file, copying text, retrying a request, and cancelling work are separate application actions.

Use [Textarea](icedcn://component/textarea) for a multiline composer and [Button](icedcn://component/button) for submission. Keep the draft in application state when sending it needs validation. Disable submission while the draft is empty or while your application cannot accept another request; disabling a button is explicit.

For long-running output, distinguish a request's loading, streaming, complete, and error states in your model. Preserve the last readable content on failure, and render the error or retry action beside the affected turn. The browser's Message Scroller examples demonstrate saved histories, prepending, and following new output.
