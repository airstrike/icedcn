### Server Rendering

Upstream `highlightCode` can run asynchronously in a React Server Component to keep Shiki out of the client bundle. A native iced application has no React server/client split. Parse expensive input when it arrives in `update`, or in an asynchronous task, then store token ranges for the next `view`. These fixed examples cache parsing before reusing it during replay.
