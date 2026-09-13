---
name: verify-upstream
description: Verify an upstream API, version, or capability claim against primary sources before pinning it. Use before pinning any dependency, when a design assumption about FastMCP, Griffe, ty, rustdoc, DataFusion or a registry needs confirming, and whenever an upstream API may have changed.
allowed-tools: Bash, Read, Grep, Glob, Edit, Task
---

# Verify an upstream claim

**Argument:** the topic, e.g. `fastmcp 4 ToolResult` or `ty implementation support`.

Launch the `upstream-verifier` subagent. Give it the specific claims to check and point it at
the relevant entries in `docs/blueprint/SOURCES.md`.

When it returns, append its rows to `docs/architecture/compatibility-matrix.md` with retrieval
dates and exact quotes.

Then act on the verdicts:

- `verified` — record the selected pin and proceed.
- `unverified` — say what could not be established and what it blocks. Do not pin on a guess.
- `contradicted` — **stop and open an ADR** (the `adr` skill). A design assumption that upstream
  no longer satisfies is a decision point, not something to route around silently. ADR-0005 is
  the worked example: a source table said `textDocument/implementation` was unsupported, a
  runtime probe said otherwise, and the record retired a gate rather than reinterpreting either.

**Never report a version you did not read in this session.** Context7 is a discovery lead, never
exact-version proof.
