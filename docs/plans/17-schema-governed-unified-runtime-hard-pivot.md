---
title: Schema-governed unified DataFusion and Delta runtime hard pivot
status: draft
date: 2026-09-16
adrs: [ADR-0041, ADR-0042, ADR-0043, ADR-0044, ADR-0045]
phase: 6
---

# Schema-governed unified DataFusion and Delta runtime hard pivot

## 1. Purpose and authority

This is the combined implementation plan requested on **2026-09-16**. It integrates the entire
remaining scope of [Plan 16](16-unified-datafusion-delta-runtime-completion.md), all S01–S12 findings
and alternatives in the [schema engineering review](../design_review/reviews/design_review_schema-engineering-typed-values_2026-09-16.md),
its [evidence](../design_review/reviews/evidence/schema-engineering-typed-values-2026-09-16/),
and additional schema opportunities found in the current Rust/Python implementation.

**Use this plan for subsequent execution.** It replaces Plan 16's work sequence and carries forward
Plan 15's complete destination: WP00–WP12, F01–F14, DFU-01–DFU-08, L01–L15 and Q01–Q13. The
[unified runtime review](../design_review/reviews/design_review_unified-datafusion-delta-runtime_2026-09-15.md)
and [Delta integration follow-up](../design_review/reviews/design_review_delta-integration-followup_2026-09-15.md)
remain inputs. This is one integrated architecture and deletion plan, not an optional schema addendum.

**Status: proposed implementation; all FP packages open.** This planning pass changed documentation
and added a bounded library probe/evidence only. It did not change product code, dependencies,
installed services or state, and ran no product qualification. New executed library observations
are in the [planning evidence](../design_review/reviews/evidence/combined-schema-runtime-plan-2026-09-16/README.md).
They qualify specific implementation assumptions, not acceptance of the target. Every original
WP remains open at full scope; Q01–Q13 remain `not_run` as final target gates.

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
previously reported 261 porcelain entries, including 76 untracked entries during Plan 16; this
is a dated checkpoint, not a current file count. The schema review and this planning evidence add
new untracked paths. HEAD alone does not identify this implementation. The original execution baseline is
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
| `rustdoc-compat-probe/stability.log` | Probe executed and demonstrated format-61 stable metadata rejected by the 0.59 parser | A confirmed defect to repair in FP07, not a successful producer acceptance result |

The daemon, native worker and executor must be rebuilt together before the next integrated live
journey. A stale built worker cannot certify current decoder/process contracts. Existing installed
Plan 14 state has not been replaced by this source. No deployment action belongs to this planning pass.

## 3. Pinned capability basis

Use the [DataFusion skill](../../.claude/skills/datafusion/SKILL.md) and
[Delta Lake skill](../../.claude/skills/deltalake/SKILL.md), their pinned indexes/prose, and exact
checked-out source. Use the [FastMCP skill](../../.claude/skills/fastmcp/SKILL.md) for 4.0.3
transport generation. **Do not use Context7 for DataFusion, Delta Lake or Arrow.** This plan keeps:

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
through another local store. These are FP05 implementation seams, not reasons to skip the integrations.

### 3.2 Schema findings qualified against the pinned implementation

The schema review's source observations and proposed direction are inputs, not proof that every
suggested API enforces the desired semantics. The new [source and probe ledger](../design_review/reviews/evidence/combined-schema-runtime-plan-2026-09-16/README.md)
records exact pins, code paths, commands and results. The following refinements govern execution.

| Concern | Evidence from this planning pass | Required design consequence |
|---|---|---|
| Extension types | Interface/source-checked: Arrow `ExtensionType` validates the storage datatype/metadata; the current `DFExtensionType` registry customizes formatting, not domain equality or value admission | Adopt a finite extension vocabulary and session registration, plus explicit native value admission and semantic plan validation. Registration alone closes nothing |
| Field propagation | Executed: projection/alias retain extension metadata; same-type CAST and `min(id)` erase it; a cross-domain UNION loses the domain in the logical field but exposes one input's domain in the collected field; cross-domain equality JOIN executes | Validate domains before coercion can erase them, and verify derived logical/physical/result contracts. Do not assume metadata survives operators or blindly restore it after an incompatible transformation |
| Struct casts | Executed: `{b:20,a:10}` cast to `{a,b}` produces `a:10,b:20`; `{a:10,c:30}` cast to `{a,b}` produces `a:10,b:NULL` | Replace `equals_datatype` passthrough with exact semantic-field comparison and admitted name-based projection. Reject unrequested missing/renamed children before native null filling |
| Optional parent with required child | Executed: native nested NOT NULL rejects a present NULL child, but also rejects an absent optional parent whose child is NULL | Derive conditional requiredness from the full parent path. Use native NOT NULL where sound; use nullable storage children plus generated parent-aware predicates where required. Do not restore every nested NOT NULL blindly |
| Delta CHECK activation | Executed: setting `delta.constraints.*` on CreateBuilder with unknown properties permitted leaves writer protocol 2 and does not enforce the predicate. Adding the same constraint through `DeltaTable::add_constraint()` enables enforcement: absent parent accepted, present missing child refused | Install generated constraints through complete native operations, verify active protocol/features and invalid-write refusal before registration. Metadata strings alone are insufficient |
| Nested collections | Source-checked: native scalar NOT NULL handling skips paths through collection elements; separate list/map element validation exists, with limited type/path coverage | Generate native element/child checks for the actual List/Map layouts. Preserve parent validity and ordinals through UNNEST; qualify every used route before assigning trusted metadata |
| UDF struct mapping | Source-checked: `StructFieldMapping` declares output fields equivalent to specific input arguments, enabling optimizer reasoning | Use `return_field_from_args` for parser outputs. Do **not** add `struct_field_mapping` to URL/PEP 440 parsers merely because they return structs: parsed pieces are not the whole input argument |
| File statistics | Source-checked and executed: default indexed-column budget counts top-level non-partition columns, including their nested leaves. Explicit `payload.required,id` emitted only `id` statistics in this fixture | Qualify dotted-column selection before adopting it. Use proven top-level choices or a narrowly qualified upstream fix; distinguish Delta log stats, DataFusion stats, kernel pruning and actual Parquet I/O |

The probe uses metadata-tagged Int64 identifiers to isolate planning behavior, not to select the
target ID storage type. Its tiny tables prove specific semantics, not production performance,
concurrency, complete nullability coverage or MCP fidelity. Final fixed-binary-ID and full-schema
oracles remain mandatory. The original projection measurement is retained only as a dated result.

### 3.3 One semantic declaration, explicit native projections

Use a finite Rust declaration built from native Arrow fields, Rust enums and DataFusion expressions.
Generate its projections mechanically. A second generic schema DSL or expression interpreter is
outside the target. Each field has an owner and, where applicable, a domain, variant/presence rule,
collection semantics, scoped reference, canonical encoding, physical mapping and wire projection.

| Projection / boundary | Authority and responsibility |
|---|---|
| Semantic field contract | Exact names, children, domain/vocabulary, logical requiredness, coordinates and collection meaning; one declaration, not inferred from physical nullability |
| Worker Arrow IPC | Generated producer fields and mechanical value encoding; raw producer facts plus explicit gaps/provenance; input schemas are untrusted until admitted |
| Delta write schema and rules | Derived physical storage types, sound NOT NULL declarations, generated CHECK predicates and aggregate/reference admission; complete native mutation operations |
| Delta read schema | Derived native read layout plus semantic registry identity; exact captured table/version/cohort; known representation adaptation only |
| Admitted query fields | Proven field contracts with domain metadata; derived outer-join nullability is legitimate and does not change stored semantic requirements |
| Canonical value identity | One native typed canonical kernel, explicit semantic version and complete dependency witnesses; schema/collection rules supply framing decisions |
| Results and wire | Native result relations and contract-derived JSON Schema/encoder/Python projections; final formatting only, with exact values and bounded bytes |

