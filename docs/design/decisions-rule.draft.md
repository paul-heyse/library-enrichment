# Operator track — changes to the enforcement layer

`AGENTS.md`, `CLAUDE.md`, `.claude/rules/`, `.claude/settings.json`, `scripts/hooks/` and
`scripts/env.sh` are the rules an agent works under. Both hooks deny writes there, shell
redirects included, and the denial message says what to do instead: *"Propose the change in
`docs/adr/` and let the operator apply it."*

This file is that proposal. Four changes, none of them blocking — the process installed by
ADR-0009 works without any of them, and each is tracked by a register row so it does not simply
evaporate.

Apply them, then:

```bash
LIBENR_ALLOW_GUARDRAIL_RERECORD=1 just guardrails-record
just guardrails-check
```

**No `.claude/settings.json` change is needed.** `Bash(just *)` is already allowed and every new
recipe runs through `just`.

---

## 1. New file: `.claude/rules/decisions.md`

Register row **R-11**. Path-scoped, so it loads when the decision surface is touched rather than
every session. Content, verbatim:

```markdown
---
description: Decision records, plans, the design document, and the review process
paths:
  - "docs/adr/**"
  - "docs/design/**"
  - "docs/plans/**"
  - "docs/design_review/**"
---

# Decisions, design, plans

Four directories, four lifecycles.

| | |
|---|---|
| `docs/design/DESIGN.md` | **Authoritative.** The design. Amended only by a decision record. |
| `docs/adr/` | **Why.** Immutable once accepted. |
| `docs/adr/register.md` | **What was deferred**, with a trigger, a check, an owner and a date. |
| `docs/design_review/reviews/` | **Evidence, never authority.** A review finds defects; the ADR that answers one changes the design. |
| `docs/plans/` | **How work is sequenced.** Living until done. When the code and a plan disagree, the code is what runs. |

`docs/blueprint/` remains frozen provenance and is not in this list. It is the origin, not the
living design: after the first ADR it stops being a true statement of what the system is, which
is exactly why `DESIGN.md` exists (ADR-0009).

## ADR front matter (charter §H)

`id`, `title`, `status` (`proposed` | `accepted` | `rejected` | `deprecated` | `superseded`),
`date`, `deciders`, `level` (`decision` | `should-deviation` | `must-gap`), `principles`
(`DM-nn`), `design` (the `DESIGN.md` sections governed, `§B1`–`§B13` included), `review`
(`path#anchor`, or `not-required: <reason>`), `evidence` (a charter §D label),
`supersedes`/`superseded-by`, `revisit` (an *observable* trigger, not a date alone),
`verification` (the test, gate or lint that shows the decision still holds).

`level: must-gap` narrows scope. It never claims compliance.

Body: Context · Scope · Drivers · Options · Decision (Consequences, Compensating controls) ·
**Evidence** (the four-column table: claim, source, retrieval date, exact quote) ·
**Verification** · **Boundaries preserved** · More information · Status history (append-only).
A small deviation fills each section in one line — that is fine; an empty section is not.

## Three vocabularies, never interchangeable

Charter §D labels grade a **design claim**. `passed`/`failed`/`blocked`/`not_run` grade an
**acceptance gate**. The six classes of `DESIGN.md` §6.2 grade a **runtime evidence record**.
A gate is never `Measured`; a design claim is never `not_run`.

## Immutability

An accepted ADR changes only in `status`, `superseded-by` and an appended `## Status history`
line. A decision that turned out wrong is **superseded**, not edited:
`just adr-supersede <old> <new>` writes symmetric links and a status-history entry.
`just adr-lint` diffs accepted records against `main`, so an edit in place is a red gate.

This is a lint and not a hook on purpose: a check that must parse a source file is a rule's job,
not a hook's, and `pre_edit.sh` is deliberately pure path classification.

## The amendment rule

`DESIGN.md` changes only in a commit that adds or changes exactly one ADR, adds a revision-history
row, adds `> Decision: ADR-NNNN` under every governed section, keeps every section number where
it was, and leaves `just adr-lint` clean. Insert `§6.2.1`; never renumber — the lint resolves
every `§` citation against a real heading.

