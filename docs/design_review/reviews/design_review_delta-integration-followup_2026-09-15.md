# Design review follow-up: reuse the complete Delta integration surface

## 1. Decision and scope

**Decision: refine the selected unified DataFusion + Delta architecture.** The new capability inventory supports more reuse than the first review made concrete: complete Delta operation builders, a queryable change-data feed, metadata-only scans, snapshot-provider serialization, paired schema/expression patches, and structured kernel telemetry. These refinements reduce independent application implementations while preserving the original control-table publication protocol and useful MCP behavior.

**Baseline:** [unified execution review](design_review_unified-datafusion-delta-runtime_2026-09-15.md), especially §§4.3–4.7. **Reviewer:** Codex. **Date:** 2026-09-15. **Status:** recommendations Proposed; source claims Interface-checked; the bounded CDF/codec experiment is Tested. This is a focused follow-up, not a new implementation plan or product acceptance campaign.

### Method and coverage

Used the [deltalake skill](../../../.claude/skills/deltalake/SKILL.md), its operation/feature/foreign-implementation catalogs and API index, the [DataFusion skill](../../../.claude/skills/datafusion/SKILL.md), and exact source. Context7 returned a discovery result before the user's clarification; it is **not used as evidence for this follow-up**. No further Context7 calls followed that clarification.

Versions are delta-rs `58f07cd62bfbce3649a7e1c87c696288068ae184`, its selected Buoyant kernel `8ba063f8f84fec222000f66d40d70911d7c79675`, DataFusion 55.1.0, Arrow/Parquet 59.3.0 and object_store 0.13.2. These are the current pair verified in the preceding review and the new skill's pins. The isolated probe preserves both git revisions in Cargo.lock.

The scope covers the supplied 26 foreign-trait families plus the omitted logical-node/UDF families and internal implementations. A generated public catalog is an entry point, not a count of every implementation: private modules, generic impls and the indexed-crate boundary explain omissions. Source inspection also found the useful `DataFusionEngine` integration. No claim of an exhaustive new census of every transitive trait is made.

Inspected provider/factory/catalog behavior, planner dispatch, codec methods and serde fields, CDF scan ordering, native validation/merge integration, metadata scans, kernel/schema/retention boundaries, storage wrappers and telemetry. The experiment exercises CDF and codec behavior only. Remote Unity/S3/OpenDAL operation, full merge/schema evolution, crash recovery, vacuum races and installed MCP clients were not run.

**Source notation:** `D:` paths are relative to [delta-rs `crates/` at the checked commit](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184/crates); `K:` paths are relative to [the selected kernel checkout](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675). Line numbers identify actual inspected code, not inferred behavior from names.

## 2. Authority and lifecycle map

| Concept | Authoritative representation | Native consumer | Required boundary |
|---|---|---|---|
| Published evidence | Existing control-table vector of table identity/version/cohort/contract revision | Captured DeltaScan providers and native views | Catalog discovery and serialized provider bytes cannot advance publication |
| Mutations | Typed command, native logical input/Expr, bound policy and predecessor | Delta Write/Update/Delete/Merge builders and DeltaExtensionPlanner | Application checks command eligibility; Delta owns its row, file, merge and commit mechanics |
| Incremental projections | Derived relation plus source-table identity and last processed version | DeltaCdfTableProvider, native joins/aggregates and Delta writes | Projector output/version checkpoint selected atomically through the control table |
| Replay | Durable operation identity, inputs, publication vector and extension revisions | Reconstruct native plans; optionally decode qualified read providers | Runtime handles and authority are rebound; physical plan bytes are not the job record |
| Storage rules | Versioned policy projected into native table properties and object-store configuration | Delta/kernel transactions, checkpoints, compaction and vacuum | Retention of files, log history, transaction markers and application results must agree |
| Diagnostics | Native operation metrics, execution metrics and kernel MetricEvent observations | One bounded Arrow diagnostic relation feeding MCP | Metrics explain work; the control transaction proves publication |

## 3. Semantic contracts and invariants