Arrow has one extension name/metadata pair per field. Do not stack independent `id`, `digest` and
`ref` extensions. An ID descriptor carries its domain and storage encoding; its reference contract
is a separate generated annotation/declaration with target and scope. `enrichment.role` can remain
a generated display description, but no consumer may parse its prefixes to make a semantic decision.
The IPC semantic registry remains the carrier for read restoration; Parquet/Delta metadata can be
a derived witness, never an independently maintained competing schema.

### 3.4 Target value and collection decisions

| Value | Target semantic/native representation | Storage / wire consequence |
|---|---|---|
| Subject, target, locator, execution target, payload, release metadata and control/command unions | Discriminator plus one nullable Struct per variant; required children interpreted only when that payload is active | Generate exactly-one-active/known-tag/required-child rules, encoders and closed JSON `oneOf` branches from the same variant declaration |
| Event/provenance/observation clocks | `Timestamp(Microsecond, UTC)` with clock meaning declared | Delta timestamp mapping qualified at pin; canonical micros; wire RFC 3339 UTC with declared precision. No incidental timezone/precision truncation |
| Content digests | `FixedSizeBinary(32)` plus algorithm domain for current 256-bit hashes | Delta Binary with length admission and exact read restoration; generated fixed hex wire form. Do not mistake binary storage metadata for length enforcement |
| Service IDs and references | `FixedSizeBinary(32)` plus identity domain; source/foreign IDs remain in their actual external format | Same bytes for key and corresponding reference domain; typed native equality; generated existing domain prefix plus hex at the external boundary; new canonical epoch |
| Source coordinates | Variant-specific line and byte coordinates: explicit base/unit, line `UInt32`, byte `UInt64` where required | Checked unsigned Delta mapping, native range/order checks; no row-dependent meaning for one generic `start` field |
| Correlated repeated records | `List<Struct<...>>` with ordinal and per-record presence when order matters | One ordered record aggregation/decode; no separately aggregated/zipped lists whose lengths/order can diverge |
| Known versus unknown collection | Nullable List: NULL means unknown/unavailable, empty means known empty; structured coverage records explain why | Remove duplicate `*_known` booleans; do not coalesce NULL to empty without a declared semantic operation |
| Ordered sequences / sets / maps | Explicit order, duplicate, element-null and key rules; unordered sets canonicalized only where semantically sets | Built-in native array/sort/distinct/UNNEST operations where applicable; duplicate-invalid input refuses rather than being silently repaired; repeated HTTP headers stay a sequence |
| UInt64 / Decimal at JSON boundary | Exact numeric value retained natively | Use decimal strings for full-range integers/decimals not provably safe as interoperable JSON numbers, or a declared bounded numeric range. Generate schema and encoder together; test beyond 2^53 |
| Large documentation and source payloads | Preserve logical ownership while keeping independently projected heavy columns top-level | Keep the measured narrow-read benefit. Nested Parquet leaf APIs do not mean the public TableProvider projection accepts nested field indices |

No Union/Variant storage, dictionary identity domain, or in-place old-schema adapter is needed for
this closed model. Dictionary/view arrays may be native transient physical optimizations with
qualified restoration. Column mapping and BatchAdapterFactory require an actual future in-place
evolution consumer; a fresh epoch does not need them. These dispositions do not exclude the native
mapping/validation operators already used by Delta internally.

## 4. Dependency order and work packages

Each package below has implementation, deletion and a decisive verification boundary. All are
open. Keep working through architecture after the focused boundary check passes. Broad regression,
installed clients and deployment belong at FP16 once the integrated replacement exists.

| Package | Prerequisites | Deliverable | Prior scope |
|---|---|---|---|
| FP00 | — | Operation/authority inventory, decision and enforcement changes | RP00; WP00 |
| FP01 | FP00 | One finite contract declaration; tagged values, field domains, collections, scoped references | S01/S02/S08/S10; new E05/E06/E09 |
| FP02 | FP01 | Typed values, native storage/read mapping, conditional nullability, canonical identity | RP01; S03/S04/S07; E03 |
| FP03 | FP01, FP02 | Semantic plan validation, full-field UDFs, reference/collection admission | S02/S05/S10; E01/E02/E05/E06 |
| FP04 | FP01–FP03 | Generated worker/result/wire schemas and exact bounded value encoding | S06/S11; E07/E09 |
| FP05 | FP02, FP03 | Complete provider/discovery/mutation routes and feature-aware schema admission | RP02; WP01/WP03 |
| FP06 | FP02, FP03, FP05 | Native control decisions, durable physical ownership and retention enrollment | RP03; WP04/WP07 |
| FP07 | FP02–FP04; FP06 for owned effects | Acquisition/producer facts, structured callable facts and native normalization | RP04; WP05/WP06; E08 |
| FP08 | FP03, FP06, FP07 | Fully bound effects and qualified execution semantics | RP05; WP07 |
| FP09 | FP03, FP07; FP08 for execution aspects | Native research selection and typed comparison | RP06; WP08; E08 |
| FP10 | FP04, FP06, FP09 | Native results/pages, generated nine-tool bindings and bounded delivery | RP07; WP08; S06/S11 |
| FP11 | FP05, FP06, FP09, FP10 | Complete CDF/rebuild and fresh-process replay | RP08; WP09 |
| FP12 | FP06, FP10, FP11 | Unified retention, maintenance and durable reclamation | RP09; WP10 |
| FP13 | Start with FP02; integrate FP06/FP08/FP10/FP12 | Structured telemetry and resource/lifetime accounting | RP10; WP01/WP07/WP10 |
| FP14 | FP05, FP07–FP13 | Measured physical choices on complete journeys | RP11; WP10; S09/S12/E04 |
| FP15 | FP00–FP14 | Source deletion, enforcement and design closure | RP12; WP11 |
| FP16 | FP15; installation deletion after candidate qualification | Final gates, fresh activation and retired runtime removal | RP13; WP12 |

The graph orders contract dependencies, not long-lived scaffolds. Define result, control, effect,
retention and callable fields in FP01; prove the storage/plan/wire path with one real evidence
family, then apply the same machinery to all families. Extend the existing event pipeline alongside
each change. Use the working native normalizers and control paths while replacing their contracts;
do not restart those architectures or ship an old/new switch.

**Implementation cadence:** establish FP00–FP04, then finish the largest semantic owners in
FP06/FP08/FP09/FP10 while integrating FP05/FP07. Finish replay, retention, resources and physical
choices before final deletion/qualification. Run one decisive boundary oracle after each changed
owner; keep advancing architecture once it passes. No package authorizes a generic workflow engine,
a second schema/expression DSL, an alternative executor or a retained compatibility layer.

### FP00 — Reconcile authority, scope and enforcement

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
6. Record decision changes before implementation: semantic contract ownership and generation,
   plan-wide domain checks, conditional physical nullability, typed wire values and the canonical/
   contract epoch. Allocate ADR numbers from the current index; do not assume ADR-0046 is available.
   Reconcile DESIGN/ADR-0040's analyzer deferral, ADR-0037's wire-generation authority and Plan 15's
   storage schema policy explicitly. Preserve Rust ownership and the thin Python boundary.
7. Inventory every schema factory and field-bearing consumer, including results, controls, grants,
   diagnostics, producer observations and raw registry facts. Classify each field as semantic,
   physical mechanism, exact external syntax or final encoding. Resolve each independent duplicate
   through FP01–FP04; do not leave daemon/control fields outside the schema review's original slice.

**Deletion / exit:** one real rule/aspect change has a discoverable authoritative definition and all
consumers. Contradictory active rules are removed through the allowed installation boundary. Frozen
provenance stays byte-identical. Q01/Q12 remain open until the actual extension and removal evidence.

