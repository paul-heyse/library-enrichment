---
title: Deliver static Python evidence
status: done
date: 2026-09-13
adrs: [ADR-0013]
phase: 2
---

# Deliver static Python evidence

## Context

Part 2 of the user-authorized implementation of all remaining blueprint Phases 1-6.
The frozen blueprint and accepted ADRs govern; implementation is not acceptance.
Depends on Phase 1. Linux is the complete execution acceptance target.

## Decisions

Rust remains the sole core/publication authority. Preserve old snapshots and the frozen envelope.
Current pins remain unless verified incompatibility requires a decision. R08/P06 move to Phase 3
under ADR-0011; P08 stays retired under ADR-0005. Runtime is explicitly enabled only.
Contract or policy changes require the ADR/design-review workflow before acceptance.

## Plan

Generalize Rust-owned records and generated interfaces; exact PyPI selection and safe wheel/source inspection; isolated static Griffe worker with separate source/stub observations; inventories and targeted docs; shared MCP retrieval; complete Python fixtures and P01-P05/P07.

## Verification

Run affected crate Clippy/tests or Python Ruff/ty/contract tests, then `just gate-phase 2`.
Generate schemas from Rust deliberately; compare regenerated artifacts without mutating them.
Separate deterministic, live and client logs. Every recorded result identifies its source tree,
locks, executed command, time, exit status and evidence digest. Missing prerequisites remain
blocked/not_run. Full suite and independent acceptance audit precede final delivery.

## Open items

The above scope remains open until executed tests and the phase gate establish it.
No user-global client/skill installation occurs during ordinary tests. Optional HTTP, native
MCP tasks, graph/vector systems and automatic project edits are excluded.

## Outcome (recorded after implementation)

### What was built

Rust now resolves exact PyPI artifacts, verifies hashes/internal metadata and wheel tags, safely
extracts source/stubs, inventories namespaces, runs the isolated static Griffe worker, preserves
conflicting observations/reexports, and publishes immutable shared snapshots. Inventory navigation
and bounded linked docs retain uncertain version claims. Generated worker/request/public schemas
are checked by reproducible JSON and Python generation. Source replay works offline.

Verified 2026-09-13: `just gate-phase 2` reports six Phase 2 gates passed; full registry has
16 passed / 0 failed / 0 blocked / 32 not_run. Independent audit reproduced 226 Rust / 79 ordinary Python / 2 live tests at digest
`567557fb0293821a2ce02dfc81dea60b12f1e91d5de76e85b430cb6407fde0ee`
(2026-09-14 00:29:41–00:30:31 UTC). The runner
still masks known pre-existing spelling diagnostics; this is not a clean spelling assertion.

### A mistake made and corrected

Audit corrections cover conflicting reexport targets, a real wheel `src` package, ambiguous
metadata, internal Requires-Python/tags, malformed stored observations, full producer configuration,
and missing immutable producer gaps. Tests include native extensions with and without stubs.
One stale RPC status assertion incorrectly omitted PyPI; corrected before the passing gate.

### Deviations from the plan, deliberate

None recorded.

The final corruption regression uses real Parquet reads through inspect, overview and search;
all now propagate query errors. Audit evidence is `.dev-state/phase2-final-audit/`.
