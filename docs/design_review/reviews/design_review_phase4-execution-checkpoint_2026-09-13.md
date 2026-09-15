# Phase 4 execution implementation checkpoint

## Current re-review — 2026-09-14 02:20 UTC

**Decision: Revise; checkpoint and stop.** This section supersedes the original finding
statuses below. The historical checkpoint is retained to explain the corrections. This was
a read-only source re-review; no independent tests, Phase 4 gate or acceptance claim is made.
Scope additionally covered the Rust-owned Python closure and requirement parser, service
writer locks, and the new transitive-admission, blocked-transfer and crash-recovery fixtures.
LSP and later phases were not reviewed. The implementer was still running focused tests.

| Finding | Current outcome | Evidence and remaining work |
|---|---|---|
| F1 — admission before resolution | Corrected at Implemented strength | `execution/python_closure.rs` parses every owning package's requirements before following dependencies; it admits registry names rather than source URLs, fetches through the Rust policy client, validates wheel hashes/metadata, and parses transitive requirements before extending the closure. Installation is offline, hashed and `--no-deps`; networked uv resolution is removed. The hostile-transitive fixture asserts no request to the injected endpoint. |
| F2 — nested Cargo source escape | Corrected at Implemented strength | `execution/capsule.rs:303` rejects path/Git/alternate-registry dependency entries recursively, including target/build/dev tables, while allowing ordinary nonescaping target source-file paths. The formerly reachable nested dependency manifests are no longer admitted. |
| F3 — launch/removal/recovery ownership | Partially corrected; residual open | `execution/mod.rs` persists owner/name/capsule/image before `podman create`, waits for creation before observing cancellation, uses `start --attach`, reaps the launcher before removal, and confirms absence before deleting ownership. Startup reconciles owned containers before `Jobs::open`. `service.rs:82` now acquires an exclusive cache-owner lock before recovery, in addition to the data-root writer lock. See residual below. |
| F4 — cancellation during download | Corrected at Implemented strength | `python_closure::fetch` races the HTTP request against cancellation and an aggregate acquisition deadline; work-loop checks prevent subsequent fetches. `PreparationError::Cancelled` maps to journal state Cancelled and the frozen VERIFICATION_FAILED error. The blocked-wheel fixture now checks that state, sub-three-second completion while its server release remains blocked, and capsule removal. |

**F3 residual (DM-29, DM-30):** There is still no in-process cleanup supervisor for a dropped
execution future or failed container removal. Dropping the future kills the attached Podman
client but retains ownership for a later restart; this is not immediate descendant cleanup.
A removal error propagates to terminal verification failure and releases the worker permit
while the owned container may still be active until its container timeout. Repeated such
failures can exceed the configured concurrent-worker bound. Creation timeout likewise retains
ownership and reports failure without claiming cleanup; no target code starts in the create
stage, so that is a materially narrower risk than the original run/remove race.

The next change should either retain a cleanup supervisor and its permit until absence is
confirmed, or quarantine new execution admission while owned cleanup is unresolved. The
executable oracle is an actual-container test that aborts the execution task and injects
cleanup failure, independently checks a descendant heartbeat/container absence, verifies
admission/concurrency behavior, and then exercises recovery. The current crash test proves a
different boundary: it kills the daemon during a real descendant heartbeat and checks that
successful restart reconciliation stops further writes. That test also now attempts a second
daemon with the same cache and a different data root, requiring rejection while the original
heartbeat continues. Neither fixture substitutes for the residual failure-path oracle.