| Contract | Enforcement | Failure behavior / oracle |
|---|---|---|
| Exact read binding | Validate identity, version, expected Arrow contract and cohort before registration or after decoding | Wrong schema/version rejected; codec's ignored `_schema` argument is not validation |
| Complete native mutation | Public operation builders construct their native validation, mapping, merge and metric nodes | Same invalid input fails every advertised command route; raw INSERT finding from the parent remains open |
| CDF is a bounded derived input | Explicit inclusive start/end versions; source table identity; change-kind-aware projection | Missing history/schema incompatibility yields unavailable/rebuild-required, never a successful empty change set |
| A CDF change is not a semantic publication | Join changes to admitted publication/cohort records | Unpublished candidate data and physical-only maintenance cannot appear as new code insights |
| Replays outlive transaction-marker expiry | Durable command/result records plus declared transaction-marker and log retention | Expired marker does not authorize a second execution; checkpoint/retry oracle |
| Concurrent control commits need actual conditional creation | Qualified backend PutMode::Create / log-store commit operation | Racy HEAD-then-PUT wrappers excluded from concurrent claim/publication storage |
| Read accelerators preserve meaning | Let Delta choose metadata/full scans, file selection and deletion-vector handling | COUNT, projection/filter/limit and maintenance equivalence; file identity never replaces semantic identity |

## 4. Derivation and execution design

### 4.1 Concrete improvements to the target

1. **Use the complete logical-input mutation route as the default.** Feed native plans/expressions to `DeltaTable` operation builders and install one `DeltaExtensionPlanner` under the existing retention planner. Its dispatch already includes write/delete/update/merge metrics and data validation. Do not recreate those internal nodes or their planner registry in the service. `D:core/src/delta_datafusion/planner.rs:45,98`; `D:core/src/operations/merge/mod.rs:255,425,716`.

   `DataValidationExec`, its table-derived predicate helper and several merge nodes are crate-private or hidden behind private modules. Use them through their operation builders. `DeltaDataWriterExt::write_plan` remains the existing lower-level physical-input writer, but it expects prepared input and returns uncommitted Add actions. A service physical-DML adapter must not copy Delta's validation/CDC/column-mapping interpretation just to bypass the complete builder. Expose an upstream composition seam if that route is required. The deceptively named deprecated `with_input_execution_plan` takes **Arc<LogicalPlan>**, not a physical ExecutionPlan (`D:core/src/operations/write/mod.rs:251`).

2. **Make CDF a first-class derived relation.** Construct `deltalake_core::delta_datafusion::cdf::scan::DeltaCdfTableProvider` using `table.scan_cdf().with_starting_version(a).with_ending_version(b)`. Register it beside the pinned evidence tables. Its scan combines partition pruning with an actual FilterExec, then projection and limit (`D:core/src/delta_datafusion/cdf/scan.rs:65`). Prefer this public provider to extracting batches from a low-level builder and reconstructing filters manually.

   Enable CDF before needed changes occur. Use it for incremental search-factor/materialized-result updates, change explanations and projection maintenance. It does not replace the append-only control facts or arbitrary semantic comparison between independently sourced releases. Persist the input version offset with the selected derived output; use versions, not timestamps, for resume. `_commit_version` is UInt64 in this provider, whereas several Delta APIs use signed versions; the boundary needs checked conversion. Include pre/postimage meaning, schema epoch and retained-history availability in the projection contract.

3. **Use native metadata scans and snapshot-aware file selection.** `DeltaScanMetaExec` synthesizes supported results from file metadata; its effective counts account for selection vectors. DeltaScan/DeltaScanExec handle actual row reads and rewrites. Configure native data-skipping statistics for common cohort/release/symbol filters and let these implementations choose the path. No second file-count index or service-level COUNT shortcut is needed. `D:core/src/delta_datafusion/table_provider/next/scan/exec_meta.rs:142`; `D:core/src/kernel/snapshot/log_data.rs:281`.

   For selective reprocessing, use DeltaScanNext's `with_file_selection`/`with_file_paths`: it resolves membership against the captured snapshot and retains Delta transforms and deletion vectors. Keep strict missing-file behavior. A selected Add supplies a path; snapshot metadata remains authoritative. File IDs can support physical provenance/maintenance, while public semantic evidence IDs survive compaction. `D:core/src/delta_datafusion/table_provider/next/mod.rs:556,569`.

