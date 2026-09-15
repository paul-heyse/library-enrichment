# Phase 4–6 completion: proposed-contract review

## 1. Decision and scope

**Decision: Accept the scoped W1 contract in ADR-0020; revise the remaining phase 4–6
contracts.** W1's F1/F2 design decisions are resolved at Proposed/Interface-checked strength.
F3–F7 remain open and require responding decisions. This is not implementation or phase acceptance.

**Proposal:** The user-approved phase 4–6 completion plan, specifically acquisition provenance,
snapshot compatibility, execution ownership and scratch limits, durable semantic/runtime jobs,
complete provenance export, and Rust-owned pruning.

**Evidence strength:** Proposed; Interface-checked for the existing local types and call paths
read below. No target execution, tests, qualification, or performance measurements were run for
this review. References describe the inspected 2026-09-14 working tree and can move during edits.

**Affected revisions:** Frozen wire envelope 1.0 remains unchanged. The original plan's proposed
snapshot 1.1 is superseded by the implementer's correction: a unified snapshot **3.0**, with
read support for actual historical **1.0 and 2.0**. This correction is grounded in
`crates/enrichment-core/src/producer/python.rs:14` (`TABLE_VERSION = "2.0"`) and
`crates/enrichment-daemon/src/ops/revision.rs:397`, which also uses that version. Existing
`status.rs:55` reports only the wire version and is not a storage-format inventory.

**Observable outcome:** Repeated acquisition preserves semantic identity and correct citations;
live execution cannot exceed admission after cancellation; inspection results remain available
after restart; exported evidence can be followed offline; pruning cannot race an owning daemon.

### Method and coverage

Read the charter, directive, template, and addendum; the frozen blueprint and execution brief;
the relevant design-spine sections; plan 08 and current status; ADRs 0015–0018; register rows
R-17–R-19; and the earlier execution review's F3 residual. Inspected artifact/fragment/snapshot
types, blob acquisition, revision publication, snapshot read/write boundaries, container
cleanup/admission, warm-session ownership, durable jobs, semantic entry points, export, and
the current Python pruning script. Only this review document is written by this review task.

Adversarial reasoning covered identical-content releases, unchanged revalidation, unknown
snapshot versions, exhausted execution slots, task drop, unconfirmed removal, hostile export
paths, and daemon/pruner overlap. Upstream API implementations, full LSP protocol behavior,
client credentials/traces, installer correctness, and the broader phase-gate assertions were
not audited. Historical gate tallies are not evidence for the claims in this review.

### W1 contract re-review — ADR-0020, 2026-09-14

**Scoped decision: Accept W1's ADR-0020 contract at Proposed strength.** This re-review concerns only
[ADR-0020](../../adr/0020-bind-snapshot-provenance.md) and findings F1/F2. It supersedes their
initial document-stage status where explicitly stated; F3–F7 and the overall decision remain
unchanged. Read the current ADR, the source of producer and snapshot keys, acquisition-record
write location, reader version check and fragment decoder, and both historical `capture.json`
records. This establishes Proposed/Interface-checked claims only. The reported fixture runs
were not independently rerun; no implementation acceptance is supplied.

