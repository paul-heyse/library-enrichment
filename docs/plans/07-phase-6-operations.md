---
title: Harden publication and operations
status: done
date: 2026-09-13
adrs: [ADR-0018]
phase: 6
---

# Harden publication and operations

## Context

Part 6 of the user-authorized implementation of all remaining blueprint Phases 1-6.
The frozen blueprint and accepted ADRs govern; implementation is not acceptance.
Depends on Phase 5. Linux is the complete execution acceptance target.

Not started at the 2026-09-13 user-requested stop; implemented on 2026-09-14 from
[the resumption handoff](08-phase-4-6-resumption.md). The Phase 6 gate reports
**4 passed / 0 failed / 0 blocked / 0 not_run**. Phase 4 verification deduplication was correctly
identified there as *not* shared-extraction acceptance; C04 is now proved against extraction,
measured at the upstream fixture's own request log.

## Decisions

Rust remains the sole core/publication authority. Preserve old snapshots and the frozen envelope.
Current pins remain unless verified incompatibility requires a decision. R08/P06 move to Phase 3
under ADR-0011; P08 stays retired under ADR-0005. Runtime is explicitly enabled only.
Contract or policy changes require the ADR/design-review workflow before acceptance.

## Plan

Two-adapter shared extraction and cancellation races; durable validated publication and crash injection; restart recovery; historical schema/snapshot compatibility; portable provenance export; preview-first regenerable-cache pruning; metrics/logs; dependency-update and failure tests; operations guide and full final acceptance. Gates C04-C07.

## Verification

Run affected crate Clippy/tests or Python Ruff/ty/contract tests, then `just gate-phase 6`.
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

**Shared extraction (C04).** `single_flight.rs` generalizes the §8.2 dedup key to the retrieval
side and is wired into `ops/resolve.rs` and `ops/python.rs` through a shared `acquisition_key()`.
Two independent adapter subprocesses resolving the same cold release get one producer run,
equivalent evidence, and — this is the part that makes it acceptance rather than a counter
checking itself — the *upstream fixture* records one wheel request, not two. The caller that
attached to someone else's run is told so, rather than believing it caused the acquisition.

**Cancellation (C05), publication (C06), pinning (C07).** Two adapters share one job; one drops
its interest and the other's work completes with its evidence available. Publication stages under
job-specific paths, validates, publishes an immutable snapshot directory and then flips the
context pointer atomically; SIGKILL at three injection points never leaves a reader seeing a
half-published snapshot. A pinned snapshot stays byte-identical and readable after later
enrichment.

**Provenance export.** `library-enrichmentd export` / `verify-bundle`, plus `just export` and
`just verify-bundle`. ADR-0018 records the bundle shape, because the blueprint gives export one
phrase at §13 and a directory at §2.3 and nothing else. A bundle is a directory with a
`sha256sum`-format manifest, so a recipient checks it with coreutils and no copy of this service.

**Pruning.** `just capsules-prune`, preview-first, keyed on what is *regenerable*: capsules live
under the cache root and are each derived from an artifact digest, an admitted image and a
recorded environment. Immutable retained evidence under the data root is never touched.
Failed-preparation capsules — which had no workflow at all — are retained as evidence and aged
out by `--keep-hours`.

**Metrics and logs (§14.3).** `metrics.rs` plus `enrichment_core::wire::status`: cache
hits/revalidations/misses, fetched bytes, producer duration, queue depth, response bytes, LSP
starts and reuses, verification outcomes, and evidence gaps — published in `service_status` and
written as one structured JSON line per request to stderr, never MCP stdout.

**Debt paid without suppressions.** `just lint` no longer swallows `typos` and `taplo` behind
`|| true`; `.taplo.toml` and expanded `_typos.toml` scopes were added with justifications; and all
44 broad-script Ruff violations were fixed properly — no `# noqa`, no `# type: ignore`, no
`#[allow(...)]`, no growing ignore list.

`docs/operations/README.md` now covers image setup and qualification, recovery from unresolved
cleanup, pruning, export, metrics, client registration and uninstall.

### A mistake made and corrected

**C06 was non-deterministic on the first attempt.** Killing the daemon at a wall-clock offset
raced the publication it was supposed to interrupt. It now uses the fixture upstream's
`held_paths` to block the wheel request at a known point, then SIGKILLs — an injection point, not
a timing guess. A second fix in the same test: the adapter returns a typed `UPSTREAM_UNAVAILABLE`
envelope when the daemon dies rather than raising, so the test asserts on that envelope instead of
expecting an exception.

**Two pre-existing failures surfaced once the masking was removed**, and both were real.
`rpc_boundary.rs::a_second_daemon_refuses_to_steal_a_live_socket` passed for the wrong reason: the
writer lock refuses a second `Service` *before* the socket check ever runs, so the test proved the
lock and called it the socket guard. It now asserts both guards separately. And
`test_the_adapter_validates_every_response_it_emits` raised `KeyError: 'job_control'` because
`_UNIMPLEMENTED_IN` had been emptied and `_not_implemented` left dead; both were deleted rather
than patched around.

Adding the metrics also introduced, and then required fixing, a hazard in the test harness: the
daemon now writes a line per request to stderr, and `tests/support/daemon.py` captured that pipe
without ever reading it. A long test would have filled the 64 KiB buffer and wedged the daemon
mid-run. The pipe is now drained continuously on a background thread, which is also what lets a
test assert on the log.

### Deviations from the plan, deliberate

**Export has no acceptance gate ID, and one was not invented for it.** The frozen
`tests/ACCEPTANCE_PLAN.md` is the authority on which IDs exist, and `scripts/acceptance-check.py`
rejects a registry ID absent from it that is not a registered successor to a superseded gate.
Registering a "C21" for export would have been exactly the invented gate that check exists to
catch. Export is Phase 6 *scope* without a gate clause; it is verified by
`enrichment_daemon::export::tests` and `tests/e2e/test_provenance_export.py`, both in `just test`,
so the phase gate fails if they fail. The same applies to the §14.3 metrics
(`tests/e2e/test_operational_metrics.py`).

**Bundles are directories, not archives, and are unsigned.** ADR-0018 records why: an archive puts
an extractor between a recipient and their first integrity check — the same extractor class gate
C11 treats as hostile input — and a signature over a manifest nobody yet verifies is ceremony
without key management. The manifest is shaped so a signature can be added over it later without
changing the layout.

**`service_status` moved its payload types into the core** (`enrichment_core::wire::status`)
rather than staying in the daemon. `data` is wire surface and §6.3 gives the core one
authoritative definition; leaving it in the daemon is what had made `service_status` the one tool
result the adapter could not check against its own contract.