4. **Split durable replay from optional provider serialization.** `deltalake_core::delta_datafusion::DeltaLogicalCodec` supports the DeltaScanNext table-provider payload; it does not support arbitrary Delta logical extension nodes, and its generic extension methods contain `todo!`. The decoded provider preserves the snapshot but loses `log_store`, operation ID and file-skipping Expr hints, all skipped by serde. Its decode method ignores the caller's table reference/schema/context. Bind and validate those separately; preserve required predicates as actual plan/view filters. `D:core/src/delta_datafusion/mod.rs:496`; `D:core/src/delta_datafusion/table_provider/next/mod.rs:487`.

   `DeltaPhysicalCodec` is deprecated and supports only the retired physical DeltaScan wrapper, not the current DeltaScanExec or complete mutation/effect plans (`D:core/src/delta_datafusion/mod.rs:449`). Do not implement a second serialization framework to preserve execution internals. Retain typed commands and immutable input references, then rebuild native plans. A native codec dispatcher may reuse the Delta provider payload for qualified read-plan reuse and handle finite service extensions explicitly; reject unsupported nodes before reaching TODO methods. CDF providers need reconstruction from their version-window descriptor, not this codec.

5. **Use upstream catalogs for discovery where they fit; keep publication binding explicit.** DeltaTableFactory handles native external-table opening and options using a supplied real session. It opens the current table; it does not select the service publication vector or enforce a command's declared schema. ListingSchemaProvider discovers storage paths, normalizes names and opens tables on lookup; refresh adds discoveries and does not produce a transactional namespace revision. Unity providers supply remote naming and temporary credentials, with cached providers. These are useful adapters for real external inventories; neither replaces the local control-table publication authority. Details and limits appear below.

6. **Pair schema and expression changes at the kernel boundary.** Where an actual Delta/kernel transform needs nested field changes, reuse `buoyant_kernel::struct_patch::ProjectionStructPatchBuilder`: one declaration builds both output SchemaRef and matching ExpressionRef and rejects invalid/conflicting field operations. Keep ordinary application relational transformations as DataFusion Expr/LogicalPlan; do not introduce a second kernel-expression version of every business rule. `K:kernel/src/struct_patch.rs:649,865`. The listed SchemaComparison and RetentionCalculator traits are **pub(crate)**, not callable application extension APIs at this pin. Request an upstream public seam when it is needed; do not assume a foreign-impl inventory makes them public.

7. **Collect native telemetry through one result relation.** Reuse Delta operation metrics and DataFusion metric sets; register the kernel's `buoyant_kernel::metrics::reporter::ReportGeneratorLayer` with a cheap MetricsReporter that emits bounded typed observations tagged by operation/attempt/publication. It already translates kernel spans into structured MetricEvent variants (`K:kernel/src/metrics/reporter.rs:24,69`). Eliminate duplicate timing/counting logic and parsing EXPLAIN strings as evidence. DisplayAs remains human-facing diagnostic formatting. A “commit completed” metric remains an observation; the committed control record is the result authority.

### 4.2 DataFusion integration dispositions

Trait paths use the canonical namespaces from the inventory. Internal implementations are consumed through their public parent operations unless explicitly stated otherwise.

