# Built with Trace

Combine `trace::line`, `trace::disclosure`, and `trace::action` to present each stage of a task.

| Pattern | Shows |
| ------------------------------------------------------------ | ---------------------------------------------------------------- |
| Chain of Thought | A summary that expands into the steps behind it. |
| Plan | The steps the agent plans to take, and how far it has got. |
| Queue | The agent's todo list, split into what is left and what is done. |
| Reasoning | The agent's thinking as it streams. |
| Sources | The links the agent used for a reply. |
| Stack Trace | A failed tool call and its stack frames. |
| Subagent | A subagent run, with its tool calls and result. |
| Task | The steps the agent takes, as it takes them. |
| Terminal | A command the agent runs, with its output as it streams. |
| Test Results | A test run by the agent, with its suites and failures. |
| Tool | A tool call the agent makes, from input to output. |