| W1 requirement | Current verdict and evidence | Remaining decision / oracle |
|---|---|---|
| Correct source locator, separate acquisition time, stable producer semantics | **Satisfied at Proposed strength.** ADR-0020 Decision explicitly includes qualified fragments, producer name/version/configuration, semantic inputs, execution profile, outcome and gaps, excluding times/logs/receipts. `Acquisition::bind_fragments` and `ProducerRun::dedupe_key` expose the corresponding mechanisms. Existing context and normalizer/storage inputs preserve the larger identity boundary. | Execute F1 metamorphic cases, including configuration-only changes and two unchanged revalidations. A current implementation path is not itself proof that every producer supplies complete inputs. |
| Retain validated evidence indefinitely | **Satisfied at Proposed strength.** ADR-0020 separates exact validated evidence from mutable registry/document lookup TTLs. Exact `cache_ok`/`offline` never expire because of age; new versions preserve old evidence; only explicit operator evidence cleanup can remove retained facts. | Add a controlled-clock test beyond every configured TTL, with upstream unavailable, and require reuse of the same exact retained snapshot. Verify newer-version discovery and cache/capsule pruning leave earlier evidence unchanged. Retention does not promote corrupt or previously partial/unvalidated evidence to validated evidence. |
| Associate raw receipt with the evidence selected by that acquisition | **Satisfied at Proposed/Interface-checked strength; F1 contract resolved.** ADR-0020 records acquisition ID, receipt artifact ID/digest and context in an acquired-but-unpublished event, then appends selected snapshot or failed/unpublished outcome. Missing completion association means interrupted/incomplete, never success. `ops/revision.rs` exposes `AcquisitionEvent`, `AcquisitionOutcome` and content-addressed event records before and after publication. | Test two different snapshots in one context, unchanged revalidation with separate acquisitions, failed publication, and interruption between publication and association. Preserve raw receipts and earlier snapshots. These behavior tests remain required; this review does not certify their results. |
| Correct actual storage history | **Satisfied at Proposed strength.** ADR-0020 fixes writer 3.0/readers 1.0/2.0/3.0 and wire 1.0, with unknown format rejection and no rewrite. Both historical capture manifests exist with producer command, source digest, snapshot ID and per-file hashes. Legacy missing locator returns artifact handle plus unknown source matching. | Run captured 1.0/2.0 through current reader and preserve their bytes. Verify 3.0 round trip and future-version rejection. No newly relabeled fixture substitutes for genuine historical bytes. |
| Reject malformed supported-format evidence | **Satisfied at Proposed/Interface-checked strength; F2 contract resolved.** ADR-0020 requires version-specific columns/types/nullability, required tags, locator JSON and manifest/table metadata agreement, with only named absent legacy fields defaulting to unknown. `tables::validate_snapshot_table` exposes column, metadata, count and typed-row validation, called before DataFusion registration; malformed fragment tags/locator return errors. | Run genuine historical fixtures and negative cases for missing/wrong/unknown columns, nullability, invalid tags/locator, conflicting IDs/versions/counts and unsupported versions. Corrupt stored evidence must not become a successful empty read or validated cache entry. |

| W1 charter gate | Scoped verdict | Contract evidence / required verification |
|---|---|---|
| G1 — Authority | Pass at Proposed strength | Rust owns fragment, acquisition and snapshot meaning; generated/read projections remain derived. Verify end-to-end citations. |
| G2 — Semantic fidelity | Pass at Proposed strength | Receipt time, semantic evidence and historical unknown provenance are distinct. Verify identical-content releases and repeated acquisitions. |
| G3 — Validity | Pass at Proposed strength | Version-specific mandatory fields, metadata agreement and malformed-tag/locator rejection are explicit. Run genuine and negative fixtures. |
| G4 — Hidden behavior | Pass at Proposed strength | Acquisition is explicit; exact retained reads do not trigger TTL-driven evidence replacement or deletion. Run beyond-TTL offline replay. |
| G5 — Consistency and recovery | Pass at Proposed strength | Immutable snapshots and append-only acquisition outcomes preserve incomplete publication as incomplete. Exercise interrupted association writes. |
| G6 — Transformation and reuse | Pass at Proposed strength | Snapshot keys include declared semantic dependencies while excluding operational time/log/receipt payloads. Run configuration and coverage metamorphic cases. |
| G7 — Truthful capability claims | Pass at Proposed strength | Unknown formats/corruption fail; absent historical provenance stays unknown; retention is limited to the recorded input/environment scope. Verify public error/coverage projection. |

This accepts a decidable design, not executed behavior. The acquisition
association protocol does not claim atomic coupling with snapshot publication: a missing final
association remains visibly incomplete. Broader publication, execution and export recovery
remain outside this W1 acceptance. F3–F7 and their affected whole-plan gates remain unresolved.

## 2. Authority and lifecycle map

