# Design review — cleanup ownership and Rust fallback

## 1. Decision and scope

**Decision: Revise this implementation slice for M1–M5.** Date: 2026-09-14.
The mechanisms inspected are **Implemented**, with source-derived counterexamples below;
this review supplies no new **Tested** or **Measured** claims. It does not certify Plan 10,
W2/W4/W6, execution qualification, or Phase 4–6 completion.

Authority: [Plan 10](../../plans/10-arrow-datafusion-architecture.md),
[Plan 09](../../plans/09-phase-4-6-completion.md),
[ADR-0021](../../adr/0021-bounded-execution-scratch.md), and the
[design charter](../design_principles/DATA_MODEL_DESIGN_CHARTER.md), directive and addendum.
The accepted hard pivot permits discarding selected development evidence; it does not permit
discarding uncertain execution ownership or unrelated operator state.

**Method and coverage:** source-only inspection of store `state.rs`; daemon `maintenance.rs`,
startup ordering in `service.rs`, `execution/budget.rs::recover_orphans`,
`execution/rustdoc.rs::BuildSpec/build`, and the local producer/publication path in
`ops/resolve.rs`. Followed calls into coordination leases, atomic record writing, capsule
inventory, execution ownership recovery and CLI result handling. Read the new focused tests,
but ran no tests, builds, containers or cleanup commands. Only this review document was written.
The shared Rust execution description is concurrently being implemented and is not certified.
Locations name reviewed functions as well as line numbers because implementation is ongoing.

## 2. Authority and lifecycle map

| Concept | Authority and identity | Lifecycle / permitted mutation |
|---|---|---|
| Service root | `state::Identity` binds format, role and canonical root | Initialize an empty root; validate an existing marker; explicitly reset selected development payloads. |
| Deletion ownership | Permanent daemon/cache/storage lock inodes and evidence retention lease | `maintenance::Guards` holds all four until `run` returns; never unlink coordination inodes. |
| Container obligation | Execution-root `owned` records, keyed by container and cache-owner digest | Remove only after creation uncertainty is settled and container absence is confirmed. |
| Quarantine reservation | Durable byte reservation plus exact quarantine path | Validate all candidates, remove quarantine bytes, then remove the corresponding accounting records. |
| Cleanup result | Rust `Report` and successful filesystem removals | Counts advance after successful removal; partial error remains visible and CLI exits nonzero. |
| Local rustdoc preparation | `BuildSpec`, unique scratch directory, resolved Cargo.lock | Acquire dependencies, freeze build inputs, run dated nightly, retain configuration/output evidence. |
| Producer attribution | Semantic `ProducerRun` plus attempt associations | Bind stable execution dependencies separately from volatile attempt receipts. |

## 3. Semantic contracts and invariants

| Invariant | Inspected enforcement | Remaining issue |
|---|---|---|
| Readers/workers exclude deletion | `maintenance.rs::guards` acquires data, cache, retention and storage locks; `run` retains the aggregate guard | Locks protect participating owners, but cannot discover an older execution root omitted from current configuration: M1. |
| A reset rejects foreign ownership before effects | Normal cleanup calls `state::verify` first | Development reset skips marker verification until after deletions: M2. |
| Failed deletion is not reported reclaimed | `remove_counted` increments bytes/files only after `remove_file`; `run` captures the first error | Implemented. Crash durability and partial-count fault tests remain necessary. |
| Unknown/link candidates fail closed | `candidate` rejects links, special files and multiply linked regular files; reservation inventory rejects linked content | Existing ownership-record recovery skips malformed owner identity; M1. Known atomic temporary files need distinct handling: M3. |
| Preview does not change admissibility | Preview returns before removal or execution recovery | Its lock creation can turn empty roots into unmarked nonempty roots: M5. |
| Requested Rust configuration governs fallback | `BuildSpec::for_environment`, `rustdoc_args`, unique `create_dir`, retained lock and resolved environment | Implemented for the declared supported target; remaining producer receipt/identity omission: M4. |

