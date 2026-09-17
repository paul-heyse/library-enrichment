---
title: Complete the schema-governed runtime and native cache hard pivot
status: in-progress
date: 2026-09-17
adrs: [ADR-0041, ADR-0042, ADR-0044, ADR-0045, ADR-0047, ADR-0048, ADR-0049, ADR-0050, ADR-0051]
phase: 6
---

# Complete the schema-governed runtime and native cache hard pivot

## 1. Scope and execution authority

This combined plan, requested **2026-09-17**, integrates all remaining implementation, deletion,
qualification and activation obligations in [Plan 17](17-schema-governed-unified-runtime-hard-pivot.md)
with the enhancements and restrictions in the
[CacheFactory review](../design_review/reviews/design_review_datafusion-cache-factory_2026-09-17.md)
and its [fourteen library probes](../design_review/reviews/evidence/datafusion-cache-factory-2026-09-17/README.md).

**Use this plan when implementation resumes.** It replaces Plan 17's remaining work sequence.
Plan 17 retains historical receipts and its complete numbered FP00–FP16 requirements. Those
requirements, L01–L24, Q01–Q13, SC01–SC10 and §7 integration dispositions remain binding in full;
the CP packages here assign their residual work without reopening completed foundations. An
omitted detail in a summary does not waive its original requirement.

**Implementation authorized 2026-09-17.** Execution proceeds in this document's dependency order.
No package or product acceptance gate is closed. The preceding planning pass was documentation-only;
its cache probes establish library behavior, not application qualification. The execution baseline
is `.dev-state/plan18/execution/baseline.json`; preserve its pre-existing changes.

### 1.1 Required destination

- DataFusion owns service transformations, selection, policy/eligibility, result composition and
  inspectable command/materialization plans. Delta owns durable relational authority. Exact external
  bytes remain in the narrow immutable blob store with native identity/reference/retention facts.
- One finite Rust/native declaration generates fields, rules, identities, provider contracts,
  operation bindings, diagnostics and transport projections. Metadata carries meaning; native
  admission and qualified execution enforce it.
- Rust retains bounded I/O, buffering, synchronization and process mechanisms. Python retains thin
  FastMCP transport and mechanical producer extraction/encoding, without a second semantic engine.
- Preserve nine MCP tools, resources, Rust/Python insights, qualified execution, durable jobs,
  cancellation, offline reuse, complete retained results and export.
- Delete replaced owners, schemas, fixtures, aliases, readers, launch paths, installs and compatibility
  branches. No historical data migration, dual writer, old/new switch or rollback executable.
- Keep frozen specification provenance and requested reviews/evidence. Target Delta history, exact
  source artifacts, command witnesses and replay remain current functionality.
- Retain published evidence and completed results until explicit removal. Reclaim unreferenced
  candidates and physically released temporary data through native retention authority.

### 1.2 Binding execution and testing order

**No full integration tests until the architecture pivot and all legacy deletions are complete,
including verified retired installations, state and client registrations.** Downtime and intermediate
functional discontinuity are accepted. This carries forward Plan 17 §8.1 and supersedes its older
candidate-qualification-before-retirement text everywhere in the combined sequence.

Before that barrier use meaningful unit fixtures, isolated native tests/probes, affected compile/
Clippy/static checks, generated-contract checks and scoped Ruff/ty. A publication/CDF/export/storage
or MCP service journey remains an integration test even inside a library module. Author those
tests during implementation; run them after the barrier. Do not run full `just ci`, `just test`,
live producers or clients as intermediate reassurance.

After the barrier run final Q/SC integrations, storage durability, real clients/profiles and complete
journey measurements. Repair target code directly and requalify affected final-source receipts.
Activate one fresh target only after qualification; no legacy restore path is introduced.

## 2. Starting boundary and preserved implementation

Planning inspected HEAD **`ccf5739a333d29e6fbabe6e054c942f1c5da8b0b`** plus the existing dirty tree.
The resumed execution began clean at this HEAD. Preserve its implementation and concurrent operator
edits. `.dev-state/plan18/planning/baseline.json` hashes six inputs and 231 pre-existing changed/
untracked files; it is a preservation checkpoint, not attribution or qualification.