### FP01 — Establish one finite schema and semantic-field declaration

**Starting evidence:** retain the typed Arrow models and native contract registry. Replace their
hand-maintained variant tables, role-prefix interpretation and consumer-specific shape copies.
Primary owners are `core/evidence/arrow_model/{encode,checks,decode,execution,metadata,provenance}.rs`,
`core/{native_identity,native_key,operation,telemetry}.rs` and `store/control.rs`.

**Actions**

1. Make each Rust enum/record declaration expose one finite variant/field table. A derive/macro or
   deterministic generator is appropriate; the enum and a second manually written allowlist are
   not. Derive Arrow fields, tagged checks, decoder dispatch, worker/wire projections and declared
   physical rules from it. Schemars may supply mechanical structure; inferred JSON schema does not
   invent missing ID/reference/presence semantics.
2. Convert SubjectRef, TargetRef, Locator and ExecutionTarget to discriminator plus per-variant
   nullable Struct payloads. Apply the same generator to existing execution/release/control unions
   and command/effect definitions. Shared nullable scalar columns whose meaning changes by tag are
   removed. Keep optional aggregate absence distinct from a present aggregate with unknown fields.
3. Define the finite ID, digest, vocabulary, coordinate and collection descriptors. Implement Arrow
   extension metadata serialization/validation and register corresponding DataFusion extension
   types in the existing session factory. A generated dictionary of accepted vocabulary values
   supplies native predicates and wire enums; no prefix parsing in admission, identities or views.
4. Define reference descriptors with source/target field paths, target fully qualified relation,
   compatible identity domain, scope-key pairs, active-variant/presence guard, null policy and
   cardinality. Bind snapshot/cohort/producer/environment/command generation where required.
   Relation name alone cannot express the existing artifact/worker reference checks.
5. Define collection meaning once: order key, whether duplicates are meaningful/invalid, key/value
   nullability, maximum cardinality where a real bound exists, and absent/empty semantics. Replace
   field-name cases in canonical set handling with the descriptor. Do not convert all lists to sets
   or all key/value sequences to maps.
6. Specify structured callable observations from the actual producer models: ordered parameters,
   parameter kind/name or Rust pattern, supplied type rendering, explicit default presence/text,
   return rendering, overload identity, language flags and supported generic facts. Preserve language
   differences and producer scope. FP07 extracts these; FP09 consumes them in existing inspect/compare.
7. Generate a deterministic contract manifest covering semantic fields, variants, domains, native
   validation/identity function revisions, storage/read mapping and wire rules. Separate cosmetic
   presentation metadata from executable meaning. Bound schema depth, field count, metadata bytes
   and vocabulary/collection growth before recursive processing; reject duplicate field names and
   unknown semantic extensions at the admission boundary.

**Deletion / oracle:** remove literal variant/decode allowlists, role-prefix dispatch and handwritten
parallel declarations. SC01/SC06/SC07 demonstrate a new variant/reference/field added once and reaching
native validation, Delta, read, identity, worker facts, result rows and wire contracts. A temporary
comparison against old tables is a development oracle only; delete it with the old tables.

### FP02 — Finish schema, intrinsic and canonical identity authority

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
8. Implement the typed clocks, digests, domain IDs and per-variant coordinates in §3.4 throughout
   evidence, control, telemetry, retained results, grants and producer contracts. Extend native key
   framing for timestamp and fixed-binary arrays; include domain/version tags and collection meaning.
   Cross-check equality/hash/order for boundary values, repartition and storage round trips. Human
   prefixes and timestamp text belong to declared presentation, never internal comparison keys.
9. Derive semantic, write and read schemas from FP01. Restore native NOT NULL only where correct for
   all legal parent states. For optional structs generate `parent IS NULL OR child IS NOT NULL`,
   extended for variant and nested-parent guards. Generate total-Boolean validity expressions with
   explicit SQL NULL behavior; install CHECK through the native constraint operation and verify its
   protocol feature. For checks not expressible in stored CHECK syntax, require generated native
   pre-admission on every write route and refuse any route that bypasses it.
10. Replace `project()`'s `equals_datatype` decision with exact child names/order/nullability and
    semantic metadata comparison. Allow only declared native name-based reorder/representation
    conversions. Reject missing/renamed/duplicate fields and incompatible metadata before coercion;
    never attach target metadata to silently null-filled data. Keep genuine outer-join widening
    explicit in derived query contracts, separate from storage admission.
11. Produce a native contract-change relation naming field paths and semantic/mapping/codec changes.
    Use its exact contract/version witnesses to invalidate result cursors, CDF projections, replay,
    read caches and process definitions. This is new-epoch invalidation, not a historical conversion
    service. Validate the actual input values before promoting a candidate schema to admitted facts;
    a non-null field label or extension annotation is not evidence of valid data.

**Deletion / oracle:** remove domain-object/JSON canonicalization and duplicated field validators
from production semantic paths. Q04 proves independent field completeness and identities stable
under repartition/reorder/compaction but changed by every meaningful dependency. Q01 proves one
field addition reaches IPC, storage/read, native rules, results and generated contracts.

SC02–SC05/SC08 supply the decisive new typed-value, nullability, cast and invalidation cases. The
planning probe resolves only its named scalar/struct fixtures; it does not close those matrices.

### FP03 — Enforce semantic contracts in native planning and admission

**Starting evidence:** the five product ScalarUDFImpl implementations return datatypes; current
key binding checks arity, and field compatibility chiefly checks role/function metadata. Native
catalogs and admission plans already supply the execution substrate.

**Actions**

1. Implement `return_field_from_args` on the identity, URL and version UDFs using the exact pinned
   `ReturnFieldArgs` contract. Check domain/vocabulary, argument cardinality and literal-dependent
   requirements; return full field metadata and correct nullability. Inspect `ScalarFunctionArgs`
   fields at execution where needed without repeating row semantics in a second implementation.
2. Use `ExprSchemable::to_field`, field metadata APIs, typed literal metadata and metadata-aware
   aliases/casts at authoritative expression construction. Key/reference binding checks domain
   and declared input meaning, not just physical datatype or arity. Metadata supplied by an
   untrusted worker/client cannot authorize a grant or establish a fact's provenance.
3. Add one finite semantic AnalyzerRule and its shared expression/field validator. Validate before
   coercion/optimization can erase domain information; fix rule ordering in the shared session.
   Revalidate derived fields and extension physical/output boundaries. Do not add a parallel query
   IR. If an operator lacks reliable metadata derivation, reject that use or supply an explicit
   domain-preserving native expression with a proven result contract.
4. Cover equality/null-safe equality and comparisons, JOIN keys, IN, CASE/COALESCE, UNION/set
   operations, casts, nested constructors, aggregates and output projection for actual semantic
   field use. ID and reference to the same domain are compatible. Different domains and accidental
   erasure refuse with a typed diagnostic naming field paths/operator/domains. Legitimate explicit
   conversions are declared in the operation contract; casting both sides to plain strings is not
   a route around service admission.
5. Generate scoped reference anti/semi-joins from FP01, preserving producer/cohort/environment and
   variant guards. Replace only rules that are actually reference contracts; log-presence, profile
   agreement and artifact-closure rules remain shared native expressions. Compare complete violation
   sets with independent fixtures, including an ID present only in the wrong scope.
6. Generate element/child/cardinality/uniqueness checks with native functions and scoped UNNEST.
   Preserve a parent row key and ordinal, and distinguish NULL parent, empty collection, NULL item,
   present item with NULL required child, duplicate map key and invalid tag. Qualify the pinned
   list/map functions for the selected representation; do not silently compact invalid data.
7. Use complete trusted constraints/statistics only after aggregate/anti-join admission establishes
   them for the exact immutable table/cohort. Provider schema metadata must not assert PK/FK or
   guaranteed non-null values merely because a candidate declaration requested them.
