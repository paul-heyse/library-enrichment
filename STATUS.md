# Implementation handoff

Updated **2026-09-16**. **Plan 15 is in progress; it is not qualified or installed.**
[Execution checkpoint](docs/plans/15-unified-datafusion-delta-runtime-hard-pivot.md#implementation-checkpoint--2026-09-16).
The user authorized the complete hard pivot. Continue implementation; do not restore replaced
mechanisms or treat the focused receipts below as completion. WP00–WP12 remain open.

## Implemented boundaries

- Pinned DataFusion/Delta runtime, aggregate Delta planner, Arrow metadata adapter, semantic
  schema registry, shared writer settings and synchronizing local object store.
- Ten evidence Delta tables, exact table/version/cohort/contract publication vectors, typed
  control transactions and fifteen control families. Removed JSON journals/catalog generations,
  evidence manifest files and the custom ExactParquet authority. Private staging is Arrow IPC.
- Native job/current-state views, shared interests, fenced claims and publication, UTC Arrow
  timestamps and policy-derived lease times. Native renewal, expiry guards and owned heartbeat cancellation are implemented; physical recovery/cleanup remains unfinished.
- Native scoring/tokens/factors; materialized Delta API/fragment search surfaces with native
  lineage/CDF updates and atomic input/output checkpoints. Explicit source/definition/history
  rebuilds, CDF endpoint checks, and fresh Delta export cohorts/projections.
- Rust sparse-index decoders emit bounded typed Arrow facts with physical line provenance.
  DataFusion selects exact/newest/neighbour releases. Python version/file selection, wheel tags,
  interpreter constraints and dependency markers now use native plans. SemVer and PEP 440 parsing
  and precedence/specifier operations are narrow typed kernels. Removed procedural selectors,
  wheel-tag loops and the Rust marker evaluator. The native dependency frontier now selects finite acquisition work.
- Core owns the evidence Arrow contracts and 26 native identity contracts, including API/
  execution observations, fragments, relationships, metadata, inputs, coverage and producer
  bindings/plans. Native expressions own ordering/SHA-256/encoding; bounded constructors
  constant-fold the same graph. Policy, command, grant and physical-operation identities are now native; result identities and epoch cleanup remain.
- Native field/tag/vocabulary/reference-shape/coordinate checks run against exact Delta providers;
  identities and cross-relation references are checked separately. Nested Arrow read fields use
  the native Delta nullable layout; required semantic fields retain enforced metadata rules.
  Native row/record-byte bounds now replace file admission and semantic DTO decoding.
  Complete execution/URI/extension/metadata-value and external resource admission remains unfinished.
- `publish_native` is the shared native contribution boundary. Bounded DTOs encode directly to
  Arrow batches; streaming producers supply retained private Arrow inputs. Native union and
  environment derivation write private Delta cohorts directly. Admission checks the resulting
  immutable vector. Removed the intermediate native-plan Arrow writer and Rust coverage folds.
- Rust generates the Python Arrow IPC fact schema; PyArrow 25.0.1 emits bounded batches and
  exact per-file/terminal receipts. The daemon streams stdout to an owned native Arrow source.
  DataFusion validates inventory/ordinals, resolves aliases with recursive closure, joins binding
  membership, expands overloads/bases, constructs evidence/provenance and writes Delta directly.
  Shared native policy owns closure depth; native failure relations distinguish missing targets,
  ambiguity, cycles and exhaustion. Deleted the Python semantic normalizer, WorkerResponse,
  PythonSymbol and the legacy Python branch of the object normalizer. Native physical casts enforce Arrow contract types
  after DataFusion string-view rewrites.
- Rustdoc/public-api emit seven bounded Arrow fact streams from the isolated native worker.
  Native recursive containment/use plans own visibility, alias paths and member reachability;
  native joins construct definitions, API observations, relationships and fragments. Every public-api
  rendering is retained, with symbol/definition association only where native uniqueness permits it.
  Definition ambiguity remains a native diagnostic. Removed the Rust semantic walker, JSON-line
  producer transport, wide symbol/relationship ingress and their reference-resolution staging.
  Four private Delta stages bound reuse of recursive native derivations.
- Build-generated source/compiler/config/dependency receipts replace the partial projection source
  fingerprint. Commands and producer components carry compiled definition revisions; a different
  command definition cannot acquire a new execution claim. Exact input/settings capture and replay
  qualification remain open. Native-worker receipts identify the actual decoder build separately.
- Private input directories survive provider/physical-plan/stream lifetimes. Native coverage
  summaries serve publication and read admission. Debug crash barriers now name Delta cohorts
  and control commits; no obsolete rename/journal barrier remains.

- Document decoders now emit typed Arrow facts. Native plans own source qualification,
  provenance, ambiguity, coverage and identities. Execution observations enter native plans for
  closure, coverage and result-to-attempt checks through `publish_execution`.
- Deleted the file-admission cache, semantic RelationWriter, dataset writer, procedural execution
  normalization and their obsolete configuration. The raw document IPC writer remains a bounded
  mechanical encoder. EvidenceBatch/EvidenceSink and all object publication APIs are deleted;
  fixtures use Arrow plans through publish_native.
- Native writes and query planning hold shared permits in abort-on-drop owned tasks with operation
  context. Current control views use DataFrame windows/joins. Terminal result retention and the
  terminal transition now share the settlement budget; daemon behavior is not yet qualified.
- DeltaScanConfig now owns semantic read schemas; the read-side ArrowContract is deleted.
  Delta builders and scans share one registered durable root ObjectStore. Inspection uses a
  native column projection over a separate documentation column. Leased and unleased inspection
  reads now skip the documentation bytes.

## Verification

Latest architectural replacement (qualification in progress): `native_effect.rs` lowers the four
finite job commands through DataFusion logical/physical extension contracts. The owned driver
registry distinguishes terminal publication from physical exit. `artifact_catalog.rs` and
`result_catalog.rs` store acquisition receipts, result sections and result references in Delta.
Artifact handles carry `receipt` with the exact descriptor; content IDs use all 64 SHA-256 digits.
Blob sidecars, first-acquisition metadata, prefix scans and the procedural blob dependency walk
are deleted. Native recursive plans now select result and export dependencies; artifact section
reads consume native section ranges. Wire 3.0 and research-result/3 schemas and Python models
have regenerated and passed conformance. End-to-end qualification remains open.

Execution readiness now consumes `execution_policy.rs`: captured receipt/configuration/process
facts enter native joins, regex parsing, resource comparisons and precondition relations. Status,
verification, semantic inspection and local rustdoc fallback share that policy. Image qualification
is independent per ecosystem. The daemon's procedural qualification evaluator and resource parser
are deleted. NativeCommand metrics expose driver start/return/drop/refusal and wall time without
claiming physical cleanup; its current statistics API describes the one completion row.

`attempt_plan.rs` now keeps attempts relational through native union, conflict/reference/log checks,
receipt selection and Delta control publication. ControlBatch receives the attempt DataFrame;
Rust attempt merge loops and metadata rebase decisions are deleted. `publication_plan.rs` owns
scope/configuration compatibility and native metadata selection. Control payloads now use the
shared nullable Delta read layout and native semantic field/vocabulary/presence checks. The
control-union schema boundary lowers to the existing native ProjectionExec adapter. These latest
publication changes passed the focused comparison/export journey; broader concurrency, failure
and installed qualification remain open.

`dependency_plan.rs` replaces the resolver's selected-package map, queue and procedural lock
composition with bounded Arrow facts, native demand/extra joins, conflict and resource checks,
and ordered lock output. The daemon parses metadata and performs only the selected acquisitions.
The separate per-dependency version-check API is deleted.

HTTP observations now persist in `delta/http_responses`, with exact body receipts enrolled in
the control catalog. `http_cache.rs` owns native response/Accept selection, negative-cache age,
validator preference and 304 metadata merging. JSON cache metadata and separate cache body
directories are deleted. `network_policy.rs` shares one native policy between URL, redirect,
resolved-address and documentation-target admission. The only new URL UDF parses syntax; native
expressions apply policy. Each HTTP hop uses a fresh client bound to the captured admitted socket
addresses, with automatic redirects and ambient proxies disabled.

`resolution_policy.rs` selects retained context/snapshot bindings, offline refusal and acquisition
through native joins over one captured control snapshot. Python names use registry normalization;
Rust acquisition preserves underscores while native catalog comparison equates underscores/hyphens.

`core/operation.rs` declares all four durable request shapes and the complete typed effective
configuration. Native policy/command/grant identities replace handler-specific JSON/Debug job keys.
Commands retain an exact policy table/version/contract binding. The complete configuration is
stored once in `delta/operation_policies`; it is not repeated in control rows. Native claim
admission reads that exact policy and joins context/snapshot inputs
and the same qualification/profile relation used by status. Claims retain the selected route/image,
command key, policy ID and grant ID; renewal checks the current command and configuration.
Native effect scopes carry privately constructed grants through shared runtime tasks. HTTP checks
live Delta ownership before network hops and rejects a fetcher bound to another configuration;
Rust/Python decoder entry points require a live grant. Control packing now follows declared field
names rather than projection positions. Control publication materializes its at-most-1024-row contribution once into bounded Arrow
values before native admission and append; it no longer repeats the producer DAG per invariant.
`Service::shutdown` is shared by the daemon and direct service callers, waits for physical work
and terminal reconciliation, and retains ownership while retrying transient settlement errors.
Contained runners now select live native grants before reservation/create/start, use exact
Delta-retained operation definitions, and record operation/authority witnesses. Warm LSP reuse
requires the new command's policy and grant; each protocol request checks live ownership.
Operator qualification has a separate fixed native probe contract. Actual contained execution,
full environment/input-scope linkage, replay/fault qualification and retention remain open.

Native diagnostics now use the core `telemetry` Arrow contract and a purpose-specific Delta
`native_events` table under the data root. Bounded, memory-reserved Arrow ingress feeds one writer;
recent query/operation selection, failure correlation and summary totals are native expressions.
The old Rust queues, JSON failure history and cumulative Rust summary counters are removed.
The writer has a single reserved admission lane on the same executor/memory/spill runtime,
without recursive self-observation. Dropped ingress observations are explicit on the status wire.
Service shutdown includes a persistence barrier. Final writer shutdown/fault/retention qualification,
remaining daemon component counters, and installed-client checks remain open.

All receipts below are under `.dev-state/plan15/`, recorded **2026-09-15–16**. They certify their
specific boundary at the source revision when executed, not the final Plan 15 product.

- `native-telemetry-runtime-contracts.log`: all 14 focused runtime contracts passed (0.84 s),
  including native persisted failure history, per-operation totals, small memory budgets,
  owned timeout cleanup and exact failure correlation.
- `native-process-readback-clippy.log`: strict core/store/daemon all-target Clippy passed (3.67 s).
- `native-process-readback-tests.log`: exact Delta operation readback into the process grant,
  route checks and cancellation revocation passed (54.74 s).
- `native-process-route-tests.log`: four native qualification/route/Delta process-grant checks
  passed (55.18 s), including opt-in, exact configuration/image and cancellation revocation.
  This uses declared qualification facts, not actual container qualification.
- `native-qualification-authority-tests.log`: fixed empty-input operator probe admission and
  rejection of changed image/argv, acquisition network and target input passed (0.08 s).
- `native-process-identity-tests.log`: process identity responds to argv/input bytes/output paths,
  resource limits, mode and containment binding (one case passed).
- `native-process-schemas-generate.log`: Rust/Python contracts regenerated; schema conformance
  passed (four positive fixtures, five negative cases).
- `native-process-grants-clippy.log`: strict core/store/daemon all-target Clippy passed (6.44 s)
  before the subsequent retained-operation readback edit.
- `native-control-drain-offline-journey.log`: cold acquisition, shutdown/settlement and exact
  offline restart replay passed (271.14 s).
- `native-control-drain-clippy.log`: strict core/store/daemon all-target Clippy passed (3.74 s)
  after bounded control-input reuse and shared shutdown/drain.
- `native-policy-relation-tests.log`: exact policy capture/reuse, retained version after policy
  change, and wrong table/contract refusal passed (0.26 s).
- `native-policy-relation-clippy.log`: strict core/store/daemon all-target Clippy passed (2.96 s).
- `native-policy-binaries.log`: worker and daemon rebuilt successfully (18.31 s).
- `native-policy-offline-journey.log`: cold acquisition published, but immediate reopen failed
  because its driver still held the writer lock during terminal reconciliation (137.29 s).
  This earlier failure is resolved by the shared shutdown/drain path.
- `native-resolution-tests.log`: native empty-catalog acquisition/offline/profile routing and
  Python/Rust physical-name handling passed (6.27 s).
- `native-operation-identity-tests.log`: complete serialized configuration coverage and policy,
  feature/environment and compiled-operation invalidation passed (one case).
- `native-command-grants-tests.log`: real Delta claim race, renewal, expiry, shared interests and
  settlement passed with the new command/policy/grant contract (92.05 s).
- `native-live-grant-clippy.log`: strict core/store/daemon all-target Clippy passed (3.97 s).
- `native-resolution-offline-journey-failed.log`: cold acquisition returned an error instead of
  a resolution (159.22 s), so offline replay was not reached. The original assertion omitted the
  returned error; `native-resolution-offline-diagnostic.log` confirms the operation budget
  expired during publication control admission (159.35 s). The policy was subsequently moved
  into its own Delta relation to remove repeated wide configuration from control validation.
  Its exact-version binding test passed (0.26 s); the later cold/offline journey passed as recorded above.

- `native-dependency-tests.log`: two frontier checks passed (1.44 s), including cyclic convergence,
  extra-triggered re-expansion, combined constraints, retained-choice conflict and native ordered
  lock output; three registry/marker checks passed (0.89 s).
- `native-http-tests.log`: two passed (17.55 s): native URL/redirect/DNS address admission,
  configured authority, exact Delta cache reopen, Accept separation, native validator/freshness
  and 304 behavior. `native-http-driver-tests.log`: the actual loopback HTTP driver passed its
  redirect/private-address/body-bound checks (1.69 s).
- `native-http-clippy.log`: strict store/daemon all-target Clippy passed (4.59 s), before the
  latest test additions and export-candidate staging change.
- `native-acquisition-clippy.log`: strict core/store/daemon all-target Clippy passed (5.38 s),
  covering the dependency, HTTP, staged-export and driver-settlement changes before the latest
  missing-command settlement assertion.
- `native-export-inline-plan-interrupted.log`: stopped the pre-staging export check after over
  five minutes of repeated native input planning. Export now selects typed record families and
  materializes one native Delta candidate before admission. The replacement passed below.
- `native-execution-policy-tests.log`: three passed (1.64 s): per-image routes, disabled profiles,
  cleanup refusal, eight receipt/configuration/process counterexamples, and the operator's native
  resource parser including unlimited/overflow refusal. These are policy fixtures, not installed
  containment qualification.
- `native-execution-policy-clippy.log`: strict core/store/daemon all-target Clippy passed (13.33 s),
  before the attempt/control replacement. `native-attempt-plan-clippy.log` passed (13.52 s) before
  the latest control read-layout/presence changes.
- `native-receipt-schemas.log`: Wire 3.0 schemas/models regenerated; four valid and five invalid
  conformance cases passed. `native-receipt-python-contract.log`: nine passed (0.03 s).
- `native-receipt-readonly-delivery-tests.log`: read-only retained overflow delivery passed (0.18 s).
- `native-receipt-acquisition-tests.log`: failed (72.46 s). Publication/delivery worked but the
  native Rust worker executable was absent; `native-worker-build.log` subsequently built it.
  The acquisition journey must run again against the latest complete source and worker.
- `native-attempt-export-tests.log`: comparison publication and complete verified export passed
  (174.11 s). Initial nested UNION
  projection failures and nullable-to-required struct casts exposed missing native format/schema
  boundaries. Distinct input references, removal of redundant passthrough aliases, the existing
  ArrowContract projection and shared nullable read fields are covered by this journey. Typed
  family selection and one Delta candidate prevent repeated recursive plan expansion.
- `native-result-flat-relations-tests.log`: passed (7.52 s): captured catalog isolation,
  native artifact lookup/section ranges, dependency integrity, read-only verification without
  scratch creation, and refusal after referenced bytes change. Before the latest wire epoch and
  pre-retention byte-validation edits. A nested UNION projection failed first; flat native
  descriptor columns fixed that query shape without disabling an optimizer.
- `native-command-contract-tests.log`: two passed (0.01 s): EXPLAIN/planning/unpolled streams
  invoke no driver; first poll invokes exactly once; one admission slot permits child queries.
- `native-driver-ownership-tests.log`: passed (18.87 s), before the latest artifact changes:
  terminal publication does not release a live driver or mark its cleanup settled.
- `native-executor-permit-tests.log`: two passed (0.08 s), covering timed-out blocking read and
  Arrow sink ownership until worker exit.
- `native-rust-declaration-scope-tests.log`: passed (1.86 s). Missing external trait-definition
  enrichment lookups do not mean missing local declarations; unresolved inputs retain bounded
  witnesses. Inherited external trait details remain explicitly outside this coverage scope.
  The fixture's rustdoc format 61 versus rustdoc-types format 59 still needs separate qualification.

- `native-projection-publication.log`: passed actual native publication/open, foreign table
  refusal, incremental/full search multiset equivalence, unpublished cohorts, OPTIMIZE, fresh
  export, native checkpoint with expired final CDF log, rebuild and unselected-output isolation.
- `native-jobs-tests.log`: passed native claims, UTC fields, future lease, shared interests and
  stale-owner rejection. Complete cleanup/crash/recovery/renewal remains unqualified.
- `native-value-kernel-tests.log`: three passed; Arrow framing distinctions and SemVer/PEP 440
  ordering checked against pinned library comparisons.
- `native-identity-tests.log`: twelve passed after the identity replacement, including independent
  typed-byte/SHA-256 vectors, feature knowledge, domain separation and derived context behavior.
- `native-registry-tests.log`: three Rust/Python selection and marker fixtures passed.
- `native-registry-build.log`: workspace/all-target check passed after native registry/markers.
- `native-identity-schemas.log`: schema/DTO regeneration and conformance passed for full-hash IDs.
- `native-contribution-clippy.log`: strict core/store/daemon all-target Clippy passed after the
  shared native publication method, presence rules and provider metadata changes.
- `native-evidence-arrow-tests.log`: six typed evidence/identity round trips passed after the
  nested-read-layout change, including actual Parquet/DataFusion reads.
- `native-plan-publication-tests.log`: direct native union/Delta publication, CDF/search, OPTIMIZE,
  export and history rebuild passed (205 s), before the latest field/layout/DTO-ingress changes.
  `native-direct-contribution-tests.log` passed the shared native contribution with the new
  read layout and intrinsic checks (203 s).
- `native-derived-environment-tests.log`: native environment derivation, source preservation and
  repeated contribution identity passed (116 s), before the final DTO ingress consolidation.
- `native-staging-ownership-tests.log`: physical-plan/stream staging ownership passed.
- `native-write-nullability-tests.log`: nullable plan with present value writes; actual NULL fails
  native Delta NOT NULL validation without advancing the table version.
- `native-field-contract-tests.log`: native invalid-coordinate/path/origin refusal passed.
- `native-presence-contract-tests.log`: an actual NULL in a nullable nested Arrow field was
  rejected by its required semantic contract; native provider metadata exposes that distinction.
- `native-identity-admission-tests.log`: native valid/forged binding-provider admission passed.
- `native-python-evidence-tests.log`: actual isolated Griffe worker → generated IPC → native
  recursive/membership/UNNEST/aggregate plans → five Delta evidence tables → independent core
  decoders passed (5 s). Chained aliases, source/stub signatures, conflicting declaration kinds,
  cycles, parse gaps, configured depth exhaustion, exact identities and truncated IPC refusal
  are checked. Large/deep-scale and full daemon/installed qualification remain open.
- `native-rust-evidence-tests.log`: native Rust's four private Delta stages and five evidence
  writes/readbacks passed (2.08 s). `native-document-delta-tests.log`: native document qualification,
  source ambiguity and coverage conflicts passed (1.45 s).
- `native-admission-tests.log`: seven native provider/identity/reference/resource cases passed
  (4.46 s), before the latest captured-projection change. `native-raw-ipc-tests.log`: bounded
  raw-fact streaming passed. No semantic evidence object writer remains.
- `native-terminal-delivery-clippy.log`: strict store/daemon all-target Clippy passed (3.19 s),
  before the native Delta read-schema changes.
- `native-scan-schema-tests.log`: full unsigned domain including `u64::MAX`, read-provider DML
  refusal and exact shared durable backend handle passed (0.08 s).
- `native-column-projection-tests.log`: leased and unleased inspection returned equal rows while
  reading about 21 KB instead of 1.02 MB (15.09 s). Documentation is a top-level Arrow column;
  the pinned Delta scan cannot prune the former nested documentation leaf through schema override.
- The shared native executor owns bounded worker/blocking pools and explicit stacks. Startup,
  request dispatch, native metadata reads and full writes use it. Boxing before tracing/task-local
  composition fixed the startup future's caller-stack overflow.
- `native-executor-acquisition-tests.log`: acquisition reached terminal publication (36.27 s),
  then failed the direct retained-descriptor assertion. The native job transition now carries its
  full Artifact descriptor and job delivery uses the captured record. The next rerun reached
  retained delivery but failed the Rust coverage assertion (external trait lookup classification);
  native Rust declaration-scope qualification subsequently passed. The later receipt-era daemon
  run failed because the native worker binary was missing (see its receipt above).
- `native-executor-clippy.log`: strict core/store/daemon all-target Clippy passed (4.28 s), before
  the latest metadata-read and native result-descriptor changes.
- `native-claim-settlement-tests.log`: live/future lease, renewal refusal and core settlement under
  an expired producer budget passed (83.60 s), before later control and terminal-artifact changes.
- `native-rust-schemas.log`: schema generation and conformance passed with `extern_type` in the
  authoritative symbol-kind vocabulary; pinned rustdoc fact decoding preserves proc-macro kind.
- `worker-python-contract-tests.log`: four actual worker/Arrow contract tests passed, including
  import canary, source/stub overloads, parse/path failures and oversized declarations.
- `native-worker-schema-conformance.log`: Rust/Python regeneration, byte-identical Arrow schema
  and existing JSON schema conformance passed. Worker Ruff and ty passed.
- `native-python-clippy.log`: strict core/store/daemon all-target Clippy passed after this slice.
  The DataFusion skill's capability-gap rules reported no hints for the new worker/planner files.
- ADR/index/register lint and four native scoring cases passed earlier scoped runs.

Last machine-ledger check: `just acceptance-check` **failed** because the report/log source
identity belongs to Plan 14 (`acceptance-check.log`). **Q01–Q13 are not_run as final target gates**;
no Plan 14 pass is transferred. `just doctor` passed its recorded prerequisite check.

## Next unmet work

1. Finish live daemon completion and executor/drop qualification. Complete remaining intrinsic and
   resource admission. Finish integration qualification of native attempt/control publication and artifact
   receipts, result sections and dependency/export closure. Wire 3.0 conformance passed; blob
   sidecars and procedural execution readiness are deleted.
2. Replace the remaining procedural result composition and acquisition/operation policy with native
   plans; finish result retention ownership and replay horizons. The dependency frontier and HTTP
   policy/cache replacements have scoped checks, but full acquisition qualification remains open.
3. Finish finite owned effects, heartbeat/expiry/cleanup qualification, shared external memory accounting, typed replay,
   codec qualification, native retention/maintenance and the required conflict/fault/race oracles.
4. Finish active epoch/schema/tool-alias/rule/fixture cleanup. Reconcile the protected harness patch
   with concurrent operator edits; it has not been applied. This does not block independent work.
5. Qualify all nine MCP tools on fresh installed state, activate once, and remove the enumerated
   retired installation/state/artifact roots. None of those activation/deletion actions occurred.

## Pins and preservation

DataFusion **55.1.0**, Arrow/Parquet **59.3.0**, object_store **0.13.2**, delta-rs
**58f07cd62bfbce3649a7e1c87c696288068ae184**, Buoyant kernel
**8ba063f8f84fec222000f66d40d70911d7c79675**, Rust **1.98.1**, FastMCP **4.0.3**.
PyArrow **25.0.1** encodes the Rust-generated IPC V5 worker contract.
Direct chrono **0.4.45** supplies explicit CDF timestamp bounds; it was already in the lockfile.

Execution baseline: HEAD `9c154de8b2ead1d082fd253399bb1694627395c1`, service digest
`5b2640ca15e4e1e6bb3355eda84edb89d4bcdab3e390897513648c0581d4026a`.
Baseline tree attribution: `.dev-state/plan15/execution-baseline.json`. Preserve pre-existing
review/skill files, the staged deleted August Delta reference, and concurrent Python-toolchain,
AGENTS/settings/hook/skill changes; do not attribute them to this implementation.

The prior installation was recorded at `/home/paul/.local/opt/library-enrichment/plan14-5b2640ca15e4`
and `/home/paul/.local/state/library-enrichment-v2`. Its current process state has not been refreshed
by this checkpoint. Source implementation work has not activated or removed that installation.
