---
description: Record an architecture decision, including any deviation from the design.
argument-hint: <short-slug>
allowed-tools: Bash, Read, Grep, Glob, Write, Edit
---

Create a decision record for: $1

`docs/design/DESIGN.md` is the design. **ADRs record why it reads the way it is.** Design reviews
under `docs/design_review/reviews/` are evidence, not authority. Plans under `docs/plans/` record
how work is sequenced and stay living; an ADR is immutable once accepted.

## 1. Does this change need one?

| Change | Needs |
|---|---|
| Alters a binding decision §B1–§B13; changes the wire contract, the schema-ownership rule, the execution-policy profiles, the epistemic classes or the publication boundary; retires or re-scopes an acceptance gate; any deviation from a charter SHOULD | **ADR + design review.** Run `/design-review` first; the verdict must be Accept or Accept-scoped before the record's status becomes `accepted` |
| A new producer, projection, tool body or error code **within** an accepted decision; a small local deviation; moving a pin; a deferred trigger in `docs/adr/register.md` fires | **ADR (short).** Review at your discretion |
| Bug fixes; refactors inside existing contracts; tests; documentation wording; a patch bump inside a pinned family; tooling | **Neither** |

A change that makes a previously-recorded decision untrue needs an ADR that **supersedes** it,
not an edit to it.

## 2. Write it

```
just adr-new $1 --title "Imperative one-liner"
```

That allocates the next number, copies `docs/adr/template.md` and stamps today's date. Then fill
the front matter — these are the charter §H fields, and `just adr-lint` checks every one:

| Field | Rule |
|---|---|
| `level` | `decision` \| `should-deviation` \| `must-gap`. A `must-gap` **narrows the supported scope**; it never claims compliance. A record that *amends* the design is a `decision`. |
| `principles` | The charter IDs the decision actually turns on, `DM-01`–`DM-60`. Not a wall. `[]` is correct when none bear on it. |
| `design` | The `DESIGN.md` sections governed: `[§6.3, §B7]`. Every citation must resolve to a real heading, `§B1`–`§B13` included. |
| `review` | A path into `docs/design_review/reviews/…` (optionally `#anchor`), **or** `not-required: <reason>`. |
| `evidence` | A charter §D label: `Proposed`, `Interface-checked`, `Implemented`, `Tested`, `Measured`, `Formally established`. Never raise a label to make a record look finished. |
| `revisit` | An **observable trigger**, not a date. If it is not imminent, add a row to `docs/adr/register.md` too. |
| `verification` | The named test, gate, recipe or `ast-grep` rule that shows the decision still holds. Not "code review". |

The three sections that matter most here:

- **Evidence** — primary-source URLs with retrieval dates and exact quotes, in the table. Not
  recollection, not a plausible inference. If you have not read it this session, it is not
  evidence. Use the `upstream-verifier` subagent when the claim concerns an external tool.
- **Verification** — the specific check that fails if this is violated later. A decision with no
  executable consequence will erode silently.
- **Boundaries preserved** — which of §B1–§B13 this does **not** change. A deviation must
  preserve the binding boundaries; say explicitly which ones still hold.

## 3. Amend the design in the same commit

1. Add an inline `> Decision: ADR-NNNN` line under **every** `DESIGN.md` section the record
   governs.
2. Add a row to `DESIGN.md`'s **Revision history** table.
3. Keep every existing section number where it is — insert `§6.2.1`, never renumber.
4. If the record defers anything, add the register row so the trigger has an owner and a date.

## 4. Before you finish

```
just adr-index     # regenerate docs/adr/README.md
just adr-lint      # records, index and register; also runs inside just ci
```

**Never edit an accepted record's argument.** An accepted ADR changes only in `status`,
`superseded-by` and an appended `## Status history` line. To change a decision, supersede it:
`just adr-supersede <old> <new>`. `just adr-lint` diffs accepted records against `main` and fails
the gate otherwise — routing around that by rewriting history is not a workaround, it is the
thing the rule exists to stop.