8. Review the remaining UDF optimizer hooks (`simplify`, `coerce_types`, bounds/preimage, ordering,
   nullability, short-circuit behavior) against real consumers. Implement only sound, independently
   proven relationships. `struct_field_mapping` is not valid for parsed URL/PEP 440 pieces; keep it
   absent there. No blanket hook implementation or invented lexicographic-order claim is required.

**Deletion / oracle:** remove role parsing, duplicate reference SQL and manual per-function semantic
checks outside this authority. SC03/SC04/SC06 and Q01/Q02 prove domains cannot disappear through the
actual registered planning routes, invalid mutations fail before effects, and valid same-domain
plans retain correct outputs. AST rules cover project-owned UDFs/consumers without banning legitimate
mechanical code or imposing unsound hooks on unrelated upstream implementations.

### FP04 — Generate typed interchange and exact wire value encoding

**Starting evidence:** worker IPC generation exists; evidence wire values still become labels and
JsonObject locators, and legacy Symbol filling invents None/0 values. Result callbacks and mutable
Envelope trimming remain FP10's larger consumer replacement.

**Actions**

1. Generate evidence/result JSON Schemas and Python/worker field projections from FP01's semantic
   declaration, with native Arrow fields as the source for shape. Derive branch requiredness from
   presence rules, not the relaxed Delta physical schema. Discriminated variants are closed `oneOf`
   objects with exact tags, required fields and `additionalProperties` policy at every nested level.
2. Preserve typed SubjectRef, TargetRef, Locator, FactSource, ApiPayload and callable facts at the
   wire boundary. Delete legacy Symbol default filling and lossy label/JsonObject substitutions.
   Opaque producer-specific bytes/extensions stay explicit observations with declared interpretation
   limits, not a generic JSON escape hatch for known semantic fields.
3. Implement one field-aware final value encoder using Arrow JSON `EncoderFactory`/default encoder
   delegation where it fits. Its finite extension dispatch derives from the same descriptors:
   prefix+hex IDs, fixed hex digests, exact timestamp formatting, precise UInt64/Decimal encodings,
   binary source payload encoding and declared null handling. Do not duplicate semantic validation
   in Python; validate native values before encoding, then mechanically validate generated transport.
4. Set JSON number/string, null, omission and ordering rules explicitly. Test full-range unsigned
   values, decimals, negative/pre-epoch clocks, invalid UTF-8 source bytes, escaping and nested optional
   aggregates. JSON Schema `format` annotations alone do not guarantee validation; required patterns,
   bounds and native checks must agree. Account for UTF-8 bytes rather than character count.
5. Replace Arrow→JSON→serde semantic decode bridges with direct typed native result consumption or
   mechanical batch encoding. Request/envelope protocol DTOs may remain where they express transport,
   but they reference generated semantic fields and never become another evidence-schema authority.
6. Generate FastMCP 4.0.3 tool `output_schema` and canonical `ToolResult(structured_content=...)`
   bindings through the existing adapter path. Regenerate all nine operation bindings in FP10;
   generated schemas alone do not complete delivery. Keep bounded optional text/resources and strict
   validation consistent with MCP's actual object envelope and installed Python model generation.
7. Apply the same schema/value codec to retained results, resources, export and recovery. Feed encoded
   size observations into FP10's native page selection; no intermediate unbounded duplicate JSON
   document or codec-specific semantic trimming. Include codec revision in identities and replay.

**Deletion / oracle:** SC05/SC07/SC08 and Q01/Q06 test exact round trips, closed union fixtures and one
field change across real worker→Delta→DataFusion→MCP paths. Schema equality against an old handwritten
DTO may aid development but is not the end-state architecture. Delete obsolete models, aliases,
field-list guards and generator inputs as their native replacements land.

### FP05 — Finish provider contracts, discovery and qualified mutations

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
7. Bind FP01–FP04's semantic/write/read/wire manifests into each table and provider definition.
   Verify exact extension metadata on the actual Delta reopen path; update preparation's current
   role/function-only comparison to all semantic keys. Validate CHECK protocol activation, binary
   width, nested conditional requiredness and collection admission on every admitted mutation route.
   Controlled registration refuses a table whose metadata advertises rules its route does not enforce.

**Deletion / oracle:** remove remaining competing preparation/session factories and custom Delta
membership/schema/planner machinery. Q02 checks valid/invalid rows through each route, duplicate
merge matches, global keys, no-side-effect refusal and one shared runtime. Q10 checks backend handle
identity and simultaneous conditional creates. Power-loss and reclamation remain assigned to FP12.

### FP06 — Complete native control decisions and durable ownership

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
   orphan candidate and stage obligations before they can become unreachable; their cleanup is FP12.
8. Apply generated tagged/presence/reference contracts to all fifteen existing and newly added
   control families. Use typed clocks/IDs/coordinates in claims, grants, retention and diagnostics
   as well as evidence. Replace correlated execution qualifier arrays with ordered List<Struct>
   records; family membership and payload validity have one generated rule source.

**Deletion / oracle:** remove remaining procedural job state/eligibility and durable ownership/
reservation readers, while retaining physical guards. Q03 uses independent processes, same-context
races, stale owners, renewal/expiry, multiple interests, lost acknowledgement and each candidate/
control boundary. Q05 proves actual resources stay owned until cleanup, including abandoned awaiters.

### FP07 — Finish acquisition facts, producer fidelity and normalization

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
8. Emit FP01's callable records directly from Griffe and the qualified rustdoc fact model. Preserve
   the producer-rendered signature alongside structured observations with explicit provenance;
   do not parse rendered text back into guessed parameters or claim cross-language type equivalence.
   Cover positional-only/keyword-only/variadic/defaulted parameters, overload identity, Rust patterns,
   receivers/generics and absent information according to what each producer actually supplies.
9. Replace Python overload/base text-and-ordinal parallel arrays, execution qualifier arrays and
   other coupled value lists with one List<Struct> record stream. Order inside native aggregation
   using the declared ordinal/tie key; an upstream sort alone does not guarantee aggregate order.
   Remove duplicate `*_known` flags once NULL/empty/present collection semantics are generated.

**Deletion / oracle:** remove residual semantic acquisition/producer branches, unsupported historical
format fixtures and duplicated source inventories. Q04 uses independent expected facts, real large/
deep Rust and Python packages, conflicts/cycles/missing targets, and the actual worker→Arrow→Delta→
MCP path. Rustdoc fixtures must include stable/unstable and const metadata plus non-null default-body
metadata on functions, associated constants and associated types; unknown/malformed formats refuse.
SC09 additionally verifies independent structured callable facts and their downstream inspect/compare
results. Their presence is evidence about declarations, not proof that `verify_usage` will execute
correctly; runtime verification remains the actual qualified effect path.

