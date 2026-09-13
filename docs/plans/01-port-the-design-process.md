---
title: port-the-design-process
status: done
date: 2026-09-13
adrs: [ADR-0009]
phase: 0
---

# Port the design process from pse-arrow

## Context

`/home/paul/pse-arrow` runs a closed design loop — blueprint → ADR → design review → plan →
deferred-decision register — with a lint, a gate and a hook guarding each edge. This repository
had the *enforcement* half (hooks, an `ast-grep` corpus, `just` gates, four-state gate reporting)
and almost none of the *design* half: no standard to review against, no durable review artifact,
no lint over its eight decision records, and nothing recording what had been deferred.

Two design reviews had already run here (commits `e74d012`, `0cd7330`) and found real defects.
Neither left an artifact, and neither produced a `rules/` entry despite `boundary-reviewer`
saying that is where findings belong.

## Decisions

[ADR-0009](../adr/0009-charter-based-design-review-and-linted-decision-records.md) — adopt a
charter-based design review, a living design document, and linted decision records.

## Plan

1. Carry the charter, the agent directive and the §1–§11 review template over **verbatim**, so
   `DM-nn` and `Gn` mean the same in both repositories. Add one repo-specific `ADDENDUM.md`.
2. `docs/design/DESIGN.md` as the authoritative spine; the blueprint stays frozen provenance.
   Give the binding decisions stable IDs §B1–§B13.
3. Port `scripts/adr.py` and `scripts/check_register.py`; migrate the eight records to charter
   §H front matter; generate the index.
4. Seed `docs/adr/register.md` from real deferrals.
5. `docs/plans/` with the outcome discipline, and `just plan`.
6. `.claude/skills/design-review/` and a rewritten `/adr` command.
7. Wire `adr-lint` into `just ci`.
8. Prepare the hook-denied changes as a draft for the operator.

## Verification

`just adr-lint` and `just adr-lint-test` in `just ci`; then run the review the port installs,
against `DESIGN.md` and the service, and act on what it finds.

## Open items

Three Rust tests in the Phase-1 slice fail (`retrieval_fixture.rs`). They are open work, not
regressions from this plan, and are listed in `STATUS.md` rather than skipped.

The operator track — `.claude/rules/decisions.md`, `AGENTS.md`'s binding table, `CLAUDE.md`'s
plan location — is drafted in [`../design/decisions-rule.draft.md`](../design/decisions-rule.draft.md)
and tracked by register rows R-09 and R-11.

## Outcome (recorded after implementation)

### What was built

- The doctrine under `docs/design_review/design_principles/`: charter (699 lines) and directive
  **byte-identical** to the source, verified with `diff`; the §1–§11 template; a
  `REVIEW_REFERENCE.md` whose §5 appendix was rewritten for this codebase; and a new
  `ADDENDUM.md`. *Implemented.*
- `docs/design/DESIGN.md`, a spine over the frozen blueprint with §B1–§B13 as citation targets,
  and `docs/design/README.md` carrying the amendment rule. *Implemented.*
- `scripts/adr.py` and `scripts/check_register.py`, standard-library only. Nine records migrated
  and linting clean; the index generated. *Tested* — `just adr-lint-test`, 33 behaviour tests.
- `docs/adr/register.md`, 15 rows, each with an observable trigger and a runnable check.
  *Tested* — `check_register.py --lint`.
- `.claude/skills/design-review/`, `docs/plans/`, `just plan`, and six new `just` recipes with
  `adr-lint` and `adr-lint-test` in `just ci`. *Implemented.*
- One design review, which found four defects and produced the repository's first
  review-derived `ast-grep` rule. *Tested* — `just rules-test`, 6 rules, all with fixtures.

### A mistake made and corrected

**The immutability lint would have shipped unexercised, and I nearly claimed it worked.** The
central safety property of ADR-0009 — that an accepted record changes only in its status fields —
cannot fire anywhere in this repository: records 0001–0005 predate the front-matter migration and
are skipped by design, and 0006–0009 do not exist on `main`, so there is nothing to diff against.
I discovered it only because I wrote a deliberate-failure script and one of the six cases printed
`NO ERROR RAISED`. Without that script the lint would have been described as enforcing
immutability while enforcing nothing.

The correction was `scripts/test-adr-lint.sh`: a throwaway git fixture where the property *can*
be demonstrated, following the precedent `scripts/test-hooks.sh` set. It is 33 cases and it is
wired into `just ci`.

The second, smaller mistake: the spine I wrote was already wrong when I wrote it. I seeded
`DESIGN.md` from `STATUS.md`, which says "no evidence retrieval exists yet", while the tree held
a substantially complete Phase-1 slice. The review caught it as finding F1. The labels are
corrected; the structural question — whether the spine should carry these labels at all — is
register row R-14.

### Deviations from the plan, deliberate

- **ADR immutability is a lint, not a hook.** pse-arrow parses the record's `status:` in a
  PreToolUse hook. `AGENTS.md` here says a check that must parse a source file is a rule's job,
  and `pre_edit.sh` is deliberately pure path classification. Recorded in ADR-0009.
- **No GitHub machinery.** Labels, a semantic-PR-title action, issue forms and the monthly
  register cron were dropped; this repository has no remote. `just register-check` replaces the
  cron.
- **The blueprint stays frozen** and `DESIGN.md` is a spine rather than a copy — copying would
  create two independently editable authorities for the same fact, a G1 failure under the
  charter being installed.
- **`scripts/test-adr-lint.sh` was not in the plan.** It came out of the mistake above.
- **Evidence-label corrections are maintenance, not amendment.** The five-clause amendment rule
  would otherwise require an ADR every time a phase lands. Stated in `DESIGN.md`'s preamble.
- **pse-arrow's `.claude/agents/design-reviewer.md` was not ported.** It is a stale copy from an
  unrelated project (principles P1–P31, `docs/library_ref/`, an MCP server that does not exist
  here); pse-arrow's own `path-allowlist.txt` admits it. `boundary-reviewer` remains the diff
  reviewer and the new skill is the design reviewer.
