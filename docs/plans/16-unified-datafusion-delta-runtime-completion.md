---
title: Complete the remaining unified DataFusion and Delta runtime pivot
status: draft
date: 2026-09-16
adrs: [ADR-0041, ADR-0042, ADR-0043, ADR-0044, ADR-0045]
phase: 6
---

# Complete the remaining unified DataFusion and Delta runtime pivot

## 1. Purpose and authority

This is the remaining-scope execution plan requested on **2026-09-16**, after reviewing
[Plan 15](15-unified-datafusion-delta-runtime-hard-pivot.md), its implementation checkpoint,
the current source, and available execution receipts. It replaces Plan 15's chronological
checkpoint as the sequence for remaining work. It does **not** narrow Plan 15's destination,
terminal definition, deletion obligations, or the aggregate requirements of the
[unified runtime review](../design_review/reviews/design_review_unified-datafusion-delta-runtime_2026-09-15.md)
and [Delta integration follow-up](../design_review/reviews/design_review_delta-integration-followup_2026-09-15.md).

**This document is a plan, not an implementation-completion or acceptance report.** The planning
pass made no runtime changes and ran no new product qualification. The previously running focused
diagnostic-close check completed; its receipt is recorded below. All original WP00–WP12 remain
open at their complete scope, and Q01–Q13 remain `not_run` as final target gates.

### 1.1 Non-negotiable destination

- DataFusion owns every service data transformation, selection, eligibility decision, result
  composition, and command plan. Delta owns every durable relational authority. Exact external
  bytes belong in the narrow immutable blob store, with native receipt/reference/retention facts.
- One native definition supplies fields, expressions, rules, identities, operation bindings,
  provider metadata, diagnostics, and generated transport contracts. Metadata reports enforcement;
  metadata alone never grants permission or establishes validity.
- The daemon retains network, filesystem, subprocess, protocol, and resource mechanisms behind
  typed native boundaries. Python retains thin FastMCP transport and mechanical worker extraction/
  generated Arrow encoding. Neither becomes a second semantic engine.
- Complete the hard pivot in one active epoch. Delete replaced code, old schemas, tests of retired
  behavior, executable aliases, state readers, sidecars, launch paths, and unused dependencies as
  replacements land. No migration, legacy fallback, compatibility DTO, dual writer, or archived
  executable baseline is delivered.
- Preserve useful bounded Rust/Python code insights through all nine MCP tools, evidence resources,
  offline reuse, actual qualified execution, durable jobs, cancellation, retained results, and export.
  Existing policies can change through the recorded decision process when they obstruct that goal.
- Keep frozen specification provenance and the requested review inputs. New-epoch Delta history,
  exact source artifacts and replay witnesses are target functionality, not legacy retention.

### 1.2 Evidence and working-tree boundary

Reviewed HEAD: `9c154de8b2ead1d082fd253399bb1694627395c1`. The tree is dirty: the planning inspection
reported 261 porcelain entries, including 76 untracked entries; directory entries are not file
counts. HEAD alone does not identify this implementation. The original execution baseline is
`.dev-state/plan15/execution-baseline.json`; the original source digest was
`5b2640ca15e4e1e6bb3355eda84edb89d4bcdab3e390897513648c0581d4026a`.

Preserve existing implementation and concurrent operator work. In particular, protected harness
changes, Python-toolchain policy/ADR-0046 work, new skills/reference material and review evidence
must not be reset or attributed wholesale to this plan. Before resuming implementation, record a
fresh path/content baseline and current source receipt. Do not recreate the start of Plan 15.

Source observations below mean **source-integrated**, not final-source or installed qualification.
Existing logs under `.dev-state/plan15/` are development receipts and can be removed by explicit
development cleanup; copy the necessary receipt/provenance into the final qualification artifact
when the corresponding oracle is rerun. No previous-plan acceptance pass transfers to this epoch.

## 2. Current implementation and the remaining boundary

Paths in this document use `core/`, `store/`, and `daemon/` for
`crates/enrichment-{core,store,daemon}/src/`. Links resolve to the actual files inspected.

| Area | Reuse already present | Remaining obligation |
|---|---|---|
| Native runtime | Shared bounded native executor, configured session/RuntimeEnv, Delta extension planner under retention, owned reads/writes/blocking work, operation context and spill | Complete external allocation/lifetime accounting, every extension contract, selected mutation routes and final failure qualification |
| Schema and identity | Core Arrow evidence/worker contracts; semantic-contract Delta registry; explicit unsigned mapping; 26 shared native identity contracts; native tag/value/key/reference admission | Complete intrinsic/recursive metadata validation; eliminate remaining semantic JSON hashes; result/cursor/replay/retention identities; ordinary-extension proof |
| Delta persistence/catalogs | Ten purpose-specific evidence tables, exact table/version/cohort/contract publication vectors, current native Delta scans, shared synchronizing local ObjectStore, qualified append | Controlled discovery/factory integration, full mutation/feature matrix, retention and power-loss qualification |
| Control/publication | Fifteen tagged families, native current-state windows/joins, fenced claims/interests/renewal, one publication/head/terminal commit, artifact receipts and retained-result metadata | Retention/cleanup/maintenance families, remaining procedural job decisions, physical ownership authority, independent-process recovery/race matrix |
| Acquisition | Native Rust/Python release selection, markers/wheel eligibility, dependency frontier, resolution routing, network policy, Delta HTTP cache | Durable raw registry/source fact authority, complete source/environment/producer witnesses, residual acquisition/revision policies and bounded resource proof |
| Normalization | Both old semantic walkers removed; bounded Rust fact streams and generated Python Arrow IPC; native closure/ambiguity/coverage/admission; document and execution plans; native attempt and metadata contribution plans | Rustdoc compatibility defect, remaining execution/LSP/runtime observation lowering, full semantic field inventory and independent deep/large oracles |
| Process authority | Exact retained policy/operation/effect definitions, private live grants, native image/configuration/containment admission, operator-only fixed qualification, warm-LSP command rebinding | Native proof of exact authorized inputs/environment/producer/action, durable physical cleanup/resources, real contained execution and revocation/restart qualification |
| Research/results | Native score/factor/explanation plans, native browse/comparison/coverage components; Delta receipt/result lookup and dependency closure; wire 3.0 | Handler selection, result sections, page/cursor decisions, result identity, overflow and recovery still have procedural owners |
| CDF | Two materialized search surfaces with atomic input/output checkpoints and explicit rebuild modes; publication changes consumed by the manifest resource | Full mutation/revision/fault equivalence, native rebuild selection, fresh-process typed replay and qualified logical read-provider cache |
| Diagnostics | Core Arrow events, bounded reserved ingress, Delta `native_events`, native windows/aggregates/failure history, dropped-event reporting, explicit writer close | Delta/kernel events, daemon/component counters, complete correlation, writer fault/retention qualification and resource accounting |
| Deployment/removal | Many old writers, journals, object normalizers and sidecars deleted; generated current schemas | Remaining live legacy behavior, active rules/fixtures/configs, final qualification, fresh installation and enumerated old-epoch deletion |

### 2.1 Corrections to the chronological checkpoint

These are important when deciding what to implement next:

1. [Control records](../../crates/enrichment-store/src/control.rs) now have **15** families,
   including `artifact_receipts` and `retained_results`; the older checkpoint's 13-family count is
   stale. Retention and maintenance records are still absent from `Table::ALL`.
2. [Attempt plans](../../crates/enrichment-store/src/attempt_plan.rs) and
   [publication plans](../../crates/enrichment-store/src/publication_plan.rs) already replaced the
   earlier object attempt/metadata merge. Finish residual consumers; do not rebuild that replacement.
3. [Dependency expansion](../../crates/enrichment-store/src/dependency_plan.rs),
   [HTTP cache](../../crates/enrichment-store/src/http_cache.rs), and
   [resolution policy](../../crates/enrichment-store/src/resolution_policy.rs) are native now.
   Their existence does not make every acquisition/source policy native or durably queryable.
