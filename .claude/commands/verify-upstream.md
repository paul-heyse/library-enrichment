---
description: Verify an upstream API, version, or capability claim against primary sources.
argument-hint: <topic, e.g. "fastmcp 4 ToolResult" or "ty implementation support">
allowed-tools: Bash, Read, Grep, Glob, Edit, Task
---

Verify upstream claims about: $1

Launch the `upstream-verifier` subagent. Give it the specific claims to check and point it at
the relevant entries in `docs/blueprint/SOURCES.md`.

When it returns, append its rows to `docs/architecture/compatibility-matrix.md` with retrieval
dates and exact quotes.

Then act on the verdicts:

- `verified` — record the selected pin and proceed.
- `unverified` — say what could not be established and what it blocks. Do not pin on a guess.
- `contradicted` — **stop and open an ADR** (`/adr`). A blueprint assumption that upstream no
  longer satisfies is a decision point, not something to route around silently.

Never report a version you did not read in this session.