| Charter gate | Current verdict | Basis |
|---|---|---|
| G1 — Authority | Pass within inspected scope | Rust-owned journal, closure and immutable results; cache ownership now has a matching exclusion lock |
| G2 — Semantic fidelity | Pass within inspected scope | Distinct preparation cancellation and actual derived environments |
| G3 — Validity | Pass within inspected scope | Original F1/F2 acquisition bypasses are rejected |
| G4 — Hidden behavior | Pass within inspected scope | Python closure fetching is Rust-owned and policy checked; target execution stays offline |
| G5 — Consistency and recovery | Unresolved | F3 in-process failure/task-drop cleanup and admission bounds |
| G6 — Transformation and reuse | Pass within inspected scope | Original pinned in-flight reuse contract retained |
| G7 — Truthful capability claims | Unresolved | Immediate cleanup across every failure/task-drop path remains unsupported |

One preliminary marker concern was withdrawn after checking the current primary standard:
the `extra` field uses membership/nonmembership against the full set of selected extras.
The parser's set-based comparisons implement that current rule; the older per-extra
interpretation is not a valid defect here. Verified 2026-09-14 UTC against
[PyPA dependency specifiers](https://packaging.python.org/en/latest/specifications/dependency-specifiers/#defined-environment-marker-fields).

The older `.dev-state/phase4-verification.log` records five passes and one failure caused by
an obsolete expected JOB_CANCELLED error. The corrected fixture expects VERIFICATION_FAILED
plus journal Cancelled. Final `.dev-state/phase4-handoff-verification.log` records six passes
from 2026-09-14 02:19:45.354258–02:20:07.696142 UTC; its execution receipt has exit zero and
valid log/source digests. The daemon library receipt likewise validates and exits zero
(02:20:10.246121–02:20:13.436901 UTC). These were inspected implementer runs, not independently
rerun tests. The original review's coverage statements below describe the earlier checkpoint,
not the new crash/transfer fixtures.

**Acceptance-report audit:** Independently read the registry/report and ran
`uv run --frozen python scripts/acceptance-check.py`: exit zero. Report generation time is
2026-09-14T02:20:22+00:00; report and current source both have digest
`11370f944668fba86525bfb66ef756ac21adf519465858ea59b10d56cdfff090`.
All 48 registered IDs appear once: **0 passed / 0 failed / 0 blocked / 48 not_run**.
There are no passed commands to rerun. These current-tree states do not erase historical
Phase 1–3 acceptance evidence or turn the six focused Phase 4 fixture passes into a phase gate.

## Historical checkpoint — original observations

## 1. Decision and scope

**Decision: Revise.** This is a bounded implementation review under ADR-0016, observed
2026-09-14 01:56 UTC (2026-09-13 local). Source was actively changing. It is not a frozen
acceptance audit, and it establishes no Phase 4 gate result.

**Evidence strength: Implemented / Interface-checked.** Reviewed the typed execution requests,
daemon execution runner and capsule preparation, durable jobs, verification operations,
server shutdown, ADR-0016, and focused execution-boundary/journal assertions. No commands
executing target code or test suites were run. Earlier test results supplied by the implementer
are not promoted to independent verification. LSP, rustdoc fallback, platform qualification,
and Phase 5–6 are outside this review. Container cleanup race analysis is static, not a
reproduced race. File/line references identify the checkpoint and may move during correction.

## 2. Authority and lifecycle map

| Fact | Authority and identity | Lifecycle / derived representation |
|---|---|---|
| Probe intent | Rust `VerifyRequest`; pinned context/snapshot and stored snippet | Validate, pin, journal, execute |
| Caller interest | Unique journal token | Detach one token; last detach signals cancellation |
| Job state | Rust `Jobs`, one JSON journal per job | Persist transitions before returning; restart fails interrupted work |
| Actual environment | Qualified image, selected closure and lock | New environment/context; original declaration retained |
| Running container | Random local name in `Runner::execute` | No durable job-to-container ownership record; see F3 |
| Result | Immutable process/lock/snippet artifacts plus terminal envelope | Explicit probe class and limitations |

## 3. Semantic contracts and invariants

| Invariant | Enforcement / failure | Assessment |
|---|---|---|
| No arbitrary host command or path in probe request | Typed DTO and `VerifyRequest::validate` | Implemented |
| Original snapshot cannot drift while queued | `ops/verify.rs:59` pins opened snapshot | Implemented |
| Registry-only acquisition before target execution | Capsule admission followed by networked resolver | Violated at F1/F2 |
| Cancellation means owned descendants are removed before terminal cancellation | Runner remove/kill sequence | Unresolved launch race and recovery ownership, F3 |
| Actual observations do not overwrite original declarations | Derived environment/context in `execute_inner` | Implemented; no claim of original project-lock reproduction |

## 4. Derivation and execution design

The service validates policy, pins input, deduplicates in-flight work and queues a concrete
Rust producer. Preparation qualifies the image, acquires a dependency closure, and stores its
lock. The probe executes offline in the capsule. The successful result names the actual
environment and limits its conclusion to the supplied snippet. This is an appropriate bounded
pipeline; a generic workflow engine is unnecessary. The admission and cleanup boundaries need
the corrections below before their stronger guarantees are supported.

## 5. Representative journeys

An ordinary probe follows the lifecycle above. A source snapshot change creates different
pinned job input; a shared submission obtains a distinct interest token. A direct-URL Python
dependency or nested Cargo manifest crosses an insufficiently checked acquisition boundary.
Cancellation during wheel download is not observed until later stages. Cancellation during
container creation can race removal. Restart fails interrupted jobs honestly but cannot recover
the exact container identity from their journals.

## 6. Acceptance gates

These are charter review gates, not `tests/gates.toml` results.

| Gate | Verdict | Evidence / required action |
|---|---|---|
| G1 — Authority | Pass within inspected scope | Rust owns requests, journals and publication; original declarations remain separate |
| G2 — Semantic fidelity | Pass within inspected scope | Probe classes and actual derived environments are explicit; preparation cancellation now has a typed process outcome |
| G3 — Validity | Fail | F1/F2 allow unadmitted dependency sources to reach acquisition |
| G4 — Hidden behavior | Fail | Networked dependency resolution precedes complete source admission |
| G5 — Consistency and recovery | Unresolved | F3 launch/drop/restart cleanup ownership; F4 cancellation progress |
| G6 — Transformation and reuse | Pass within inspected scope | In-flight key includes pinned request, image, environment and execution policy; no completed-result cache assessed |
| G7 — Truthful capability claims | Unresolved | `cleanup_confirmed` is stronger than the inspected launch/removal protocol and existing assertion oracle |

## 7. Principle findings

| Finding | Principle IDs | Concrete evidence or gap | Consequence | Proposed correction | Verification |
|---|---|---|---|---|---|
| F1 High: Python requirement-source admission occurs too late | DM-07, DM-28 | `execution/capsule.rs:168` only checks whether Requires-Dist exists; lines 197–217 run networked uv resolution before lock inspection | A direct wheel URL in root or transitive metadata can cause access outside the admitted registry before Rust examines the resulting lock. Wheel-only resolution does not establish registry-only inputs | Reject unadmitted requirement sources before resolver access, including transitive metadata; enforce acquisition origin policy at the effective fetch boundary | `just` gate / adversarial registry fixture: root and transitive metadata reference a second unapproved endpoint; assert rejection and zero requests there |
| F2 High: Cargo source admission omits nested manifests | DM-07, DM-28 | `execution/capsule.rs:368` validates only root Cargo.toml; `validate_cargo_sources:451` permits nonescaping relative paths | A root dependency pointing to `deps/local` leads Cargo fetch to a nested manifest with Git, alternate registry or escaping path sources that were never checked | Reject dependency path settings or validate every reachable manifest before acquisition; distinguish dependency paths from ordinary target source paths | `just` gate / nested-manifest fixture: reject prohibited nested sources before network activity |
| F3 High: Cleanup confirmation lacks stable launch and recovery ownership | DM-29, DM-30 | `execution/mod.rs:213–219` removes while the launching client is still alive, then kills it; `rm --ignore` accepts absent names. Line 207 can return early on wait error. Random name is absent from `JobRecord`; `jobs.rs:86` fails interrupted jobs without identifying their container; `server.rs:54–59` can return before workers finish | Immediate cancellation can remove an absent name before the client creates the container, yet later return `cleanup_confirmed=true`. Future drop or shutdown failure cannot recover an exact owned container from the journal. The launch race is static analysis, not experimentally reproduced | Establish job/container ownership before spawn, stop/reap launch activity before final absence confirmation, and reconcile owned interrupted execution on restart. Preserve cleanup uncertainty when reconciliation fails | Actual-container `just` gate: cancel during launch, abort/restart with a descendant writing a marker, inject broker cleanup failure; independently inspect absence and marker cessation before terminal cleanup claims |
| F4 Medium: Wheel acquisition ignores cancellation and aggregate deadline | DM-29, DM-30 | `execution/capsule.rs:229–305` iterates package wheels and awaits fetcher.get without observing cancel; shutdown has only request timeout + 30 seconds | Cancellation during one slow dependency continues subsequent transfers; multiple slow responses can outlast shutdown's budget | Observe cancellation between artifacts and during fetch; use a job-wide preparation deadline, retaining completed acquisition evidence | `just` gate / two slow wheel responses: cancel during first, assert second is never fetched and shutdown finishes within its declared bound |

**Applicability:** Validity, effects, execution lifecycle, failure recovery and provenance bear
directly on this slice. Numerical optimization, graph representation and broad extension
architecture are not material to these bounded changes. Other charter principles are not
claimed exhaustively satisfied. F1/F2 are implementation violations of ADR-0016's explicit
registry-only boundary; F3 is an unresolved cleanup protocol, not a claim that leakage was
measured. References above are relative to `crates/enrichment-daemon/src/`.

## 8. Alternatives and architectural leverage

| Alternative | Correctness and cost | Decision |
|---|---|---|
| Static-only baseline | Avoids execution effects; cannot satisfy explicit verification | Retain for static operations, insufficient for Phase 4 |
| Current concrete producer/journal pipeline | Appropriate scope; incomplete admission and cleanup protocols | Correct the boundaries |
| Simpler viable initial execution subset | Admit dependency-free wheels and registry-only crates with no nested dependency paths; reject unsupported closures explicitly | Viable narrowing while transitive admission is completed; does not remove cleanup obligations |

## 9. Verification and measurement plan

The existing `execution_boundary_bounds_output_deadline_and_cancels_descendants` test checks
returned status and the runner's own `cleanup_confirmed` Boolean. It does not independently
observe descendant/container absence, and it cancels after two seconds rather than attacking
container creation. Strengthen that oracle as F3 specifies. Journal unit tests establish state
semantics separately; they do not prove process recovery. Add F1/F2/F4 fixtures before the full
Phase 4 gate and independent frozen reruns. No performance claim is made here.

## 10. Exceptions and unresolved decisions

No exception is proposed for registry-only acquisition or truthful cleanup. Support may be
explicitly narrowed as in section 8. Restart currently requires human inspection of execution
state; decide and document how exact owned containers are identified and reconciled, including
failure to access the broker. This review does not alter ADR-0016 or frozen blueprint scope.

## 11. Decision and implementation changes

**Revise the inspected checkpoint.** The canonical ownership and evidence boundaries are useful,
but input admission has concrete gaps and cleanup assurance needs a stronger protocol/oracle.

| Priority | Change | Principles | Required evidence |
|---|---|---|---|
| 1 | Admit dependency sources before network resolution | DM-07, DM-28 | F1/F2 negative fixture endpoints remain untouched |
| 1 | Stabilize container ownership and cleanup confirmation | DM-29, DM-30 | Independent launch-cancel and restart absence checks |
| 2 | Bound and cancel dependency acquisition | DM-29, DM-30 | F4 slow-transfer cancellation fixture |

No source files, primary acceptance reports, or gate logs were changed by this review.
