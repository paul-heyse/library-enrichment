---
id: ADR-0010
title: Ship every agent workflow as a skill, so both runtimes see it
status: accepted
date: 2026-09-13
deciders: [paul-heyse]
level: decision
principles: [DM-02, DM-55]
design: [§11]
review: not-required: harness configuration; it governs no design surface of the service
evidence: Tested
supersedes: []
superseded-by: null
revisit: A runtime gains a workflow format the other cannot read, or Codex stops discovering `.codex/skills`
verification: `just lint-agents` — `scripts/check_agent_config.py` fails if a runtime-specific instruction surface exists, or if a runtime cannot see `.claude/skills` and `.claude/agents`

---

# ADR-0010: Ship every agent workflow as a skill, so both runtimes see it

## Context

`.claude/skills/` was already symlinked from `.codex/` and `.agents/`, and
`.claude/skills/README.md` said so: "Claude Code and Codex discover the same files." But five
workflows — `adr`, `phase-gate`, `verify-upstream`, `acceptance-report`, `handoff` — lived in
`.claude/commands/`, which is a Claude Code directory that no symlink exposed and Codex never
read. The claim was true of skills and false of the surface that actually carried the work.

ADR-0009 then made it incoherent: it added `design-review` as a shared skill while `adr`, the
other half of the same process, stayed Claude-only. A Codex session could review a design and
then have no way to record the decision the review called for.

## Scope

This binds where agent workflows live and which runtimes must be able to reach them. It does not
change any workflow's content, and it governs nothing about the service — `DESIGN.md` §11 is
cited because that is where the skill boundary is stated, not because the shipped
`library-research` skill is affected. It is not.

## Drivers

- One instruction, every runtime. A workflow written once should not be reachable from one
  client only (DM-55).
- No second surface for the same thing. Two directories holding workflows is two places to look
  and one to forget (DM-02).
- The claim in `README.md` had no oracle, so it went stale without anyone noticing.

## Options

1. **Every workflow is a skill; `.claude/commands/` is removed.** Chosen.
2. Symlink `.codex/commands -> ../.claude/commands`. Rejected: it asserts Codex reads a
   Claude-shaped command directory, which is not established, and it keeps two surfaces.
3. Duplicate each workflow into both directories. Rejected: two editable copies of one
   instruction is the defect, not the fix.
4. Leave it, and document that commands are Claude-only. Rejected: it was already documented as
   the opposite, and a reader has no reason to expect the split.

## Decision

**Agent workflows are skills under `.claude/skills/<name>/SKILL.md`.** There is no
`.claude/commands/`. `.codex/` and `.agents/` expose both `skills` and `agents` from `.claude/`
by symlink, so one file serves every runtime, and both surface a skill as `/name`.

A skill's `name` must equal its directory, because that is what dispatch uses.

### Consequences

Losing `argument-hint`, which commands support and skills do not: the argument is now stated in
the skill's body instead. Skill front matter also carries a longer `description`, since that is
what a runtime matches on to decide the skill applies — which is more work to write than a
command's one-liner, and better for it.

`.claude/rules/process-immutable.md` still lists `.claude/commands/` in its "what you can change
freely" table. That file is enforcement-layer and hook-denied, so the correction is drafted for
the operator (register row R-16) and the path is allowlisted until then.

### Compensating controls

`just lint-agents`, in `just ci`. It fails if a runtime-specific instruction surface reappears,
if `.codex/` or `.agents/` stops exposing `skills` or `agents`, if a skill's `name` disagrees
with its directory, or if any path or `just` recipe the instructions name does not exist.

## Evidence

| Claim | Source | Retrieved | Quote |
|---|---|---|---|
| The shared surface was documented as covering everything, and covered only skills | `.claude/skills/README.md` before this change | 2026-09-13 | "`.claude/skills/` is the single source. `.codex/skills` and `.agents/skills` are symlinks to it, so Claude Code and Codex discover the same files." — while five workflows sat in `.claude/commands/`, which no symlink exposed |
| Only `skills` was linked; `agents` was not linked at all | `ls -la .codex .agents` before this change | 2026-09-13 | one entry each: `skills -> ../.claude/skills`. `.codex/agents` did not exist |
| The symlinks are tracked as symlinks, so a clone reproduces them | `git ls-files -s .codex .agents` | 2026-09-13 | mode `120000` on every entry |
| The source repository treats both as shared | `/home/paul/pse-arrow/.codex/`, `/home/paul/pse-arrow/.claude/` | 2026-09-13 | skills canonical in `.codex/skills` with `.claude/skills` linked to it; agents canonical in `.claude/agents` with `.codex/agents` linked to it; no `.claude/commands/` at all |

## Verification

`just lint-agents` runs `scripts/check_agent_config.py` and is a dependency of `just ci`. On the
commit that introduced it, it found five stale references and two false positives in its own
first draft; it now reports 6 skills and 3 subagents shared by every runtime, with 108 path
references and 38 recipe references resolving.

Exceptions are `.claude/path-allowlist.txt`, one line per path with a comment saying when it will
exist — so a deliberate exception can be removed rather than accumulating.

## Boundaries preserved

- **§B1 Repository boundary** — symlinks are inside the repository and point inside it; no
  write outside, no change to the guard.
- **§B2, §B3** — no service code touched. This is harness configuration.
- **§B11 Excluded mechanisms** — nothing added; a directory was removed.
- **The enforcement layer** — `.claude/rules/`, `.claude/settings.json`, `scripts/hooks/`,
  `AGENTS.md` and `CLAUDE.md` are unchanged; `just guardrails-check` passes. The one correction
  needed there is drafted for the operator.
- **The shipped skill** — `skills/library-research/` is the product, is frozen provenance, and is
  untouched. It is not a development skill and does not live under `.claude/skills/`.

## More information

- `.claude/skills/README.md` — why a workflow is a skill and never a command.
- `docs/design/decisions-rule.draft.md` — the operator track, including register row R-16.
- ADR-0009, which introduced `design-review` as a skill and left `adr` as a command.

## Status history

- 2026-09-13 — accepted.
