---
title: Deliver isolated semantics and durable verification
status: done
date: 2026-09-13
adrs: [ADR-0017]
phase: 4
---

# Deliver isolated semantics and durable verification

## Context

Part 4 of the user-authorized implementation of all remaining blueprint Phases 1-6.
The frozen blueprint and accepted ADRs govern; implementation is not acceptance.
Depends on Phase 3. Linux is the complete execution acceptance target.

Stopped at the user's requested checkpoint on 2026-09-13, then resumed from
[plan 08](08-phase-4-6-resumption.md) and completed on 2026-09-14. The Phase 4 gate reports
**13 passed / 0 failed / 0 blocked / 0 not_run** (P08 is retired in place under ADR-0005).

## Decisions

Rust remains the sole core/publication authority. Preserve old snapshots and the frozen envelope.
Current pins remain unless verified incompatibility requires a decision. R08/P06 move to Phase 3
under ADR-0011; P08 stays retired under ADR-0005. Runtime is explicitly enabled only.
Contract or policy changes require the ADR/design-review workflow before acceptance.

## Plan

Qualify rootless Podman with offline execution and explicit resources/mounts/env; reproducible Rust/Python capsules and allowlisted dependency import; persistent jobs and per-caller cancellation; Cargo/ty/runtime probes and dated rustdoc fallback; reusable rust-analyzer/ty LSP sessions, negotiated positions and P08a/P08b; C20 across every enabled profile.

## Verification

Run affected crate Clippy/tests or Python Ruff/ty/contract tests, then `just gate-phase 4`.
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

Durable verification jobs, rootless execution and capsules, Rust-owned Python dependency
admission, Cargo/ty/runtime probes, derived contexts and MCP job control — then, after the
checkpoint: an in-process cleanup supervisor, operator image setup and qualification, warm
language-server sessions against real ty and rust-analyzer, the dated-nightly rustdoc fallback,
and the full-tree C20 canary.

Gate evidence, 2026-09-14: `just gate-phase 4` reports 13 passed / 0 failed / 0 blocked /
0 not_run — C10, C11, C12, C16, C17, C18, C20, P08a, P08b, P09, P10, R09, R10. P08 stays retired
in place. 278 Rust tests and 108 Python tests pass; the sandbox tier runs against the two
qualified images recorded in `admitted-images.json`.

### A mistake made and corrected

Networked uv could follow transitive source instructions before Rust admission. ADR-0017 replaces
it with Rust-owned metadata admission and offline hashed installation. Recovery initially locked
only the data root despite cache-scoped ownership; both roots now have exclusive locks.

The review's F3 residual — a dropped execution future or a failed removal left no in-process
owner, and a failed job released its worker permit while its container might still be alive — is
closed with **both** halves the review asked for. `execution/cleanup.rs` retains the
`OwnedSemaphorePermit` synchronously in `Drop` (taking it inside the spawned retry task was the
first, wrong, attempt: the permit was already gone by then) and quarantines admission until
absence is confirmed. Three real-container oracles prove it, including an independently observed
descendant heartbeat.

Two smaller corrections worth recording. `cleanup_confirmed` was hard-coded `true`; an
unconfirmed removal now returns `partial` with the missing assurance named, keeping the probe
evidence rather than discarding it behind an `Err`. And the container tests failed
intermittently until `owned_since(root, before)` scoped them to the run's own containers and a
nextest `containers` test-group serialised them — leftovers in a shared Podman root, not a bug in
the supervisor.

### Deviations from the plan, deliberate

ADR-0017 supersedes ADR-0016's Python resolution step while retaining its other contracts.
The user requested a checkpoint and stop before the remaining Phase 4–6 implementation.

**The rustdoc fallback runs inline-bounded, not as a separate durable job kind.** Plan 08 and the
resumption plan both describe it as a durable job, and it is not one: `execution/rustdoc.rs` runs
it inside the resolve call under `require_capsule_budget` and the ordinary execution deadline.
The reason is that the durable-job machinery exists to carry work across a client disconnect with
per-caller interests, and a single bounded `cargo rustdoc` in a capsule does not need that — it
completes or it fails within one call, like every other producer. Making it a job kind would have
added a journal state, a payload variant and a recovery path for work that never outlives its
caller. The gate that matters is unaffected: R09 still distinguishes stable from the dated
nightly, and the fallback is still gated on both `allow_local_build` and an enabled `build`
profile. If a future fallback needs to outlive a call — a whole-workspace build, say — that is
the trigger to revisit.