4. The second CDF consumer exists:
   [manifest](../../crates/enrichment-daemon/src/ops/manifest.rs) calls
   `Repository::publication_changes`, which executes
   [EvidenceTables::change_plan](../../crates/enrichment-store/src/delta_evidence.rs).
   Its remaining work is fidelity, policy, fault, retention, and public-path qualification.
5. [Process grants](../../crates/enrichment-store/src/process_grants.rs) retain and read back the
   selected operation before dispatch. Grant existence and exact operation hashing are not yet
   proof that every argv/input/output action was selected by the command's semantic contract.
6. [Telemetry history](../../crates/enrichment-store/src/telemetry_history.rs) already implements
   `Close`, including concurrent-close handling, and service shutdown invokes it. Do not list basic
   writer termination as wholly unimplemented; crash/fault/lifetime/retention qualification remains.
7. [Result catalog](../../crates/enrichment-store/src/result_catalog.rs) makes lookup and dependency
   closure native, but [result encoding](../../crates/enrichment-store/src/result.rs) still assigns
   section aliases from JSON field names. [Delivery](../../crates/enrichment-daemon/src/delivery.rs)
   still clears DTO fields and reconstructs recovery fields. This is substantial replacement work.
8. The native score UDF replacement is present. Remaining handler token/kind/cursor and page logic
   must consume that authority; reimplementing scoring is not the next task.

### 2.2 Existing receipts and their limits

Inspected **2026-09-16**; these are existing executions, not new tests run for this planning pass.

| Receipt under `.dev-state/plan15/` | Recorded outcome | What remains outside the receipt |
|---|---|---|
| `native-control-drain-offline-journey.log` | 1 passed, 271.14 s | Predates the latest process/telemetry changes; not installed qualification |
| `native-process-route-tests.log` | 4 passed, 55.18 s | Native synthetic route/grant fixtures; not actual containment |
| `native-process-readback-tests.log` | 1 passed, 54.74 s | Exact retained operation/readback/revocation; not complete command-to-input authorization |
| `native-qualification-authority-tests.log` | 1 passed, 0.08 s | Fixed qualification scope only; not real operator/container qualification |
| `native-telemetry-runtime-contracts.log` | 14 passed, 0.84 s | Before the final close change; not writer crash/power-loss/retention qualification |
| `native-telemetry-close-tests.log` | 1 passed, 0.80 s | Concurrent close/refusal/reopen behavior; does not certify every lifecycle boundary |
| `native-telemetry-schemas-generate.log` | 4 valid / 5 invalid conformance cases passed | Current generated schemas; full nine-tool installed fidelity still open |
| `native-telemetry-clippy.log` | Strict affected-crate all-target check passed, 6.37 s | Before the final concurrent-close tweak; no final-source broad quality claim |
| `native-projection-publication.log` | 1 passed, 106.16 s | Focused publication/projection behavior; not the full CDF mutation/fault matrix |
| `native-column-projection-tests.log` | 1 passed, 15.09 s | Native projection/read-I/O boundary only |
| `rustdoc-compat-probe/stability.log` | Probe executed and demonstrated format-61 stable metadata rejected by the 0.59 parser | A confirmed defect to repair in RP04, not a successful producer acceptance result |

The daemon, native worker and executor must be rebuilt together before the next integrated live
journey. A stale built worker cannot certify current decoder/process contracts. Existing installed
Plan 14 state has not been replaced by this source. No deployment action belongs to this planning pass.

## 3. Pinned capability basis

Use the [DataFusion skill](../../.claude/skills/datafusion/SKILL.md) and
[Delta Lake skill](../../.claude/skills/deltalake/SKILL.md), their pinned indexes/prose, and exact
checked-out source. **Do not use Context7 for these library references.** This plan keeps:

| Component | Exact basis |
|---|---|
| DataFusion | 55.1.0 and matching split crates |
| Arrow / Parquet | 59.3.0 |
| object_store | 0.13.2 |
| delta-rs | `58f07cd62bfbce3649a7e1c87c696288068ae184` |
| Buoyant kernel | `8ba063f8f84fec222000f66d40d70911d7c79675` |
| Rust / FastMCP | 1.98.1 / 4.0.3 |
| Python Arrow encoder | PyArrow 25.0.1; generated IPC contract conformance remains required |

Do not follow moving upstream HEAD as part of completion. A necessary upstream improvement needs
an exact replacement pin, evidence, and affected-route qualification. Reuse existing primary-source
receipts; research only unresolved consequential seams.

### 3.1 Capability decisions governing the remaining work

| Native contract and pinned reference | Completion use / boundary |
|---|---|
| `datafusion_session::{catalog::CatalogProvider, schema::SchemaProvider, table::TableProvider}`; [catalog skill topic](../../.claude/skills/datafusion/content/topics/catalogs.md), [provider map](../design_review/capability-maps/datafusion_provider_contracts.md) | Preserve complete immutable inventories and native forwarding. Async preparation happens before cheap synchronous lookup; no second policy registry or live directory lookup as publication authority |
| `deltalake_core::table::DeltaTable`; [operations](../../.claude/skills/deltalake/content/catalogs/operations.md) | Use complete logical-input builders and their validators/metrics. `DeltaOps` is removed. Raw provider INSERT remains excluded because the pinned route bypasses table CHECK validation |
| `deltalake_core::delta_datafusion::{DeltaTableFactory, DeltaLogicalCodec}`; [API](../../.claude/skills/deltalake/content/api/deltalake_core.delta_datafusion.md) | Factory opening is controlled preparation, followed by exact capture. Codec use is qualified immutable read-provider caching only; typed command/CDF descriptors own replay |
| `deltalake_core::data_catalog::storage::ListingSchemaProvider`; [API](../../.claude/skills/deltalake/content/api/deltalake_core.data_catalog.storage.md) | Current source collects the complete listing, normalizes names and adds entries; table lookup opens current state. Bound the discovery source, reject collisions/errors and capture versions before publishing an immutable inventory |
| `deltalake_core::operations::vacuum::VacuumBuilder`; [API](../../.claude/skills/deltalake/content/api/deltalake_core.operations.vacuum.md) | Pass native protected versions through `with_keep_versions`; configure bounded scan concurrency and retention. File protection alone does not prove log/CDF reconstruction or reader-enrollment safety |
| `buoyant_kernel::struct_patch::ProjectionStructPatchBuilder`; [API](../../.claude/skills/deltalake/content/api/buoyant_kernel.struct_patch.md) | Pair actual kernel schema/expression edits; `build` returns both. Ordinary application rules stay DataFusion expressions; private SchemaComparison is not copied |
| `buoyant_kernel::metrics::reporter::{ReportGeneratorLayer, MetricsReporter}`; [API](../../.claude/skills/deltalake/content/api/buoyant_kernel.metrics.reporter.md) | Cheap any-thread event capture into the existing bounded Arrow ingress; native aggregation/correlation. No display-string parser or second telemetry authority |
| Native CDF and maintenance; [CDF](../../.claude/skills/deltalake/content/topics/change-data-feed.md), [maintenance](../../.claude/skills/deltalake/content/topics/maintenance.md) | Inclusive version windows, explicit pre/postimage semantics, atomic output/offset selection, full rebuild and native maintenance over one retention definition |

Exact-source qualifications already observed remain load-bearing: codec logical-extension hooks
contain TODOs, provider decode ignores the supplied expected schema, and `DeltaPhysicalCodec` is
deprecated for a retired physical wrapper. Reject unsupported nodes before invoking those hooks;
rebind all live runtime/store/lease handles. Do not interpret successful deserialization as admission.

`ListingSchemaProvider`'s unrestricted `try_collect` and collision handling cannot be adopted as
the service's bounded discovery policy. Use a controlled finite inventory and the narrowest upstream
composition improvement if needed; do not reproduce a second catalog scanner to claim native reuse.
The factory must also preserve the selected backend/session rather than accidentally reopening
through another local store. These are RP02 implementation seams, not reasons to skip the integrations.

## 4. Dependency order and work packages

Each package below has implementation, deletion and a decisive verification boundary. All are
open. Keep working through architecture after the focused boundary check passes. Broad regression,
installed clients and deployment belong at RP13 once the integrated replacement exists.

