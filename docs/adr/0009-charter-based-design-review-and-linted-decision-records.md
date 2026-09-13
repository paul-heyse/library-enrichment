---
id: ADR-0009
title: Adopt a charter-based design review, a living design document, and linted decision records
status: accepted
date: 2026-09-13
deciders: [paul-heyse]
level: decision
principles: [DM-02, DM-55, DM-59, DM-60]
design: [§1.1, §15]
review: not-required: this record establishes the review process; it governs no design surface of the service itself
evidence: Implemented
supersedes: []
superseded-by: null
revisit: A second maintainer joins; or a design review produces no finding worth acting on, which would mean the machinery costs more than the risk it addresses (charter §H)
verification: `just adr-lint` and `just adr-lint-test`, both inside `just ci`; the latter is 33 behaviour tests over a throwaway git fixture in `scripts/test-adr-lint.sh`

---

# ADR-0009: Adopt a charter-based design review, a living design document, and linted decision records

## Context

Two design reviews have already happened here — commits `e74d012` and `0cd7330` — and between
them they found a gate C20 repository-boundary breach, gate C19 passing on an assertion it had
not earned, and a skipped test reported as `failed`. None of that left a durable artifact: the
findings survive only as prose in `STATUS.md` § *What the reviews caught*.

There was also nothing to review *against*. `blueprint §15` is a ten-question design-review
checklist, but it is frozen provenance, referenced by no command, no subagent and no gate.
`boundary-reviewer` says every finding should name the executable oracle that would have caught
it, "because it becomes a new entry in `rules/`" — and `rules/` has gained no entry since the
first commit. The loop was declared and never ran.

The eight existing decision records had no `verification:` field, no `revisit:` trigger, no lint,
a hand-maintained index, and no immutability rule. Nothing recorded what had been deliberately
**deferred**, so a deferral had no owner and no date.

`/home/paul/pse-arrow` runs a closed version of this loop. This record adopts it.

## Scope

This binds the *process*: where the design lives, what a decision record must contain, what a
review must establish, and what mechanically checks all three. It changes **no** binding decision
of the service — §B1–§B13 are untouched, and the frozen blueprint and contracts are not edited.

It deliberately leaves out pse-arrow's GitHub machinery: labels, a semantic-PR-title action,
issue forms, a PR template and a monthly `register-review.yml` cron. This repository has no
remote and a linear history. If a remote is added, that is a new decision.

## Drivers

- **Correctness.** A design claim with no standard to be measured against cannot be wrong, which
  is the problem (DM-59).
- **Authority.** The blueprint is frozen, so it cannot also be the living design; after the first
  ADR it stops being a true statement of what the system is.
- **Durability.** A review whose output lives in prose in `STATUS.md` is rewritten by the next
  `/handoff` (DM-46).
- **Regression control.** A correction with no oracle regresses (DM-60).
- **Proportionality.** Charter §H: do not add review machinery that costs more than the risk and
  the decision warrant.

## Options

1. **Adopt the pse-arrow loop, adapted** — chosen.
2. **Unfreeze the blueprint and amend it in place, as pse-arrow does.** Rejected: `AGENTS.md`,
   `.claude/rules/frozen-contracts.md` and both hooks enforce the freeze, and the freeze is what
   makes a later deviation visible rather than quietly absorbed.
3. **Let the ADR set *be* the living design, with a generated index joining records to frozen
   blueprint sections.** Rejected: there would never be one readable statement of what the design
   is now; a reader would reconstruct it from the blueprint plus N records.
4. **Write a slimmer, repo-specific principle set instead of the charter.** Rejected: the charter
   is technology-neutral and already fits an evidence service, and a bespoke set would make
   `DM-nn` and `Gn` mean different things in different repositories.
5. **Do nothing.** Rejected: two reviews have already paid for themselves and left nothing behind.

## Decision

**1. `docs/design/DESIGN.md` is the authoritative design.** The blueprint stays frozen
provenance — the specification as delivered, and the fixed point every later claim is measured
against. `DESIGN.md` is a **spine**: one to three sentences per section, mirroring the blueprint's
§-numbers, citing `blueprint §N.M` for the full text. It is amended only by a decision record,
under the five-clause rule in `docs/design/README.md`.

**2. The thirteen binding decisions get stable IDs, §B1–§B13.** §B1–§B11 are blueprint §1.1;
§B12 and §B13 are blueprint §5.2 and §4.1, which `AGENTS.md` also treats as binding. They are
citation targets, not new decisions.

