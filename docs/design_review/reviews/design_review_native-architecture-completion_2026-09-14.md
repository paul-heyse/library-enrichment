# Native architecture contracts and removal review — 2026-09-14

## 1. Decision and scope

**Proposal:** ADR-0028–ADR-0033: durable comparison and qualified reuse, explicit inspection
projections, precommitted delivery, native parser allocation and scalable producer limits.
**Reviewer:** Codex. **Decision:** accept the specified architecture contracts.

This is a scoped decision review, not acceptance of all Plan 12 functionality. The supported
target is Linux static extraction and the existing qualified execution interfaces, with one
Rust evidence model and native store. Client journeys and final performance are outside this
decision review and remain required in Plan 12. No measured speedup is claimed.

### Method and coverage

Read the six proposed ADRs, charter, directive, addendum, generated worker boundary, native
ingestion/query/admission, job delivery and concrete comparison/inspection selection consumers.
Inspected assertions and current logs: `.dev-state/plan12-architecture-tests.log`,
`plan12-provenance-selection.log`, `plan12-native-parser-tests.log`, `plan12-python-worker.log`,
`plan12-export-e2e.log` and `plan12-publication-export.log`. Paths are relative to `.dev-state`.
The first e2e pass exposed stale bundle assertions and an incorrectly selected context mode;
corrected focused reruns passed. Native corruption, same-context enrichment, terminal I/O
failure and independent exports were attacked. Actual ty/rust-analyzer/client acceptance and
all Inspect/Verify interruption combinations were not established by this review.

## 2. Authority and lifecycle map

| Concept | Authority and identity | Lifecycle / derived representations |
|---|---|---|
| Definition, public binding, observation | Rust canonical domain records and semantic IDs | Ten typed Parquet relations; DataFusion views are derived |
| Raw language input | Verified immutable artifact and producing attempt | Bounded native/Python worker; temporary producer records have no publication authority |
| Reference resolution | Core ID kernels plus native staging joins | Unpublished exact typed files; no second persistent index |
| Successful publishing job | Admitted snapshot/catalog job-publication row | Prepared immutable delivery precedes visibility; journal is recoverable delivery state |
| Comparison | Closed two-version request and terminal journal | Derived native read, without invented producer evidence |
| Execution | Rust policy, image/helper/controller identity and qualified environment | Concrete jobs/capsules; observations remain context qualified |

Domain traversal, alias interpretation, canonical hashing and scoring remain ordinary Rust.
They do not independently perform corpus-wide query selection. Worker JSON is a finite transfer
boundary, not a queried table payload or a persistent alternative engine.

## 3. Semantic contracts and invariants

| Contract | Enforcement and failure | Evidence |
|---|---|---|
| One current evidence interpretation | Native schema/admission and removed legacy modules; unknown formats reject | `state_identity_refuses_legacy_roots_links_and_overlap`; `just architecture-check` |
| Same facts preserve semantic identity | Native distinct/global sort and incremental canonical array digest | `reacquisition_preserves_snapshot_bytes_and_adds_attempt_attribution`, now with reordered duplicate input |
| Projections declare omitted docs | `ApiObservationProjection.docs_included`, native leaf selection | `production_inspection_view_skips_large_documentation_leaves` |
| Required delivery precedes selection | `JobCompletion.prepare_delivery` in publication coordinator | `delivery_failure_prevents_job_catalog_commit`; `exported_job_delivery_has_complete_artifact_closure` |
| Large parser allocations are external | Native worker limit installation before parsing; Python request-bound limits before source extraction | Actual hostile native parser tests; separate Python worker test |
| Malformed/oversized declarations remain explicit | Locator constructor returns Result; signature failures propagate; Python oversized declarations become gaps | Native malformed input test; `test_long_documentation_is_retained_and_oversized_declarations_are_explicit` |

Unknown, absent, omitted, unsupported and failed remain separate. Exact source/stub alternatives
are retained. Reordering or changed acquisition clocks must not change semantic identity; changed
qualified facts or extractor semantics do. Byte immutability is stronger than semantic equality
and is checked separately for pinned snapshots.

## 4. Derivation and execution design

Extraction visits raw language structures behind the bounded worker. Rust record kernels feed
typed staging; DataFusion resolves bindings/endpoints/fragments and provenance. Distinct canonical
relations undergo admission, native semantic-key ordering and streaming digest before publication.
The async query feeds a one-batch channel; a blocking writer retains inputs and the store lease
until it finishes or rejects cancellation. There is no successful finalization on interrupted input.

Queries use native eligibility/order/limit before DTO hydration. Acquisition descriptors and
attempt logs now select and deduplicate in native plans too. Direct derived-context reuse has
explicit source, environment, image/controller, producer and query predicates; ambiguous matches
require a selected context. Its complete real-producer qualification remains Plan 12 FN1.

The common delivery factory writes a bounded answer before catalog selection. Recovery reads
committed bytes. Comparison commits its own terminal read result and handles failed encoding/I/O
explicitly, retaining an unpersisted terminal failure in bounded memory until restart recovery.

## 5. Representative journeys

- A new extracted symbol uses the existing core ID kernel and typed staging, followed by native
  reference closure and admission; it does not introduce another normalization engine.
- An exact release is acquired, researched, then read offline after restart. A later registry
  selection adds real provenance in a distinct snapshot for the same context without extracting
  unchanged documentation. The earlier snapshot's bytes and mtimes remain unchanged.
- A signature-only request preserves observation/source identity while excluding docs before
  hydration. A large result is read through a complete immutable artifact.
- A malformed raw crate terminates only the worker; a subsequent valid crate succeeds. A failed
  delivery write prevents catalog publication. A corrupt export is independently rejected.