| Package | Prerequisites | Deliverable | Plan 15 coverage |
|---|---|---|---|
| RP00 | — | Current operation/authority inventory, reconciled rules and executable scope ledger | WP00 |
| RP01 | RP00 | Complete native schemas, intrinsics and identity authority | WP02 |
| RP02 | RP01 | Complete provider/discovery/mutation routes | WP01, WP03 |
| RP03 | RP01, RP02 | Native control decisions, durable physical ownership and retention enrollment | WP04, WP07 |
| RP04 | RP01, RP03 for owned effects | Complete acquisition/producer facts and native normalization | WP05, WP06 |
| RP05 | RP03, RP04 | Fully bound effect plans and qualified execution semantics | WP07 |
| RP06 | RP01, RP04; RP05 for execution aspects | Complete native research selection and comparison | WP08 |
| RP07 | RP01; integrate RP03, RP06 | Native results/pages/encoding and generated nine-tool bindings | WP08 |
| RP08 | RP02, RP03, RP06, RP07 | Complete CDF/rebuild and fresh-process replay | WP09 |
| RP09 | RP03, RP07, RP08 | Unified retention, native maintenance and durable reclamation | WP10 |
| RP10 | RP03; integrate RP05, RP07, RP09 | Complete structured telemetry and resource/lifetime accounting | WP01, WP07, WP10 |
| RP11 | RP02, RP04–RP10 | Measured native physical choices on complete journeys | WP10 |
| RP12 | RP00–RP11 | Source deletion, enforcement and design closure | WP11 |
| RP13 | RP12; installation deletion after candidate qualification | Final gates, fresh activation and retired runtime removal | WP12 |

This is a dependency graph, not a demand for long sequential scaffolding. Define RP01's result and
retention contracts early so RP07 and RP03 can use them immediately. Integrate a concrete Rust/Python
journey as its dependencies become available. RP10 extends the existing event pipeline alongside
each changed producer; it does not wait until the end to discover unowned resources. No package
authorizes a generic workflow engine, alternative executor or retained compatibility layer.

### RP00 — Reconcile authority, scope and enforcement

**Starting evidence:** ADR-0041–0045 and native dependency/architecture checks exist. Plan 15's
protected patch is supplied but unapplied; its earlier apply-check is stale. The plan index still
described Plan 15 as a draft before this review corrected it.

**Remaining actions**

1. Record one operation-to-dataflow inventory covering every nine-tool route, resource, export,
   startup/recovery, operator qualification, capsule cleanup and background maintenance path.
   For each, identify input/output schemas, native selection/validation plan, durable tables,
   exact witness/identity, effect driver, final encoding, and L-row deletion owner.
2. Finish a finite native operation definition path. The current four command argument schemas do
   not cover all read tools, maintenance, result projection and refusal generation. References to
   native `Schema`, `Expr`, `LogicalPlan`, options and typed descriptors remain authoritative;
   do not invent a parallel DSL or callback-based plugin system.
3. Reconcile living design and ADR supersession edges with actual target ownership. ADR-0046's
   operator Python-tooling decision is independent work; do not undo it or change the ty producer.
4. Rebase `15-native-enforcement.patch` against current protected files. Remove obsolete bans and
   old journal/worker/result expectations while adding positive native and thin-Python boundaries.
   Protected harness installation remains an operator-applied step under current rules. Prepare a
   concrete reviewed patch; continue independent work while that installation remains outstanding.
5. Define route-specific capability facts joining exact version, feature envelope, implementation,
   effective policy, and executed conformance. Unsupported, unconfigured, denied and unknown must
   remain distinguishable. Enum enumeration or provider metadata never advertises a qualified route.

**Deletion / exit:** one real rule/aspect change has a discoverable authoritative definition and all
consumers. Contradictory active rules are removed through the allowed installation boundary. Frozen
provenance stays byte-identical. Q01/Q12 remain open until the actual extension and removal evidence.

### RP01 — Finish schema, intrinsic and canonical identity authority

**Starting evidence:** reuse core `evidence/arrow_model`, `native_identity.rs`, `native_key.rs`,
`operation.rs`, `telemetry.rs`, and store `native_delta.rs`, `arrow_contract.rs`, `admission.rs`.
Do not recreate the completed evidence/attempt/process identity paths.

**Remaining actions**

1. Complete the native field inventory for commands, grants, physical observations, raw registry
   facts, result sections, cursors, replay descriptors, retention and maintenance. Generate worker
   and wire projections from the same definitions, with a field-addition oracle exposing omissions.
2. Finish native intrinsic admission for exact execution scope, URI/source authority, extension
   fields, metadata vocabulary/values, nested coordinates and producer/environment witnesses.
   Keep syntactic protocol/path parsing mechanical; put semantic eligibility in native predicates.
3. Qualify the complete **actual** Arrow inventory through store and read: null versus absent,
   tagged struct variants, nested lists/maps, string views/dictionaries, binary, decimals,
   timestamps/timezones, UInt64 boundaries, metadata and required values. Types outside the chosen
   mapping fail explicitly. No cast-to-NULL or invented default may silently repair evidence.
4. Resolve actual kernel boundary edits with the paired patch builder, inheriting Delta's mapping
   and validators where they already do the work. Prove nested insert/drop/replace consistency
   where used. If a route delegates the edit entirely upstream, name the actual native consumer
   and its receipt; a disconnected patch demonstration does not close DFU-07.
5. Replace remaining semantic JSON/digest call sites with typed native identity expressions:
   research/cursor/request bindings, comparison support, execution/producer configuration and
   result identity. Audit `ops/{search,overview,compare_job,inspect_execution,publication,verify}.rs`,
   `store/comparison.rs`, and schema contract identity construction. Preserve only genuine raw-byte
   cryptographic hashing and mechanical encoding; inspect meaning before deleting by symbol name.
6. Declare full dependency witnesses: input values/bytes, exact table versions/cohorts/contracts,
   functions/transform revision, effective policy, producer/build/environment identities and result
   encoding. Derive native cache/replay selection from them; a broad build digest alone is incomplete.
7. Keep known pinned-layout constraints explicit, including `map_entries` nullable value/entry
   layout. Native validation must enforce actual required map values after that physical adaptation.

**Deletion / oracle:** remove domain-object/JSON canonicalization and duplicated field validators
from production semantic paths. Q04 proves independent field completeness and identities stable
under repartition/reorder/compaction but changed by every meaningful dependency. Q01 proves one
field addition reaches IPC, storage/read, native rules, results and generated contracts.

### RP02 — Finish provider contracts, discovery and qualified mutations

**Starting evidence:** complete append uses captured native logical input, shared runtime, writer
options and zero internal commit retries. Published providers are read-only; exact Delta scans and
the local synchronizing store exist. Service-wide DML/metadata/maintenance coverage is incomplete.

**Remaining actions**

1. Enumerate every exposed mutation route, including internal SQL, DataFrame, direct provider,
   typed command, metadata and maintenance. Implement the finite update/delete/merge consumers
   required by projections/retention through complete public builders. Deliberately reject
   unsupported DML before writing; do not build unused arbitrary DML merely to expose an API.
2. Bind the same concrete session/runtime/store, row constraints, writer properties, transaction
   keys, metrics and ownership to every admitted operation. Preserve aggregate Delta planner
   composition under `RetentionPlanner`; inherit validation/mapping/merge/metric delegates.
3. Complete schema/protocol/feature admission before registration. Validate metadata updates with
   upstream validators; derive feature inventory from native features plus actual supported routes.
   Keys/references still require aggregate/anti-join admission before trusted optimizer metadata.
4. Add the Plan 15 controlled `DeltaTableFactory` and `ListingSchemaProvider` consumers for actual
   service-owned table discovery/preparation. Bound listing before accumulation, reject normalized
   collisions and failed/incomplete inventories, and capture table IDs/versions/schema before
   installing immutable catalogs. Reuse or narrowly improve the upstream preparation seam where
   its default store/session/listing behavior cannot meet this contract.