## 4. Derivation and execution design

Cleanup computes a bounded metadata inventory, takes ownership locks, rechecks candidates and
deletes files with actual progress counts. Startup acquires daemon/cache ownership before
execution recovery and reservation recovery, and opens the interrupted-job journal afterward.
That order prevents the ordinary same-root path from reporting terminal jobs before recovery.

Rust fallback now validates the requested target, forwards requested features/defaults,
creates a unique UUID scratch directory with `create_dir`, fetches dependencies, retains its
resolved lock and uses `cargo rustdoc --frozen`. This corrects the previous deterministic
scratch-directory collision. Only the installed `x86_64-unknown-linux-gnu` target is supported;
other targets produce a stated gap rather than silently building the host target.

The JSON, lock and stable configuration artifacts enter the acquisition and target publication
path. The execution observations returned in `LocalBuild`, however, are discarded by resolve;
the retained configuration also omits the image/helper/containment binding (M4).

## 5. Representative journeys

1. **Execution-root change:** a daemon dies with a container in engine root A. Configuration
   now points at empty B. Recovery scans B only and cleanup removes A's still-mounted capsule
   input or its job/reservation evidence. A dead daemon plus an empty new root is not absence.
2. **Foreign marker:** a selected development root contains a marker naming another role/root.
   Reset inventories and deletes its payloads, then rejects the marker in `mark_reset`.
3. **Reservation publication crash:** the process dies after the atomic temporary reservation
   file is flushed but before it is renamed. Restart rejects that known temporary filename as
   an unknown reservation, and cannot complete its ordinary recovery route.
4. **Same toolchain, changed execution image:** two local builds use identical rustc text,
   lock and output but different image/helper content. Current local producer configuration
   can remain identical, and neither retained provenance nor exported receipt explains the
   actual execution identity and bounded process/cleanup observations.
5. **Fresh preview:** empty development data/cache roots could initialize successfully. A
   reset preview creates lock inodes in them. Ordinary startup subsequently rejects them as
   unmarked nonempty roots.

These are source-derived journeys, not executed reproductions.

## 6. Acceptance gates

| Gate | Verdict | Evidence / required action |
|---|---|---|
| G1 — Authority | fail | M1 permits current configuration to displace prior execution ownership without reconciliation. |
| G2 — Semantic fidelity | fail | M4 drops required execution provenance while retaining local build results. |
| G3 — Validity | fail | M1/M2 allow deletion before complete ownership validation. |
| G4 — Hidden behavior | fail | M1 permits deletion of a configured engine root inside a cleanup candidate; M5 changes startup admissibility during preview. |
| G5 — Consistency and recovery | fail | M1 can outlive deleted inputs/journals; M3 does not recover its own interrupted atomic-write protocol. |
| G6 — Transformation and reuse | fail | M4 omits image/helper/containment from local producer semantic dependencies. |
| G7 — Truthful capability claims | unresolved | No phase-completion claim is made. Source mechanisms do not establish live cleanup/fallback qualification; new oracles must execute on the completed source. |

## 7. Principle findings