## 6. Acceptance gates

These verdicts address the specified contracts and cited representative mechanisms, not blanket
certification of all existing code or product use cases.

| Gate | Verdict | Evidence / scope rationale | Required action |
|---|---|---|---|
| G1 — Authority | pass | Core identities, native evidence/catalog; no worker publication or old reader | Keep removal gate in CI |
| G2 — Semantic fidelity | pass | Explicit projection scope and gaps; qualified nested alternatives; native joins | Final library output review remains Phase 5 |
| G3 — Validity | pass | Admission, parser bounds, typed errors and precommit delivery rejection | Preserve targeted negative tests |
| G4 — Hidden behavior | pass | Retained inspection cannot schedule execution; explicit intent/policy; bounded workers have no library execution authority | Real execution boundary checks remain FN2 |
| G5 — Consistency and recovery | pass | Precommit delivery, native catalog selection, retained leases, explicit failed journals | Complete remaining real producer interruptions in FN1/FN2 |
| G6 — Transformation and reuse | pass | Canonical identity, global semantic order, exact qualified selection and ambiguity rule are specified | Complete producer dependency/real replay audit in FN1 |
| G7 — Truthful capability claims | pass | Explicit unsupported/partial outcomes; resource capacity is not a measured speedup claim | Actual client and final latency evidence remain required |

## 7. Principle findings

Applicability: authority/types/identity, transformations, effects/ownership, dependencies,
interchange, provenance and regression controls apply. Generic compiler/plugin machinery and
numeric approximation do not apply to these contracts. The design-stage hard cutover explicitly
requires no historical migration; future retained evidence still has its own retention contract.

| Finding | Principles | Concrete evidence | Consequence | Correction | Verification |
|---|---|---|---|---|---|
| Corrected: terminal encoding failure could leave comparison running | DM-30, DM-35 | `jobs.rs::finish`, `compare_job.rs::run` | A completed task has no usable terminal answer | Shared failure settlement, visible-terminal reconciliation and bounded nondurable state | `failed_terminal_delivery_leaves_a_failed_job_and_restart_never_reexecutes` |
| Corrected: raw parsing preceded allocation bounds | DM-07, DM-45 | `producer/normalize.rs::deserialize_crate`; pinned public-api whole parse | Allocator/stack failure can terminate daemon | Mandatory native worker, no in-process fallback | Native hostile-footer and malformed Rustdoc tests |
| Corrected: docstrings silently lost their suffix | DM-08, DM-42 | Python `_observation` formerly sliced at 32000 characters | Agents could miss guidance while receiving no omission signal | Full bounded docstrings; oversized file gap with retained source | Long-documentation worker regression |
| Corrected: recovery ignored unknown execution children | DM-30, DM-45 | `execution/mod.rs::recover_root` formerly skipped non-JSON entries | Unresolved ownership could disappear from startup accounting | Refuse unknown children for operator inspection | `unknown_execution_ownership_children_prevent_recovery` |
| Remaining qualification, not a new design decision | DM-53, DM-59 | Plan 12 FN1–FN4 and Phase 5 | Architecture tests alone cannot establish complete producer/client usefulness | Run the specified actual journeys and report their scope | Plan 12 execution ledger and final source-bound gates |

## 8. Alternatives and architectural leverage

| Alternative | Authority / failure surface | Cost and performance evidence | Decision |
|---|---|---|---|
| In-daemon whole-input collectors plus post-allocation limits | Duplicate relational operations; parser can fail before bounds | Fewer process launches, no safe allocation argument | Reject |
| Typed staging/native joins, bounded workers, one publication protocol | One evidence/query authority and explicit transfer lifetime | Added staging/process I/O; final latency unmeasured | Select |
| Simpler direct upstream streaming plus inline-only answers | Viable only if upstream exposes enforceable streaming and answers fit | Current public-api materializes its input; inline-only rejects required useful results | Reject for present requirements; recorded upstream revisit trigger |

No Delta Lake, custom optimizer, generic workflow framework or second storage abstraction is
needed for this scope. Existing closed jobs and the native runtime provide the necessary seams.

## 9. Verification and measurement

Native model/fixture tests passed in `plan12-architecture-tests.log`; this excludes the eleven
ignored real-container cases. Subsequent provenance-selection, worker, staging recovery and
export logs identify their own narrower runs. C07's focused rerun passed after fixing explicit
context mode. Initial failed logs remain diagnostic evidence, not successful receipts.
Rust all-target Clippy and Python ty passed for the boundary changes. Final suite and real
producer/client/performance qualification remain Plan 12 work. No acceptance percentage is used.

## 10. Exceptions and unresolved decisions

No new SHOULD deviation is required. Parser process limits reject some large otherwise valid
inputs; this is an explicit supported resource bound. It is not an aggregate RSS guarantee.
Paul Heyse owns revisit when upstream supplies enforceable streaming/preallocation, when a
concrete result exceeds the supported bound, or when measured workload evidence warrants a
different resource contract. Register rows capture those triggers.

## 11. Decision and implementation changes

| Decision | Priority | Reason | Regression protection |
|---|---|---|---|
| Accept ADR-0028–ADR-0033 as architecture contracts | correctness | The decisions identify one authority, valid boundaries and explicit failure/reuse semantics | Native admission, publication, worker and job tests |
| Finish deletion and owned development cutover before functional expansion | next | Old development evidence is outside the target contract | Scoped inventories/locks, `architecture-check`, Rust rules |
| Keep actual functional and performance acceptance open | after cutover | Representative architecture evidence is narrower than complete Plan 12 scope | FN1–FN4 and final source-bound acceptance |
