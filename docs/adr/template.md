---
id: ADR-NNNN
title: TITLE
status: proposed
date: YYYY-MM-DD
deciders: [paul-heyse]
level: decision
principles: [DM-00]
design: [§1.1]
review: not-required: REASON
evidence: Proposed
supersedes: []
superseded-by: null
revisit: An observable trigger, not a date.
verification: The test, gate or lint that shows this holds.

---

# ADR-NNNN: TITLE

*Delete the guidance under each heading as you fill it in. A small deviation
fills each section in one line — charter §H: "a short, concrete decision record
is sufficient". Cite `design §N.M` rather than restating the design.*

## Context

Two or three sentences: what forced this decision, the `docs/design/DESIGN.md`
section it governs, what the design assumed, and — if this is a deviation —
exactly how reality differs from that assumption. Name the design-review finding
that raised it, if any.

## Scope

What this record binds and what it deliberately leaves open. A `must-gap`
**narrows the supported scope** here; it never claims compliance. If the record
*amends* the design rather than deviating from it, say so — that is why its
level is `decision`.

## Drivers

The forces, one line each: correctness, reproducibility, supply chain, the
repository boundary, cost. The charter principle IDs go in `principles:`, not
here.

## Options

Each option in one line, with the reason it was not taken. "Do nothing" counts.
An ADR with one option is a memo, not a decision.

## Decision

What we will do, stated as a rule someone can follow or break — so that a later
reader can tell whether the code still follows it.

### Consequences

What becomes easier, what becomes harder, what is now foreclosed, including the
unpleasant parts and the cost of reversing this if it turns out wrong.

### Compensating controls

What keeps the downside bounded: a gate, an `ast-grep` rule, a pin, a register
row, a fallback.

## Evidence

Primary sources with URL, retrieval date and exact quote. Recollection is not
evidence; if it was not read this session, it does not belong here. Use the
`upstream-verifier` subagent when the claim concerns an external tool.

| Claim | Source | Retrieved | Quote |
|---|---|---|---|
|  |  |  |  |

## Verification

The specific test, acceptance gate, `just` recipe or `ast-grep` rule that fails
if this decision is violated later — the same thing named in `verification:`,
plus where its output lands. Not "code review". A decision with no executable
consequence will erode silently.

## Boundaries preserved

Confirm the binding decisions this does **not** change, by ID: §B1–§B13
(`docs/design/DESIGN.md` §1.1). A deviation must preserve the binding
boundaries; say explicitly which ones still hold.

## More information

Links: design sections, the design-review finding, the compatibility-matrix row,
the register row, the plan that implements it.

## Status history

- YYYY-MM-DD — proposed.
