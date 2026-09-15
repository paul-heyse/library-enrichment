---
title: Complete required Phase 4 through Phase 6 scope
status: in-progress
date: 2026-09-14
adrs: [ADR-0020]
phase: 4
---

# Complete required Phase 4 through Phase 6 scope

**2026-09-14 architecture pivot:** execution of this plan is paused. Follow
[Plan 10](10-arrow-datafusion-architecture.md) for the new implementation sequence and target
architecture. The user explicitly removed preservation of existing development evidence and
historical snapshots: the historical-reader/migration obligations below are superseded planning
history, not remaining work. Plan 10 carries the still-required product behavior into one fresh
typed Arrow/DataFusion design. No existing state was deleted during planning.

User approved implementation on 2026-09-14. Evidence strength: Proposed until the
individual checks below execute. Preserve the frozen blueprint, accepted decisions,
historical reports, and all concurrent work. Plan 08 is a historical checkpoint.

## Baseline

The starting tree is uncommitted over `cd6d9490a6433ae355b04a465538062e1320544c`.
`.dev-state/phase456-completion-baseline/` preserves source, the binary diff, file
digests, and prior reports. The prior receipt-bound report is valid as a record of
executed assertions; it does not establish all required scenarios. No previous
result is silently relabelled or promoted to current-tree acceptance.

## Agreed decisions

- Required blueprint scope plus correctness gaps; unrelated expansion stays deferred.
- Typed, bounded Python runtime object inspection, separate from source/stub facts.
- Explicit existing-login reuse inside disposable client homes for acceptance.
- Read-only disk-backed admitted inputs and bounded memory-backed execution scratch.
- Linux execution; existing pins and conservative Python dependency admission remain.
- Snapshot storage 3.0 reads genuine 1.0 and 2.0 snapshots; wire envelope remains frozen 1.0.
- Validated library evidence is retained indefinitely, without age-based expiry, automatic
  pruning or regeneration for an unchanged exact context. New versions acquire separate
  evidence and preserve older pinned snapshots. Cleanup of retained evidence requires an
  explicit operator action. Mutable latest/version pointers may be revalidated without
  expiring the underlying evidence; execution observations retain their environment scope.

## Completion ledger and dependency order

| Work | Requirements and implementation | Executable exit oracle | State |
|---|---|---|---|
| W1 provenance and compatibility | R-17 fragment acquisition locators; R-18 receipt-independent semantic inputs; storage 3.0; genuine historical fixture; persistent identity vectors | same bytes across releases cite correctly; unchanged revision revalidation keeps identity; old snapshots readable unchanged | passed |
| W2 execution ownership | continuous execution lease through cleanup; warm-session accounting; retained-capsule single-flight/integrity; tmpfs scratch/reservations; Rust broker description; qualification invalidation | actual task abort, cleanup failure, resource exhaustion, concurrency and restart tests | not_run |
| W3 durable inspection | concrete versioned jobs, bounded pending/control/replay; snippets and selected LSP methods; diagnostics/encoding/lifecycle; immutable semantic and typed runtime observations | real ty/RA locations, Unicode, reuse/eviction, pending/restart, runtime/stub disagreement | not_run |
| W4 Rust and Phase 4 acceptance | hosted first, explicit qualified fallback, requested target/features, retained locks/producer evidence; identical stable/nightly consumer | R09/R10, P08a/P08b/P09/P10, C10-C12/C16-C18/C20; C20 requires completed operations in both languages | not_run |
| W5 installation and clients | locked bootstrap, absolute entry points, lifecycle locking; manifested preview/apply install/update/uninstall; isolated authenticated homes and retained traces; separate Context7 | A01-A06 plus real upgrade/runtime scenarios in both clients, actual correlated calls/results and reviewed briefs | not_run |
| W6 operations | atomic extraction single-flight; durable validated publication; crash barriers; same-context enrichment; closure export; locked Rust pruning; metrics and guide | strengthened C04-C07, export/pruning/metrics/dependency-update tests | not_run |
| W7 terminal acceptance | full current-tree CI, ignored containment/cleanup, live and real-client tests; receipts; phase evidence reuse; independent reproducibility and assertion audit | phases 4/5/6, acceptance-report/check; 47 active IDs passed and retired P08 not_run; mandatory nongate scope passes | not_run |

W1 precedes new evidence publication. W2 precedes W3/W4. W5 uses the accepted Phase 4
surface. Publication foundations needed by earlier work are implemented before their
callers; W6 establishes their operational acceptance. W7 runs after the source is stable.

## Public contracts

Keep nine tools and existing resources, errors, epistemic classes and envelope. Extend
inspection additively with consumer snippets, selected semantic queries, execution profile
and separately labelled runtime observations. Extend journals with concrete versioned
operation variants and legacy decoding. Distinguish wire/storage compatibility in status.
Rust owns DTOs, publication, pruning and execution policy; regenerate Python/schema projections.

## Required assertion repairs

- R09 must compile the identical consumer on stable and dated nightly.
- Rust navigation must return the fixture's known locations, not merely a non-error.
- C20 must await and assert each intended producer outcome before accepting immutability.
- C05 needs two real adapters and completed surviving evidence after one cancellation.
- C06 must reach publication barriers; acquisition interruption is separate coverage.
- C07 must create a distinct later snapshot for the same context.
- Client oracles parse real correlated events/results; prose tool names are not execution.
- Export proves referenced evidence closure; pruning proves locking and actual removals.

## Verification and delivery

Run affected Rust Clippy/tests, Python Ruff/ty/contracts and relevant producer gates during
implementation. Run full CI, live/client and explicit ignored containment once on the stable
tree; reuse only complete matching receipts for phase certification. Missing prerequisites
are blocked, unattempted work not_run, failures failed. No fabricated gate IDs or mocked
production/client passes. Independent audit checks assertion adequacy as well as replay.

Update STATUS, design evidence labels, ADRs/register, plans and operations instructions from
actual evidence. Finish with exact commands and no task-owned processes/unresolved cleanup.

## Outcome (recorded after implementation)

### What was built

W1 is implemented and passed focused validation on 2026-09-14: affected Rust Clippy/tests,
two genuine historical snapshot fixtures with identity/hash checks and corruption rejection,
10 retention/revalidation MCP tests, schema generation/conformance, Ruff and ty. ADR-0020's
contract received Accept-scoped review. Commands and results were observed in the implementation
session; these focused checks are not terminal phase acceptance.

W2 has continuous execution leases and warm-session capacity handling in progress. Its
three real cleanup tests passed after the persistent user-session environment repair
([report](../reports/user-session-environment-2026-09-14.md)). Retained capsule integrity,
bounded scratch, qualification binding and the rest of W2 remain open. W6 extraction
single-flight now atomically registers ownership and lets waiters recover from leader abort;
publication/export/pruning work remains open.

### A mistake made and corrected

Planning inspection found that several prior gate tests were weaker than their named
scenarios. Their historical logs remain; the strengthened assertions above govern closure.

### Deviations from the plan, deliberate

The plan originally named storage 1.1 based on status. Source inspection found existing
Python/revision storage 2.0. The new unified writer uses 3.0 and preserves readers for
both historical formats; wire 1.0 is unchanged. Work remains in progress.
