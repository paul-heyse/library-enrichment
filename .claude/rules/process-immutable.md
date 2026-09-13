---
paths:
  - "AGENTS.md"
  - "CLAUDE.md"
  - ".claude/rules/**"
  - ".claude/settings.json"
  - "scripts/hooks/**"
  - "scripts/env.sh"
  - "docs/blueprint/**"
  - "docs/provenance/**"
---

# The enforcement layer is not part of the work

`docs/blueprint/**` is the governing specification as delivered. `docs/provenance/**` records
the bytes it was delivered with. Neither is editable: they are the fixed point every later
claim is measured against. If the specification is wrong, that is an ADR — a record of a
decision to deviate — not an edit to the specification.

`AGENTS.md`, `CLAUDE.md`, `.claude/rules/`, `.claude/settings.json`, `scripts/hooks/` and
`scripts/env.sh` are the rules an agent works under. **An agent that can edit its own
guardrails does not have guardrails.** Both the pre-edit and pre-bash hooks deny writes here,
including through a shell redirect.

When a guardrail fires and it is genuinely wrong, say so and stop. Do not route around it, do
not add a narrow exemption to make the current task pass, and do not disable a hook. Propose
the change in `docs/adr/` and let the operator apply it. A guardrail that is wrong often enough
to be annoying is worth fixing properly.

## What you can change freely

This boundary is deliberately narrow. The working surface is meant to grow as phases land, and
none of it is blocked:

| | |
|---|---|
| `justfile` | new recipes and gates |
| `scripts/*.sh` | gate and tooling scripts, except `hooks/` and `env.sh` |
| `rules/`, `rule-tests/` | the ast-grep corpus — **growing this is encouraged** |
| `.claude/agents/`, `.claude/commands/` | subagents and slash commands |
| `.claude/skills/` | development skills |
| `tests/gates.toml` | registering tests against gates |

`boundary-reviewer` findings are supposed to land in `rules/` as new executable oracles. That
only works if adding a rule is frictionless, so it is.

`scripts/hooks/*.sh` must stay fast and dependency-free: they run on every matching tool call.
`scripts/env.sh` is shared by `.envrc` and the session hook, so editing it changes the
environment for both a human and an agent at once — which is exactly why it is frozen.
