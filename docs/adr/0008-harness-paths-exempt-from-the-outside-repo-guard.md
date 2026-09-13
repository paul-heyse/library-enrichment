# ADR 0008: harness paths are exempt from the outside-repository write guard

- **Status:** accepted
- **Date:** 2026-09-13
- **Binding boundary touched:** repository boundary (§1.1, §2.3) — narrowed, not removed

## Context

`scripts/hooks/pre_edit.sh` denied every write outside the repository. That guard exists for
blueprint §2.3 and gate C20: *"a repository under study is never a subprocess working directory
or an extraction destination."*

It was written as a blanket `case "$abs" in "$root"/*) ;; *) deny ;; esac`, with no allowlist.
`scripts/hooks/pre_bash.sh:72-76` already had one for the same class of write:

```sh
      "$root"|"$root"/*) ;;
      "${LIBENR_HOME:-/nonexistent}"/*) ;;
      /tmp/claude-*|/tmp/claude-*/*) ;;
      /dev/null|/dev/stdout|/dev/stderr) ;;
```

So the two hooks disagreed about the same question. The bash hook conceded that some outside-repo
destinations are legitimate; the edit hook had never learned it.

This surfaced when the agent harness could not write its own plan file to `~/.claude/plans/`.
That is neither service state nor a repository under study — it is the harness's working
surface, the equivalent of the session scratch directory the bash hook already exempts.

`.claude/rules/process-immutable.md` is explicit that an agent must not widen its own guardrails:
*"do not route around it, and do not add a narrow exemption to make the current task pass.
Propose the change in `docs/adr/` and let the operator apply it."* That is what happened — the
agent stopped, explained the conflict, offered the ADR-first route, and **the operator applied
the patch themselves**. This ADR records the decision after the fact, which the process
requires and which had been missed.

## Decision

`pre_edit.sh` exempts exactly two path shapes from the outside-repository guard:

```sh
    "$HOME"/.claude/plans/*) ;;
    /tmp/claude-*/*) ;;
```

Everything else outside the repository stays denied.

**Scoped to `plans/`, deliberately not `$HOME/.claude/**`.** A blanket exemption would unblock
`~/.claude/skills` and `~/.claude/settings.json` — precisely the user-scope configuration
`pre_bash.sh` gates behind `LIBENR_ALLOW_USER_INSTALL=1`, and precisely what `AGENT_HANDOFF.md`
forbids writing without explicit setup invocation: *"Do not install it into user configuration
without explicit setup invocation."* Widening the rule to all of `.claude` would have punched
through that in one line.

## Evidence

| Claim | Source | Retrieved | Quote |
|---|---|---|---|
| The guard protects repositories under study | `docs/blueprint/IMPLEMENTATION_BLUEPRINT.md` §2.3 | 2026-09-13 | "The **working repository is never a subprocess working directory or an extraction destination**." |
| An agent must not widen its own guardrails | `.claude/rules/process-immutable.md` | 2026-09-13 | "do not route around it, and do not add a narrow exemption to make the current task pass… Propose the change in `docs/adr/` and let the operator apply it." |
| The bash hook already allowlists harness paths | `scripts/hooks/pre_bash.sh:72-76` | 2026-09-13 | `/tmp/claude-*\|/tmp/claude-*/*) ;;` |
| User-scope skill installation stays gated | `AGENT_HANDOFF.md` | 2026-09-13 | "Do not install it into user configuration without explicit setup invocation." |

## Tests that prove it

`scripts/test-hooks.sh`, run by `just hooks-test` and by CI — 70 cases, six of them added for
this decision. The deny cases are the load-bearing half:

| Case | Expected |
|---|---|
| `$HOME/.claude/plans/some-plan.md` | allow |
| `/tmp/claude-1000/x/scratchpad/notes.md` | allow |
| `$HOME/.claude/skills/library-research/SKILL.md` | **deny** |
| `$HOME/.claude/settings.json` | **deny** |
| `$HOME/some-other-project/src/main.rs` | **deny** |
| `$HOME/.cache/library-enrichment/blobs/x` | **deny** |

Widening the pattern to `$HOME/.claude/*` fails two of those immediately.

## Consequences

Easier: the harness can persist a plan, and the two hooks now answer the same question the same
way instead of disagreeing.

Harder: the outside-repository guard is no longer a single unconditional rule, so a future
reader must check the allowlist rather than the first `case` arm. The test table above is the
mitigation — the narrowness is executable, not a comment.

Foreclosed: nothing. Reverting is deleting two lines and two test cases.

A real residual risk, worth naming: **a guard cannot check itself.** Nothing mechanically
prevents a future edit to `scripts/hooks/` from going unrecorded, because the deny list that
protects those files lives inside one of them. The cheapest oracle would be a manifest digest
over `AGENTS.md`, `CLAUDE.md`, `.claude/rules/`, `.claude/settings.json`, `scripts/hooks/` and
`scripts/env.sh`, verified by a `just` gate shaped like `provenance-check`. That does not exist
today and is the highest-value guardrail this repository is still missing.

## Boundaries preserved

Gate C20's subject is untouched: no repository under study, no real XDG path, and no
service-state directory became writable. User-scope configuration — `~/.claude/skills`,
`~/.claude/settings.json`, and the `claude mcp add` / `codex mcp add` commands in
`pre_bash.sh` — remains denied and still requires `LIBENR_ALLOW_USER_INSTALL=1`. The product
skill still installs only through an explicit `just install-skill --apply`.
