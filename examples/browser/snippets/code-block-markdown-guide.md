Route fenced code to `CodeBlock` while leaving inline code in the Markdown paragraph. The native recipe parses prose when its state advances, highlights the fenced source, and keeps Copy disabled until the closing fence arrives. Replay is hidden while the message is streaming. Streamdown's `data-block`, `useIsCodeFenceIncomplete` and `isAnimating` have no native runtime equivalents; the example keeps that fence state explicitly.

The pinned chat transport sends one word plus its following whitespace every 30ms; the native replay uses the same boundaries.