| Finding | Principles / severity | Concrete evidence and consequence | Bounded correction | Required oracle |
|---|---|---|---|---|
| **M1 — Cleanup does not establish complete execution-root ownership** | DM-07/29/30/31/45; high | `maintenance.rs::run` at 181–183 calls `Runner::new` with current configuration and then `recover_owned`; `service.rs` uses the same current-root recovery. `state.rs::Identity` at 12–15 has no durable execution-root association. `execution/mod.rs::recover_owned` scans only `self.root/owned` and skips any record whose `owner` is unequal, including a missing/malformed owner. Changing storage root A to B can leave A's live containers behind while cleanup deletes their inputs/journals. Separately, `Runner::new` permits an engine root under a named cleanup payload such as `cache/capsules`; deleting that payload also deletes images or another owner's engine records. | Durably bind each cache to its execution root before execution, and refuse a change until old obligations are resolved, or reconcile every durably associated root. Preflight execution-root overlap with every selected deletion root. Validate ownership record shape/link/name before distinguishing a valid foreign owner from uncertain ownership. Preserve uncertain records and fail before payload deletion. | Focused integration gate: owned execution in A, configure B, request cleanup/reset/startup, assert no selected payload removed until A absence is confirmed. Cover engine-root overlap with cleanup candidates, missing/malformed owner, linked ownership record, and a valid foreign-owner record that remains unchanged. A real-container extension must verify the still-live A case. |
| **M2 — Development reset validates an existing marker after deleting payloads** | DM-07/29/45; high | `maintenance.rs::reset_development` at 99–124 validates path/checkout and root directories but not existing markers. `run` deletes at 184–204, then calls `state::mark_reset` at 206–207. `state.rs::mark_root` at 123–126 only then rejects an existing wrong-role/wrong-root/malformed/incompatible marker. The error is truthful but arrives after the protected payloads are gone. | Preflight every existing marker before the first deletion. Foreign/malformed ownership must refuse with zero deletions. Explicitly supported development cutover state may replace its format marker only after successful deletion under the locks; no legacy reader/migration is required. | Negative cleanup test: wrong root, wrong role, malformed marker, linked marker and unsupported foreign format each preserve payload bytes and report refusal before deletion. Separately exercise permitted unmarked development cutover and interrupted reset completion. |
| **M3 — Reservation recovery rejects its own atomic-write crash residue** | DM-29/30; medium | `Reservation::acquire` uses `atomic::write_atomic`; `atomic.rs:22` creates `.<uuid>.json.<pid>-<counter>.tmp` and flushes before rename. `budget.rs::recover_orphans` at 202–209 accepts only complete `<uuid>.json` filenames, so a crash in that ordinary publication window blocks every startup until manual cache cleanup. The current `mem::forget` test starts after successful record publication and cannot expose this window. | Recognize only the exact known reservation temporary-record protocol under the existing cache/storage locks; validate physical file kind and bounded name/size, then safely discard uncommitted temporary records. Preserve truly unknown records. Keep quarantine-before-accounting deletion order. | Inject interruption after reservation-temp creation, after write/fsync and before rename, then restart. Recovery must reclaim only known uncommitted work and restore capacity. Unknown names and links must still refuse without deleting committed records. |
| **M4 — Local rustdoc drops execution receipts and omits execution identity from its semantic binding** | DM-31/32/46/48; high | `rustdoc.rs::LocalBuild` returns bounded process observations. `resolve.rs` stores JSON, Cargo.lock and configuration at 1253–1291, but the configuration contains only environment/rustc/format; its local `ProducerRun` at 1294–1311 omits actual image/helper/protocol/effective containment. There is no consumer retaining `built.observations`. Equal rustc/lock/output under changed images/helpers can retain the same local producer binding, and offline evidence cannot explain the actual command/status/cleanup observations. | Consume the shared Rust execution description being implemented, binding stable image/helper/protocol/containment into local producer configuration. Retain raw process receipts through attempt-owned output associations, excluding timestamps and unique scratch paths from semantic hashes. Keep the actual resolved lock as an input dependency. | Actual fallback plus offline export must expose the command, image/helper, lock, process outcome and cleanup observation. Change only image/helper/meaningful containment: semantic producer binding must change. Change only timestamps/scratch UUID with equivalent result: semantic evidence remains stable while an additional attempt receipt is retained. |
| **M5 — Preview changes whether empty roots can initialize** | DM-20/28/30; medium | `maintenance.rs::run` always calls `guards` before its `!apply` branch. `state::exclusive` and `leases::initialize` create permanent lock inodes. On empty unmarked development roots, this leaves nonempty roots which `state.rs::initialize` at 70–75 then refuses. Preview is therefore not semantically non-mutating despite removing no payload. | Avoid creating new coordination inodes for this preview case, or explicitly recognize and strictly validate only the known empty coordination files during fresh initialization. Never generally adopt an unmarked nonempty payload root. | Empty physical roots → reset preview → ordinary startup must have the same admissibility as without preview. Repeat with an unrelated file, a linked lock and a nonempty lock; each remains refused. |