Rustdoc follow-up evidence: the completed read-only verifier inspected exact installed 0.59/0.60/
0.61 source and the [format-61 model commit](https://github.com/rust-lang/rustdoc-types/blob/2e63bdc44b94e86fe12d6e0034d1ed9c2ac6e9dc/src/lib.rs),
checked [public-api release metadata](https://crates.io/api/v1/crates/public-api) on 2026-09-16,
and executed the parser probe named in §2.2. `rustdoc-types 0.61.0` is the proposed exact fact-model
target; its adoption and a compatible renderer remain implementation work, not a changed pin.

### FP08 — Finish command-to-effect binding and execution semantics

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

### FP09 — Finish native research selection, comparison and evidence shaping

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
6. Compare callable fields natively for the existing inspection/comparison use cases: parameter
   addition/removal/order/kind/type/default and return changes, scoped to exact overload/producer
   observations. Keep textual render changes distinguishable from structured declaration changes
   and unavailable facts. This adopts the review's deferred signature opportunity because these
   existing consumers already compare signatures; it adds no speculative MCP tool or call-validity
   inference engine.
7. Replace `coverage.rs`/`query.rs` correlated Vec columns and zips with record-valued native query
   output and mechanical row decoding. Distinguish absent, empty, ambiguous and unsupported results
   without invented IDs, zero coordinates, first-winner choices or coalesced unknown lists.

**Deletion / oracle:** delete handler sort/dedup/selection, semantic excerpt/page trimming and duplicate
comparison/request identities. Q06 checks exact/fallback/ambiguous/absent outcomes, independent
aspects, factor/tie equivalence, source support and cold/offline comparison against independent facts.
Inspect native plans as supporting evidence, not as a substitute for semantic results.

### FP10 — Replace result composition and complete generated MCP delivery

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
10. Consume FP04's exact typed codec for every evidence section, result resource and recovery path.
    Record native row/section identity and per-field wire rules in the result declaration. Keep
    domain IDs typed until final output; reference authorization uses the native key, not rendered
    string prefix parsing. Close SC07 through actual tools/resources rather than schema snapshots.

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

### FP11 — Complete CDF, rebuild selection and fresh-process replay

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
8. Consume the generated contract-change relation for domain, variant, requiredness, physical mapping,
   canonical kernel, wire codec and callable fact revisions. Do not accept old cached plans or cursors
   merely because their physical schemas match. CDF unknown-history/schema outcomes must trigger the
   declared native rebuild/refusal; metadata restoration never converts an old semantic epoch.

**Deletion / oracle:** remove procedural incremental/rebuild policy and any generic persisted-plan
assumption. Q07 compares incremental results with full native recomputation across the matrix and
interrupted offset publication. Q08 restarts with no handles, advances underlying heads, tests wrong
schemas/identities/history, unsupported codecs and dependency-by-dependency invalidation.

### FP12 — Implement unified retention and native maintenance

**Starting evidence:** retained providers carry lifetime guards and manual filesystem cleanup exists.
There is no complete control-owned retention horizon/enrollment/maintenance system. The old
[maintenance inventory](../../crates/enrichment-daemon/src/maintenance.rs) still names retired payloads.

**Remaining actions**

1. Define one finite configurable retention contract and native validation of horizon relationships:
   source artifacts, evidence/projection/control/definition/event tables, data/log/checkpoint history,
   active reads/results, CDF resume, command replay and application transaction marker lifetime.
   Persist the effective policy witness with maintenance and affected operations.
2. Compute protected versions/cohorts/artifacts and reclamation candidates by native joins over
   FP06 enrollments and selected references. Include exact policy/process-operation/process-effect
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
   preview/apply behavior. Keep one-time retired-install deletion in FP16 separate from ongoing
   native maintenance; neither may touch user repositories or shared package caches.
8. Complete actual storage durability qualification: simultaneous conditional create, file/directory
   synchronization, failures around data/log commit and directory changes, then the required
   filesystem power-loss/crash oracle. Process kill alone is insufficient. An unavailable harness/
   privilege is `blocked` with the exact prerequisite, and prevents terminal completion.

**Deletion / oracle:** remove old retention metadata/reservation/catalog cleanup authorities and
bespoke incremental file inventories. Q09 proves protection and actual safe reclamation; Q10 proves
the selected backend's durability boundary. No `ConditionalPutShim` or unsafe overwrite compensation.

### FP13 — Finish structured telemetry and resource/lifetime accounting

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
   Retention/compaction of `native_events` follows FP12.
6. Qualify cancellation/abandoned awaiters during decoding, index materialization, Delta writes,
   commit reconciliation, process creation, LSP and cleanup. Counts of zero must correspond to
   actual exited work, released reservations and reconciled durable ownership.

**Deletion / oracle:** remove independent semantic counters/history/recovery policy and redundant
resource authorities. Q11 checks native pool/spill and external allocations under pressure, bounded
diagnostic support and concurrent lineage. Q05 checks physical ownership; diagnostics cannot stand
in for those observations.

### FP14 — Select native physical choices from complete journeys

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
6. Set statistics fields from the contract's actual predicate/join/key/cohort consumers, then verify
   emitted `add.stats`, exposed PruningStatistics, native kernel file pruning and measured Parquet
   reads separately. At the current pin, explicit dotted names omitted nested leaf stats in the
   planning probe; do not deploy that setting as if it worked. Qualify an exact upstream fix if
   needed, or use proven native top-level/default settings with a recorded disposition. An upstream
   disabled nested-DF-statistics test does not prove absence of kernel or Parquet pruning.
7. Measure typed binary IDs/digests and timestamps through filtering, join/hash, canonicalization,
   compression, file statistics, wire encoding and peak allocation. Half the uncompressed digest
   width is not a measured whole-journey speedup. Do not add duplicate hex ID columns merely to
   assume better statistics; require a single generated projection and measured consumer if needed.
8. Preserve independently projected heavy documentation/source fields and compare top-level versus
   nested layouts using actual providers. Consider cohort partitioning and clustering/Z-order only
   against real cardinality, small-file/log growth, maintenance and read cost. Record negative results
   and selected native defaults; no custom nested scanner or duplicated file-statistics authority.

**Exit:** Q11 plus recorded complete-journey measurements, source/options and before/after native
plan evidence justify the selected defaults. Do not claim speedup from the existence of an API.

### FP15 — Close source deletion and architecture

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

### FP16 — Qualify final source, activate fresh state and remove retired runtime

**Remaining actions**

1. Implement/register the Q01–Q13 commands in §6 with concrete test selection and receipt capture;
   these proposed recipes are not present in the inspected `just --list`. Preserve existing gate
   meanings and the four-state evidence model. Run decisive affected checks during FP00–FP15;
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

## 5. Complete deletion obligations

Every L-row remains open until its full source and behavioral closure. Already deleted files must
not be restored for comparison or old tests. Remaining work includes behaviors surviving under new
names, not just the baseline file names.

| Plan 15 ID | Already replaced or removed | Remaining closure / owner |
|---|---|---|
| L01 | `catalog_generation.rs`, pointer/manifest publication replaced | Control retention/log/maintenance and old cleanup consumers; FP06/FP12/FP15 |
| L02 | JSON job journal and old authoritative map replaced | Procedural job validation/interest decisions and durable physical ownership; FP06/FP08 |
| L03 | Ordinary evidence writer, semantic RelationWriter and staging writer removed | Audit residual `dataset.rs`/`record_writer.rs` mechanics, every write route and resource lifetime; FP05/FP07/FP13 |
| L04 | Native contribution, attempts and metadata rebase present | Result publication callbacks, remaining acquisition/environment object merges and complete dependency selection; FP06/FP07/FP10 |
| L05 | Rust and Python semantic walkers removed | Producer fidelity/scale, residual execution/LSP normalization and old fixtures; FP07 |
| L06 | Generated Arrow worker interchange and direct native ingestion present | Execution/runtime semantic JSON interpretation, fact completeness, residual staging resources; FP02/FP07 |
| L07 | Native registry version/marker/dependency selection present | Remaining source/revision/freshness/environment eligibility and durable raw facts; FP07 |
| L08 | Whole-score UDF/tokenizer replaced | Handler factor/kind/token duplication and full semantic oracle; FP09 |
| L09 | Shared evidence/policy/process native identities present | Research/result/cursor/config/schema semantic hashes and dead canonical folds; FP02/FP10 |
| L10 | Native command/routes/process grants present | Complete command-input scope, procedural readiness/selection/job/resource policy and all physical qualifications; FP06/FP08/FP09 |
| L11 | Native retained receipt/section/dependency lookup present | Procedural body/aliases/callbacks/DTO trimming/recovery and result identity; FP10 |
| L12 | Native Delta scans, CDF projections and diagnostic history present | Retention stores, physical reservation journals, old cleanup inventories, component metrics and replay policy; FP11–FP14 |
| L13 | Wire 3.0, research-result/3 and protocol 3 replacements generated | Final generated operation/result/cursor epoch, packaged aliases and obsolete fixtures; FP00/FP10/FP15 |
| L14 | Many alternate writers/readers and dependency exceptions removed | Whole-tree imports/features/scripts/rules/configs/package contents and inactive executables; FP15 |
| L15 | No target installation/deletion claimed | Verified old-root/process/registration manifest, fresh activation and actual removal; FP16 |

The combined schema scope adds these deletion rows. They are mandatory, alongside L01–L15.

| ID | Delete as replacement lands | Replacement / owner |
|---|---|---|
| L16 | Handwritten tagged variant tables, decoder allowlists and mixed per-tag scalar payloads | Generated discriminated per-variant contracts; FP01/FP02 |
| L17 | Semantic `enrichment.role` prefix parsing and field-name-specific domain/set rules | Finite extension/field/collection descriptors and shared native consumers; FP01/FP03 |
| L18 | Internal semantic RFC 3339/hex/prefixed-string IDs and row-dependent coordinate units | Typed timestamps, digests, domain IDs and coordinate variants; FP02; exact external source text remains source evidence |
| L19 | Blanket nullable-write adaptation, datatype-only passthrough and target metadata pasted onto unchecked casts | Derived conditional requiredness, exact field checks and admitted native mapping; FP02/FP05 |
| L20 | Handwritten reference SQL that restates declared reference contracts; duplicated list/known rules | Generated scoped anti-joins and collection admission; FP01/FP03 |
| L21 | Correlated parallel arrays/Vec zips, independently aggregated ordinals and duplicate known flags | Ordered List<Struct> facts/results with declared NULL/empty semantics; FP06/FP07/FP09 |
| L22 | Lossy evidence labels/JsonObject locators, legacy Symbol filling, Arrow→JSON→serde semantic bridges and independent evidence DTO schemas | Generated semantic wire projections and direct native result encoding; FP04/FP10 |
| L23 | String-only semantic signature comparison where producer-native structured facts exist | Structured callable observations and native field comparison; FP07/FP09; preserve rendered producer text as its own observation |
| L24 | Old contract/canonical/wire epoch readers, obsolete generated aliases/fixtures, stale recipe/rule assumptions | One target epoch and regenerated current artifacts; FP15/FP16; no compatibility adapters or historical runtime backup |

## 6. Acceptance implementation and run discipline

These are the original mandatory Q oracles, retained in full. Their final-target status is
**`not_run`**. Focused development receipts in §2.2 support implementation decisions; they do not
close these aggregates. Register recipe implementations as the owning boundary becomes executable.

| ID / recipe to implement | Remaining decisive oracle | Packages |
|---|---|---|
| Q01 `native-contracts-check` | Complete generated field/operation path, real aspect addition and one-policy-change consistency; no duplicate semantic owner | FP00/FP02/FP08/FP10/FP15 |
| Q02 `delta-mutation-check` | Builder/planner/metadata/feature composition; actual valid/invalid nested/tag/CHECK/null routes, duplicate merge/global keys, side-effect-free refusal and pinned inventories | FP02/FP05 |
| Q03 `delta-control-check` | Independent-process claim/head/interest/renewal races, stale owner, lost acknowledgement, fresh duplicate, publication/ownership crash boundaries | FP06/FP08 |
| Q04 `native-evidence-check` | Complete Arrow mappings/identity metamorphism, independent real/deep Rust/Python facts including rustdoc metadata, partial coverage and bounds | FP02/FP07 |
| Q05 `native-effect-check` | No planning effects, one dispatch, revoked/wrong-scope refusal, actual child/writer cleanup, drop/restart/warm-resource ownership | FP06/FP08/FP13 |
| Q06 `native-research-check` | Native factor/scope/aspect/comparison semantics, all result sections/references, exact encoded limits and retained completion through actual MCP | FP09/FP10 |
| Q07 `delta-incremental-check` | Incremental/full equivalence for mutations/unpublished cohorts/OPTIMIZE/schema-rule-history rebuild and output/offset faults | FP11/FP12 |
| Q08 `native-replay-check` | Fresh-process typed replay, qualified immutable provider codec, rejected wrong schema/identity/unsupported routes, exact dependency invalidation | FP11 |
| Q09 `delta-retention-check` | Query/result/version/CDF protection, optimized-away lease, enrollment/vacuum races, marker expiry/log reconstruction and actual reclamation | FP06/FP12 |
| Q10 `delta-storage-check` | Real conditional creates/backend identity, write/commit/storage faults, file/directory synchronization and required power-loss evidence | FP05/FP12 |
| Q11 `native-resources-check` | Native and external resource caps/lifetimes, strict missing files/DV metadata equivalence, bounded correlated telemetry and complete-journey physical measurements | FP05/FP08/FP13/FP14 |
| Q12 `native-removal-check` | L01–L24 source/package/config/rule and installed-path closure, separated into source and installation phases | FP15/FP16 |
| Q13 `unified-runtime-qualify` | Final-source quality, all nine tools/resources, actual installed Rust/Python and Codex/Claude, required profiles/offline/export, deployed smoke and joined Q01–Q12 | FP16 |

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

### 6.1 Schema oracles within the existing acceptance gates

These are focused sub-oracles, not a replacement acceptance registry or new pass-count inflation.
Every SC row is **`not_run` for the target implementation**. The planning probe is evidence for
the current pinned library behavior, including demonstrated hazards, not a passing implementation
of these requirements. Register concrete tests under the named Q owner as each boundary lands.

| ID | Required executable result | Owner / aggregate |
|---|---|---|
| SC01 one declaration | Add a real Locator variant and one evidence aspect/reference once; encoders, all variant rules, Delta checks, decoders, IPC, results and generated wire follow. Enumerate every existing union variant; reject missing/extra/inactive payloads | FP01/FP04; Q01/Q04 |
| SC02 typed values and identities | Round trip timestamps/timezones/precision, binary length/domain, UInt64/Decimal boundaries and coordinate bases through worker→Delta→provider→wire. Independent canonical vectors distinguish NULL/empty/order/domain and remain stable across native physical changes | FP02/FP04; Q04/Q06 |
| SC03 real requiredness | Valid optional parent, required child, inactive variant, List<Struct> child, map element/value and false/NULL CHECK truth tables through every admitted mutation. Verify active constraint feature, actual invalid-write refusal and no data/control side effect | FP02/FP03/FP05; Q02/Q04 |
| SC04 semantic plans | Correct same-domain plans work; incompatible JOIN/UNION/CASE/IN/casts/aggregates and metadata erasure refuse at the appropriate boundary. Assert logical, optimized, physical and collected contracts through the real shared session and current Delta providers | FP03/FP05; Q01/Q02/Q06 |
| SC05 exact fields | Reorder by name only when declared; reject renamed/missing/duplicate child, wrong extension or unsupported cast before null filling. Exercise nested maps/lists, outer-join nullable outputs and restored Delta metadata | FP02/FP05; Q02/Q04 |
| SC06 references and collections | Same ID in wrong cohort/producer/environment refuses; valid key/ref domain works. Verify optional variant guards, duplicate keys, ordered aggregation under repartition, NULL/empty/item-null and real cardinality limits against independent violation sets | FP01/FP03/FP06/FP07/FP09; Q01/Q03/Q04/Q06 |
| SC07 generated wire fidelity | Typed SubjectRef/Locator/payload/callables and all union branches survive actual MCP with strict generated schemas; full-range integers/decimals, Unicode/escaping and null presence remain exact within encoded caps; no legacy default filling | FP04/FP10; Q01/Q06/Q13 |
| SC08 semantic invalidation | Change one domain/variant/requiredness/mapping/identity/codec/policy/function witness at a time; native cache/cursor/replay/CDF/process-definition use invalidates or rebuilds correctly. Fresh epoch refuses old semantic contracts despite same physical shape | FP02/FP08/FP11; Q04/Q07/Q08 |
| SC09 callable observations | Real Griffe/rustdoc structured parameter/overload/return facts match independent expectations; inspect/compare exposes native field changes and unknown coverage separately from rendered-text changes. No fabricated cross-language/runtime guarantees | FP07/FP09; Q04/Q06/Q13 |
| SC10 schema/layout effectiveness | Record Delta log stats, DF stats, kernel pruning, file/byte I/O and full-journey memory/latency separately for adopted physical choices. Preserve documentation projection and strict/DV correctness; publish workload and negative results | FP14; Q11 |

For SC04, verify rule ordering with the real SessionStateBuilder composition; a unit test calling
the rule directly before coercion is insufficient. For SC03, metadata-only assertion and a
predicate unit test are insufficient: the invalid row must reach the actual admitted builder
route and be refused. For SC07, generated schema conformance is necessary but actual client values
and encoded frame limits are the terminal oracle.

## 7. Review and integration traceability

### 7.1 Review findings

| Finding | Remaining packages | Closure |
|---|---|---|
| F01 validated mutations | FP02/FP05/FP12 | Q02/Q10; L03 |
| F02 claims and publication | FP06/FP10–FP12 | Q03/Q05/Q08–Q10; L01/L02/L04 |
| F03 semantic mapping/trusted metadata | FP02/FP05/FP07 | Q02/Q04; L06/L09 |
| F04 owned effects | FP06/FP08/FP13 | Q03/Q05/Q11; L10 |
| F05 obstructing rules | FP00/FP15 | Q01/Q12; L13/L14 |
| F06 Arrow-first normalization | FP07 | Q04/Q13; L05–L07 |
| F07 complete operation policy | FP00/FP06/FP08–FP10 | Q01/Q05/Q06; L10 |
| F08 native scoring | FP09 | Q06; L08 |
| F09 native identities | FP02/FP10–FP12 | Q04/Q08/Q09; L09 |
| F10 native result composition | FP10 | Q06/Q13; L11/L13 |
| F11 Delta persistence/maintenance | FP05/FP06/FP12/FP15 | Q02/Q03/Q09/Q12; L01–L04/L12 |
| F12 resources and lifetime | FP05/FP06/FP08/FP13 | Q05/Q10/Q11 |
| F13 route-specific capability claims | FP00/FP05/FP11/FP15/FP16 | All Q receipts and dispositions below |
| F14 planner/configuration | FP05/FP13 | Q02/Q05/Q11 |
| DFU-01 codec/replay | FP11 | Q08 |
| DFU-02 retention/conditional storage | FP06/FP12 | Q03/Q09/Q10 |
| DFU-03 complete builders/private delegates | FP05 | Q02/Q12 |
| DFU-04 incremental CDF | FP11/FP12 | Q07/Q09 |
| DFU-05 discovery versus publication | FP05 | Q02/Q03 |
| DFU-06 native scans/kernel integration | FP05/FP13/FP14 | Q02/Q11 |
| DFU-07 paired kernel edits | FP02/FP05 | Q04 |
| DFU-08 structured telemetry | FP10/FP13 | Q06/Q11 |

### 7.2 Complete integration dispositions

This carries forward every family in Plan 15 §8. No optional integration exemption permits keeping
procedural authority for a required local service operation.

| Integration family | Remaining disposition / evidence owner |
|---|---|
| Delta TableProvider / CDF provider | Existing production consumers; finish exact schema/vector, mutation and retention evidence; FP05/FP11/FP12 |
| DeltaTableFactory / ListingSchemaProvider | Implement bounded controlled discovery/preparation, capture/admit then immutable registration; FP05 |
| Unity schema/catalog/list | No configured source identified in this review; retain explicit unconfigured disposition. If an actual enabled source is found, qualify upstream adapter credentials/list failures/version capture; FP00/FP05 |
| DeltaPlanner / DeltaExtensionPlanner | Existing composed planner; finish complete mutation/resource routes while inheriting private delegates; FP05/FP13 |
| Eight ExecutionPlan / nine DisplayAs implementations | Inherit current scans/validation/mapping/merge/metrics through public operations; no retired physical wrapper or display authority; FP05/FP14 |
| PruningStatistics | Qualify native exactness/DV/filter/COUNT behavior and measured consumers; FP14 |
| DeltaLogicalCodec | Add qualified immutable read cache with full revalidation/rebinding; FP11 |
| DeltaPhysicalCodec | Excluded; generate fresh current physical plans and test refusal; FP11/FP15 |
| DataValidation / MergeBarrier / MergeValidation / MetricObserver logical nodes | Inherit complete public builders, never copy/register private delegates separately; FP05 |
| MakeParquetArray / ToJson / ZOrderUDF | Inherit supported parent operations; no private imports, JSON semantic identity or Z-order scoring; FP02/FP05/FP14 |
| DeltaDataWriterExt / datafile writers | Complete logical-input builder remains default. Qualify upstream preparation only if a required physical-input consumer exists; FP05 |
| DataFusionEngine / kernel Engine | Existing native scan integration; finish default-engine/log resource accounting; FP13 |
| ObjectStore implementations | Mandatory qualified local conditional/synchronizing path; ConditionalPutShim excluded. S3/experimental DeltaIO require actual deployment and separate evidence; FP05/FP12 |
| ParquetObjectReader / FileStream | Native ranges/footer/pages/finite streams and explicit missing/corrupt input outcomes; FP07/FP14 |
| Kernel SchemaComparison | Private; inherit mapping/validation, qualify actual service semantic schema rather than copy internals; FP02/FP05 |
| Kernel RetentionCalculator | Private; inherit checkpoint/log/property behavior with service-derived protected inputs; FP12 |
| ExpressionItem / SchemaPatchItem / paired patch builder | Actual paired kernel edit or evidenced upstream delegation at the real boundary; FP02 |
| Serde Deserialize / Serialize | External format/protocol and final mechanical encoding only; no semantic hash or runtime-handle replay; FP02/FP07/FP10/FP11 |
| TableFeature EnumCount / IntoEnumIterator | Derived inventory joined to protocol/policy/route receipts; FP00/FP05/FP15 |
| TableMetadataUpdate Validate / ValidateArgs | Reuse for native metadata command inputs, not generic row rules; FP05 |
| ReportGeneratorLayer / MetricsReporter | Integrate into the existing bounded Arrow/Delta event path; FP13 |
| Broader provider-map source/sink/adaptation/defaults/pushdown APIs | Complete actual contracts at native layers; async remote catalogs, UDTFs and statistics registries only for a concrete consumer; no duplicate scheduler; FP02/FP05/FP07/FP14 |
| DataFusion FFI | No cross-process transfer; generated Arrow IPC remains worker transport. No invented plugin requirement; FP07/FP15 |

### 7.3 Complete schema-review coverage

All S01–S12 are accounted for. Refined API choices preserve the requested outcome while correcting
assumptions using exact source and executed probes. The full Plan 16 RP mapping is in §4; its
architecture, removal, qualification and activation actions are carried into the FP packages.

| Review finding / principles | Integrated decision | Packages / oracle |
|---|---|---|
| S01 tagged union authority — DM-02/DM-06/DM-17 | Generate all variant tables and per-variant Struct payloads from one declaration; apply to existing unions too | FP01/FP02; SC01/SC03; L16 |
| S02 finite semantic types — DM-06/DM-09/DM-16 | Arrow extensions plus DF registry, one descriptor per field and explicit semantic enforcement; stop role parsing | FP01/FP03; SC01/SC04/SC06; L17 |
| S03 typed scalars — DM-06/DM-24/DM-42 | Typed UTC clocks, fixed binary digests/domain IDs and per-variant coordinates; native canonical and exact wire rules | FP02/FP04; SC02/SC07/SC08; L18 |
| S04 storage guarantees — DM-07/DM-43 | Derive safe physical requiredness plus parent-aware native checks; install actual Delta CHECK feature and qualify collection paths | FP02/FP03/FP05; SC03; L19 |
| S05 full-field planning — DM-22/DM-24/DM-26 | Full-field UDFs, role-aware binding, one semantic rule plus boundary validation; use only truthful optimizer hooks | FP03; SC04; L17 |
| S06 wire source and fidelity — DM-52/DM-41 | Generate typed evidence/section schemas and encoding; delete legacy Symbol fill and label/JsonObject loss | FP04/FP10; SC01/SC07; L22 |
| S07 field identity through casts — DM-24/DM-42 | Exact field compatibility; declared name-keyed reorder; refuse silent missing-child fill | FP02/FP05; SC05; L19 |
| S08 repeated/present values — DM-06/DM-17 | Correlated List<Struct>, ordered aggregation, NULL versus empty; remove zipped arrays and known flags | FP01/FP06/FP07/FP09; SC06; L21 |
| S09 selected statistics — DM-36/DM-39 | Workload-derived effective fields; qualify dotted names and each native pruning layer before adopting; measure partition/Z-order choices | FP14; SC10; Q11 |
| S10 derived references — DM-02/DM-16 | Generate anti-joins with full domain/scope/variant/null contract; preserve distinct non-reference predicates | FP01/FP03; SC06; L20 |
| S11 mechanical row boundary — DM-41/DM-52 | Delete semantic Arrow→JSON→serde bridges; direct generated result encoding, with protocol-only DTOs where needed | FP04/FP10; SC07; L22 |
| S12 heavy-column projection — DM-36/DM-39 | Preserve top-level independently projected docs/source payloads; validate full-provider bytes/read behavior before layout changes | FP14; SC10 |

### 7.4 Additional findings and review refinements

E IDs belong to this planning pass. They distinguish new consumer opportunities from qualifications
of S findings; none claims that proposed implementation has already landed.

| ID / basis | Change beyond the review's specified implementation | Code/evidence anchor | Closure |
|---|---|---|---|
| E01 executed + source | Protect semantic domains before coercion and validate optimized/physical/output contracts, including cross-domain UNION and CAST erasure; equality-only validation is insufficient | `store/preparation.rs`, `store/provider.rs`; planning probe `PLAN` rows | FP03/FP05; SC04 |
| E02 source | Adopt full-field UDFs without falsely declaring parsed outputs equal to input arguments; qualify optimizer hooks individually | `core/native_url.rs`, `core/native_version.rs`; `ScalarUDFImpl::struct_field_mapping` | FP03; SC04 |
| E03 executed + source | Parent-aware requiredness, collection-child gaps and mandatory active Delta CHECK feature; storing a rule is not enabling it | `store/native_delta.rs`; probe `NULLABILITY` cases and constraint protocol actions | FP02/FP03/FP05; SC03 |
| E04 executed + source | Correct top-level statistics budget interpretation and dotted-selection behavior; separately measure log/DF/kernel/Parquet consumers and new binary types | Delta `writer/stats.rs`; probe `delta-actions.json` | FP14; SC10 |
| E05 source | References include composite scope, target field/domain, active-variant guard and cardinality; one relation annotation cannot generate current semantics | `store/admission.rs` producer-bound worker/artifact rules; control/grant scopes | FP01/FP03; SC06 |
| E06 source | Put order/uniqueness/map/null/cardinality meaning in the contract; generate canonical/admission/aggregation behavior; explicitly cover required children inside repeated records | `core/native_key.rs`, `store/coverage.rs`, `store/query.rs`, `store/python_normalize.rs` | FP01/FP03/FP07/FP09; SC06 |
| E07 source/interface | Field-aware native JSON codec for binary IDs/digests, clocks and exact large integers/decimals, with generated validation and actual encoded byte bounds | Arrow JSON `EncoderFactory`; `core/wire/evidence.rs`; `store/projection/render.rs` | FP04/FP10; SC02/SC07 |
| E08 source; promoted review alternative | Producer-native callable records consumed by existing inspection/comparison; preserve rendered signatures and provenance, avoid text parsing or runtime inference | Python worker Griffe extraction, Rust worker ItemFact/Signature, `store/comparison.rs` | FP01/FP07/FP09; SC09 |
| E09 source; new cross-cutting contract work | Deterministic semantic/mapping/codec manifest, native change relation, candidate/admitted/derived schema distinctions, bounded metadata and field-path validation | `store/native_delta.rs`, `store/preparation.rs`, `core/native_key.rs`, replay/result contracts | FP01/FP02/FP11; SC05/SC08 |

### 7.5 Alternative and conditional capability dispositions

- The review's simpler equality-test/manual-codec first slice is useful only as a temporary
  development comparison. The delivered design removes those duplicates and ships one epoch.
- Structured callable facts are adopted for existing inspect/compare consumers. Producer-specific
  type rendering remains evidence, and native runtime execution remains necessary for verify_usage.
- Arrow Union and kernel Variant are not selected for the finite persisted domain. No generic JSON
  escape hatch, domain DSL or column-mapping migration layer is introduced. External extensible raw
  evidence remains explicitly typed by producer/format contract with bounded bytes and coverage.
- Delta field metadata may carry generated witnesses if an actual consumer needs them. It does not
  become a second semantic registry. Parquet metadata settings and read restoration are qualified
  on the real Delta provider path, not inferred from a standalone Arrow file round trip.
- Nested stats fixes, partitions, Z-order, dictionaries/views, Bloom/page/dynamic filters and UDF
  optimizer hooks need actual consumers and correct semantics. Physical selection remains mandatory
  work, but adopting every knob is not a completion criterion. Record each disposition and evidence.
- Actual current external rustdoc formats remain explicit producer contracts. Historical internal
  epochs, format downgrades, wire aliases, imported old state and rollback executables remain deleted.

## 8. Resume instructions and completion record

1. Read this plan and its evidence, the current source baseline, living design and ADRs. Preserve
   unfinished target work and unrelated operator edits. Completed normalizers, scoring, dependency
   selection, CDF consumers, process readback and diagnostic history are the substrate to extend.
2. Start FP00's authority/decision inventory and FP01's finite schema declaration. Implement
   FP02–FP04's typed storage, semantic checks and generated encoding on a real evidence path,
   covering optional variants, binary IDs, scoped references and a typed evidence result end to end.
3. Finish control/effect scope, producer fidelity and native research/result composition using those
   contracts. Fix the demonstrated rustdoc mismatch before claiming the real Rust producer works.
4. Record progress by FP ID with changed/deleted paths, source receipt, focused command/log and
   remaining boundary. S/E matrix rows close only with their named behavioral oracles. Preserve all
   original Q requirements and deletion obligations; do not infer completion from generated files.
5. Finish replay, retention, maintenance, resources and source deletion before final installed
   qualification. Protected rule installation and power-loss/client prerequisites stay explicit;
   they do not block independent architecture work or justify narrowing mandatory scope.
6. Do not roll out a schema-only intermediate epoch. Activate the fully integrated target once,
   with fresh state, new canonical/contract/wire identities and the scoped retired-runtime deletion.

## Outcome (recorded after implementation)

Not implemented by this planning pass. Record what was built, corrected mistakes, deliberate
changes and final-source acceptance evidence here as execution completes. This plan is complete
only when all implementation, deletion, qualification and activation obligations are satisfied.
Creating this document and executing library probes closes no runtime work package.