`ResolveRequest.allow_local_build` defaults to `false` and is required *in addition to* the
`build` profile, per the user's direction: neither the caller nor the operator alone can cause a
local compilation.

## Implementation order refined after Phase 3

1. **Execution-image admission and process boundary.** Add operator-invoked preview/apply setup
   for service-owned Podman roots and immutable producer images. The verified candidates are
   Rust1.98.1 and Python3.14.7 slim-trixie linux/amd64 bases; final derived images must include
   pinned analyzer/source/dated-nightly components or ty0.0.80/uv0.12.13 and be recorded by actual
   OCI digest. Base hashes alone do not qualify a derived image. Add readiness qualification that
   executes non-root, checks actual cgroup limits, denies network/home/repository access, bounds
   output/time/processes, and removes the exact owned container and descendants on cancellation.
   Configured enablement alone never establishes an operational profile.
2. **Reproducible capsules.** Use only validated selected artifacts and service-owned paths.
   Separate bounded network acquisition from offline execution. Rust resolves a sanitized consumer
   manifest/lock and admits registry dependencies; discard unadmitted Cargo configuration, Git
   dependencies, wrappers and credential helpers. Python uses explicit interpreter/platform,
   Rust-owned bounded registry closure under ADR-0017; Rust parses all requirements before following
   their edges, fetches and verifies selected wheel URLs/hashes, then installs the admitted closure
   offline. Unsupported/conflicting/native/source cases remain unresolved. Reject source URLs,
   editables/local directories/Git before invocation. Never combine uv --no-build with
   --only-binary (the pinned CLI rejects that combination). Retain complete locks, artifacts,
   producer/image identities, target/features/extras and explicit unreproduced project inputs.
3. **Durable explicit jobs.** Rust request/result types govern verification, semantic queries and
   job control. Persist every transition in the seven-state vocabulary. Pending receipts carry an
   independent caller-interest token; cancelling one interest cannot remove another's work.
   Bound queues, waits and concurrent workers. Cancellation and timeout must terminate owned
   containers before reporting terminal cancellation. After restart, preserve terminal results;
   mark interrupted non-idempotent work failed with a next action, rather than silently rerunning
   runtime code. Return an ordinary MCP pending envelope and finish via job_control/resource reads.
4. **Probe outcomes and derived context.** Compile, typecheck and runtime requests have distinct
   scopes. Preserve requested versus actual environment identity. Resolving a capsule creates a
   new context, never rewrites the old declared environment. Store the caller's snippet and test
   intent, exact executable/image/command, logs, exit/deadline state and evidence class. Successful
   process exit is not a universal compatibility or complete assertion-coverage claim. Implement
   valid/invalid consumers for each language, a successful typecheck followed by failing runtime
   assertion, and unavailable-native/dependency cases. Only explicitly enabled runtime policy
   permits runtime requests. A caller cannot enable a profile through the request.
5. **LSP and fallback producers.** Reuse bounded warm sessions by server/image/capsule identity,
   negotiate UTF-8/UTF-16, implement framing/document versions/capabilities/diagnostics/cancellation
   and shutdown. A snippet or symbol-derived anchor is mapped by the service; callers do not
   manufacture coordinates. Preserve unsupported, unresolved, incomplete and empty results as
   distinct observations. ty initialization disables uv/external integrations; rust-analyzer's
   build scripts/proc macros/check-on-save have separate explicit settings. Signature-only reads
   never start an LSP. When hosted Rust JSON is absent, an explicitly selected build producer may
   use the dated nightly inside its capsule; verify emitted format before normalization and
   retain complete producer/environment provenance.
6. **Acceptance.** Wire actual existing and new fixtures to the Phase 4 IDs without weakening
   their frozen assertions. C20 covers every enabled profile with a content-and-path digest over
   the studied canary repository, not merely unchanged filenames or sizes. Run schema/Rust/Python
   checks, phase gate and independent audit. Missing tools/images/capabilities stay blocked with
   the exact setup prerequisite; a successful startup probe is not a containment acceptance pass.

The compatibility matrix's Phase 4 rows are primary-source/probe evidence for this sequence.
No derived execution image or target-execution capability is accepted by this planning refinement.