| Concept | Identity and owner | Revision / update path | Derived representation |
|---|---|---|---|
| Artifact bytes | SHA-256; Rust blob store | Immutable bytes; never relabel existing bytes as a later acquisition | Service artifact handles; bundle artifact files |
| Acquisition | Source locator, validation observation, time, artifact reference; Rust acquisition path | Append a distinct immutable receipt; associate it with the selected snapshot without rewriting that snapshot | Freshness response and acquisition history |
| Evidence fragment | Fragment identity within an immutable snapshot; Rust normalizer | New snapshot records the acquisition-specific locator and version-match qualification | Arrow row and bounded wire evidence |
| Snapshot | Context plus declared semantic inputs, producer/configuration, normalizer and storage version | Validate, synchronize, publish immutable directory, then current pointer | DataFusion queries and selected-snapshot bundle |
| Execution lease | One admitted execution slot; Rust broker | Transfer continuously from preparation to container/session to cleanup; release after confirmed absence | Queue, active, cleanup and quarantine metrics |
| Job | Versioned concrete operation, pinned inputs and separate caller interests; Rust journal | Persist transitions/results; fail interrupted execution without implicit runtime replay | Pending envelope, job-control response, immutable result handles |
| Runtime observation | Selected object, actual environment, producer and attempt; Rust-owned record | Append executed observation; preserve static/stub evidence independently | Typed runtime aspect and exportable evidence |
| Prune operation | Rust operator command under data/cache exclusion locks | Inventory and delete only recognized regenerable content while holding locks | Preview and truthful removal receipt |

The producer algorithms, filesystem copying, LSP framing and subprocess supervision remain
ordinary code behind these contracts. No generic workflow engine or general graph is justified.

## 3. Semantic contracts and invariants

| Invariant | Representation / enforcement boundary | Failure behavior | Review evidence |
|---|---|---|---|
| Equal blob bytes do not imply equal source acquisition | Optional historical locator; required new-fragment locator, retained receipt | Old evidence lacking trustworthy locator uses an artifact handle and an explicit qualification | Interface-checked: `StoredBlob::acquired` already separates records; fragments do not yet carry the URI |
| Revalidation time does not change semantic snapshot identity | Explicit semantic-input projection at producer publication | New acquisition is retained separately; unchanged snapshot bytes remain unchanged | Proposed; exact input projection needs F1's decision |
| Storage formats are interpreted under their actual contract | Reader dispatch for 1.0/2.0/3.0 before table decoding | Unknown version or malformed required field fails; absent legacy provenance stays unknown | Proposed; F2 |
| Execution capacity covers every live container | Owned lease retained across warm sessions and cleanup | No admission after uncertain ownership; no terminal cleanup claim without absence | Proposed; F3 |
| Package execution cannot grow host scratch without bound | Read-only admitted inputs and bounded tmpfs working output | Resource exhaustion is a retained failure, never empty success | Proposed; F4 |
| Jobs retain one result meaning for all callers | Pinned operation inputs and durable result independent of response budget | Per-interest cancellation; per-caller bounded projection | Proposed; F5 |
| Export is both intact and reference-complete | Typed reference traversal and staged bundle publication | Missing/corrupt references or unsafe entries fail export/verification | Proposed; F6 |
| Pruning cannot remove an active or unknown workspace | Same exclusion locks as daemon, held through inventory and deletion | Busy/unknown/unsafe inputs are refused; partial deletion is reported accurately | Proposed; F7 |

Absence of a locator, unsupported navigation, unresolved environment, incomplete indexing,
empty successful result, import failure, and cleanup uncertainty remain distinguishable.
No hashing claim establishes general semantic equivalence or source truth.

## 4. Derivation and execution design

