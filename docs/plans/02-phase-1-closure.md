---
title: Close the static Rust slice
status: done
date: 2026-09-13
adrs: []
phase: 1
---

# Close the static Rust slice

## Context

Part 1 of the user-authorized implementation of all remaining blueprint Phases 1-6.
The frozen blueprint and accepted ADRs govern; implementation is not acceptance.
Depends on Phase 0. Linux is the complete execution acceptance target.

## Decisions

Rust remains the sole core/publication authority. Preserve old snapshots and the frozen envelope.
Current pins remain unless verified incompatibility requires a decision. R08/P06 move to Phase 3
under ADR-0011; P08 stays retired under ADR-0005. Runtime is explicitly enabled only.
Contract or policy changes require the ADR/design-review workflow before acceptance.

## Plan

Preserve feature knowledge; correct retrieval assertions; publish metadata/source-only snapshots; bind replay to its snapshot; complete producer provenance; wire R01-R03, R05-R07 and C13; add a real-crate cold/offline test; make acceptance evidence and schema validation reproducible.

## Verification

Run affected crate Clippy/tests or Python Ruff/ty/contract tests, then `just gate-phase 1`.
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

Proposed, except where subsequent checkpoint entries record implemented/tested behavior.

### A mistake made and corrected

Record concrete corrections as implementation proceeds.

### Deviations from the plan, deliberate

None recorded.

## Checkpoint — 2026-09-13

Tested: declared feature knowledge, partial source snapshots, consistent snapshot replay,
completed normalization provenance, version-mismatch quarantine, corrected fixture assertions,
non-mutating schema check, and source-bound execution receipts. Independently reproduced 214
Rust tests, 72 Python tests and one live cold/offline itoa test. Seven Phase 1 gates passed.
The complete runner passed before the final mismatch regression; that regression then passed
in the refreshed suites and independent audit. Existing lint recipe masks spelling errors in
vendor/lockfile names; this is an operational tooling defect, not a verified clean spelling gate.

Mistakes corrected: live test initially assumed a nested Release.key despite serde flattening;
the mismatch MCP test initially omitted required ecosystem. Both failed openly and were fixed.
ADR-0011 moves comparison to Phase 3; ADR-0012 records feature knowledge and historical-ID reads.
Same old offline resolve arguments do not recover pre-knowledge-flag context hashes; explicit
old context/snapshot references remain readable. No runtime or Python acceptance is claimed.

Evidence: docs/reports/acceptance.json at source digest
46d23252c43eba2d94cd47f515ef7a1fcf931d4014484e0aed5024286a2c025a;
independent receipts under .dev-state/phase1-audit (2026-09-13 23:41 UTC).
Later source edits require refreshed current-tree evidence.