**Applicability:** authority, validation, mutable ownership, declared effects, dependencies,
provenance and recovery apply directly (groups 1–3, 6–7, 9–10, 12). Query lowering, numerical
equivalence, alternative backends and broad physical-layout performance are outside this
bounded review. Resource bounds were inspected only on cleanup inventories/reservations;
end-to-end RSS and recovery timing are unmeasured.

## 8. Alternatives and proportionality

| Alternative | Benefit | Risk / cost | Decision |
|---|---|---|---|
| Old Python prune with socket check | Small command | Cannot establish ownership; reports failed removals as reclaimed | Replaced appropriately by Rust cleanup. |
| Current four-lock Rust cleanup | One owner and truthful partial counts | Root discovery/preflight and crash-temporary handling remain incomplete | Retain and correct locally. |
| One durable cache-to-engine-root binding, refusing unresolved moves | Small explicit invariant and bounded recovery | Moving roots requires deliberate resolution of existing obligations | Simplest adequate option if multi-root execution is not a product requirement. |
| Generic cleanup registry/workflow engine | Broad extensibility | Adds machinery without a demonstrated consumer | Unnecessary; ordinary Rust code and typed records suffice. |

## 9. Verification plan

| Claim | Current evidence | Necessary verification |
|---|---|---|
| Locks remain held throughout deletion | Implemented: aggregate `Guards` lives to function return | Cross-process daemon/reader/exporter contention, including a paused deletion. |
| Partial counts are truthful | Implemented: count updates follow successful removals; CLI checks `report.error` | Inject failure after one file and during directory removal/fsync; distinguish logical bytes removed from durability uncertainty. |
| Root and record admission | Implemented with M1/M2/M5 gaps | Negative ownership/overlap/preview oracles above; unknown/link records preserve evidence. |
| Reservation recovery | Implemented with M3 gap | Actual interrupted atomic-write stages and quarantine deletion failure; retry never over-admits storage. |
| Requested Rust fallback and lock | Implemented; unit argument test read only | Real feature-gated fixture under default/no-default/selected feature settings, unavailable target refusal, immutable retained Cargo.lock, overlapping unique scratch owners. |
| Local execution provenance | Incomplete: M4 | Actual producer/receipt/export identity and replay oracles above. |

Known recovery implementation also hashes quarantine contents with `capture(..., u64::MAX)`.
Measure restart work under the configured storage ceiling; if malformed orphan state can
exceed that ceiling, use a bounded metadata validation inventory or explicit aggregate byte/time
limits rather than an unbounded content read. This is an unresolved resource oracle, not a
measured performance finding.

## 10. Exceptions and unresolved decisions

No new architecture exception is needed for these corrections. The one-target support limit
is explicit and honest. Shared execution-description/qualification work and W3 cancellation
propagation remain separate active work; this review does not count them complete. No
historical evidence compatibility or preservation requirement is reintroduced.

## 11. Decision and implementation changes

| Priority | Change | Acceptance evidence |
|---|---|---|
| 1 | M1: bind execution-root ownership, reject overlap/uncertain owner records | Root-switch, overlap and real outstanding-container refusal oracles. |
| 1 | M2: validate reset ownership before deleting | Foreign/malformed marker causes zero deletions; allowed cutover completes. |
| 2 | M4: retain and bind actual local execution provenance | Actual fallback and offline export with stable semantic/dependent identity tests. |
| 2 | M3: recover known atomic reservation temporaries | Interrupted publication windows recover without clearing unknown state. |
| 2 | M5: preserve fresh-root admissibility across preview | Preview/startup regression plus strict unknown/link rejection. |

**Revise** follows from the concrete ownership and provenance failures, not from missing broad
phase certification. Existing locking, unique scratch creation, frozen builds and honest
partial counts are useful implemented mechanisms and should remain the basis for the fixes.
