# Authoritative design

## What "authoritative" means

[`DESIGN.md`](DESIGN.md) is the design. When an implementation and `DESIGN.md` disagree, one of
them is a bug, and an ADR says which. Nothing else in this repository has that status.

| Document | Status |
|---|---|
| [`../blueprint/IMPLEMENTATION_BLUEPRINT.md`](../blueprint/IMPLEMENTATION_BLUEPRINT.md) | **Frozen provenance.** The specification as delivered, verified byte-for-byte by `just provenance-check`. It is the origin and the fixed point every later claim is measured against. It is never edited — both hooks deny the write. |
| [`DESIGN.md`](DESIGN.md) | **Authoritative.** One file, revised in git, amended only by a decision PR. A thin spine over the blueprint, not a copy of it. |
| [`../adr/`](../adr/README.md) | **Why.** One record per decision, immutable once accepted. A record binds; it does not restate the design. |
| [`../adr/register.md`](../adr/register.md) | **What was deferred.** Every deferral with an observable trigger, a runnable check, an owner and a next-check date. |
| [`../design_review/reviews/`](../design_review/reviews/) | **Evidence, not authority.** A review finds defects and cites measurements. It does not change the design; the ADR that responds to a finding does. |
| [`../design_review/design_principles/`](../design_review/design_principles/) | **The standard.** The charter (DM-01–DM-60, gates G1–G7), the directive, the review template, the reference, and the repo-specific addendum. |
| [`../plans/`](../plans/README.md) | **How work is sequenced.** Living until done. When the code and a plan disagree, the code is what runs. |
| [`../architecture/compatibility-matrix.md`](../architecture/compatibility-matrix.md) | **Evidence.** What the pinned upstream tools actually do, with primary-source quotes and retrieval dates. |
| [`../../STATUS.md`](../../STATUS.md), [`../reports/acceptance.json`](../reports/) | **State of work.** What runs today and which gates hold. Generated from real test output. |

### Why the design is not the blueprint

The blueprint is frozen on purpose: it is the specification as delivered, and freezing it is what
makes a later deviation *visible* instead of quietly absorbed. But a frozen document cannot be the
living design — the moment an ADR changes something, the blueprint is no longer a true statement
of what the system is, and reading it would mislead.

So the two roles are split. The blueprint answers *what was specified*. `DESIGN.md` answers *what
the design is now*. `DESIGN.md` is a **spine**: one to three sentences per section citing
`blueprint §N.M` for the full text. Copying the blueprint's 721 lines into it would create two
independently editable statements of the same fact — a G1 failure under the very charter this
process installs (DM-02), and precisely the defect a review here is supposed to catch.

## Section numbers are citation targets

ADRs, reviews, plans and the lint all cite the design by section: `§6.3`, `§B7`, `§14.1`. Those
numbers mirror the blueprint's and are **stable**. New material is inserted as a sub-section
(`§6.2.1`), **never** by renumbering, and `python3 scripts/adr.py lint` resolves every `§`
citation in every ADR against a heading in `DESIGN.md` — so a renumbering fails `just adr-lint`
rather than quietly rotting every record that cited it.

The thirteen binding decisions carry stable IDs **§B1–§B13**. They are citation targets, not new
decisions: §B*n* means exactly what the blueprint row says. Their mapping onto the charter's DM
principles and G1–G7 gates is in
[`../design_review/design_principles/ADDENDUM.md`](../design_review/design_principles/ADDENDUM.md) §1.

## The amendment rule

`DESIGN.md` changes only through a commit that:

1. adds or changes **exactly one** decision record under [`../adr/`](../adr/README.md);
2. adds a row to `DESIGN.md`'s **Revision history** table naming the revision, the date, the
   change, and the ADR that decided it;
3. adds an inline `> Decision: ADR-NNNN` line under the heading of **every** section the decision
   governs, so a reader of the section finds the record without searching;
4. keeps every existing section number exactly where it was;
5. leaves `just adr-lint` clean.

A record may sit at `status: proposed` while a review is outstanding. It becomes `accepted` only
when the review's verdict is Accept or Accept-scoped — see
[`../adr/README.md`](../adr/README.md) and the `adr` skill for which changes need a review at
all.

There is no GitHub machinery behind this. pse-arrow, where this process comes from, enforces the
equivalent with PR labels, a semantic-title action and required checks; this repository has no
remote and a linear history, so `just adr-lint` inside `just ci` is the enforcement, and the
maintainer's commit is the approval.

## Evidence labels are mandatory

Every claim in `DESIGN.md`, in an ADR, in a plan's verification section and in a register row
carries a charter §D label — `Proposed`, `Interface-checked`, `Implemented`, `Tested`,
`Measured`, `Formally established`. `Tested` and `Measured` must name the test or benchmark
**and the conditions**. `Proposed` is not a failure state; an unlabelled claim is.

These labels grade **design claims** only. Acceptance gates use `passed`/`failed`/`blocked`/
`not_run`; runtime evidence records use the six epistemic classes of §6.2. Three vocabularies,
three domains, never interchangeable — see
[`../design_review/design_principles/ADDENDUM.md`](../design_review/design_principles/ADDENDUM.md) §3.

## What is not editable from a session

`AGENTS.md`, `CLAUDE.md`, `.claude/rules/`, `.claude/settings.json`, `scripts/hooks/` and
`scripts/env.sh` are the enforcement layer. Both hooks deny writes there, shell redirects
included. An agent that can edit its own guardrails does not have guardrails.

When one of them needs to change, the change is proposed here as an ADR and the **operator**
applies it, then re-records the manifest:

```bash
LIBENR_ALLOW_GUARDRAIL_RERECORD=1 just guardrails-record
just guardrails-check
```

Everything this process needs is outside that boundary: `docs/`, `scripts/adr.py`, the
`justfile`, `rules/`, `.claude/agents/`, `.claude/skills/` and `tests/gates.toml`.

Workflows are **skills**, never a runtime-specific command directory: `.codex/` and `.agents/`
expose `.claude/skills` and `.claude/agents` through symlinks, so one file serves Claude Code and
Codex alike. `just lint-agents` fails if that stops being true.

One item is outstanding and belongs to the operator. `AGENTS.md` carries a nine-row summary of
the binding decisions that is **not** the same set as blueprint §1.1 — it adds §B12 and §B13 and
omits §B1, §B9, §B10 and §B11. Until that table points here instead, two documents state the
binding set. A reviewer should treat a disagreement between them as a finding, not as an
ambiguity to resolve silently. The proposed replacement text is in
[`decisions-rule.draft.md`](decisions-rule.draft.md) together with the `.claude/rules/` file this
process wants.
