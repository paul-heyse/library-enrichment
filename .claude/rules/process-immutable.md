---
paths:
  - "AGENTS.md"
  - "CLAUDE.md"
  - ".claude/**"
  - ".codex/**"
  - ".agents/**"
  - "justfile"
  - "scripts/**"
  - "rules/**"
  - "rule-tests/**"
  - "docs/blueprint/**"
  - "docs/provenance/**"
---

# Process and provenance are operator-owned

`docs/blueprint/**` is the governing specification as delivered. `docs/provenance/**` records
the bytes it was delivered with. Neither is editable: they are the fixed point that every
later claim is measured against. If the specification is wrong, that is an ADR — a record of
a decision to deviate — not an edit to the specification.

`AGENTS.md`, `CLAUDE.md`, `.claude/**`, `justfile`, `scripts/**`, and the `rules/` corpus are
the governance layer. An agent working under these constraints does not get to relax them
mid-task. Propose changes through `docs/adr/` and let the operator apply them.

When a guardrail fires and the guardrail is genuinely wrong, say so and stop. Do not route
around it, do not add a narrow exemption to make the current task pass, and do not disable a
hook. A guardrail that is wrong often enough to be annoying is worth fixing properly.

`scripts/hooks/*.sh` must stay fast and dependency-free. They run on every matching tool call.