**3. The Data Model–Based Design Charter is the normative standard**, carried verbatim under
`docs/design_review/design_principles/` together with the agent directive, the §1–§11 review
template and the reference — so that `DM-nn` and `Gn` mean the same here as in pse-arrow. The one
repo-specific layer is `ADDENDUM.md`, which maps §B1–§B13 onto DM IDs and gates, routes
blueprint §15's ten questions onto G1–G7, and keeps the three grading vocabularies distinct.

**4. Three vocabularies, never interchangeable.** Charter §D labels grade a *design claim*;
`passed`/`failed`/`blocked`/`not_run` grade an *acceptance gate*; the six epistemic classes of
§6.2 grade a *runtime evidence record*. A gate is never `Measured`; a design claim is never
`not_run`. The existing rules for the second and third are unchanged.

**5. A decision record carries the charter §H fields in YAML front matter** — `id`, `title`,
`status`, `date`, `deciders`, `level`, `principles`, `design`, `review`, `evidence`,
`supersedes`, `superseded-by`, `revisit`, `verification` — and keeps the two body sections this
repository already does better than pse-arrow: the four-column **Evidence** table (claim, source,
retrieval date, exact quote) and **Boundaries preserved**.

**6. An accepted record is immutable** except for `status`, `superseded-by` and an appended
`## Status history` line. To change a decision, supersede it: `just adr-supersede <old> <new>`.

**7. Every deferral gets a register row** in `docs/adr/register.md` with an observable trigger, a
runnable check, an owner and a next-check date. A row is closed by the ADR that decides it, never
by deleting it.

**8. A design review writes a file** under `docs/design_review/reviews/`, following the template's
§1–§11 plus a Method-and-coverage note and an applicability note. **Every §7 finding's
Verification column names an executable oracle — a hook, an `ast-grep` rule, a `just` gate, or a
test — or says explicitly that none exists.** That last case is the most valuable output, because
`rules/` is where it lands.

**9. Immutability is enforced by a lint, not by a hook.** `AGENTS.md`'s enforcement tiers are
explicit that a check needing to parse a source file is a rule's job, not a hook's, and
`pre_edit.sh` is deliberately pure path classification. `scripts/adr.py lint` diffs accepted
records against `main` and runs inside `just ci`.

### Consequences

Every ADR now costs more to write: fourteen front-matter fields, an observable trigger, and a
named runnable check. That is the intended cost — a record with no executable consequence erodes
silently — but it is real, and `just adr-new` exists so the floor is low.

`DESIGN.md` is a second document describing the same system as the blueprint. That is the price
of a frozen origin, and it is why `DESIGN.md` is a spine rather than a copy: a copy would be two
independently editable authorities for the same fact, which is a G1 failure under the charter this
record adopts. The spine must be kept honest — a `DESIGN.md` section that drifts from the
implementation is a finding, and the reference's "when the subject is both" lens exists to catch
it.

`§`-numbers are now load-bearing. Renumbering a `DESIGN.md` heading breaks `just adr-lint` for
every record that cited it. Insert `§6.2.1`; never renumber.

Three grading vocabularies coexist. That is a real conflation risk, which is why `ADDENDUM.md` §3
states it flatly rather than leaving it to be inferred.

Reversing this is cheap: the documents are additive, `just adr-lint` can be dropped from `just ci`
in one line, and no service code depends on any of it.

### Compensating controls

`just adr-lint` in `just ci` (fourteen required fields, four enums, `DM-NN` shapes, contiguous
numbering, every `§` citation resolving to a real `DESIGN.md` heading, `review:` paths existing,
symmetric supersession, immutability against `main`, and a non-stale index).
`scripts/check_register.py --lint` fails a non-closed row whose `next-check` is in the past.
`just register-check` runs the `$`-prefixed checks. Register row **R-08** watches the one thing
this record cannot enforce: whether findings actually become `rules/` entries.

## Evidence