| Stage | Inputs and result | Effects / ownership | Invalidation or publication rule |
|---|---|---|---|
| Acquire and normalize | Exact source bytes and acquisition locator to normalized evidence | Rust fetch policy; static worker remains no-import | Exclude acquisition clock/receipt encoding; include meaning-changing source/producer/configuration inputs |
| Prepare | Pinned snapshot, selected environment, producer image, policy to admitted immutable inputs | Service-owned disk preparation under an admission reservation | Reuse only after validating retained digests, lock and producer/environment identity |
| Execute or inspect | Typed query/object/snippet to bounded raw observations | One lease, read-only inputs, bounded writable tmpfs, declared profile | Warm reuse keeps lease; active request cannot be idle-evicted |
| Retain result | Validated output plus observed environment and producer to immutable result | Rust journal/store owns publication | Publish terminal reference only after required artifacts are durable; no runtime replay on restart |
| Publish snapshot | Valid tables, references and declared coverage to coherent snapshot | Synchronize staged files and directory, rename, synchronize parent, publish pointer | Same identity cannot silently accept different semantic tables |
| Export | One pinned snapshot and transitive typed evidence dependencies to bundle | Private sibling staging; no daemon needed for immutable reads | Validate closure and checksums before atomic destination publication |
| Prune | Locked cache inventory to removal result | Rust holds daemon-equivalent ownership locks | No socket-probe authority; no traversal of symlink entries |

## 5. Representative journeys

- **Meaningful change:** A README is byte-identical in two releases. Its blob ID is shared;
  each snapshot retains its own exact release locator. Reading the later snapshot never consults
  first-acquisition metadata to invent a later citation. Revalidating either source appends an
  acquisition record without changing the corresponding unchanged snapshot ID.
- **Alternate representation:** A 2.0 Python snapshot remains byte-identical on disk. The 3.0
  reader uses the explicit legacy branch and does not infer missing provenance. A future 4.0
  manifest is rejected before rows can be decoded under current defaults.
- **Ordinary extension:** A new supported semantic method adds a Rust query variant, a concrete
  LSP mapping, capability validation and focused tests. Generated DTOs carry that contract;
  Python does not select providers or construct execution policy.
- **Interruption:** Two warm sessions hold both worker slots. A query for an existing session
  reuses its slot; unrelated queued work evicts an idle session and waits for confirmed removal.
  Dropping the request transfers existing ownership to cleanup rather than reacquiring capacity.
  Exhausted cleanup retries quarantine new admission and retain recovery ownership.
- **Offline use:** Export pins the selected snapshot once. Fragment source artifacts, producer
  inputs/logs/output and required environment/lock provenance accompany it. Verification rejects
  a recomputed checksum manifest that omits a referenced artifact: byte integrity alone is weaker
  than the promised closure.

## 6. Acceptance gates

These are independent charter verdicts on the proposed design, not acceptance-registry states.

| Gate | Verdict | Evidence or gap | Required action |
|---|---|---|---|
| G1 — Authority | Pass at Proposed strength | Rust remains owner; receipts, fragments, execution and views have separate roles | Record responding ADRs; maintain mechanical Python boundary |
| G2 — Semantic fidelity | Unresolved outside accepted W1 | ADR-0020 resolves F1/F2; durable runtime/semantic result scope remains undecided | Set F5 result and observation rules |
| G3 — Validity | Unresolved outside accepted W1 | ADR-0020 resolves format validation; export and pruning need precise rejection boundaries | Set F6/F7 admission rules |
| G4 — Hidden behavior | Unresolved | Runtime is explicit, but writable mount/output handoff details determine actual effects | Set F4 and typed runtime limits in F5 |
| G5 — Consistency and recovery | Unresolved | Continuous lease is the right direction; warm reuse, cleanup exhaustion and durable result ordering need decisions | Set F3/F5 and publication/export commit protocol |
| G6 — Transformation and reuse | Unresolved outside accepted W1 | ADR-0020 resolves receipt/semantic keys; caller-specific result shaping remains undecided | Set F5 shared result and per-caller projection rules |
| G7 — Truthful capability claims | Pass at Proposed strength | Plan requires real clients, exact navigation and explicit gaps; supported implementation routes exist | Retain Proposed labels until the named real oracles execute |

## 7. Principle findings

F1/F2 below preserve the initial review evidence. Their contract decisions are now resolved by
ADR-0020 as recorded in the W1 re-review above; their executable acceptance remains outstanding.
F3–F7 retain their original unresolved status.