| Trait / resolved implementations | Disposition | Concrete evidence and limit |
|---|---|---|
| `datafusion_session::table::TableProvider` — DeltaScan, DeltaCdfTableProvider | Adopt snapshot reads and bounded CDF relations | `D:core/src/delta_datafusion/table_provider/next/mod.rs:711`; `cdf/scan.rs:57`. Registration is not full DML authorization |
| `datafusion_session::table::TableProviderFactory` — DeltaTableFactory | Reuse for native preparation/import; explicit version loading for published views | `D:core/src/delta_datafusion/mod.rs:541`: opens one location with options; derives session on fallback; no publication-vector binding |
| `datafusion_session::schema::SchemaProvider` — ListingSchemaProvider, UnitySchemaProvider | Optional upstream discovery adapters, then captured provider inventory | `D:core/src/data_catalog/storage/mod.rs:60,106`: collecting list, normalized-name collisions, current-table lookup, additive refresh. `D:catalog-unity/src/datafusion.rs:140,220`: snapshot of names, cached providers, external credentials |
| `datafusion_session::catalog::CatalogProvider` — UnityCatalogProvider | Use if Unity is an actual source catalog | `D:catalog-unity/src/datafusion.rs:75,101`: preloaded schema map; not the local publication transaction |
| `datafusion_session::catalog::CatalogProviderList` — UnityCatalogList | Optional remote root; native local inventory remains sufficient | `D:catalog-unity/src/datafusion.rs:28,48`: preloads catalogs, mutable registration. Some non-success list responses become empty inventories; qualify failure semantics before using as evidence discovery |
| `datafusion_session::planner::QueryPlanner` — DeltaPlanner | Reuse shared planner, or compose its extension planner into the service planner | `D:core/src/delta_datafusion/planner.rs:63,73`: shared stateless planner; retain service lifetime wrapper |
| `datafusion_session::planner::ExtensionPlanner` — DeltaExtensionPlanner, DataValidationExtensionPlanner, DeleteMetricExtensionPlanner, MergeMetricExtensionPlanner, UpdateMetricExtensionPlanner, WriteMetricExtensionPlanner | Register DeltaExtensionPlanner once; it owns the five internal delegates | `D:core/src/delta_datafusion/planner.rs:45,98`. Do not maintain six parallel application registrations |
| `datafusion_physical_plan::execution_plan::ExecutionPlan` — ColumnMappingExec, DataValidationExec, DeltaScan, DeltaScanExec, DeltaScanMetaExec, MergeBarrierExec, MergeValidationExec, MetricObserverExec | Inherit mapping, validation, scans, merge safeguards and metrics through native builders | `D:core/src/delta_datafusion/{column_mapping.rs:173,data_validation.rs:576,physical.rs:80}`; `operations/merge/{barrier.rs:73,validation.rs:56}`. DeltaScan is the retired physical wrapper; current provider also has the short name DeltaScan |
| `datafusion_physical_plan::display::DisplayAs` — the preceding eight plus DeltaDataSink | Use for EXPLAIN/diagnostics; typed metrics remain queryable evidence | `D:core/src/delta_datafusion/table_provider/data_sink.rs:213`; display implementations accompany the listed nodes |
| `datafusion_common::pruning::PruningStatistics` — AddContainer, EagerSnapshot, LogDataHandler | Delegate native partition/file/statistics pruning | `D:core/src/kernel/transaction/state.rs:110,183`; `kernel/snapshot/log_data.rs:281`. Missing/inexact stats are not proof a row is absent |
| `datafusion_proto::logical_plan::LogicalExtensionCodec` — DeltaLogicalCodec | Reuse its snapshot-provider payload selectively | `D:core/src/delta_datafusion/mod.rs:496`. Generic logical-node hooks TODO; CDF provider unsupported; runtime handles omitted |
| `datafusion_proto::physical_plan::PhysicalExtensionCodec` — DeltaPhysicalCodec | Exclude from new target; regenerate current physical plans | `D:core/src/delta_datafusion/mod.rs:449`: explicitly deprecated, retired-wrapper-only |
| `datafusion_expr::logical_plan::extension::UserDefinedLogicalNodeCore` — DataValidation, MergeBarrier, MergeValidation, MetricObserver | Preserve native builder-produced plans and their semantics | `D:core/src/delta_datafusion/{data_validation.rs:162,logical.rs:106}`; `operations/merge/{barrier.rs:429,validation.rs:437}`. Merge match validation does not establish global semantic-key uniqueness |
| `datafusion_expr::udf::ScalarUDFImpl` — MakeParquetArray, ToJson, ZOrderUDF | Reuse through supported parser/kernel/OPTIMIZE routes; do not copy them | `D:core/src/delta_datafusion/{expr.rs:87,engine/expressions/to_json.rs:47}`; `operations/optimize.rs:1578`. Private helpers are not general service functions; JSON rendering is not canonical identity; Z-order is physical layout, not semantic ranking |

