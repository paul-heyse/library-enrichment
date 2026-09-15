---
title: Deliver comparisons and bounded research
status: done
date: 2026-09-13
adrs: [ADR-0014, ADR-0015]
phase: 3
---

# Deliver comparisons and bounded research

## Context

Part 3 of the user-authorized implementation of all remaining blueprint Phases 1-6.
The frozen blueprint and accepted ADRs govern; implementation is not acceptance.
Depends on Phase 2. Linux is the complete execution acceptance target.

## Decisions

Rust remains the sole core/publication authority. Preserve old snapshots and the frozen envelope.
Current pins remain unless verified incompatibility requires a decision. R08/P06 move to Phase 3
under ADR-0011; P08 stays retired under ADR-0005. Runtime is explicitly enabled only.
Contract or policy changes require the ADR/design-review workflow before acceptance.

## Plan

Normalized API additions/removals and selected compatibility interpretation; doc/config/release changes; environment comparability; deterministic lexical ranking and filtered DataFusion queries; complete response budgets and cursors; conditional revalidation and negative caches; immutable revision research. Gates R08, P06, C01-C03, C08-C09, C15.

## Verification

Run affected crate Clippy/tests or Python Ruff/ty/contract tests, then `just gate-phase 3`.
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

## Implementation checkpoint — 2026-09-13

Implemented ADR-0014 and ADR-0015: comparison, scoped search/cursors, complete-envelope budgets,
HTTP validators/negative caching and immutable public GitHub revision acquisition. Python
revisions use the static worker; Rust revisions expose source/config/docs with missing compiled
API. Repository/root-scoped replay is preserved across restart. Wrong commit/PAX identity never
publishes. Same-path API variants and meaningful code whitespace survive comparison.

`just gate-phase 3`: 8 phase gates passed; aggregate 24 passed / 24 not_run out of 48;
231 Rust, 89 ordinary Python and two live tests. Source digest
`a627b37b7e9abebde0b168770e211daea845eab1c3349509ab481e1d32b84a13`.
Independent acceptance rerun remains pending; source is frozen for that audit.

The first full run caught older tests assuming an unimplemented comparison method, payload-only
budgets and replay across changed source registries. Their intended assertions now exercise the
current contract. C15's registration was corrected from a configuration test to a real missing
worker failure; configuration comparison remains exercised under R08's supplementary phase clause.
A repeated-token query actually crashed the daemon before deduplication and complexity bounds;
its real MCP regression and oversized escaped-result recovery now pass.


### Independent audit correction

The first independent rerun reproduced 231/89/2 tests but exposed C02's inadequate assertions:
a 1024-byte request emitted a 1055-byte envelope, and FastMCP duplicated the envelope in text.
Audited tally was therefore 23 passed / 1 failed / 24 not_run, not the generated 24/0/24.
The correction uses one structured envelope plus a compact text summary and a measured fixed
512-byte MCP framing allowance, enforces the final overflow fallback, advertises the canonical
output schema and tests actual stdio frames. SDK tests inspect raw structured_content because
FastMCP converts its convenience data field into schema-derived models. Fresh final gates and
independent audit are required after this correction.

The raw transport regression now passes at requested1024/2048/4096-byte envelope budgets, each
with the fixed512-byte MCP framing allowance. Canonical advertised schemas are compared after
reference inlining; malformed tool output and unimplemented-operation output are validated by
actually invoking the response projection. The full gate and independent audit are being rerun
at the corrected source, rather than reusing earlier receipts.

Final independent audit accepted corrected source `e282153379fb21df723c8887749ba6e15edb5bb3d76ebdf20eba36524a382962`:
231 Rust / 90 ordinary Python / 2 live tests; 24 passed / 0 failed / 0 blocked / 24 not_run.
Receipts `.dev-state/phase3-final-audit/`, 2026-09-14 01:31:30–01:32:36 UTC. Actual raw frames
are below the envelope budgets plus 512 bytes; previous C02 finding is resolved.