| Finding | Principle IDs | Concrete evidence or gap | Consequence | Proposed correction | Verification |
|---|---|---|---|---|---|
| **F1 High — snapshot equivalence needs an explicit projection. Unresolved.** | DM-12, DM-15, DM-31, DM-46 | Plan step 2 excludes receipts but does not enumerate retained meaning. `ops/revision.rs:215` builds a receipt containing HTTP acquisition observations, then `:369` folds all acquisition artifact digests into snapshot inputs. `ops/common.rs:196` resolves a citation from first-acquisition blob metadata. | Revalidation still mints changed IDs, or an overly broad exclusion reuses a snapshot after source locator/version-match or producer meaning changed. Rewriting the old manifest to attach a new receipt would break pinning. | Define a Rust semantic-input projection including source content, selected locator/version-match, environment, producer/configuration and normalizer/storage versions. Append acquisition receipts outside snapshot files with an explicit selected-snapshot link. Preserve earlier snapshot bytes and IDs. | **just gate / metamorphic fixture:** unchanged revalidation yields same ID plus distinct receipt; changed locator/version-match/producer yields changed evidence identity; identical-content releases retain their own citations; old bytes remain unchanged. |
| **F2 High — the planned 1.1 migration omits a real historical format. Violated in original plan; corrected direction remains Proposed.** | DM-07, DM-42, DM-51 | `producer/python.rs:14` already emits table version 2.0; `ops/revision.rs:397` uses it; `status.rs:55` reports wire 1.0 only. `query.rs:109` opens tables without storage-version dispatch. `tables.rs:325` skips unknown fragment kinds/classes and defaults invalid locator JSON to an empty object. | A reader advertised as compatible can silently drop rows or misinterpret genuine 2.0 artifacts; a forged future schema can enter current decoders. | Adopt unified storage 3.0, preserve wire 1.0, accept explicit 1.0/2.0 legacy readers and 3.0 writer. Capture genuine prechange fixtures for both old versions. Validate required fields/types/tags and manifest/table version agreement; only documented absent legacy fields may default to unknown. | **just gate / compatibility and negative fixtures:** open genuine 1.0/2.0 without rewriting, round-trip 3.0, reject future versions, missing required columns, unknown tags, malformed locator and conflicting metadata. |
| **F3 High — continuous ownership needs a warm-session and cleanup-exhaustion protocol. Unresolved.** | DM-29, DM-30, DM-35 | Plan step 3 requires continuous lease ownership. Current `cleanup.rs:263` uses `try_acquire_owned().ok()`; `ops/semantics.rs:113` holds its permit only for one observation while `lsp/mod.rs:125` retains the live session. | Cleanup may run without capacity ownership. Moving permits into sessions without changing lookup ordering can deadlock when warm sessions hold every slot and queries acquire another before reuse. | Move the admitted lease into container/session ownership and transfer that same lease to cleanup on task drop. Lookup reusable sessions before requesting new capacity; evict only idle sessions when queued work needs a slot. Hold ownership through confirmed absence; on exhausted retries retain the slot and quarantine admission until reconciliation. | **just gate / actual-container adversarial tests:** fill all slots with warm sessions, reuse them, queue a different probe, abort an active request, inject removal failure, independently count live containers/heartbeat and prove bounded admission/restart recovery. |
| **F4 High — bounded scratch is not yet a complete filesystem contract. Unresolved.** | DM-07, DM-28, DM-29, DM-45 | Plan step 3 names 512 MiB tmpfs and read-only inputs but does not fix the output handoff. Current `execution/mod.rs:195` mounts all `/capsule` writable; Cargo output and environment/cache paths are under it. `capsule.rs:464` admits from a point-in-time measurement that ignores unreadable content. | A producer may still fill host disk through another writable path, mutate retained preparation, or exceed aggregate storage through concurrent reservations. Copying output after deleting tmpfs loses the only evidence. | Declare read-only admitted input mounts and bounded tmpfs working paths for generated consumer/build/cache/output. Collect only named bounded outputs before removal; reject links/devices/escape paths and digest-validate the handoff. Reserve aggregate preparation bytes atomically and fail closed on measurement errors; release reservations on every exit. | **just gate / actual-container and concurrent admission fixtures:** writes to admitted inputs fail, scratch exhaustion stays bounded and retains failure evidence, output survives cleanup, oversize/link output is rejected, concurrent preparation cannot exceed reservations. |
| **F5 High — durable inspection must separate execution identity from response shape. Unresolved.** | DM-12, DM-13, DM-30, DM-32, DM-44 | Plan step 4 adds concrete job variants and caller interests. Existing `jobs.rs:20` persists only `VerifyRequest` and `:28` stores one result envelope; `ops/semantics.rs:48` performs synchronous semantic work. Exact inspect key/result and runtime object limits remain unspecified. | Shared callers can inherit the first caller's budget or a later current snapshot; restart can expose a terminal result whose artifact was not committed. Runtime serialization can invoke arbitrary descriptors or representations beyond the selected query. | Pin snapshot before queueing; version a concrete operation enum with explicit legacy verification decode. Share durable bounded evidence/result handles keyed on semantic inputs, never caller output budget; project an envelope per interest. Persist artifacts before terminal reference. Runtime selects the resolved object, uses static attribute access where applicable, emits bounded primitive metadata, records import/signature failures, and never silently executes arbitrary user-selected commands. | **just gate / journal and real inspection fixtures:** two budgets share one producer but retain independent response sizes, pointer changes do not alter queued work, terminal replay survives restart, interrupted runtime is not replayed, descriptor/repr side effects are not induced by member serialization. |
| **F6 High — closure-complete export requires typed reference and commit rules. Unresolved.** | DM-14, DM-30, DM-42, DM-46 | Plan step 7 promises transitive closure; current `export.rs:93` copies only the release artifact, silently skips read failure, and `:159` joins unchecked manifest paths. ADR-0018's all-or-nothing statement is stronger than its direct destination writes. | A checksummed bundle can omit cited files/locks/logs or read outside its root during verification. Failed copying leaves an apparent destination that cannot be safely retried. | Traverse typed snapshot fragments, producer inputs/raw outputs/logs, and selected context/environment provenance; distinguish digest entries from actual artifact references. Do not scrape arbitrary JSON strings. Fail on missing/corrupt closure members. Stage beside destination, validate, then publish once. Reject absolute/traversal paths, symlinks, devices, duplicates and unlisted files before content reads. | **just gate / real-store bundle fixture:** offline dereference every required reference; remove one referenced blob while updating checksums and require failure; reject hostile paths/links/duplicates; inject copy failure and retain no published partial bundle. |
| **F7 High — pruning must lock before inventory and recognize ownership. Unresolved.** | DM-02, DM-07, DM-29, DM-35 | Plan step 7 moves pruning into Rust. Current `scripts/capsules-prune.py:64` uses a socket probe, `:60` classifies every other directory as a job, and `:155` ignores deletion errors. Daemon `service.rs:91` and `:104` instead own data/cache locks. | A daemon starts after the socket probe, unknown directories are deleted, or a reported successful removal left storage behind. Symlinked roots can escape intended ownership. | Reuse the daemon's data-then-cache lock order and hold locks from inventory through deletion/receipt. Validate service-root ownership and recognized capsule records/names; refuse unsafe or unknown entries. Use no-follow traversal; report actual completed removals and individual failures, with nonzero exit on incomplete apply. | **just gate / operator concurrency and filesystem fixtures:** live daemon with alternate data root sharing cache blocks apply; startup/prune race has one owner; unrelated/symlink entries survive; failed removal is not counted as reclaimed. |