5. Finish provider inventory consistency: names/types/owners/metadata, asynchronous preparation,
   immutable lookup, fully qualified references, native constraints/default/source forwarding,
   exact cohort predicates and accurate pushdown/statistics. No latest lookup may choose publication.
6. Complete all custom logical/physical extension contracts: expressions/child replacement,
   boundedness/eagerness, order/partition/cardinality, current statistics APIs, cancellation and
   metric identity. Audit view/subquery and optimized-away scan ownership under the final composition.

**Deletion / oracle:** remove remaining competing preparation/session factories and custom Delta
membership/schema/planner machinery. Q02 checks valid/invalid rows through each route, duplicate
merge matches, global keys, no-side-effect refusal and one shared runtime. Q10 checks backend handle
identity and simultaneous conditional creates. Power-loss and reclamation remain assigned to RP09.

### RP03 — Complete native control decisions and durable ownership

**Starting evidence:** [native control jobs](../../crates/enrichment-store/src/control_jobs.rs)
own claims, interests, transitions and predecessor checks. [Daemon jobs](../../crates/enrichment-daemon/src/jobs.rs)
still validate resolution/specification state and build interest sets procedurally. The handle
registry is useful physical ownership; it must not become semantic current state.

**Remaining actions**

1. Move remaining `Specification`/`ResolutionStage` semantic validation, interest counts/selection,
   resolution pinning and shutdown/cancellation transition selection into native command plans.
   Keep typed request decoding and mechanical task wakeups/handle lookup in the daemon.
2. Add finite control families for physical ownership, cleanup obligations, query/result/version
   protection, retention enrollment and maintenance generation. Use a shared commit whenever
   correctness depends on a common predecessor. Do not add a second authoritative journal.
3. Replace `.execution-roots.json`, per-operation ownership JSON and capsule reservation files as
   durable authority with typed native facts. Capture actual filesystem/container observations at
   a bounded mechanical boundary; native joins decide ownership, quota, reclaimability and recovery.
   Permanent OS lock inodes and resource handles may remain mechanisms, with explicit scope.
4. Implement reader/result/replay/CDF enrollment before loading selected versions. Fence enrollment
   against maintenance, revalidate the captured generation, and retain exact dependencies through
   optimized-away logical scans, streams, exports, codecs and warm resources. Global file locking
   alone is not the target version/horizon protocol.
5. Audit all contested predecessor transaction keys: job, context head, interest, physical cleanup,
   result/retention and projection checkpoint. Distinct jobs racing one head must conflict. Reload
   and recompute native preconditions on conflict; never allow log retries to silently rebase policy.
6. Reconcile retained command/result identity and transaction version after lost acknowledgements,
   process restart and fresh duplicate requests. Lease expiry cannot authorize a second physical
   producer until prior ownership is reconciled. Terminal publication is not physical exit.
7. Commit complete result dependency selection with publication/head/terminal as required. Record
   orphan candidate and stage obligations before they can become unreachable; their cleanup is RP09.

**Deletion / oracle:** remove remaining procedural job state/eligibility and durable ownership/
reservation readers, while retaining physical guards. Q03 uses independent processes, same-context
races, stale owners, renewal/expiry, multiple interests, lost acknowledgement and each candidate/
control boundary. Q05 proves actual resources stay owned until cleanup, including abandoned awaiters.

### RP04 — Finish acquisition facts, producer fidelity and normalization

**Starting evidence:** native version selection, dependency expansion, URL/DNS admission, cache
decisions, both normalizers, attempt plans and metadata contribution exist. Preserve those. Remaining
source handling, artifact maps, execution producers and rustdoc format support need targeted work.

**Remaining actions**

1. Retain bounded typed registry/revision/source facts with request/representation/receipt/producer
   provenance in Delta. HTTP cached bytes are not by themselves a queryable registry authority.
   Use native file/JSON scans where they preserve exact source structure; unmatched decoders emit
   raw facts and explicit processed-input/gap observations.
2. Complete native source preference, revision reachability, freshness, fallback eligibility,
   package/source/archive/header policy, dependency witnesses and derived environment binding.
   Audit `ops/{resolve,resolve_job,python,revision,source_tree,source_documents}.rs` and fetch failure
   routes. Keep network/archive byte mechanics bounded; Cargo/uv remains an observed solver effect.
3. Complete native execution/LSP/runtime-object observation plans. Preserve raw protocol replies
   and process logs as evidence bytes; lower scope, diagnostics, positions, method outcomes,
   artifact references, usable observations and limitations through typed facts and native plans.
   Remove procedural observation sort/merge and semantic JSON transcript interpretation.
4. Fix the demonstrated rustdoc format mismatch. The current worker deserializes complete payloads
   with `rustdoc-types 0.59.0` (format 59), while the dated producer emits format 61. The probe shows
   stable metadata can reject the payload; format 60 also introduced default-body instability
   metadata which the older model silently drops. Current `ItemFact` does not preserve these fields.
5. Adopt an exact qualified format-61 fact model and preserve ordinary, const and default-body
   stability in generated Arrow facts. Align the retained pure signature renderer to that same
   representation. The verifier found no released matching upgrade beyond `public-api 0.52.2`;
   prefer a minimal reviewed exact-pinned upstream renderer dependency/source update. If necessary,
   use a bounded target-format renderer kernel, not a general format bridge. Do not relabel payloads,
   strip richer facts, downgrade authoritative input to format 59, or retain old-format compatibility
   solely for historical fixtures. Hosted-source formats required for real acquisition must have
   explicit qualified **current source-format** contracts; old internal epochs do not return.
6. Audit full Rust/Python semantic coverage: public binding versus definition, visibility, reexports,
   trait/inherited detail, overloads/bases, unresolved external targets, independent `.py`/`.pyi`,
   source spans and epistemic classes. No arbitrary first winner or metadata borrowing across scopes.
7. Complete recursion/cycle/work/depth exhaustion and partial-coverage behavior at meaningful scale.
   Bound parser input/object allocation, Arrow stream rows/batches/bytes, temporary Delta stages,
   malformed/truncated streams, worker exit and owned cleanup. Native plans must consume the facts
   directly without an object decode/re-encode detour.

**Deletion / oracle:** remove residual semantic acquisition/producer branches, unsupported historical
format fixtures and duplicated source inventories. Q04 uses independent expected facts, real large/
deep Rust and Python packages, conflicts/cycles/missing targets, and the actual worker→Arrow→Delta→
MCP path. Rustdoc fixtures must include stable/unstable and const metadata plus non-null default-body
metadata on functions, associated constants and associated types; unknown/malformed formats refuse.