| Claim | Source | Retrieved | Quote |
|---|---|---|---|
| The blueprint carries a ten-question design-review checklist that nothing references | `docs/blueprint/IMPLEMENTATION_BLUEPRINT.md` §15, line 702 | 2026-09-13 | "Before declaring a design or implementation complete, ask: …" — ten questions, cited by no command, subagent or gate in the tree |
| A hook that must parse a source file is the wrong enforcement tier here | `AGENTS.md`, "Navigation" | 2026-09-13 | "A hook that needs to parse a source file is the wrong tier — write a rule instead." |
| `pre_edit.sh` is deliberately path-only, so ADR immutability cannot live there | `scripts/hooks/pre_edit.sh:2` | 2026-09-13 | "PreToolUse(Edit\|Write\|NotebookEdit). Pure path classification -- no file parsing." |
| The blueprint is frozen and an ADR is the only route to deviating from it | `scripts/hooks/pre_edit.sh:26` | 2026-09-13 | "`docs/blueprint/…` is frozen provenance … If the specification is wrong, that is a decision to deviate, recorded with `/adr`. It is not an edit to the specification." |
| `docs/adr/` is not under the provenance manifest, so the records can be migrated | `docs/provenance/bundle-2026-09-13/MANIFEST.sha256` | 2026-09-13 | 15 entries; none under `docs/adr/`. `just provenance-check` passes after the migration |
| Findings are supposed to become `rules/` entries, and none have | `.claude/agents/boundary-reviewer.md` | 2026-09-13 | "If there is no such oracle, say so: that is the most valuable output you produce, because it becomes a new entry in `rules/`." `rules/` holds 5 files, all from commit `2b217cf` |
| The source process, verbatim where it is technology-neutral | `/home/paul/pse-arrow/docs/design_review/design_principles/DATA_MODEL_DESIGN_CHARTER.md`, `scripts/adr.py`, `scripts/check_register.py` | 2026-09-13 | The charter and the agent directive are byte-identical to the source, verified with `diff` |

## Verification

`just adr-lint`, which runs in `just ci`:

```
python3 scripts/adr.py lint
python3 scripts/adr.py index --check
python3 scripts/check_register.py --lint
```

A record missing `verification:`, citing a `§` that resolves to no `DESIGN.md` heading, or editing
an accepted record's argument in place fails the gate. A register row that is not `closed` and
whose `next-check` has passed fails it too. `just register-check` reports due rows and runs their
checks.

The lint itself has regression cover: `just adr-lint-test` runs `scripts/test-adr-lint.sh`, 33
behaviour tests against a throwaway git repository, following the precedent `scripts/test-hooks.sh`
set for the hooks. It exists because of something found while building this: **the immutability
check cannot fire anywhere in the real tree.** Records 0001–0005 predate the front-matter
migration and are skipped by design; 0006–0009 do not exist on `main` at all, so there is nothing
to diff them against. The central safety property of this record would have shipped unexercised.
The fixture repository is the only place it can be demonstrated today, and it is.

Two things this deliberately does **not** verify, because no mechanical oracle exists for either:
that a review was actually conducted before a decision was accepted, and that `DESIGN.md` still
describes the implementation. The first is the maintainer's judgment; the second is what a design
review is for.

## Boundaries preserved

- **§B1 Repository boundary** — every new file is inside `docs/`, `scripts/` or `.claude/`; no
  service state, no write outside the repository, no change to the guard.
- **§B2 Core ownership**, **§B3 Python boundary** — no service code changed. `scripts/adr.py` and
  `scripts/check_register.py` are repository tooling, standard-library only, not part of the
  service or its Python boundary.
- **§B4–§B8, §B12, §B13** — untouched.
- **§B9 Design inference** — reinforced, not weakened: charter §D labels make the strength of a
  design claim explicit, and `agent_inferred` remains the evidence class for judgment.
- **§B11 Excluded mechanisms** — no dependency added at all; both scripts are standard library,
  as they must run before `uv sync`.
- **Frozen provenance and contracts** — `docs/blueprint/`, `docs/provenance/`, `contracts/` and
  `tests/ACCEPTANCE_PLAN.md` are unchanged; `just provenance-check` passes.
- **The enforcement layer** — `AGENTS.md`, `CLAUDE.md`, `.claude/rules/`, `.claude/settings.json`,
  `scripts/hooks/` and `scripts/env.sh` are unchanged. The three changes this process wants there
  are drafted in `docs/design/decisions-rule.draft.md` for the operator, and tracked as register
  rows R-09 and R-11.

## More information

- `docs/design/README.md` — what "authoritative" means and the amendment rule.
- `docs/design_review/design_principles/ADDENDUM.md` — §B-to-DM mapping, blueprint §15 routed onto
  G1–G7, and the three vocabularies.
- `.claude/skills/design-review/SKILL.md` — how a review is conducted and what it must establish.
- `docs/adr/register.md` rows R-08 (findings → `rules/`), R-09 (`AGENTS.md`'s divergent binding
  table) and R-11 (the uninstalled `.claude/rules/decisions.md`).
- `docs/plans/01-port-the-design-process.md` — how this work was sequenced, and its outcome.

## Status history

- 2026-09-13 — accepted. Landed together with the process it establishes, which is the only way to
  land it: the record is the first artifact the new lint validates.