**Applicability:** Semantic authority/types, identity/equivalence, effects and owned execution,
reuse/concurrency, boundary fidelity, provenance, compatibility and regression controls apply
directly. Each finding carries the applicable verdict. DM-02/DM-03/DM-13 are satisfied at
Proposed strength by the separate ownership map, provided the responding ADRs preserve it;
DM-56–DM-58 favor concrete variants and specialized ordinary code. Numerical precision,
high-volume physical layout optimization and broad template/IR machinery do not bear on this
scope. No exhaustive all-principle score is claimed.

## 8. Alternatives and architectural leverage

| Alternative | Semantic locality and correctness | Cost / performance evidence | Decision |
|---|---|---|---|
| Current implementation | Existing useful Rust authority, but first-acquisition citations, partial cleanup accounting and thin exports have inspected gaps | Lowest change cost; no performance comparison performed | Insufficient for full completion |
| Full generalized receipt/workflow/provenance framework | Could centralize roles, but adds extensibility and migration machinery without a present consumer | Higher maintenance; no measured benefit | Reject as unnecessary scope |
| Small concrete completion | One typed acquisition association, three explicit snapshot reader versions, concrete job enum, owned execution lease, typed artifact-reference collector and locked prune command | Focused changes; tmpfs/cold-prepare cost requires measurement | Preferred, subject to F1–F7 decisions |