## Reviews and the rule corpus

Every §7 finding names the executable oracle that would catch a regression — a hook, an
`ast-grep` rule in `rules/`, a `just` gate, or a test — or says explicitly that none exists.
Adding a rule needs no ADR; growing that corpus is encouraged. Register row R-08 watches whether
findings actually land there, because for two review rounds they did not.

## Plans

`just plan <slug>` creates `docs/plans/NN-<slug>.md`. Front matter lists the ADRs it implements.
When the work lands, append `## Outcome` with what was built (with §D labels), **a mistake made
and corrected**, and **deliberate deviations**. Both of those are the point; omitting them makes
the plan a memo. Plans live here, not in `~/.claude/plans/` — a plan in a home directory is
invisible to Codex and to the next session.
```

---

## 2. `AGENTS.md` — point the binding table at `DESIGN.md`

Register row **R-09**. `AGENTS.md`'s "Binding decisions" table is a nine-row paraphrase that is
**not** the same set as blueprint §1.1: it adds Static extraction (§B12) and Rust API source
(§B13), which live in blueprint §5.2 and §4.1, and omits Repository boundary (§B1), Design
inference (§B9), Completion (§B10) and the exclusion list (§B11). Two documents currently state
the binding set, which is a second authority for the same fact (DM-02).

Suggested replacement for the table's lead-in and the table itself:

> The thirteen binding decisions are §B1–§B13 in
> [`docs/design/DESIGN.md`](docs/design/DESIGN.md) §1.1, which is the single place they are
> stated and the citation target every ADR resolves against. They are settled; changing one
> requires an ADR with primary-source evidence and tests.
>
> §B1 Repository boundary · §B2 Core ownership · §B3 Python boundary · §B4 Python semantic
> engine (ty) · §B5 Rust semantic engine (rust-analyzer) · §B6 Context7 · §B7 Storage ·
> §B8 MCP transport · §B9 Design inference · §B10 Completion · §B11 Excluded mechanisms ·
> §B12 Static extraction (Griffe, `allow_inspection=False`) · §B13 Rust API source (docs.rs
> before local compilation).

Two smaller edits in the same pass:

- **Navigation** — add `docs/design/README.md` as the entry point for the design, the decision
  process and the review standard.
- **Command surface** — add a row: `Anything decided` → `just adr-lint`. It already runs inside
  `just ci`.
- **Phases and gates** — `AGENTS.md` says "all 46 acceptance IDs"; `tests/gates.toml` and
  `STATUS.md` say 48. The two extras are P08a and P08b, registered by ADR-0005 against a frozen
  plan that has 46. Register row **R-10**. Either say "the 46 IDs of the frozen plan plus the
  replacements ADR-0005 registered", or drop the count and let `tests/gates.toml` be the number.

---

## 3. `.claude/rules/process-immutable.md` — the freely-editable table

Register row **R-16**. Its "What you can change freely" table lists `.claude/commands/`. That
directory no longer exists: ADR-0010 moved every workflow to `.claude/skills/`, because a command
is visible to Claude Code alone while a skill is exposed to every runtime through the `.codex/`
and `.agents/` symlinks. Replace the row:

| | |
|---|---|
| `.claude/agents/`, `.claude/skills/` | subagents and workflows — **shared with Codex by symlink; a workflow belongs here, never in a runtime-specific command directory** (ADR-0010) |

`just lint-agents` enforces it, and the stale reference is allowlisted in
`.claude/path-allowlist.txt` until this lands.

## 4. `CLAUDE.md` — say where a plan lives

No register row; a one-line clarification. Plan mode writes to `~/.claude/plans/`, which is a
single session's scratch proposal. When a session's plan becomes work that will land,
`just plan <slug>` gives it a numbered, versioned home in `docs/plans/` that Codex and the next
session can both read. Worth one sentence in the Claude-harness notes so the distinction is made
at the point of use rather than only in `docs/plans/README.md`.