**Additional engine connection:** `deltalake_core::delta_datafusion::engine::DataFusionEngine` implements `buoyant_kernel::Engine` and is already constructed from the supplied session inside DeltaScan's scan path (`D:core/src/delta_datafusion/table_provider/next/mod.rs:736`). Reuse this; do not build a service kernel engine. It connects TaskContext's store registry/runtime to kernel handlers, while evaluation delegates to Arrow and file formats use the kernel's default handlers. This is not evidence that every allocation is charged to DataFusion's MemoryPool. LogStore::engine uses a different default-engine construction (`D:core/src/logstore/mod.rs:415,594`); audit those actual consumers when binding resources rather than asserting all Delta work automatically inherits query policy.

### 4.3 Other supplied integrations

| Family / implementations | Target use | Boundary and source |
|---|---|---|
| `object_store::ObjectStore` — ConditionalPutShim | Exclude for concurrent control storage | `D:opendal/src/shim.rs:16,45`: explicitly racy HEAD-then-overwrite emulation; sequential success is not a concurrency guarantee |
| ObjectStore — DeltaIOStorageBackend | Optional native I/O-runtime adapter | `D:core/src/logstore/storage/runtime.rs:109,123`: experimental, spawns on a separate runtime. Bind and observe ownership if selected; no second application I/O scheduler |
| ObjectStore — S3StorageBackend | Native backend only for an actual S3 deployment | `D:aws/src/storage.rs:193,232`: forwards conditional puts; separately exposes unsafe-rename choice. Qualify the exact log-store route; the trait name proves neither atomicity nor local durability |
| `parquet::arrow::async_reader::AsyncFileReader` — ParquetObjectReader | Reuse native range/footer/page-index handling | `D:core/src/logstore/parquet_reader.rs:42,72`. Delta data scans still use the snapshot-aware provider; do not substitute a bare file reader |
| `futures_core::stream::Stream` — FileStream | Inherit kernel file streaming/prefetch | `K:default-engine/src/file_stream.rs:74,94`: fail-on-error default; skip-on-error would require explicit partial-coverage observations |
| `buoyant_kernel::schema::compare::SchemaComparison` — DataType, StructField, StructType; also private Nullable | Interface reference; not a direct service call | `K:kernel/src/schema/compare.rs:55,78,91,138`: private, nullability/name/type checks. It does not establish application evidence semantics or arbitrary Arrow metadata preservation |
| `buoyant_kernel::action_reconciliation::RetentionCalculator` — CheckpointWriter, LogCompactionWriter | Inherit through native checkpoint/compaction and table properties | `K:kernel/src/action_reconciliation/mod.rs:43`; `checkpoint/mod.rs:373`; `log_compaction/writer.rs:42`. Private; expires transaction markers and reconciles tombstones, not active service publication leases |
| `buoyant_kernel::struct_patch::ExpressionItem` / `SchemaPatchItem` — ExpressionRef / StructField, plus ProjectionItem | Use public paired patch builders at the kernel boundary | `K:kernel/src/struct_patch.rs:487,577,649,865`. One output schema/expression construction; no independent nested-field walker |
| `serde_core::de::Deserialize` / `serde_core::ser::Serialize` — catalog's 120 / 102 implementors | Reuse native Action/metadata/provider encodings | Actual serde fields determine preservation. Provider runtime fields are skipped; Action JSON is storage protocol, not the service's canonical semantic hash |
| `strum::EnumCount` / `strum::IntoEnumIterator` — TableFeature | Generate feature inventory from the pinned enum | Pair enumeration with provider/operation support and effective protocol; existence of an enum variant is not executable support |
| `validator::traits::Validate` / `ValidateArgs` — TableMetadataUpdate | Reuse native metadata-update validation | `D:core/src/operations/update_table_metadata.rs:16,24`: validates name/description bounds and presence, not arbitrary row or policy rules |
| `tracing_subscriber::layer::Layer` — ReportGeneratorLayer | Adopt structured kernel observations | `K:kernel/src/metrics/reporter.rs:69,108`; feed one bounded diagnostics relation |