Use [Plan 17 §2](17-schema-governed-unified-runtime-hard-pivot.md#2-current-implementation-and-the-remaining-boundary)
and [§8.2](17-schema-governed-unified-runtime-hard-pivot.md#82-completion-record) for detailed source
credit and historical receipts. Preserve these foundations:

| Implemented source | Remaining boundary |
|---|---|
| Native declarations, manifest/change relation and generated codecs | Complete field/reference inventories and all semantic consumers |
| Binary ReleaseId/EnvironmentId/ContextId/SnapshotId, typed clocks and native request/cursor/configuration preimages | Remaining ID/digest families, witnesses and full storage/operator/codec matrices |
| Semantic analyzer, full-field UDFs, scoped admission and exact projection | Complete operator/mutation coverage and trusted provider properties |
| Atomic listing, controlled Delta factory and composed planner | Full mutation/metadata/feature routes and cache extension integration |
| 23 Delta control families, physical ownership, retention roots/leases and candidate obligations | All read/replay/export/warm-owner consumers, crash reconciliation and reclamation |
| Format-61 Rust facts, structured callables, native normalizers, protected registry/revision definitions | Remaining source/runtime/LSP facts, effect authorization and producer fidelity |
| Native research policy, retained ResultRecord/per-tool relations and nine generated MCP bindings | Remaining handler/window/byte-fit/result composition and actual all-route fidelity |
| Native CDF/checkpoint selection, constraints and maintenance helpers | Fresh-process replay, restricted provider cache and complete retention consumers |
| Twenty kernel metric variants, runtime lanes and joined physical/release/diagnostic tasks | Builder lineage, external accounting and complete lifecycle/pressure qualification |
| Once-only spill-backed operation indexes | Replace procedural entry/count channel with execution-owned CacheFactory materialization |

Starting source identities are state/snapshot **10**, wire **4.0**, codec **native-json/3**. Do not
retain an identity after changing its semantics: derive required final contract/epoch changes,
refuse incompatible state and activate fresh roots without conversion.

No FP package or complete Q/SC matrix is closed. Prior unit/static receipts are development evidence;
prior integrated journeys are historical and are not rerun during the pivot. STATUS reports the
registered acceptance report stale for current source; this plan promotes no tally.

### 2.1 Known failures retained for closure

| Existing boundary / receipt | Owner and disposition |
|---|---|
| Navigation List child metadata — `typed-citation-navigation.log` | CP02/CP05/CP06; current-worker full oracle in CP12 |
| Execution/export artifact Struct — `integrated-publication.log` | CP01/CP03/CP05/CP06; actual typed execution/export in CP12 |
| Decimal diagnostics, nullable document kind, result clock/window and shutdown — `native-research-focused.log` | CP01/CP02/CP06/CP09; separate stale fixtures from target defects without weakening semantics |
| Cleanup fixtures using retired `owned/*.json` | CP03/CP10 replace with native ownership/grant contracts |
| Older publication/CDF/export/shutdown pass with diagnostic drops | Preserve source limits; CP09 addresses loss/pressure, CP12 qualifies current source |
| Stale worker identity and quality/acceptance receipts | CP11 matching candidate, CP12 current-source receipts |

The full history remains in Plan 17 §2.3. A unit repair does not close an integrated journey; an
old failure is not asserted to reproduce without execution.

## 3. Capability basis and cache architecture

Use the pinned [DataFusion](../../.claude/skills/datafusion/SKILL.md),
[Delta Lake](../../.claude/skills/deltalake/SKILL.md) and, when needed,
[FastMCP](../../.claude/skills/fastmcp/SKILL.md) skills, exact source and consequential targeted probes.
The prior cache investigation's explicitly requested Context7 discovery is retained evidence, not
exact-version authority. Use skills/probes for further DataFusion/Arrow/Delta/FastMCP clarification;
other libraries may use Context7.

Keep DataFusion **55.1.0**, Arrow/Parquet **59.3.0**, object_store **0.13.2**, delta-rs
**58f07cd62bfbce3649a7e1c87c696288068ae184**, Buoyant kernel
**8ba063f8f84fec222000f66d40d70911d7c79675**, Rust **1.98.1**, FastMCP **4.0.3**,
PyArrow **25.0.1**, rustdoc-types **0.61.0** and the recorded patched public-api **0.52.2** renderer.
Revalidate lock/vendor provenance at candidate build; no moving-upstream upgrade is planned.

Keep the **16-core / 32-thread, 192 GiB workstation** defaults: **32 GiB** native pool, **64 GiB**
spill, **2 GiB** metadata cache, **16** native partitions and **16 workers plus 16 blocking threads
per owned compute/I/O lane**. These are ceilings, not preallocations. Preserve distinct parser/
transport bounds; tiny pressure fixtures never become the normal development policy.

### 3.1 Three cache lifetimes

| Surface | Native route and ownership | Restriction |
|---|---|---|
| Operation intermediate | `datafusion::execution::session_state::CacheFactory`, declared logical node and composed execution extension | Four initial families; one operation owner; execution-time fill; disposable after physical work/readers |
| File statistics/listing/metadata | Shared `datafusion_execution::cache::cache_manager::CacheManager` and native `DefaultCache` | Keep cache-specific validity; Delta next scan already consumes metadata cache; do not presume listing/statistics consumers |
| Immutable read-provider descriptors | Restricted `deltalake_core::delta_datafusion::DeltaLogicalCodec` with retained typed descriptors | Exact table/version/cohort/semantic contract; revalidate/reacquire store/runtime/retention; no old effect/cache handles |

None replaces durable job claims, Delta publication, retained results or immutable-definition
capture whose exact value participates in authoritative hash/write consistency.

`CacheFactory::create(LogicalPlan, &SessionState)` synchronously returns a LogicalPlan. It supplies
admission/plan integration, not storage, invalidation, reservation or shared initialization. Do not
copy the reviewed example: P03/P04 prove evaluation in planning/plain EXPLAIN; P10 proves duplicate
concurrent misses. Default `.cache()` is eager MemTable materialization; P09/P12 show tested
unreserved retention and lost key/order annotations.

Native `DefaultCache` supplies LRU, byte limits, TTL, invalidation and inspection. Eviction does
not release values held by readers. File-statistics `SchemaFingerprint` omits top-level semantic
metadata and cannot replace our contract identity. The pinned DeltaLogicalCodec still has TODO
logical-extension hooks and ignores expected schema on provider decode: pre-refuse unsupported
nodes and validate/rebind decoded providers. Exclude DeltaPhysicalCodec and persisted operation
cache state; implement the actual restricted consumer despite these limitations.

### 3.2 Required materialization contract

1. Initially admit only **SearchIndex, ComparisonKeys, OverviewChildren, OverviewNamespaces**.
   Existing declarations own eligibility/output fields. Carry an explicit immutable plan binding:
   operation, exact inputs/versions/cohorts, semantic schema and function/policy/build witnesses.
   Do not infer eligibility from names or ambient mutable globals.
2. Bind/analyze/optimize/lower/EXPLAIN without filling or polling input. Compose with NativePlanner's
   contract, retention, command and Delta planners.
3. Execute one owned initialization per binding, shared by separately prepared physical plans from
   the same cached logical handle. Cancelled waiters do not cancel others; owner cancellation governs
   cleanup. No recursive acquisition of an outer capacity-one permit or separate scheduler.
4. Distinguish unstarted/filling/ready/failed/cancelled. Publish ready only after complete successful
   fill; no partial success, error-as-empty or ungoverned retry. Waiters see the defined terminal state.
5. Preserve binding through equivalent rewrites; rebind/refuse changed meaning. Consumer filters,
   projections and limits cannot shrink the shared base. Schema, order, partitions, constraints and
   statistics are truthful; unknown properties stay unknown.
6. Retain reservations, spill, deadline and exact leases through fill and last physical reader.
   Eviction/dropped awaiter/terminal job state is not physical exit.
7. Express counts/pages as native aggregation/selection over the shared relation. Delete
   `CompletedIndex.rows` and the replaced eager materialization route.
8. Use existing native spill IPC for the first complete backing. Evaluate reservation-backed Arrow
   memory plus native spill overflow as a physical option with conformance/whole-journey evidence.
   No daemon-wide result cache, bespoke LRU or parallel legacy API is introduced.

## 4. Dependency order and integrated packages

CP identifiers assign work, not new acceptance IDs. CP00–CP10 implementation/deletion exits use
unit/static evidence; integrated acceptance stays open until CP12. No intermediate exit requires
a full service journey.

| Package | Implementation prerequisites | Remaining inherited scope |
|---|---|---|
| CP00 authority | Existing checkpoint | FP00 and decision/enforcement portions of FP15 |
| CP01 declarations/values/codecs | CP00 inventory | FP01/FP02/FP04 residuals |
| CP02 planning/providers | CP01 contracts | FP03/FP05 and FP02 storage/kernel boundaries |
| CP03 control/effects/owners | CP01/CP02, existing producer contracts | FP06/FP08, physical ownership |
| CP04 operation materialization | CP01/CP02/CP03 ownership interfaces | Cache review, FP05/FP09/FP13/FP14 |
| CP05 acquisition/producers | CP01–CP03 | FP07, producer/environment effect consumers |
| CP06 research/results/MCP | CP01–CP05 | FP09/FP10, remaining FP04 consumers |
| CP07 retention/reclamation | CP03/CP06, replay contract declared in CP01 | FP12, FP06 protection consumers |
| CP08 CDF/replay/provider cache | CP02/CP03/CP06, CP07 enrollment APIs | FP11, dependency-change consumers |
| CP09 telemetry/resources | Starts CP01, integrates CP03–CP08 | FP13 and cache/native-cache observations |
| CP10 source/package/enforcement deletion | CP00–CP09 implemented | FP15, source L01–L24 plus L25 |
| CP11 matching candidate/retirement | CP10, enforcement installation disposition | FP16 build/retirement and installed removal |
| CP12 final qualification/physical choices | CP11 complete pivot/deletion barrier | FP14, all Q/SC/CF and final acceptance |
| CP13 activation/completion | CP12 qualified current candidate | FP16 activation/smoke and Q13 |

CP03 establishes effect/ownership contracts before CP05 extends actual producer consumers. CP07
establishes protection APIs before CP08 installs replay consumers, then closes its consumer audit
after integration. These are concrete steps, not cycles requiring two whole packages to finish
first. CP09 runs alongside owner changes; deletion proceeds as each replacement lands.

### CP00 — Reconcile authority, inventories and decisions

**Preserve:** the operation/dataflow inventory, common operation declarations and ADR-0047–0051.

1. Complete field/operation owners for evidence, acquisition, results, controls, effects, grants,
   cursors, replay, retention and telemetry. Distinguish semantics from mechanical/external syntax.
   Include every tool/resource/internal/maintenance route and the three cache lifetimes.
2. Consolidate residual policy consumers and join capability facts to exact pin/features, configured
   route/policy and actual conformance. Unsupported, unconfigured, denied and unknown stay distinct.
3. Record the cache lifecycle/eligibility decision and amend living design before implementation.
   Reconcile existing supersession edges rather than reopening already superseded ADR-0040. Allocate
   an ADR from the live index where a binding decision changes, using the cache review as evidence.
4. Reconcile vendor/decision provenance, preserving independent ADR-0046 operator tooling and the
   ty producer boundary. Prepare the current protected enforcement patch against actual files.
   Installation remains operator-applied under repository rules; do not edit protected files.
   Continue independent architecture work while installation is outstanding.

**Exit:** complete owner/route inventory and recorded decisions. **Checks:** document, ADR, provenance
and traceability checks. **Deletion:** obsolete declarations/guidance through the permitted boundary.

### CP01 — Complete declarations, typed values, references and codecs

**Preserve:** native generators, manifest/change relation, native key kernel and bounded JSON encoder.

1. Finish domain/field/reference/collection declarations across control/result/retention/replay/grant/
   acquisition/diagnostic families. Include scoped key pairs, variant guards, cardinality, order,
   duplicate policy and NULL/empty meaning. Remove residual coupled arrays and known flags.
2. Deploy remaining fixed-binary semantic IDs/digests, clocks/units, coordinates including LSP UTF-16,
   exact widths and canonical vectors. Preserve external identifiers in their real source format.
3. Complete dependency witnesses: values/bytes, exact versions/cohorts/contracts, function/transform,
   effective policy, producer/build/environment and encoding. Bind them into cache, cursor, process,
   replay and CDF contracts; a broad build digest or physical schema alone is insufficient.
4. Finish depth/field/metadata/vocabulary bounds and segment-valued field paths; reject duplicate
   fields, unknown extensions and literal-dot/case misinterpretation before recursive processing.
5. Propagate generated exact codecs to worker/results/resources/export/recovery. Preserve full-range
   numeric strings, timestamps, binary/source bytes, union presence and escaped UTF-8 allocation bounds.
6. Remove remaining semantic Arrow→JSON→serde bridges and handwritten field/reference/vocabulary
   authority. Regenerate schemas/Python/wire projections together; transport-only DTOs use generated
   fields. No manual generated edits or historical aliases.

**Unit/static exit:** independent canonical vectors, one real variant/reference/field addition,
exact codec/presence/bounds and dependency-change fixtures. Complete SC01/SC02/SC05–SC08 remain CP12.
**Deletes:** residual L06/L09/L13/L16–L22/L24; do not delete legitimate raw-byte hashes by name alone.

### CP02 — Complete semantic planning, storage and providers

1. Finish JOIN/UNION/CASE/IN/cast/aggregate/nested/output validation before coercion and at derived
   logical/physical boundaries. Preserve valid outer-join widening; refuse erasure/undeclared
   conversions. Implement UDF optimizer hooks only when their claimed relationship is proven.
2. Finish total-Boolean parent/variant requiredness, List/Map child/cardinality/uniqueness and scoped
   reference admission. Trust PK/Unique/statistics only after admission; native anti-joins enforce
   scoped references, not a claimed DataFusion foreign-key constraint API.
3. Complete allowed/refused mutation/metadata routes through public Delta builders with captured
   runtime/store/session, constraints, writer options, transaction keys and ownership. Refuse
   unsupported DML before effects; raw provider INSERT that bypasses checks stays excluded.
4. Complete protocol/feature/schema admission and interrupted constraint installation. Implement
   concurrent-installer/invalid-populated-table coverage for CP12; metadata strings alone prove nothing.
5. Finish immutable inventory, qualified references, exact versions/cohorts, forwarded defaults/
   constraints and accurate pushdown/statistics. Preserve atomic listing/controlled factory; audit
   all four vendor manifests and actual store/session injection consumers.
6. Complete custom extension child/expression replacement, boundedness/eagerness, order/partition/
   cardinality/statistics, cancellation and metric identity, including views and optimized-away scans.
7. Resolve actual paired kernel schema/expression edits or record the real upstream consumer that
   performs them. Do not copy private mapping, comparison or validation internals.

**Unit/static exit:** operator/domain/property truth tables, pure planning refusal and exact field
projection through configured sessions where isolated. Full Q01/Q02/Q04/Q10 and SC03–SC06 stay CP12.
**Deletes:** L03/L14/L17/L19/L20.

### CP03 — Complete control, exact effects and physical owners

1. Move residual request/specification/interest/shutdown/claim/eligibility decisions into native
   command plans. Keep only mechanical handle lookup, wakeups, decoding and process drivers.
2. Complete control scoped references and common-predecessor transaction keys for head/job/interest/
   result/retention/checkpoint/cleanup. Distinct jobs racing one head conflict; reload and recompute
   native preconditions instead of silent policy rebasing after a conflict.
3. Bind command authorization to argv, environment/image, input bytes/modes/inventory, outputs,
   network and budgets together. A matching supplied digest does not prove authorized scope.
4. Finish derived environments and durable warm-LSP ownership across commands, per-request grants,
   method/document/position scope, revocation and actual physical exit. Qualification routes cannot
   become caller-controlled bypasses. CP05 applies these contracts to every producer.
5. Reconcile lost acknowledgements, expired claims, duplicate requests and boot/start identity before
   reuse. Terminal publication is not physical exit. Finish candidate/stage/result obligations before
   bytes become unreachable; extend owned tasks/releases to external/uninstrumented work and cache fill.
6. Replace direct-driver/ignored/ownership-JSON fixtures with target grants and observed native owners.
   Preserve sandbox/process-group/framing/lock mechanics without a second durable journal.

**Unit exit:** exact authorization/decision negatives, retained tokens under abandoned waiters and
capacity-one ownership interfaces. Author independent-process/contained/crash tests for CP12.
**Deletes:** L02/L04/L10/L12/L21. **Final owners:** Q03/Q05/Q09/Q11.

### CP04 — Implement operation-scoped CacheFactory materialization

1. Add eligibility and immutable bindings to existing family/operation declarations. Refuse unknown
   families, effects, unbound mutable heads and unsupported volatile inputs before work.
2. Install one CacheFactory in the QueryRuntime template and preserve it in bound sessions. Return
   a logical materialization node without input polling/I/O. Compose its ExtensionPlanner with the
   existing NativePlanner; never replace contract/retention/command/Delta planner composition.
3. Implement §3.2's execution-owned shared initialization. Independently built physical plans from
   cloned admitted logical handles share one fill. Mutable fill state is not semantic hash input,
   a durable provider descriptor or a persisted runtime handle.
4. Reuse native DiskManager/SpillFile/IPC and bounded reader/writer primitives. Values/readers own
   reservations and operation protection; no unbounded MemTable fallback or global LogicalPlan map.
5. Preserve exact schema and truthful properties through rewrites. Consumer narrowing remains outside
   the shared base. Failure/cancellation never exposes partial ready data, and one abandoned waiter
   cannot revoke other readers. Avoid recursively acquiring the outer execution permit.
6. Switch SearchIndex/ComparisonKeys counts and pages, then OverviewNamespaces/OverviewChildren
   selection/summary readers to one cached handle per admitted intermediate. Native aggregates read
   the same base; view registration cannot accidentally create another binding.
7. Delete `operation_index::materialize`, CompletedIndex and its row-count side channel when replaced.
   Retain only underlying mechanisms used by the target. No optional legacy route remains.
8. Emit typed fill/wait/read/failure/retained/spill observations through CP09. A memory tier must use
   the same contract and ownership; CP12 measures it before final physical selection.

**Unit exit:** CF01–CF06: actual DataFusion on small Arrow fixtures, including plain EXPLAIN, missing
planner, wrong owner/schema, concurrent misses, filter/projection/limit-first reader, capacity-one,
upstream error, cancellation and last-reader release. **Final:** Q01/Q02/Q05/Q06/Q09/Q11.
**Deletes:** L25 and affected L11/L14.

### CP05 — Finish acquisition, producer observations and environment consumers

1. Complete bounded typed source/registry/revision/freshness/receipt families and native preference,
   reachability/fallback/source/archive/header policy. Preserve existing protected captures; cached
   HTTP/source bytes alone are not relational source authority.
2. Apply CP03 contracts to rustdoc fallback, static workers, Cargo/uv, rust-analyzer, ty and runtime
   verification. Bind exact staged/derived environment, lock/installed closure and producer/operation
   across preparation, readiness, claim/grant, request and result usability.
3. Complete LSP/runtime-object raw facts and native lowering of scope, diagnostics, positions,
   outcomes, artifacts and limitations. Keep bounded protocol/log bytes; remove procedural semantic
   JSON transcript interpretation and observation sort/merge.
4. Finish independent Rust/Python binding/definition/visibility/reexport/inheritance/overload/base/
   external-target/source-span/epistemic fidelity, including independent `.py`/`.pyi`. Preserve
   format-61 stability/const/default-body facts and its aligned renderer; no format downgrade.
5. Finish ordered List<Struct> callables/qualifiers and native aggregation. Retain producer-rendered
   signatures alongside structured observations; no display-text inference or runtime-validity claim.
6. Complete cycle/depth/work/large-input/parser/IPC bounds, malformed/truncated stream/worker-exit
   outcomes, explicit partial coverage and owned stage cleanup. Account before large allocations.

**Unit/static exit:** independent fact, callable, normalization and bounded failure fixtures.
Real producer/restart/export and worker→Delta→MCP qualification remain CP12. **Deletes:** L04–L07/L21/L23.

### CP06 — Finish native research, result composition and MCP delivery

1. Complete residual overview/inspection/ambiguity/aspect/evidence-window/recovery decisions as native
   plans. Preserve completed source/version policies; share exact rank factors with explanations,
   deterministic ties and kind/scope normalization.
2. Complete typed before/after field/callable changes, environment conflicts and correctly scoped
   coverage. Preserve unknown/empty/ambiguous outcomes; remove coupled arrays, first-winner choices,
   JSON/string semantic equality and handler assembly of alternatives.
3. Use CP04's handles for four reusable families. Bind all cursors/sections/requests to exact input,
   policy/transform/semantic/codec witnesses; matching field shape cannot grant reuse.
4. Move handler page/result/encoded-fit selection into native plans over complete retained results
   and typed encoded-size facts. Include mandatory envelope/descriptor and real MCP framing cost.
   The sink encodes only; impossible caps yield declared refusal, never semantic trimming.
5. Finish native artifact receipt/section/window/reference authorization, export/recovery closure and
   result admission before terminal success. No arbitrary paths, prefix authority, synthetic recovery
   artifacts or hand-picked JSON outcome reconstruction.
6. Propagate exact generated schema/codec to nine tools, five resource registrations, export and
   recovery. Keep FastMCP 4.0.3 strict bindings/canonical structured content/bounded optional content;
   update product skill and setup guidance without adding semantic policy in Python.

**Unit/static exit:** independent selection/count/page/difference/byte-bound fixtures, regenerated
schema conformance and scoped Python checks. Full Q01/Q06/Q13 and SC07/SC09 remain CP12.
**Deletes:** L08–L11/L13/L21–L23/L25.

### CP07 — Complete retention, explicit removal and native reclamation

1. Finish definition/result/artifact/export/read-only/CDF/replay/query/cache/warm-owner enrollment
   before opening providers/bytes. Capture exact dependencies and maintenance generation through
   optimized-away scans, failed fills and last physical readers; global file locks alone are insufficient.
2. Finish data/log/checkpoint/CDF/replay/transaction-marker horizon policy consumers and native
   cross-horizon admission; persist the effective witness with maintenance and affected operations.
3. Implement explicit published evidence/result root removal, complete native dependency closure
   and orphan/unselected cohort/stage/export-candidate selection from typed obligations. Failures
   do not count as freed space, and root removal cannot free a live physical owner.
4. Connect protected versions to actual owned OPTIMIZE/checkpoint/log-compaction/VACUUM commands,
   native `with_keep_versions`, properties and bounded scans. Preserve files/DVs and log/CDF
   reconstruction; no custom tombstone/log engine and no permanently disabled reclamation.
5. Complete reader/writer/CDF/replay maintenance fencing and unknown-task/commit reconciliation.
   CP08 uses these APIs; close its protection inventory after integration.
6. Replace operator cleanup/reset inventories with control-selected ownership decisions. Ongoing
   reclamation remains separate from CP11's one-time retired installation/state removal.
7. Implement retention/fault/storage durability harnesses without running full journeys before CP12.
   Required filesystem power-loss evidence cannot be replaced by process-kill evidence.

**Unit exit:** independent protection/closure/removal/fencing/error-selection fixtures, including
cache ownership. **Final:** Q03/Q07–Q11. **Deletes:** L01/L02/L04/L12 and cleanup bypasses.

### CP08 — Complete CDF, replay and immutable provider caching

1. Finish native rebuild/incremental eligibility, inclusive version boundaries, pre/postimages,
   mutation/cohort vectors, unpublished candidates and no-op OPTIMIZE. Select exact outputs/offsets
   atomically; missing history/schema/contract means rebuild or explicit refusal, not empty success.
2. Complete output/control failure obligations and unknown/duplicate commit reconciliation.
   Retained command/result evidence still reconciles after application transaction markers expire.
3. Implement actual fresh-process typed-command replay with complete dependencies/output descriptors.
   Rebuild current plans; rebind runtime/store/lease/effect handles under current authority. Revoked
   scope never revives from a receipt.
4. Implement the real restricted DeltaLogicalCodec immutable-read cache consumer. Validate table
   identity/version/full semantic schema/cohort; obtain CP07 protection before use. Refuse unsupported
   hooks before invocation, reconstruct CDF from descriptors, exclude DeltaPhysicalCodec and never
   serialize operation materialization state.
5. Feed contract-change relations into cache/replay/CDF selection for domain/variant/requiredness/
   mapping/canonical/wire/callable/function/policy revisions. Equal physical schemas or file-statistics
   fingerprints cannot authorize reuse.
6. Where repeated descriptor lookup needs bounded in-process retention, use native `DefaultCache`
   with typed keys/size/TTL/invalidation and value-owned resources, not a bespoke LRU. Record the
   actual consumer or an explicit unused disposition/revisit trigger; no second policy registry.

**Unit exit:** descriptor eligibility, changed-witness and wrong-identity/schema/owner/node negatives,
native cache value lifetime. Fresh-process/full-CDF journeys remain CP12. **Final:** Q07/Q08/Q09,
SC08, CF07/CF08. **Deletes:** L09/L12/L14.

### CP09 — Finish telemetry, external accounting and lifecycle coverage

1. Complete Delta builder metrics and attempt/grant/job/publication/maintenance lineage through
   the existing typed events; preserve exhaustive kernel mapping and non-recursive history.
2. Add cache binding/family/state, fill/wait/read/failure/retained/spill observations. Capture useful
   native CacheManager entry/limit facts through bounded typed telemetry; do not assume CLI cache
   table functions are registered or derive semantics from display strings.
3. Finish component/status/recovery policy as native relations; encoded recovery payloads cannot
   hide policy. Atomics remain physical sequencing/admission/drop mechanisms only.
4. Complete preallocation/external accounting for parsers, IPC, canonical kernels, kernel/log replay,
   writer buffers/uploads, encoding, diagnostics, caches and children. Cache occupancy excludes
   evicted-but-live readers; their reservations/leases remain until actual release.
5. Complete queue saturation/fault/drop/conflict/unknown append/restart/concurrent-close handling,
   explicit loss and joined physical/release/diagnostic drain. Metrics are not commit/cleanup authority.
   Event history retention uses CP07.
6. Qualify isolated cancellation boundaries as units and implement the full child/process/LSP/writer/
   cache pressure and shutdown matrix for CP12. Zero counters must correspond to physical exit.

**Unit exit:** typed lineage/state, last-reader accounting, saturation/close and capacity-one cases.
**Final:** Q05/Q09/Q11, CF05/CF09. **Deletes:** L10/L12/L14.

### CP10 — Close source, package and enforcement deletion

1. Close source portions of L01–L24 plus L25 (§7) across production/error/recovery paths, generated
   files, imports/dependencies/features, tests/fixtures, configs/scripts/packages and product skill.
   Never restore deleted code for comparison or an obsolete failing fixture.
2. Complete positive native-boundary rules and independent real extension/policy-change oracles.
   Inspect skill capability-gap findings as syntax leads; no suppressions or blanket allowances.
3. Reconcile DESIGN/ADRs/plan/STATUS/setup/launch guidance and capability dispositions. Install the
   reviewed protected patch through its permitted operator boundary and record actual state; an
   unapplied patch is not installed enforcement.
4. Produce the source/package deletion and implemented-owner inventory for CP11. No unexplained
   semantic owner, alternate cache path or obsolete package artifact remains.

**Exit:** CP00–CP09 implemented, source/package removal complete, final tests authored and affected
unit/static checks passed. Q01/Q12/Q13 integrated acceptance remains CP12/CP13.

### CP11 — Build one matching candidate and retire legacy before integration

1. Implement/register Q01–Q13 recipes with concrete SC/CF selection and receipt capture. `just --list`
   on 2026-09-17 lacks them. Distinguish unit/static selection from integration; no hidden journeys.
2. Build locked matching daemon/native worker/executor/non-editable Python package with one source/
   configuration identity. Inspect package/schema/launch contents without full service journeys.
3. Inventory actual retired service-owned processes/installs/state/artifacts/caches/registrations;
   revalidate exact paths, ownership and dependencies before deletion. Old plan14/v2 paths are leads,
   not blind deletion commands.
4. Stop/drain/reconcile old processes and apply only the verified deletion manifest. Remove obsolete
   task-owned artifacts in scope; preserve frozen/review provenance, shared Cargo/uv caches and user
   repositories. No executable legacy backup, old-state import or compatibility registration.
5. Record the barrier: CP00–CP10 implemented; current candidate built; source/package/enforcement
   and retired install/state/registration removal evidenced; no old processes. Unresolved ownership
   or required operator installation prevents this barrier from closing, not independent earlier work.

**Exit:** complete §6 barrier receipt. This is build/removal evidence, not candidate qualification.
The user accepts downtime; full qualification follows retirement throughout this plan.

### CP12 — Final qualification and native physical selection

1. Execute final Q01–Q12 and complete SC01–SC10/CF01–CF10 matrices with independent expected
   results and source-bound receipts. Run unresolved integrated failures with the matching worker.
2. Qualify installed nine tools/resources, Rust/Python static/contained paths, correct/incorrect
   snippets, offline/cold comparison, retained results, export/readback/restart, Codex/Claude and
   required Linux profiles from unrelated directories on fresh dedicated state. Include C20 and XDG
   leak checks; missing prerequisites are named blocks, not mocked passes or static-only substitutes.
3. Run real conditional-create/commit/file-directory sync and required filesystem power-loss/crash
   proof. A process kill does not close storage durability.
4. Measure complete cold/warm/offline/research/export/execution journeys including planning/control/
   admission/diagnostics: latency distributions, native/external memory, spill/I/O, file/log/history
   growth and cleanup. Record fill/read counts; the old index already computed once, so do not claim
   a nonexistent saved-fill benefit.
5. Select native file/group/partition/statistics/compaction/clustering/Z-order/filter/cache/IPC
   settings from evidence. Separately verify Delta log stats, DataFusion stats, kernel pruning and
   Parquet reads; dotted nested statistics require actual proof. Preserve heavy-column projection
   and strict-missing-file/DV/COUNT/full-scan equivalence.
6. Measure typed ID/clock effects and the optional target memory-backed cache versus native spill.
   Compare supported target configurations or retained source-bound observations; never restore
   legacy production code for a baseline. Without a comparable baseline, report absolute results
   and no speedup claim.
7. Repair target code, regenerate/rebuild and repeat affected qualification for its final identity.
   Record negative/unused capabilities; reverify deletion if repair changes packaging/registration.

**Exit:** current candidate meets all mandatory preactivation oracles. Qualification/packaging is
terminal work, not a new prerequisite before every edit or commit.

### CP13 — Activate fresh state and close completion

1. Activate the qualified matching target on fresh paired state with current generated client
   registrations. Do not revive old state/registrations or retain a rollback executable.
2. Run deployed smoke, verify source/build/config/process/client identities and zero unexplained
   jobs/children/leases/reservations. Recheck Q12's final installation scope.
3. Run Q13 aggregate, acceptance-report/check and final G1–G7 review against matching receipts.
   Update STATUS, plan/index, capability dispositions and product setup from observed evidence.
4. Close only after implementation, every L/Q/SC/CF obligation, required design gates and installed
   acceptance are satisfied. Required `failed`, `blocked` or `not_run` work remains open.

## 5. Cache-specific regression obligations

CF01–CF10 are sub-oracles under existing Q owners, not new product acceptance IDs. All are
**not_run for the target implementation**. P01–P14 remain dated library characterization only.

| ID | Required target oracle | Owner / aggregate |
|---|---|---|
| CF01 admission/session | Factory survives bound-session creation; exact family/owner/inputs admitted; unknown/effectful/unsupported volatile/unbound inputs refuse before work | CP01/CP02/CP04; Q01/Q02/SC04 |
| CF02 preparation purity | No fill/input polls at bind/analyze/optimize/physical-plan/plain EXPLAIN; execution starts work; absent extension planner refuses | CP04; Q05 |
| CF03 shared fill | Separately planned clones/two overlapping readers fill once; cancelled waiter/upstream failure/owner cancellation have defined results; no partial ready entry or capacity-one deadlock | CP03/CP04; Q05/Q11 |
| CF04 transformations | Filter/projection/limit-first reader cannot poison full/count read; exact metadata/nullability/order/partitions and trusted properties through rewrites | CP02/CP04; Q02/Q06/SC04/SC05 |
| CF05 physical lifetime | Reservation/spill/lease survives descendants and last reader/drop/eviction; pressure/quota/corrupt IPC cleanup produces no false release | CP04/CP07/CP09; Q05/Q09/Q11 |
| CF06 four consumers | Independent expected search/comparison/namespace/children counts/pages/summaries; one fill per binding, native aggregates over shared base | CP04/CP06; Q06 |
| CF07 invalidation | Change snapshots/cohorts/domain/variant/requiredness/mapping/canonical/codec/function/policy/build separately; reject wrong-owner reuse and physical-shape-only compatibility | CP01/CP04/CP08; Q04/Q07/Q08/SC08 |
| CF08 cache boundaries | Operation cache vanishes on restart; real restricted provider cache revalidates/rebinds; forbidden codecs/nodes refuse; optional native lookup eviction never grants stale authority | CP07/CP08; Q08/Q09 |
| CF09 observations/cost | Bounded non-recursive typed events; occupancy distinct from live memory; complete-journey performance/options tied to source/workload | CP09/CP12; Q11/SC10 |
| CF10 deletion/composition | Existing native planners preserved; no eager index/count side channel/global map fallback; installed research/resources use target route | CP02/CP04/CP10–CP13; Q01/Q12/Q13 |

Small fixture forms of CF01–CF08 may run before the barrier. Their service, persistence,
fresh-process and installed forms run in CP12. Whole-journey CF09 and installed CF10 are post-barrier.

## 6. Acceptance, receipts and the deletion barrier

All complete final-target Q01–Q13 and SC01–SC10 matrices remain **not_run**. Preserve original
meanings from Plan 17 §6; cache work extends their scope without weakening or replacing them.

| Q / recipe to implement | Complete remaining obligation, including cache additions |
|---|---|
| Q01 `native-contracts-check` | One real declaration/field/aspect/reference/policy change reaches all consumers; cache binding and composed planner included |
| Q02 `delta-mutation-check` | Actual valid/invalid nested/tag/CHECK/null, feature/metadata/global-key/merge routes; effect-free refusal and extension fidelity |
| Q03 `delta-control-check` | Independent-process claim/head/interest/renewal/reservation races, lost acknowledgements, stale/duplicate owners and publication faults |
| Q04 `native-evidence-check` | Full Arrow/value/identity matrices, independent real/deep Rust/Python facts, partial coverage, bounds and complete witnesses |
| Q05 `native-effect-check` | Pure planning, exact authorization/one dispatch/revocation, actual children/writers/warm resources/drop/restart, execution-owned cache fill |
| Q06 `native-research-check` | Native factor/scope/aspect/callable/count/page/result semantics, exact bytes/references and retained completion through actual MCP |
| Q07 `delta-incremental-check` | Full/incremental mutation/cohort/schema/rule/history equivalence, unpublished candidates/OPTIMIZE and output/offset faults |
| Q08 `native-replay-check` | Fresh-process typed replay, real immutable provider cache, semantic invalidation and wrong-schema/identity/unsupported-node/codec refusal |
| Q09 `delta-retention-check` | Query/result/cache/CDF/replay protection, optimized-away scans, enrollment/vacuum races, marker expiry/log reconstruction and actual reclamation |
| Q10 `delta-storage-check` | Backend identity/conditional create, write/commit/storage faults, file/directory synchronization and power-loss proof |
| Q11 `native-resources-check` | Native/external pressure/lifetime, strict files/DV/statistics, bounded correlated telemetry and complete-journey layout/cache measurements |
| Q12 `native-removal-check` | L01–L25 source/package/fixture/config/rule and real installation/process/state/registration closure by boundary |
| Q13 `unified-runtime-qualify` | Final-source quality, installed nine tools/resources/real clients/profiles/offline/export, Q01–Q12 and deployed smoke |

| Original schema oracle | Complete scope retained and cache extension |
|---|---|
| SC01 one declaration | Real Locator variant/aspect/reference added once through all encoders/rules/storage/decoders/results/wire; every union branch |
| SC02 values/identities | Actual typed clock/binary/domain/unsigned/decimal/coordinate round trips; independent canonical NULL/empty/order vectors |
| SC03 requiredness | Real mutation rejection for optional-parent/required-child/variant/List/Map and NULL CHECK truth tables; active features and no effects |
| SC04 semantic plans | Same-domain valid, incompatible operator/erasure refusal through shared session and Delta; add cached logical/optimized/physical/collected fields |
| SC05 exact fields | Declared reorder only; reject missing/renamed/duplicate/wrong-extension before null filling; add cache lowering/replay properties |
| SC06 references/collections | Independent wrong-scope/cohort/producer/environment/guard/duplicate/ordinal/null/empty/cardinality violation sets |
| SC07 generated wire | All typed branches through actual strict MCP, exact full-range values/presence/escaping and actual frame caps; no old default filling |
| SC08 invalidation | Every semantic/mapping/identity/codec/policy/function change independently invalidates cache/cursor/replay/CDF/process use despite same physical shape |
| SC09 callable facts | Real Griffe/rustdoc facts and native inspect/compare differences match independent expectations; unknown coverage remains distinct |
| SC10 layouts | Separately measured log stats/DF stats/kernel pruning/Parquet reads, projection/strict-DV correctness and full-journey cache/layout cost |

Complete Plan 17 §6.1 cases remain binding, not just the summaries above. A direct rule unit does
not establish real analyzer ordering; predicate checks do not establish actual builder rejection;
generated schema equality does not establish real MCP value fidelity.

The CP11 barrier receipt enumerates: completed architecture CP00–CP09, source/package L closures,
protected-patch installation/disposition, matching candidate identities, exact retired deletion
manifest with applied results, stopped/drained old processes and removed old registrations. A dry
run, path list, prepared patch or compile pass alone cannot open the integration phase.

Each execution receipt records exact source/build/pins, command/test/input selection, profile,
bounds, logs/digests and `passed`, `failed`, `blocked` or `not_run`. No inference from counts, library
probes, syntax inventories or stale reports. Final acceptance generation is terminal reporting work,
not a recurring interruption during owner replacement. Repeat tests for changed source/failures or
unresolved evidence, not redundant reassurance.

## 7. Complete deletion ownership

All original L01–L24 behavioral/packaging obligations persist. Rows below name residual closure,
not a claim that already deleted code exists. Include errors/recovery, tests, imports/features,
configs, generated/package outputs and applicable installed consumers in every row.

| ID | Remaining closure | Owner |
|---|---|---|
| L01 | Complete native publication/retention/log/maintenance/cleanup authority | CP03/CP07/CP10 |
| L02 | Residual caller/claim/physical-owner policy and retired ownership-JSON fixtures/readers | CP03/CP10 |
| L03 | All native mutation/admission/resource routes; retain only bounded writer mechanics | CP02/CP05/CP09 |
| L04 | Residual acquisition/environment/result merges, dependency/orphan/publication closure | CP03/CP05/CP06/CP07 |
| L05 | Full producer/LSP native lowering, fidelity and no historical-format compatibility | CP05/CP10 |
| L06 | Remaining semantic JSON bridges/generic row owners and duplicate source inventories | CP01/CP05 |
| L07 | Complete native source/registry/revision/freshness/environment authority | CP05 |
| L08 | Duplicate handler score/kind/scope/token decisions and actual tool semantics | CP06/CP12 |
| L09 | Remaining typed IDs/digests and dependency invalidation; do not recreate removed JSON hashes | CP01/CP06/CP08 |
| L10 | Exact effects/readiness/handler/physical policy under one native authority | CP03/CP05/CP06/CP09 |
| L11 | Residual handler/page/result/recovery composition and semantic trimming | CP06/CP10 |
| L12 | Complete protection/replay/maintenance/telemetry and duplicate resource authority removal | CP07/CP08/CP09 |
| L13 | Generated wire/fixtures/packages/config/product guidance and actual client fidelity | CP01/CP06/CP10–CP13 |
| L14 | Unused imports/features/dependencies/scripts/rules and alternate executables | CP02/CP08/CP10/CP11 |
| L15 | Verified stop/drain and retired install/state/client removal before full integration | CP11; recheck CP13 |
| L16 | All-family tagged/presence declaration coverage and real extension oracle | CP01/CP12 |
| L17 | Duplicate vocabulary/reference/field policy and presentation-role dispatch | CP01/CP02/CP10 |
| L18 | Remaining binary ID/digest/unit inventory; preserve actual external text | CP01/CP12 |
| L19 | All semantic coercion/NULL-fill bypasses across mutation/reopen/operator routes | CP02/CP12 |
| L20 | Control/result/retention scoped references and independent wrong-scope cases | CP01/CP02/CP03/CP07 |
| L21 | Remaining correlated arrays/zips/known flags; correct order and absence | CP01/CP05/CP06 |
| L22 | Mechanical row boundary for every resource/export/MCP path; no lossy defaults | CP01/CP06 |
| L23 | Callable before/after semantics and independent real-tool fidelity | CP05/CP06/CP12 |
| L24 | Current epoch/fixture/rule/package/install audit; no historical runtime backup | CP10/CP11/CP13 |
| L25 — added | Procedural operation materialization/CompletedIndex count channel and replaced consumers/fixtures; no alternate cache fallback or eligibility registry | CP04/CP06/CP10; CF10 |

Retain comparison results as documentation where useful. Deleted runtime code cannot remain
executable to benchmark the target or satisfy old tests.

## 8. Public functionality coverage

The existing operation/dataflow inventory remains the exact route inventory. Every route reaches
target contracts and installed qualification; caching adds no new MCP tool.

| Tool / resource | Remaining target behavior | Owner |
|---|---|---|
| `resolve_library` | Native source/freshness/environment, exact effects, durable progress and complete result | CP03/CP05/CP06 |
| `library_overview` | Native package/context/coverage and shared namespace/children summaries | CP04/CP06 |
| `search_evidence` | Shared score/key relation, native count/page/factors/explanations and witnesses | CP04/CP06 |
| `inspect_symbol` | Exact/fallback/ambiguity/aspect/evidence windows and scoped execution observations | CP05/CP06 |
| `compare_releases` | Typed field/callable differences and side coverage, shared changed-key count/page | CP04/CP05/CP06 |
| `verify_usage` | Exact granted action/environment, real outcome and native usability/result evidence | CP03/CP05/CP06 |
| `read_artifact` | Native receipt/section/window authorization, exact bytes/cursors/encoded caps | CP06 |
| `job_control` | Native interests/state/cancellation, physical cleanup distinct from terminal result | CP03/CP06/CP09 |
| `service_status` | Effective native readiness/policy/history/counters and explicit missing qualification/loss | CP03/CP06/CP09 |
| Five resource registrations; export CLI | Same selected publications/results/codecs; exact manifest changes, retained content and portable dependency closure | CP06/CP07/CP08 |

## 9. Traceability and conditional capabilities

### 9.1 Plan 17 package coverage

Every original numbered action remains binding; source credit in §2 prevents unnecessary rework.

| Original package | Remaining execution owners |
|---|---|
| FP00 authority | CP00/CP10 |
| FP01 declarations | CP01, cache additions CP04 |
| FP02 typed values/identity/storage | CP01/CP02/CP08; CP12 qualification |
| FP03 semantic planning/admission | CP02/CP04 |
| FP04 interchange/wire | CP01/CP06 |
| FP05 providers/mutations | CP02/CP04; CP12 storage/mutation qualification |
| FP06 control/ownership | CP03/CP07 |
| FP07 acquisition/producers | CP05 |
| FP08 effects | CP03/CP05; CP12 real profiles |
| FP09 research | CP04/CP06 |
| FP10 results/MCP | CP06; CP12 real routes |
| FP11 CDF/replay | CP08, CP07 protection |
| FP12 retention/maintenance/durability | CP07; CP12 power-loss/reclamation |
| FP13 telemetry/resources | CP09 plus CP03/CP04; CP12 full lifecycle |
| FP14 physical choices | CP12, declared options CP02/CP04/CP09 |
| FP15 deletion/design | CP00/CP10; CP11 installed deletion |
| FP16 qualification/activation | CP11/CP12/CP13 |

### 9.2 Cache review coverage

Cache findings use prefix CF-F below to distinguish them from the older F01–F14.

| Review obligation | Integrated work / proof |
|---|---|
| CF-F1 planning-time effects | CP04 execution-only node; CF02/Q05 |
| CF-F2 schema/property fidelity | CP01/CP02/CP04; CF01/CF04/SC04/SC05 |
| CF-F3 shared initialization/ownership | CP03/CP04/CP07/CP09; CF03/CF05 |
| CF-F4 dependencies and live memory | CP01/CP04/CP08/CP09; CF05/CF07/CF08 |
| CF-F5 procedural materialization | CP04/CP06 four families/native counts; L25/CF06/CF10 |
| CF-F6 native bounded-cache leverage | CP08 DefaultCache disposition, CP09 native cache observations; CF08/CF09 |
| Review §11 changes 1–7 | CP01/CP02/CP04; CP03/CP04/CP09; CP04/CP06; CP08; CP09/CP12; CP10; CP12/CP13 respectively |
| Unresolved G2–G6 application gates | Schema/admission, pure planning, shared lifecycle and semantic reuse proofs above; final G1–G7 review in CP13; no pass inferred from factory registration |

Older traceability remains complete through Plan 17 §7 and §9.1 above: **F01–F14,
DFU-01–DFU-08, S01–S12 and E01–E09**. Explicit residual routing:

- F01/F03/F14 and DFU-03/DFU-05/DFU-07: CP01/CP02. F02/F04/F07 and DFU-02: CP03/CP07.
  F06: CP05. F08/F10: CP06. F09: CP01/CP08. F11/DFU-04: CP07/CP08.
  F12/DFU-06/DFU-08: CP09/CP12. F05/F13: CP00/CP10–CP13. DFU-01: CP08.
- S01/S02/S03/S08/S10 and E05/E06/E09: CP01/CP02. S04/S05/S07 and E01/E02/E03: CP02.
  S06/S11/E07: CP01/CP06. S09/S12/E04: CP12. Callable E08: CP05/CP06.

All Plan 17 §7.2 integration dispositions persist. Complete public Delta operations inherit private
scan/validation/mapping/merge/metric delegates; do not copy them. Unity/remote catalogs and experimental
stores stay unconfigured without a real source/deployment. Keep the qualified local conditional/
synchronizing store, excluding ConditionalPutShim. No cross-process DataFusion FFI, general schema DSL,
Variant/Union persistence or historical migration layer is added. Current external source formats
remain explicit producer contracts; historical internal formats do not return.

Physical options (nested statistics, partitions, Z-order, dictionary/view arrays, Bloom/page/dynamic
filters and UDF hooks) retain their consumer/equivalence/measurement requirement. Adopting every knob
is not completion. Record adopted and unused dispositions, including descriptor LRU and memory-backed
materialization, with concrete revisit triggers. Those conditional enhancements do not defer the
mandatory factory route, real provider-cache consumer or final physical evaluation.

## 10. Execution ledger and completion rule

**Execution checkpoint — 2026-09-17. CP00–CP13 remain open.** Source implementation below is
credited without closing the broader package or its final application oracle. The execution
baseline preserves pre-existing work in `.dev-state/plan18/execution/baseline.json`; these edits
are not a claim of sole ownership of the shared dirty tree.

Current source identities: state **11**, snapshot **10.0**, wire **5.0**, codec **native-json/3**.
State 11 reflects the new materialization diagnostic contract. Wire 5.0 reflects complete endpoint
response budgeting; no wire 4.0 reader remains. Installation and fresh activation have not run.

| Package | Implemented in this execution | Remaining boundary |
|---|---|---|
| CP00 | Baseline captured; proposed ADR-0052/0053/0054, exact pinned cache/codec/SDK evidence; prior tool-guide provenance mismatch repaired through a new dated seal | Complete owner/field/rule inventory and protected-file disposition |
| CP01 | Checked Page wire/Arrow decoder shares one field declaration; decimal-string pagination rules and three additional schema negatives; generated wire 5.0/framing profile | Remaining identity/digest families, witnesses, declaration bounds and complete storage/operator/codec matrix |
| CP02 | Immutable captured batches distinguished from mutable providers; materialization admission follows views/leases; exact global statistics and ordering facts preserved | Complete semantic operator, mutation, provider-property and feature-route matrix |
| CP03 | Materialization captures actual operation identity/cancellation/deadline and forwards the paid query permit through async/blocking fill ownership | Exact process grants, warm LSP ownership and crash reconciliation |
| CP04 | OperationCacheFactory installed; four declared families use execution-owned single-flight handles, native spill/readback, cancellation and physical metrics; planner composition retained | Complete CF01–CF10 application/pressure proof, four-consumer workloads and ownership edge matrix |
| CP05 | Existing producer foundations retained | Remaining acquisition/runtime/LSP facts, effects, fidelity and worker qualification |
| CP06 | Native delivery byte UDF; DataFusion inline and ranked retained fit; terminal job failure retained; Rust owns tool/error/resource projection; Python trimming/allowance deleted; actual SDK framing profile and parity checks | Handler page/aspect/window/recovery composition, artifact prefix fitting, complete cursor/section witnesses and final all-route qualification; local protocol failures remain distinct |
| CP07 | Complete evidence table vector validated by native anti-join before provider opening; committed exact dependencies carried by durable guard; root namespace identity checked | Exact durable read-only/export enrollment, all remaining consumers, explicit removal, reclaim and maintenance/crash fences |
| CP08 | Restricted DeltaLogicalCodec provider consumer; DefaultCache with typed keys, bounded writer/decoder reservations, TTL/eviction, fresh current protection and validated runtime rebinding; raw mutation excluded | Persisted descriptors, fresh-process command replay, reconciliation and full CDF/dependency invalidation |
| CP09 | Typed materialization events and cache-family status; descriptor memory retained through plans/readers and eviction; owned fill/tasks/releases | Complete external/builder allocation lineage, source/pressure/shutdown/diagnostic matrix and operational consumer coverage |
| CP10 | Deleted CompletedIndex/eager materialization/count channel/custom index IPC paths, obsolete index telemetry, old eager index benchmark and Python semantic preview/fit code | Remaining L01–L24, complete L25 residual inventory, stale typed integration fixtures and package/launch removals |
| CP11 | No retirement/activation attempted | Matching final candidate; verified retired installed/state/client removal barrier |
| CP12 | Focused development units/probes only | Full Q01–Q13, SC01–SC10, CF01–CF10 and performance/durability/client matrices **not_run** |
| CP13 | No activation attempted | Fresh-state activation, deployed smoke and aggregate closure **not_run** |

### 10.1 Development receipts, not product acceptance

All paths below are relative to `.dev-state/plan18/execution/`, dated 2026-09-17.

- `operation-cache-units.log`: 9 units passed; planning purity, source/operation admission,
  cancellation/owned fill, shared execution, ordering/statistics, spill and retained reservations.
- `provider-cache-units.log`: 3 units passed; real isolated native empty-table codec/rebinding,
  identity/root/config/mutation refusal, bounded encoding and eviction with outstanding ownership.
- `retention-vector-units.log`: 1 unit passed; exact URI/id/version/cohort/contract and writer-scope
  negatives. This does not certify read-only consumer enrollment.
- `page-codec-units.log`: 1 unit passed; shared checked decimal wire/Arrow pagination.
- `mcp-native-units.log`: 2 units passed; oversized measurement and untrimmed mandatory recovery.
- `result-measure-units.log`: native Arrow size and inline-boundary unit passed, including tool and
  resource profiles. Inline input is a captured typed Struct, without a reconstructed field-expression tree.
- `mcp-framing-units.log`: 10 Python units passed; actual SDK byte identity and stamp/bound negatives.
- `mcp-native-parity.log`: 144 isolated native/SDK serialization checks passed across tool/resource
  responses, protocol eras and RPC-ID shapes; `fastmcp-framing/native_parity.py` reproduces the probe.
- `affected-clippy.log`: selected production core/store/daemon libraries and binaries passed with
  warnings denied for application crates; two pinned vendor Parquet deprecation warnings remain.
- `schemas-generate.log`: generated schemas/DTOs and conformance passed (4 valid fixtures, 8 negatives).
- `mcp-ruff.log`, `mcp-ty.log`, `adr-lint.log`: selected Python lint/types and 54-record ADR lint passed.
- `provenance-check.log`: all three bundles verified (5 + 15 + 15 entries). The baseline guide
  mismatch was repaired through ADR-0054's recovered exact predecessor and new dated seal; old
  manifests/maps remain unchanged. Product guidance now uses wire 5.0 budgets and decimal strings.

`workspace-targets-check.log` records a failed compile-only all-target check: stale typed-ID,
request/default and removed resource-method test fixtures remain. Production Clippy passes do not
close that fixture work. No full integration tests, real client journeys, candidate activation or
retired installed/state deletion ran. Old acceptance receipts are not promoted to current source.

Next architectural work: remove remaining procedural artifact/page/result fitting and complete
exact retention consumers and durable replay. Continue CP01–CP09 source work with bounded decisive
units; finish CP10 and actual CP11 retirement before entering any full integration suite.

Completion requires implementation, actual deletion, final-source/installed evidence, fresh activation
and updated living guidance. A missing required harness/operator action is reported precisely when
verified, not presumed blocked during planning, and does not stop independent architecture work.

## Outcome (recorded after implementation)

Not complete: this is an active implementation ledger. After CP13,
record what was built, a mistake made and corrected, and deliberate deviations with decision/evidence
links. Until then the implementation and qualification boundaries remain open.