Rustdoc follow-up evidence: the completed read-only verifier inspected exact installed 0.59/0.60/
0.61 source and the [format-61 model commit](https://github.com/rust-lang/rustdoc-types/blob/2e63bdc44b94e86fe12d6e0034d1ed9c2ac6e9dc/src/lib.rs),
checked [public-api release metadata](https://crates.io/api/v1/crates/public-api) on 2026-09-16,
and executed the parser probe named in §2.2. `rustdoc-types 0.61.0` is the proposed exact fact-model
target; its adoption and a compatible renderer remain implementation work, not a changed pin.

### RP05 — Finish command-to-effect binding and execution semantics

**Starting evidence:** [process grants](../../crates/enrichment-store/src/process_grants.rs) retain
immutable operations and effects, recheck parent ownership and compare current native route/config/
image/containment facts. [Runner](../../crates/enrichment-daemon/src/execution/mod.rs) consumes the
read-back operation. Warm LSP requests rebind/check live command authority. Full action scope is open.

**Remaining actions**

1. Derive each actual operation from the native command definition and exact input/environment/
   producer facts. Validate argv, input inventory/digests/modes, outputs, execution mode, acquisition
   network, budgets and image together. A matching hash proves what was supplied; native joins must
   prove it is what this command authorizes. Drivers cannot supply unrestricted work under a valid
   parent grant.
2. Bind derived environments for local-build resolution and staged Cargo/uv preparation, including
   cases whose initial claim has no environment. Preserve exact source snapshot, lock, installed
   closure, producer identity and operation revision through subsequent process observations.
3. Apply the same native policy to readiness, claim/renewal, status, refusal, process creation/start,
   every LSP request/notification and result usability. Separate fixed operator qualification from
   ordinary command authority; qualification cannot acquire caller inputs/outputs/argv/network.
4. Finish durable warm-LSP ownership across commands: physical lifetime, exact environment reuse,
   new per-command grants, method/document/position scope, cancellation and revocation. Do not release
   ownership because a request ended while the shared child remains alive.
5. Route rustdoc fallback, static workers, Cargo/uv, rust-analyzer, ty and runtime verification through
   finite native effects. Preserve OS sandbox/process-group/timeout/framing mechanics. Keep EXPLAIN,
   analysis, optimization and rejected plans effect-free; first execution takes each dispatch once.
6. Rewrite old ignored execution fixtures that invoke `Runner` without real command authority to
   exercise target grants or the narrowly scoped qualification entry point. Do not restore a bypass
   to make the tests pass. Rebuild images/receipts when their input/protocol identities require it.

**Deletion / oracle:** remove duplicated profile, readiness, producer-mode and input-selection
branches. Q05 runs actual contained Rust/Python correct/incorrect snippets, unqualified refusal,
revocation, repeated execution, stream/future drop, warm reuse, process crash and restart. Prove
permits, reservations, claims and cleanup remain held until physical exit. C20 covers every enabled
execution profile, and affected runs include `just state-leak-check`.

### RP06 — Finish native research selection, comparison and evidence shaping

**Starting evidence:** native scoring/browse/comparison/coverage exists. Handlers still perform
semantic set/sort choices, cursor/request hashing, candidate extraction and response-size trimming.

**Remaining actions**

1. Move overview selection, package/context/environment contribution and latest/exact/fallback
   decisions into bound native request plans. Keep exact namespace/snapshot references and explicit
   empty/unknown/unavailable coverage outcomes.
2. Replace inspection's handler definition sorting/deduplication, ambiguity and aspect eligibility
   decisions with native candidate/coverage/window plans. Bind independent aspect cursors to exact
   selection, snapshot, transform/policy and result contracts.
3. Reuse score factor rows for both ranking and explanations. Remove remaining duplicate kind/token
   normalization in handlers; preserve deterministic ties, caps and string representation semantics.
4. Complete typed field-level before/after comparison and environment contribution conflicts.
   Remove JSON/string equality shortcuts for semantic fields and handler assembly of alternatives.
   Before/after coverage must stay scoped to the correct producer/environment/subject.
5. Native plans select evidence references, exact source/window eligibility, excerpts and diagnostic
   recovery choices. Byte decoding and final text/JSON rendering remain narrow output mechanisms.

**Deletion / oracle:** delete handler sort/dedup/selection, semantic excerpt/page trimming and duplicate
comparison/request identities. Q06 checks exact/fallback/ambiguous/absent outcomes, independent
aspects, factor/tie equivalence, source support and cold/offline comparison against independent facts.
Inspect native plans as supporting evidence, not as a substitute for semantic results.

### RP07 — Replace result composition and complete generated MCP delivery

**This is a major remaining implementation package.** Delta-backed receipt/index lookup does not
make the current JSON body and mutable Envelope pipeline native.

**Remaining actions**

1. Define typed Arrow/Delta result relations for outcome, sections, ordered rows, evidence/artifact
   references, coverage, diagnostics, retained identity and per-aspect continuations. Each operation
   references these definitions; field-name aliases are generated metadata, not a Rust match table.
2. Keep complete native results through publication. Replace `JobDeliveryFactory`/`DeliverySlot`
   semantic callbacks, repeated envelope construction and object-based result rebase with native
   result plans and exact selected versions/receipts. Commit complete result/dependency selection
   with terminal publication; no successful terminal record depends on later rendering or writing.
3. Implement native page/window selection and retained replay. Refuse incompatible/stale cursors
   using exact witnesses. Use the same selection for resource and tool reads, export and recovery.
4. Implement one bounded final sink for actual escaped UTF-8 JSON and MCP content/frame cost.
   Feed measured encoded sizes back as typed inputs to native selection when necessary. The sink
   handles serialization/index offsets, not semantic DTO trimming. Include mandatory envelope and
   retained-descriptor overhead; an impossible cap returns the declared minimum-budget refusal.
5. Remove `overflow`'s ordered clearing of data/evidence/artifacts/limitations and handlers' `pop`
   loops. Retained completeness and selected inline views derive from the same native result;
   byte accounting includes quotes, slashes, controls, Unicode and nested values without unbounded
   duplicate documents. A failure while encoding cannot publish a successful result.
6. Replace recovery's hand-picked JSON fields and status/job/error reconstruction with an admitted
   typed native outcome projection. Keep read-only recovery after terminal success; it must not
   synthesize new artifacts or reinterpret old envelopes.
7. Generate all nine tool input/output bindings, operation metadata and annotations from shared
   definitions. Keep FastMCP 4.0.3 strict validation, canonical structured content and bounded optional
   text/resources. The adapter performs no acquisition, native policy or result reconstruction.
8. Complete artifact authorization, source windows and export closure against native receipt and
   publication/result descriptors. Preserve exact full content identities and service-issued handles;
   do not admit arbitrary paths or prefix directory lookup. Use the existing native dependency graph.
9. Regenerate current wire/JSON/Python models and packaged schemas together. Remove superseded
   aliases/fixtures/configurations and synchronize `skills/library-research/` and setup guidance.

**Deletion / oracle:** replace the remaining semantic content of `store/result.rs`, `projection/render.rs`,
`daemon/delivery.rs`, result callbacks and handler trimming. Retain bounded encoding mechanics only.
Q01/Q06 exercise one new aspect through the common declaration and real MCP calls with large escaped/
Unicode/nested values, exact per-section pages, missing artifacts, retained replay and cancellation.

#### Required public routes

| Tool / resource | Native completion obligation |
|---|---|
| `resolve_library` | Exact source/environment/freshness routing; durable progress and complete retained terminal result |
| `library_overview` | Native package/context selection, coverage and stable result sections |
| `search_evidence` | Shared factors/rank/explanation, scoped references, native page/cursor and byte selection |
| `inspect_symbol` | Native exact/fallback/ambiguity/aspect outcomes; independently scoped execution observations |
| `compare_releases` | Native before/after field differences and coverage, alternatives, cold preparation and retained completion |
| `verify_usage` | Exact granted environment/action, true process outcome, native usability/limitations and retained evidence |
| `read_artifact` | Native receipt authorization/section/window selection, exact bytes and bounded encoding |
| `job_control` | Native interest/count/state/cancellation decisions; physical cleanup distinguished from terminal result |
| `service_status` | Native effective readiness/policy/counters/history; missing qualification and dropped diagnostics explicit |
| Snapshot manifest, artifact and overview resources; export CLI | Same selected publications/results/policy as tools; publication-scoped changes and complete verifiable dependencies |

### RP08 — Complete CDF, rebuild selection and fresh-process replay

**Starting evidence:** reuse both search surfaces, `EvidenceTables::change_plan`, atomic selected
checkpoints and the public manifest consumer. Typed commands/build revisions exist; the qualified
logical read-provider cache and complete replay consumer do not.

**Remaining actions**

1. Complete typed native projection definitions and rebuild eligibility. Move remaining Rust
   source-vector/revision/mode decisions in `SearchProjection::prepare` and semantic checkpoint
   validation into native rules. Driver loops may dispatch the finite selected physical operations.
2. Qualify inclusive version windows and signed/unsigned boundaries, preimage/postimage handling,
   inserts/deletes/updates, same/different cohorts, unchanged versions, multi-table source vectors,
   unpublished candidates and no-op OPTIMIZE. Change timestamps do not select a publication.
3. Complete schema/rule/source/history rebuild behavior from retained target snapshots. Missing CDF
   history is an explicit rebuild or refusal, never an empty successful feed. Manifest explanations
   and search projections must agree on semantics and limitations.
4. Record orphan output and replay obligations at output/control failure boundaries. Select exact
   output versions and input offsets together. Reconcile duplicated/unknown commits before retry.
5. Implement a fresh-process typed-command replay consumer with complete input/version/function/
   policy/environment/result descriptors. Reconstruct current native plans and explicitly rebind
   permitted runtime/store/lease/effect handles. Revoked authority does not revive on replay.
6. Add a **real immutable read-provider cache consumer** for `DeltaLogicalCodec`, with a narrow
   dispatch allowlist. Validate decoded table identity, exact version, semantic schema/contract and
   explicit cohort predicates; reacquire retention before use. Reject unsupported logical nodes
   before TODO hooks, reconstruct CDF from descriptors, and exclude `DeltaPhysicalCodec` entirely.
7. Change each dependency witness separately and prove reuse invalidation. Retained command/result
   evidence must continue to reconcile requests after Delta application transaction markers expire.

**Deletion / oracle:** remove procedural incremental/rebuild policy and any generic persisted-plan
assumption. Q07 compares incremental results with full native recomputation across the matrix and
interrupted offset publication. Q08 restarts with no handles, advances underlying heads, tests wrong
schemas/identities/history, unsupported codecs and dependency-by-dependency invalidation.

### RP09 — Implement unified retention and native maintenance

**Starting evidence:** retained providers carry lifetime guards and manual filesystem cleanup exists.
There is no complete control-owned retention horizon/enrollment/maintenance system. The old
[maintenance inventory](../../crates/enrichment-daemon/src/maintenance.rs) still names retired payloads.

**Remaining actions**

1. Define one finite configurable retention contract and native validation of horizon relationships:
   source artifacts, evidence/projection/control/definition/event tables, data/log/checkpoint history,
   active reads/results, CDF resume, command replay and application transaction marker lifetime.
   Persist the effective policy witness with maintenance and affected operations.
2. Compute protected versions/cohorts/artifacts and reclamation candidates by native joins over
   RP03 enrollments and selected references. Include exact policy/process-operation/process-effect
   definition versions, semantic contracts, retained results, exports and projection inputs.
3. Complete maintenance-generation fencing against new readers, writers, result retention, CDF and
   replay. Protect dependencies before provider loading; preserve logical leases even when physical
   optimization removes scans. Refuse unsafe expiration and report the blocking owner/reason.
4. Run native checkpoint, log compaction, OPTIMIZE and VACUUM through finite owned commands with
   explicit version inputs. Use native retention properties/private retention machinery through
   public operations; do not implement a substitute Delta tombstone/log engine.
5. Qualify `with_keep_versions` against file retention **and** log/CDF reconstruction. Retained old
   target publications remain readable through checkpoint, compaction, marker expiry and concurrent
   cleanup. Then actually reclaim unprotected data. Permanently disabling reclamation is incomplete.
6. Reclaim abandoned producer stages, `export_candidate_*` tables, failed projection writes and
   unselected cohorts through typed cleanup obligations. Physical directory deletion follows the
   native candidate decision and verified ownership; errors cannot be counted as freed space.
7. Replace old operator cleanup/reset inventories with target control-selected inventories and
   preview/apply behavior. Keep one-time retired-install deletion in RP13 separate from ongoing
   native maintenance; neither may touch user repositories or shared package caches.
8. Complete actual storage durability qualification: simultaneous conditional create, file/directory
   synchronization, failures around data/log commit and directory changes, then the required
   filesystem power-loss/crash oracle. Process kill alone is insufficient. An unavailable harness/
   privilege is `blocked` with the exact prerequisite, and prevents terminal completion.

**Deletion / oracle:** remove old retention metadata/reservation/catalog cleanup authorities and
bespoke incremental file inventories. Q09 proves protection and actual safe reclamation; Q10 proves
the selected backend's durability boundary. No `ConditionalPutShim` or unsafe overwrite compensation.

### RP10 — Finish structured telemetry and resource/lifetime accounting

**Starting evidence:** reuse `core/telemetry.rs`, `telemetry_history.rs`, native counters, bounded
ingress and explicit close. [Daemon metrics](../../crates/enrichment-daemon/src/metrics.rs), LSP/cache
component counters and durable physical-resource bookkeeping still have separate owners.

**Remaining actions**

1. Feed Delta builder operation metrics and structured kernel `MetricEvent` through
   `ReportGeneratorLayer`/`MetricsReporter` into the existing native event contract. Extend typed
   fields for attempt/grant/job/publication/maintenance lineage; keep callbacks cheap and bounded.
2. Replace daemon request/fetch/verification counters, component summaries and semantic JSON metrics
   with raw observations plus native views. Atomics remain only physical sequencing/admission/drop
   mechanisms where appropriate. Metrics never prove successful commit or cleanup.
3. Derive typed failure/recovery actions from native rule/result definitions. Audit the current
   opaque `recovery_payload`: it may carry final wire encoding, but cannot hide policy selection
   or become an independently interpreted decision record.
4. Complete allocation accounting before or at capture for parser objects, IPC batches, native
   inputs, canonical kernels, default-kernel/log replay, writer buffers/uploads, result encoding,
   diagnostics and child processes. Arrow batch reservation after allocating a potentially large
   object is insufficient. Keep one effective bound and identify external budgets explicitly.
5. Qualify event queue saturation, writer failure/unknown append, conflicting writes, actor/runtime
   drop, shutdown ordering, concurrent close, restart, and missing history. Make lost observations
   explicit; no recursive observation of diagnostic reads/writes or deadlock at concurrency one.
   Retention/compaction of `native_events` follows RP09.
6. Qualify cancellation/abandoned awaiters during decoding, index materialization, Delta writes,
   commit reconciliation, process creation, LSP and cleanup. Counts of zero must correspond to
   actual exited work, released reservations and reconciled durable ownership.

**Deletion / oracle:** remove independent semantic counters/history/recovery policy and redundant
resource authorities. Q11 checks native pool/spill and external allocations under pressure, bounded
diagnostic support and concurrent lineage. Q05 checks physical ownership; diagnostics cannot stand
in for those observations.

### RP11 — Select native physical choices from complete journeys

**Remaining actions**

1. Measure the completed cold/warm/offline Rust/Python, search/inspection/comparison, export and
   contained execution journeys. Include control commit/admission and diagnostic overhead; current
   multi-minute development receipts are not an accepted final latency profile.
2. Record latency distribution, peak native/external memory, spill, bytes/files read, table/log growth,
   retained history and cleanup cost. Identify repeated DAG evaluation and stage only bounded native
   contributions with explicit ownership/invalidation, extending existing reuse where justified.
3. Select native file sizing, partitions, statistics columns, compaction, useful clustering/Z-order,
   page/row-group/Bloom/dynamic filters, metadata caches and IPC batch sizes from those measurements.
   One configuration/contract path owns each chosen option and its effective read/write consumer.
4. Prove DeltaScanMetaExec/full-scan equivalence with deletion vectors, strict missing-file failures,
   exact/inexact statistics, nested mapping and optimizer rewrites. Reuse current native scan/kernel/
   Parquet readers; no custom count index, file-membership shortcut or independent scheduler.
5. Record adopted and deliberately unused physical capabilities, with workload rationale and
   semantic-equivalence evidence. Optional remote stores/catalogs remain unconfigured unless an
   actual existing consumer makes them required.

**Exit:** Q11 plus recorded complete-journey measurements, source/options and before/after native
plan evidence justify the selected defaults. Do not claim speedup from the existence of an API.

### RP12 — Close source deletion and architecture

**Remaining actions**

1. Close every deletion row in §5 by production imports/call paths, recovery/error/CLI consumers,
   generated outputs and package contents. Rename residual files where an old name hides a now
   purely mechanical role; moving old behavior to another module does not count as deletion.
2. Delete obsolete schemas, aliases, inactive executable fixtures, feature flags, scripts, launch
   configs and dependencies. Rewrite target tests instead of retaining retired-format acceptance
   oracles. Keep frozen provenance and requested reviews as the explicit source exceptions.
3. Complete positive architecture/AST rules with fixtures, source policy and locked dependency
   checks. Run the skills' project-gap rules as leads and inspect their actual findings; syntax
   alone does not prove semantic ownership. No suppressions or broad dependency allowances.
4. Demonstrate one real observation/aspect addition and one policy-rule change through the common
   definition. Native normalization/eligibility/results/status/diagnostics and generated bindings
   must follow without new independent semantic branches in Python or handlers.
5. Reconcile living design, setup/product skill, implementation plan, capability dispositions and
   enforcement installation. Remove active instructions for retired runtime/state paths.

**Exit:** Q01 and source portion of Q12, complete build/package inspection and G1–G7 review evidence.
No unexplained imperative semantic owner remains. Review readiness is separate from acceptance states.

### RP13 — Qualify final source, activate fresh state and remove retired runtime

**Remaining actions**

1. Implement/register the Q01–Q13 commands in §6 with concrete test selection and receipt capture;
   these proposed recipes are not present in the inspected `just --list`. Preserve existing gate
   meanings and the four-state evidence model. Run decisive affected checks during RP00–RP12;
   run the final aggregate only once the integrated source is ready.
2. Build daemon, native worker, executor and Python package from one locked source/configuration
   identity. Qualify a non-editable installed candidate on dedicated fresh state from an unrelated
   directory, including all nine tools/resources, Rust and Python static/contained paths, correct
   and incorrect snippets, offline reuse, cold comparison, export, retained results and restart.
3. Run actual Codex and Claude client workflows and required Linux execution profiles. Include C20
   repository digest and XDG leak checks. Missing images/credentials/tools are named prerequisites,
   not mocked passes or permission to narrow functionality to static-only.
4. Build a concrete deletion manifest of retired service-owned installations/state/artifacts/caches,
   client registrations and obsolete task-owned development artifacts. Reverify actual paths,
   ownership and running processes; do not glob shared Cargo/uv caches or user repositories.
   Previously recorded roots include `/home/paul/.local/opt/library-enrichment/plan14-5b2640ca15e4`
   and `/home/paul/.local/state/library-enrichment-v2`; these are candidates to reverify, not unchecked
   deletion commands.
5. After candidate qualification, stop old daemon/adapters, finish physical cleanup, activate the
   target with fresh state and regenerated bindings, and apply only the reviewed deletion manifest.
   No old-state import, historical backup or rollback executable belongs to the delivered design.
6. Run deployed smoke; prove process/install/source/config/client identities and zero unexplained
   jobs/children/leases/reservations. Close Q12's installation half, then Q13's aggregate. Use the
   acceptance-report skill at this final reporting boundary and update STATUS/design/plan/index
   from recorded current-source evidence.

**Exit:** every mandatory oracle passed for the final source and deployment, every deletion row
closed at its applicable boundary, one installed runtime and one fresh target epoch. Required
`failed`, `blocked`, or `not_run` work prevents a completion claim. Release/client qualification
remains a terminal/manual step, not a new check before every edit or commit.

## 5. Complete deletion carry-forward

Every L-row remains open until its full source and behavioral closure. Already deleted files must
not be restored for comparison or old tests. Remaining work includes behaviors surviving under new
names, not just the baseline file names.

| Plan 15 ID | Already replaced or removed | Remaining closure / owner |
|---|---|---|
| L01 | `catalog_generation.rs`, pointer/manifest publication replaced | Control retention/log/maintenance and old cleanup consumers; RP03/RP09/RP12 |
| L02 | JSON job journal and old authoritative map replaced | Procedural job validation/interest decisions and durable physical ownership; RP03/RP05 |
| L03 | Ordinary evidence writer, semantic RelationWriter and staging writer removed | Audit residual `dataset.rs`/`record_writer.rs` mechanics, every write route and resource lifetime; RP02/RP04/RP10 |
| L04 | Native contribution, attempts and metadata rebase present | Result publication callbacks, remaining acquisition/environment object merges and complete dependency selection; RP03/RP04/RP07 |
| L05 | Rust and Python semantic walkers removed | Producer fidelity/scale, residual execution/LSP normalization and old fixtures; RP04 |
| L06 | Generated Arrow worker interchange and direct native ingestion present | Execution/runtime semantic JSON interpretation, fact completeness, residual staging resources; RP01/RP04 |
| L07 | Native registry version/marker/dependency selection present | Remaining source/revision/freshness/environment eligibility and durable raw facts; RP04 |
| L08 | Whole-score UDF/tokenizer replaced | Handler factor/kind/token duplication and full semantic oracle; RP06 |
| L09 | Shared evidence/policy/process native identities present | Research/result/cursor/config/schema semantic hashes and dead canonical folds; RP01/RP07 |
| L10 | Native command/routes/process grants present | Complete command-input scope, procedural readiness/selection/job/resource policy and all physical qualifications; RP03/RP05/RP06 |
| L11 | Native retained receipt/section/dependency lookup present | Procedural body/aliases/callbacks/DTO trimming/recovery and result identity; RP07 |
| L12 | Native Delta scans, CDF projections and diagnostic history present | Retention stores, physical reservation journals, old cleanup inventories, component metrics and replay policy; RP08–RP11 |
| L13 | Wire 3.0, research-result/3 and protocol 3 replacements generated | Final generated operation/result/cursor epoch, packaged aliases and obsolete fixtures; RP00/RP07/RP12 |
| L14 | Many alternate writers/readers and dependency exceptions removed | Whole-tree imports/features/scripts/rules/configs/package contents and inactive executables; RP12 |
| L15 | No target installation/deletion claimed | Verified old-root/process/registration manifest, fresh activation and actual removal; RP13 |

## 6. Acceptance implementation and run discipline

These are the original mandatory Q oracles, retained in full. Their final-target status is
**`not_run`**. Focused development receipts in §2.2 support implementation decisions; they do not
close these aggregates. Register recipe implementations as the owning boundary becomes executable.

| ID / recipe to implement | Remaining decisive oracle | Packages |
|---|---|---|
| Q01 `native-contracts-check` | Complete generated field/operation path, real aspect addition and one-policy-change consistency; no duplicate semantic owner | RP00/RP01/RP05/RP07/RP12 |
| Q02 `delta-mutation-check` | Builder/planner/metadata/feature composition; actual valid/invalid nested/tag/CHECK/null routes, duplicate merge/global keys, side-effect-free refusal and pinned inventories | RP01/RP02 |
| Q03 `delta-control-check` | Independent-process claim/head/interest/renewal races, stale owner, lost acknowledgement, fresh duplicate, publication/ownership crash boundaries | RP03/RP05 |
| Q04 `native-evidence-check` | Complete Arrow mappings/identity metamorphism, independent real/deep Rust/Python facts including rustdoc metadata, partial coverage and bounds | RP01/RP04 |
| Q05 `native-effect-check` | No planning effects, one dispatch, revoked/wrong-scope refusal, actual child/writer cleanup, drop/restart/warm-resource ownership | RP03/RP05/RP10 |
| Q06 `native-research-check` | Native factor/scope/aspect/comparison semantics, all result sections/references, exact encoded limits and retained completion through actual MCP | RP06/RP07 |
| Q07 `delta-incremental-check` | Incremental/full equivalence for mutations/unpublished cohorts/OPTIMIZE/schema-rule-history rebuild and output/offset faults | RP08/RP09 |
| Q08 `native-replay-check` | Fresh-process typed replay, qualified immutable provider codec, rejected wrong schema/identity/unsupported routes, exact dependency invalidation | RP08 |
| Q09 `delta-retention-check` | Query/result/version/CDF protection, optimized-away lease, enrollment/vacuum races, marker expiry/log reconstruction and actual reclamation | RP03/RP09 |
| Q10 `delta-storage-check` | Real conditional creates/backend identity, write/commit/storage faults, file/directory synchronization and required power-loss evidence | RP02/RP09 |
| Q11 `native-resources-check` | Native and external resource caps/lifetimes, strict missing files/DV metadata equivalence, bounded correlated telemetry and complete-journey physical measurements | RP02/RP05/RP10/RP11 |
| Q12 `native-removal-check` | L01–L15 source/package/config/rule and installed-path closure, separated into source and installation phases | RP12/RP13 |
| Q13 `unified-runtime-qualify` | Final-source quality, all nine tools/resources, actual installed Rust/Python and Codex/Claude, required profiles/offline/export, deployed smoke and joined Q01–Q12 | RP13 |

**Efficient sequence:** affected strict Rust Clippy then meaningful boundary tests; schema generation
and conformance after contract changes; affected Ruff/ty/Python contract checks after Python changes;
dependency/provenance/rule checks at their relevant boundaries. Do not run full suites or actual
clients repeatedly while replacing a semantic owner. After the integrated source is ready, run
the registered final quality, execution/live/client tiers, then acceptance generation/check once
against those receipts. Repeat only for changed source, failures or unresolved evidence.

Do not weaken tests to preserve a retired direct-driver or old-format route. Replace those fixtures
with the target's real entry path and independent expected results. Test invocations, exact source,
pins, inputs, profile and bounds must be retained. Mocks or syntax scans do not qualify product
behavior. Q12's installation deletion follows candidate qualification; it must not prevent building
and testing the fresh candidate first.

## 7. Review and integration traceability

### 7.1 Review findings

| Finding | Remaining packages | Closure |
|---|---|---|
| F01 validated mutations | RP01/RP02/RP09 | Q02/Q10; L03 |
| F02 claims and publication | RP03/RP07–RP09 | Q03/Q05/Q08–Q10; L01/L02/L04 |
| F03 semantic mapping/trusted metadata | RP01/RP02/RP04 | Q02/Q04; L06/L09 |
| F04 owned effects | RP03/RP05/RP10 | Q03/Q05/Q11; L10 |
| F05 obstructing rules | RP00/RP12 | Q01/Q12; L13/L14 |
| F06 Arrow-first normalization | RP04 | Q04/Q13; L05–L07 |
| F07 complete operation policy | RP00/RP03/RP05–RP07 | Q01/Q05/Q06; L10 |
| F08 native scoring | RP06 | Q06; L08 |
| F09 native identities | RP01/RP07–RP09 | Q04/Q08/Q09; L09 |
| F10 native result composition | RP07 | Q06/Q13; L11/L13 |
| F11 Delta persistence/maintenance | RP02/RP03/RP09/RP12 | Q02/Q03/Q09/Q12; L01–L04/L12 |
| F12 resources and lifetime | RP02/RP03/RP05/RP10 | Q05/Q10/Q11 |
| F13 route-specific capability claims | RP00/RP02/RP08/RP12/RP13 | All Q receipts and dispositions below |
| F14 planner/configuration | RP02/RP10 | Q02/Q05/Q11 |
| DFU-01 codec/replay | RP08 | Q08 |
| DFU-02 retention/conditional storage | RP03/RP09 | Q03/Q09/Q10 |
| DFU-03 complete builders/private delegates | RP02 | Q02/Q12 |
| DFU-04 incremental CDF | RP08/RP09 | Q07/Q09 |
| DFU-05 discovery versus publication | RP02 | Q02/Q03 |
| DFU-06 native scans/kernel integration | RP02/RP10/RP11 | Q02/Q11 |
| DFU-07 paired kernel edits | RP01/RP02 | Q04 |
| DFU-08 structured telemetry | RP07/RP10 | Q06/Q11 |

### 7.2 Complete integration dispositions

This carries forward every family in Plan 15 §8. No optional integration exemption permits keeping
procedural authority for a required local service operation.

| Integration family | Remaining disposition / evidence owner |
|---|---|
| Delta TableProvider / CDF provider | Existing production consumers; finish exact schema/vector, mutation and retention evidence; RP02/RP08/RP09 |
| DeltaTableFactory / ListingSchemaProvider | Implement bounded controlled discovery/preparation, capture/admit then immutable registration; RP02 |
| Unity schema/catalog/list | No configured source identified in this review; retain explicit unconfigured disposition. If an actual enabled source is found, qualify upstream adapter credentials/list failures/version capture; RP00/RP02 |
| DeltaPlanner / DeltaExtensionPlanner | Existing composed planner; finish complete mutation/resource routes while inheriting private delegates; RP02/RP10 |
| Eight ExecutionPlan / nine DisplayAs implementations | Inherit current scans/validation/mapping/merge/metrics through public operations; no retired physical wrapper or display authority; RP02/RP11 |
| PruningStatistics | Qualify native exactness/DV/filter/COUNT behavior and measured consumers; RP11 |
| DeltaLogicalCodec | Add qualified immutable read cache with full revalidation/rebinding; RP08 |
| DeltaPhysicalCodec | Excluded; generate fresh current physical plans and test refusal; RP08/RP12 |
| DataValidation / MergeBarrier / MergeValidation / MetricObserver logical nodes | Inherit complete public builders, never copy/register private delegates separately; RP02 |
| MakeParquetArray / ToJson / ZOrderUDF | Inherit supported parent operations; no private imports, JSON semantic identity or Z-order scoring; RP01/RP02/RP11 |
| DeltaDataWriterExt / datafile writers | Complete logical-input builder remains default. Qualify upstream preparation only if a required physical-input consumer exists; RP02 |
| DataFusionEngine / kernel Engine | Existing native scan integration; finish default-engine/log resource accounting; RP10 |
| ObjectStore implementations | Mandatory qualified local conditional/synchronizing path; ConditionalPutShim excluded. S3/experimental DeltaIO require actual deployment and separate evidence; RP02/RP09 |
| ParquetObjectReader / FileStream | Native ranges/footer/pages/finite streams and explicit missing/corrupt input outcomes; RP04/RP11 |
| Kernel SchemaComparison | Private; inherit mapping/validation, qualify actual service semantic schema rather than copy internals; RP01/RP02 |
| Kernel RetentionCalculator | Private; inherit checkpoint/log/property behavior with service-derived protected inputs; RP09 |
| ExpressionItem / SchemaPatchItem / paired patch builder | Actual paired kernel edit or evidenced upstream delegation at the real boundary; RP01 |
| Serde Deserialize / Serialize | External format/protocol and final mechanical encoding only; no semantic hash or runtime-handle replay; RP01/RP04/RP07/RP08 |
| TableFeature EnumCount / IntoEnumIterator | Derived inventory joined to protocol/policy/route receipts; RP00/RP02/RP12 |
| TableMetadataUpdate Validate / ValidateArgs | Reuse for native metadata command inputs, not generic row rules; RP02 |
| ReportGeneratorLayer / MetricsReporter | Integrate into the existing bounded Arrow/Delta event path; RP10 |
| Broader provider-map source/sink/adaptation/defaults/pushdown APIs | Complete actual contracts at native layers; async remote catalogs, UDTFs and statistics registries only for a concrete consumer; no duplicate scheduler; RP01/RP02/RP04/RP11 |
| DataFusion FFI | No cross-process transfer; generated Arrow IPC remains worker transport. No invented plugin requirement; RP04/RP12 |

## 8. Resume instructions and completion record

1. Read this plan, Plan 15's destination/ADRs and the current source baseline. Preserve unfinished
   target changes and unrelated operator edits. Do not restart completed normalizers, scoring,
   dependency selection, CDF consumers, process readback or diagnostic history.
2. Start RP00's finite inventory and RP01's remaining result/retention/effect witness contracts.
   Build the largest missing semantic replacements—RP03/RP05 control/effect scope and RP06/RP07
   research/results—using the existing native runtime. Repair the demonstrated rustdoc defect in
   RP04 before claiming the real producer path works.
3. Record progress against RP IDs with exact changed/deleted paths, source receipt, focused command/
   log and remaining boundary. Keep this document as the remaining-scope authority; avoid another
   chronology that makes already completed replacements look unimplemented.
4. Finish replay/retention/maintenance/resources and source deletion before final installed
   qualification. Protected rule installation and power-loss/client prerequisites stay explicit;
   they do not block independent architecture work or justify narrowing mandatory scope.

## Outcome (recorded after implementation)

Not implemented by this planning pass. Record what was built, corrected mistakes, deliberate
deviations and final-source acceptance evidence here as execution completes. This plan is complete
only when its remaining implementation, deletion, qualification and activation obligations are
all satisfied; creating the document does not close Plan 15.