**Retention policy refinement:** keep file retention, log history, CDF resume windows, retained results and `set_transaction_retention_duration` in one service policy definition, projected into native properties/maintenance inputs. Native reconciliation can expire application transaction markers (`K:kernel/src/table_properties/mod.rs:186`). Keep the control table's command/result evidence for the promised replay horizon; never rely on a marker alone. Avoid automatic log reclamation until protected versions can still be reconstructed. The parent review's expected-head/claim keys and single control commit remain necessary.

## 5. Representative journeys

- **Ordinary extension:** add a derived search-factor relation. Declare its native query over a bounded CDF window, retained input offset and output schema. Publish the derived output/version checkpoint through the control transaction. No new file-diff loop or independent job journal is introduced.
- **Meaningful change:** a rule revision changes normalization. Preserve the prior evidence; rebuild affected projections from pinned input snapshots, then resume CDF under the new rule revision. CDF alone cannot reinterpret old rows under a new rule.
- **Boundary:** serialize a qualified read provider, advance the underlying table, then decode under a newly bound operation context. Validate expected schema/identity and explicit predicates before registration; the probe retained the earlier snapshot. Reconstruct CDF from its descriptor because DeltaLogicalCodec rejects it.
- **Interruption:** projection output commits but the control commit does not. The old output/offset remains selected. Retry reconciles command identity and source window, then selects one complete result. Marker expiration or a metric event cannot shortcut that reconciliation.

## 6. Acceptance gates

The parent review's **full-target readiness** remains unchanged. The new evidence narrows particular integration claims; it does not certify implementation of the full pivot.

| Gate | Verdict | Follow-up evidence / required action |
|---|---|---|
| G1 — Authority | Unresolved | Target owners are explicit; binding factories, discovered tables and control-selected providers still needs implementation |
| G2 — Semantic fidelity | Unresolved | CDF version/filter behavior and snapshot codec tested; schema evolution, full nested mapping and MCP fidelity remain open |
| G3 — Validity | Fail for the parent's raw provider-INSERT candidate | Existing exact-revision CHECK counterexample remains; complete builders are the selected correction, not a newly certified adapter |
| G4 — Hidden behavior | Unresolved | Native planner/engine reuse is specified; command lifecycle, runtime rebinding and cleanup must be qualified |
| G5 — Consistency and recovery | Unresolved | Codec/runtime limitations and marker retention are now explicit; control/CDF crash recovery and storage races remain untested |
| G6 — Transformation and reuse | Unresolved | Additional native reuse is identified; complete relational equivalence and incremental invalidation remain open |
| G7 — Truthful capability claims | Pass for bounded review claims | Public/private/deprecated routes, exact pins, executed probe and untested behavior are distinguished |

One pass, one failed candidate route, five unresolved; no product acceptance tally is regenerated.

## 7. Principle findings

These are refinements to the proposal, not newly claimed defects in the installed service.