A single artifact-reference collector shared by publication checks and export removes duplicate
closure interpretation. It need not be a generic graph engine. A small Rust broker description
shared by setup/qualification/execution is justified by the present duplicated policy choices.

## 9. Verification and measurement plan

| Claim | Evidence label | Oracle and expected result | Remaining gap |
|---|---|---|---|
| Semantic identity and truthful locator | Proposed | F1 revalidation/cross-release/producer-change tests | New mechanism and test runs absent from this review |
| Genuine compatibility | Interface-checked baseline; Proposed new behavior | F2 actual old snapshots plus malformed/future-version rejection | Capture fixtures before writer changes; no invented historical format |
| Bounded lifetime and scratch | Proposed | F3/F4 real-container stress and failure injection with independent absence checks | Interface inspection cannot establish containment |
| Durable semantic/runtime work | Proposed | F5 real engine/object observations, two interests, budgets, restart | Exact implementation and client round trip still required |
| Offline evidence closure and safe reclamation | Proposed | F6/F7 real-store, path hostility, interrupted export and exclusion races | Checksums alone are an insufficient oracle |

Measure cold and warm preparation, LSP start/reuse, peak scratch and retained bytes, queue delay,
cleanup duration and bundle size for the fixed acceptance tasks. The 512 MiB scratch default is
a configured budget, not evidence that all admitted workloads fit. No speedup is claimed.

## 10. Exceptions and unresolved decisions

No MUST-level exception is granted. Unsupported platform, native/source Python dependency and
broader project-lock scope remain the plan's explicit limits. Historical evidence with missing
acquisition provenance remains readable but cannot earn an invented exact locator.

The implementer owns resolution of F1–F7 through the responding scoped ADRs and focused tests.
Before acceptance, those ADRs must make the correction choices above explicit or record an
equally complete alternative. This review does not itself amend the design spine or supersede
ADR-0015/0018. New storage version 3.0 is a justified plan correction, not a wire-version change.

## 11. Decision and implementation changes

**Accept W1's scoped ADR-0020 contract; revise the remaining affected contracts.** Actual storage
history, semantic inputs, indefinite retention and acquisition associations are now decided.
Execution, durable inspection, export and pruning details in F3–F7 still require decisions.
Implementation and executable acceptance remain separate obligations for every workstream.

| Priority | Change | Principles | Acceptance evidence | Regression protection |
|---|---|---|---|---|
| 1 | Fix actual storage-version contract and semantic/acquisition separation | DM-12, DM-15, DM-46, DM-51 | F1/F2 genuine fixtures and invariants | Compatibility, identity and malformed-table tests |
| 1 | Fix execution lease, warm reuse and scratch/output ownership | DM-29, DM-30, DM-35, DM-45 | F3/F4 independent live-container evidence | Explicit ignored containment gate plus saturation/failure cases |
| 1 | Make typed jobs and export preserve complete result meaning | DM-13, DM-30, DM-42, DM-44 | F5/F6 durable replay and closure proof | Cross-budget, restart and hostile bundle tests |
| 1 | Make pruning a Rust ownership operation | DM-02, DM-07, DM-35 | F7 exclusion and truthful-removal tests | CLI race, unknown-entry and failure fixtures |
| 2 | Re-review responding decisions, then audit implementation evidence | DM-53, DM-54, DM-59, DM-60 | Fresh scoped review and full required acceptance | Preserve unresolved claims until actual oracles pass |

No phase is declared complete. The prior F3 implementation residual is not closed by this
proposal review; it needs the revised lifetime implementation and independent real-container
evidence specified above.
