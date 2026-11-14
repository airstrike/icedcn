## Streaming

While the code streams, the hook tokenizes only the new lines, starting where the last line left off. Lines that did not change keep the same nodes, so React skips them.