| Finding | Principles | Concrete gap / consequence | Correction | Verification oracle |
|---|---|---|---|---|
| DFU-01 · P0 · Generic Delta plan replay exceeds codec support | DM-24, DM-42, DM-44, DM-48 | Parent §4.3 needs route-specific replay: current logical hooks TODO, physical codec deprecated, decoded schema unchecked. Replaying a job from those bytes can lose binding or fail on a supported operation | Qualified read-provider codec only; durable operation descriptors and reconstructed command plans | **Test / just gate:** provider round trip under changed head/wrong schema, unsupported CDF/logical/physical nodes, fresh process with no runtime handles |
| DFU-02 · P0 · Retention and backend capabilities must cover command replay | DM-14, DM-30, DM-35 | Transaction markers can expire; ConditionalPutShim is racy. Treating either native trait as the whole claim protocol permits duplicate claims or lost control commits | One retention definition; durable command reconciliation; genuinely conditional backend operations | **Test / just gate:** checkpoint across marker expiry, simultaneous creates, unknown commit outcome and preserved old publication |
| DFU-03 · P1 · Prefer complete native mutations over assembling internal nodes | DM-17, DM-19, DM-25, DM-56 | Parent §4.4's adapter could duplicate private validation/merge/metric setup; lower-level physical writer does not supply it | Public builders + one DeltaExtensionPlanner; upstream physical preparation seam only if needed | **Plan/conformance test:** actual builder plans include their validators/metrics; duplicate merge matches and CHECK failures rejected; no copied internal planner registry |
| DFU-04 · P1 · CDF can replace custom incremental data comparison | DM-23, DM-31, DM-33 | Parent mentions CDF without a queryable projection contract; repeated snapshot/file diffing loses native filter and resume behavior | Bounded CDF provider + native projection + atomic selected output/offset | **Differential/fault test:** full recomputation equals incremental projection across inserts/updates/deletes, no-op maintenance and interrupted control commit |
| DFU-05 · P1 · Native discovery must be captured before evidence binding | DM-08, DM-14, DM-45 | Factory/listing/Unity open current or cached providers; listing normalization can collide and some remote list errors become empty. Discovery cannot certify one publication | Reuse discovery adapters where needed, then validate and bind captured names/identities/versions | **Test:** concurrent head changes, normalized-name collision, failed remote listing, expired credentials and exact published vector |
| DFU-06 · P1 · Reuse native physical metadata and kernel integration | DM-24, DM-36, DM-39 | Adding custom count/file readers would duplicate DeltaScanMetaExec and snapshot-aware transforms; DataFusionEngine already exists in scans | Native stats, file selection, scan/engine paths; bind remaining actual resource consumers | **Plan + behavioral test:** metadata/full-scan equivalence with deletion vectors, strict missing files, physical rewrite and resource cancellation |
| DFU-07 · P1 · Pair boundary schema and expression changes | DM-06, DM-22, DM-42 | Independently editing schema and kernel field projection can silently misalign nested values | ProjectionStructPatchBuilder where kernel patches are required; native DataFusion projection elsewhere | **Round-trip/property test:** nested insert/drop/replace, metadata and nullability preservation; invalid field edits fail |
| DFU-08 · P1 · Derive diagnostics from existing structured telemetry | DM-46, DM-47, DM-50 | Independent counters or parsed display strings can disagree with actual native execution or misrepresent attempted work as publication | Delta/DataFusion metrics + kernel MetricEvent → one typed diagnostic relation | **Test:** correlate concurrent attempts, dropped/retried work and committed result; bounded event buffering; no metric-only success decision |

**Applicability and principle verdicts:** the applicable requirements cited above remain **Unresolved** at full-target scope pending implementation/oracles. The hypothetical blanket codec/backend candidates would violate DM-42/DM-44 and DM-35 respectively and are explicitly excluded. The proposal retains distinct authoritative/derived identities (DM-11–DM-13) and these are **Satisfied at design-definition scope**, not runtime-certified. Groups 2–12 bear on the named boundaries; group 1's underlying domain model is unchanged and is not regraded in this follow-up. Unrelated producer language semantics, public tool selection and source capture are outside this focused reassessment.

## 8. Alternatives and architectural leverage

| Alternative | Duplication / correctness | Decision |
|---|---|---|
| Parent target without these refinements | Correct direction; leaves CDF, internal operation composition, codec boundaries and native telemetry underspecified | Refine |
| **Complete builders + native CDF/scans + qualified read codecs + paired patches/telemetry** | Removes independent dataflow, writer and instrumentation decisions; preserves explicit publication authority | **Select** |
| Simpler viable option: native snapshot plans only, no persisted plan bytes or CDF projections | Preserves the unified engine and useful MCP output; full recomputation can be the declared recovery path | Use as the simplest correct execution/rebuild path; add CDF for concrete derived consumers, not an independent state authority |
| Import every discovered trait implementation as a public plugin | Private/deprecated APIs, remote-current catalogs and incomplete codecs acquire promises they do not implement | Reject |

No new catalog service, workflow interpreter, generic codec framework or schema-rule language is justified. Unity remains available when an actual external catalog consumer exists. Z-order/advanced layout stays a native physical choice qualified on the eventual workload.

## 9. Verification and measurement plan

New evidence is in [the follow-up evidence directory](evidence/delta-integration-followup-2026-09-15/). The standalone source/lockfile/log/receipt make the bounded probe reproducible.

| Claim | Strength | Conditions / result |
|---|---|---|
| CDF is directly queryable with bounded versions | Tested | Three append versions; provider window [1,1]; exactly one change. Filter on id/change type while projecting commit version with LIMIT retained the expected row |
| Snapshot codec retains its captured revision | Tested | Encode version-1 provider, write version 2, decode and query: earlier two rows retained |
| Decoding does not enforce supplied schema | Tested | Supply empty expected schema; decoded provider still exposes its stored id schema |
| Provider serialization does not preserve mutation capability | Tested | Decoded INSERT rejected for missing runtime log_store handle |
| CDF is outside DeltaLogicalCodec's provider support | Tested | Encoding DeltaCdfTableProvider rejected |
| Private nodes, planner dispatch, retired physical codec, schema patch, retention, telemetry and storage semantics | Interface-checked | Exact source and indexed APIs above; source tests inspected where relevant, not claimed executed |
| Full lifecycle / remote integrations / speedup | Proposed or unmeasured | No full service/client, remote catalog/storage, crash/retention or performance qualification in this follow-up |

An initial probe assertion assumed signed CDF commit-version storage and failed after the codec assertions. The corrected source uses the provider's actual UInt64 field and completed successfully; both logs are retained. This was a probe type assumption, not a CDF execution failure.

Measure whole MCP journeys when implementing: CDF projection versus full native recomputation, metadata/full-scan plans, file counts and I/O, catalog preparation, bounded telemetry and retained-history cost. No speedup or resource containment is inferred from names or source comments.

## 10. Exceptions and unresolved decisions

No exception to the user's full DataFusion pivot is introduced. Native Arrow/kernel mechanisms remain part of Delta's supported implementation, not a parallel application semantic engine. Existing command/publication/fencing and evidence-fidelity requirements remain.

The implementation owner—sole project owner acting through Codex—must choose each concrete CDF consumer and schema policy; qualify the native mutation adapter; define replay/retention horizons; and prove actual storage atomicity/durability. Triggers for additional upstream work are specific: a required physical-input mutation preparation API, a required public schema-compatibility contract, or a required current physical-node codec. Their current absence does not justify recreating an entire subsystem.

## 11. Decision and implementation changes

**Decision: revise the target with these refinements.** Keep the core architecture; make upstream composition, queryable changes and binding boundaries explicit before implementation.

| Priority | Change to the target | Principles | Acceptance evidence / protection |
|---|---|---|---|
| P0 | Persist commands and input/version descriptors; limit codec use to qualified read providers | DM-24, DM-42, DM-44, DM-48 | Codec binding/unsupported-node and restart tests |
| P0 | Unify file/log/CDF/transaction/result retention and exclude racy conditional-create storage | DM-14, DM-30, DM-35 | Concurrent commit and expired-marker recovery gate |
| P1 | Use complete Delta builders and their aggregated extension planner | DM-17, DM-19, DM-25, DM-56 | Native plan/operation conformance; no copied validators/merge/planner registries |
| P1 | Expose bounded CDF views and native incremental derived projections | DM-23, DM-31, DM-33 | Full-versus-incremental equivalence and atomic offset/output selection |
| P1 | Keep native discovery adapters separate from control-selected immutable binding | DM-08, DM-14, DM-45 | Version/name/error/credential boundary tests |
| P1 | Reuse native metadata scans, strict file selection, paired kernel patches and structured telemetry | DM-22, DM-24, DM-42, DM-47, DM-50 | Schema/scan equivalence and correlated bounded diagnostics |

The result is fewer application-owned semantic mechanisms while retaining the same MCP functionality and full DataFusion direction.
