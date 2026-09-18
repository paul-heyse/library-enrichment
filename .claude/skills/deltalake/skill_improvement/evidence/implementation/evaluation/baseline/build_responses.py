from pathlib import Path
import json, hashlib, datetime, re
OUT=Path(__file__).resolve().parent
BASE=Path('/home/paul/library-enrichment/.claude/skills/deltalake/skill_improvement/evidence/implementation/.evaluation/baseline')
CARGO=Path('/home/paul/.cargo/git/checkouts/delta-rs-dcb716bfdc369320/58f07cd')
DF=next(Path('/home/paul/.cargo/registry/src').glob('*/datafusion-55.1.0'))
task_doc=json.loads((OUT.parent/'tasks.json').read_text()); tasks={t['id']:t for t in task_doc['tasks']}
responses=[]
def add(id,candidates,selected,inputs,outputs,effects,evidence,symbols,unknowns):
    responses.append(dict(id=id,request=tasks[id]['request'],code_required=tasks[id]['code_required'],candidates=candidates,selected=selected,inputs=inputs,outputs=outputs,effects=effects,evidence=evidence,canonical_symbols=symbols,unknowns=unknowns))
T='baseline:content/api/deltalake_core.table.md'
W='baseline:content/api/deltalake_core.operations.write.md'
P='baseline:content/api/deltalake_core.delta_datafusion.table_provider.md'
CDF='baseline:content/api/deltalake_core.operations.load_cdf.md'
TX='baseline:content/api/deltalake_core.kernel.transaction.md'
SES='cargo:crates/core/src/delta_datafusion/session.rs'
CDFS='cargo:crates/core/src/operations/load_cdf.rs'
add('E01A',['Keep a loaded snapshot/provider','Reload latest before every query','Open an explicitly recorded version'],
 'Load once when the report starts, record table identity and version, and build one provider from that snapshot. Reuse it for all report queries; a later report may choose a newer snapshot. Reopen with load_version(v) when resuming the same report.',
 'Table URL/storage options and report-start version v.', 'Rows from v; capture v in report provenance.', 'Read-only log and data reads; no refresh during the report.',
 [T,P,'baseline:content/topics/datafusion.md','baseline:content/topics/table-identity.md'],['deltalake_core::table::DeltaTable::load_version','deltalake_core::table::DeltaTable::table_provider'],
 'Retention must preserve the files and log needed for v; pinning alone does not prevent vacuum. E01B tests provider pinning under append, not retention.')
add('E01B',['Reuse the old provider','Refresh table and rebuild/replace provider in reused context','Build a fresh context each cycle'],
 'Reuse SessionContext, call table.update_state() (or update_incremental(None)), build table.table_provider().await?, deregister the old name, and register the new provider. Plan each new polling query after replacement. Serialize replacement against name resolution if concurrent callers require an atomic switch.',
 'Loaded table, reusable context, poll boundary.', 'Current loaded version and queries seeing its rows; already planned queries retain their earlier snapshot.', 'Log replay, provider replacement; no data mutation.',
 [T,P,'baseline:content/topics/datafusion.md'],['deltalake_core::table::DeltaTable::update_state','deltalake_core::table::DeltaTable::table_provider'],
 'A writer can commit after the refresh boundary; this supplies a coherent refreshed snapshot, not an ever-moving read. Deregister/register is not atomic.')
add('E02A',['DataFusion read_parquet/register_parquet or ListingTable','Delta provider after conversion','Manual Parquet reader'],
 'Use SessionContext::read_parquet(directory, ParquetReadOptions::default()) and select/filter the DataFrame, or register_parquet for SQL. A ListingTable offers more listing control. Immutable plain files need no Delta log or format conversion.',
 'Directory URL, explicit schema/options if needed, projected columns and predicate.', 'DataFrame, executable plan and batches containing matching rows.', 'Lists/reads existing Parquet; no table-format mutation.',
 ['datafusion:src/execution/context/parquet.rs','baseline:content/topics/datafusion.md'],['datafusion::execution::context::SessionContext::read_parquet'],
 'Directory schema consistency, file discovery and desired pruning need an application fixture; this route was source-inspected, not executed.')
add('E02B',['Delta snapshot provider','Plain Parquet directory scan','Manual reconstruction of active files'],
 'Open/load DeltaTable and use table.table_provider().await? through DataFusion. The log defines active files and row-level table semantics. Plain directory scans include removed files and can resurrect deleted or pre-update rows.',
 'Delta root URL, supported reader protocol/features, credentials.', 'Logical contents of the selected Delta version.', 'Read log/checkpoints and selected live data; do not delete old files.',
 [T,P,'baseline:content/topics/snapshot-and-log.md','baseline:content/topics/dml.md'],['deltalake_core::table::DeltaTable::table_provider'],
 'Feature-specific compatibility is separate from recognizing the directory as Delta. This task has no direct runtime test; E06B/E14B cover active-file changes.')
add('E03A',['scan_table().with_columns(["id"])','Register a provider and issue SELECT','Collect every column then project'],
 'Use table.scan_table().with_columns(["id"]).await? and process SendableRecordBatchStream one batch at a time with try_next(). If passing a caller session, prepare its store mapping. The returned LoadBuilder is callable but its module is private; do not name it in a public signature.',
 'Loaded table and column-name selection; optional Arc<dyn Session>.', '(DeltaTable, SendableRecordBatchStream) projected to id; errors remain fallible during consumption.', 'Read-only incremental batch consumption; no mandatory whole-result collection.',
 ['baseline:content/api/deltalake_core.operations.load.md','cargo:crates/core/src/operations/load.rs',T],['deltalake_core::table::DeltaTable::scan_table','deltalake_core::operations::load::LoadBuilder::with_columns'],
 'Batch sizes are execution settings, not a semantic promise; the fixture explicitly configures size 1 and checks multiple batches.')
add('E03B',['Delta TableProvider in existing DataFusion query','Standalone scan_table stream','Collect and register MemTable'],
 'Prepare the existing session, register table.table_provider().await?, and join/filter using the existing SQL or logical plan. The provider is composable and exposes projection/filter pushdown; a standalone stream is suitable for simple consumption but loses this planning seam.',
 'Existing context, Delta table, second registered source and join/filter expressions.', 'Joined/filtered DataFrame or execution plan and rows.', 'Read-only scans; provider registration changes session catalog state.',
 [P,'baseline:content/topics/datafusion.md',T],['deltalake_core::table::DeltaTable::table_provider'],
 'Inspect EXPLAIN/metrics for actual pruning and join cost; capability existence is not a performance measurement.')
add('E04A',['Pass concrete SessionState','Internal default session','Derive from Session trait'],
 'Pass Arc::new(caller_state) using with_session_state and RequireSessionState to fail closed if that contract changes. A concrete SessionState is cloned with its environment. Ensure its query planner supports Delta extension nodes; compose DeltaExtensionPlanner into a custom planner or use DeltaPlanner when there is no other planner to retain.',
 'Concrete SessionState with UDF, runtime policy and Delta-capable planner; write data/plan.', 'Committed DeltaTable or a propagated planning/validation error.', 'Retains runtime/UDF state; stages data then publishes a log version on success.',
 [W,SES,'baseline:content/api/deltalake_core.delta_datafusion.planner.md'],['deltalake_core::operations::write::WriteBuilder::with_session_state','deltalake_core::delta_datafusion::session::SessionFallbackPolicy::RequireSessionState','deltalake_core::delta_datafusion::planner::DeltaExtensionPlanner'],
 'A plain DataFusion planner fails on MetricObserver in E05B; session preservation alone is insufficient. Arbitrary custom planner composition is not exercised.')
add('E04B',['DeriveFromTrait','RequireSessionState and obtain a concrete state','Default InternalDefaults'],
 'Use with_session_state(custom_session).with_session_fallback_policy(DeriveFromTrait) when trait-visible runtime, config, execution properties, table options and function registries are sufficient. It creates a Delta-capable SessionState. Reject this route and require a concrete state when policy depends on catalogs, custom planners or optimizer rules; those do not transfer. Do not silently accept InternalDefaults.',
 'Arc<dyn Session> and explicit preservation requirements.', 'Derived SessionState internally used by the operation, or an explicit unsupported-environment decision.', 'May create new planning/catalog state while sharing caller runtime and UDFs.',
 [SES,'baseline:content/api/deltalake_core.delta_datafusion.session.md',W],['deltalake_core::delta_datafusion::session::SessionFallbackPolicy::DeriveFromTrait'],
 'Custom Session integration is source-supported, not runtime-tested in this evaluation. Trace each required runtime policy to a copied field before deployment.')
add('E05A',['DeltaTable::write with RecordBatches','RecordBatchWriter with manual flush','Logical-plan input'],
 'Use table.write(batches).with_save_mode(SaveMode::Append).await? for batches already resident in memory. The operation supplies file staging, schema checks and commit publication; RecordBatchWriter is available if streaming file-window control is needed.',
 'Owned Arrow RecordBatches and loaded destination table; explicit schema policy.', 'Updated DeltaTable at the committed version.', 'Writes Parquet and Add actions; existing rows remain.',
 [W,'baseline:content/topics/writing.md'],['deltalake_core::table::DeltaTable::write','deltalake_core::operations::write::WriteBuilder::with_save_mode'],
 'Application retry policy remains necessary. Memory batches do not imply a byte-identical Arrow/Delta schema round trip.')
add('E05B',['with_input_plan(LogicalPlan)','with_input_execution_plan(Arc<LogicalPlan>)','Collect into RecordBatches then write'],
 'Use table.write(Vec::<RecordBatch>::new()).with_input_plan(plan).with_session_state(caller_state).with_session_fallback_policy(RequireSessionState).with_save_mode(Append).await?. Despite its name, with_input_execution_plan also takes a logical plan. Preserve the caller environment and install/compose Delta planner extensions; test both the UDF and a source requiring its object-store mapping.',
 'Existing LogicalPlan and compatible concrete SessionState with UDF/runtime/store registrations.', 'Appended rows and updated destination DeltaTable.', 'Executes the plan in the supplied environment; stages files and commits.',
 [W,SES,'baseline:content/api/deltalake_core.delta_datafusion.planner.md','datafusion:src/execution/session_state.rs'],['deltalake_core::operations::write::WriteBuilder::with_input_plan','deltalake_core::delta_datafusion::planner::DeltaPlanner'],
 'The passing test adds DeltaPlanner using SessionStateBuilder::new_from_existing, preserving runtime/config/UDF. Replacing an application custom planner wholesale would need a separate composition test.')
add('E06A',['SaveMode::Overwrite','Delete then append','Predicate overwrite'],
 'Use table.write(new_batches).with_save_mode(SaveMode::Overwrite).await? with no replacement predicate. This replaces all logical rows in one transaction. Retain schema compatibility unless a separate schema change is intended.',
 'Complete replacement batch and current table.', 'New current version containing exactly the replacement rows.', 'Adds new files and removes old files from active state; old files remain until vacuum.',
 [W,'baseline:content/topics/writing.md'],['deltalake_core::operations::write::WriteBuilder::with_save_mode'],
 'Overwrite is not automatic schema replacement. Retained old files depend on retention and are not a backup guarantee.')
add('E06B',['Overwrite with with_replace_where','Whole-table overwrite','Delete predicate then append'],
 'Use SaveMode::Overwrite plus with_replace_where("id >= 2"). Expect exact validation of incoming rows; reject out-of-predicate rows rather than silently appending them. If the business policy wants filtering, explicitly filter/quarantine those rows before writing and disclose that loss. Preserve id < 2 even when it shares a file with replaced rows.',
 'Current rows, replacement batches all satisfying id >= 2, predicate.', 'New version with untouched rows plus replacement rows; invalid input errors.', 'Atomic active-file rewrite; validation error does not publish a new log version.',
 [W,'cargo:crates/core/src/operations/write/mod.rs'],['deltalake_core::operations::write::WriteBuilder::with_replace_where'],
 'Failed writes may stage files before failing; no publication does not imply no storage side effects. Fixture tests mixed rows in one file and invalid replacement values.')
add('E07A',['Default fixed schema write','SchemaMode::Merge','Prevalidate exact Arrow contract plus default write'],
 'Keep schema evolution disabled and validate the fixed table contract before append. Delta writes can cast/conform compatible input, so if fixed means exact Arrow fields/order/nullability, perform that equality/validation explicitly. Reject extras, incompatible types and invalid nulls; do not use merge to make validation pass.',
 'Expected Delta schema, intended Arrow conversion contract, incoming batches.', 'Accepted conforming rows or clear rejection; table schema unchanged.', 'Successful append commits; rejected input must not advance the table version.',
 [W,'baseline:content/topics/schema-and-types.md','cargo:crates/core/src/writer/record_batch.rs'],['deltalake_core::operations::write::WriteBuilder','buoyant_kernel::schema::StructType'],
 'Exact nested/nullability/casting acceptance needs fixture coverage for the chosen writer. This evaluation does not certify every fixed-schema violation.')
add('E07B',['Append with SchemaMode::Merge','Add columns first then append','SchemaMode::Overwrite'],
 'Use Append plus with_schema_mode(SchemaMode::Merge) when incoming columns intentionally expand the schema. Preserve existing data and expose missing historical values as null where permitted. add_columns is available for an independent metadata migration; whole-schema overwrite is not required.',
 'Incoming schema with allowed new fields, data, existing table schema and evolution policy.', 'Merged table schema and combined old/new rows.', 'Writes new data and schema metadata in the write transaction; preserves earlier rows.',
 [W,'baseline:content/topics/schema-and-types.md'],['deltalake_core::operations::write::SchemaMode::Merge','deltalake_core::table::DeltaTable::add_columns'],
 'Reject incompatible type changes and verify nested fields/constraints; merge is not unrestricted coercion. No dedicated evolution runtime fixture here.')
add('E08A',['MergeBuilder with ordered clauses','Separate update and append','Overwrite entire table'],
 'Use table.merge(source_df,"target.id = source.id").with_source_alias("source").with_target_alias("target"), then when_matched_update and when_not_matched_insert. Qualify source expressions and set every inserted field. Order specific clauses before broad ones; reuse a suitable source session. Confirm target keys meet application uniqueness assumptions too.',
 'Unique non-null source keys, target table, explicit update/insert assignments.', 'Updated table and MergeMetrics.', 'Atomic matched-row changes and unmatched inserts; file work depends on pruning.',
 ['baseline:content/api/deltalake_core.operations.merge.md','baseline:content/topics/dml.md'],['deltalake_core::operations::merge::MergeBuilder'],
 'Source uniqueness alone does not imply target uniqueness. No direct merge runtime test was required or run.')
add('E08B',['Deterministically normalize source keys then merge','Rely on merge duplicate validation','Allow nulls with explicit null-safe matching'],
 'Define null-key policy first: reject/quarantine null keys, or deliberately use a null-safe equality predicate with tests; ordinary equality does not match nulls. Deduplicate by a deterministic, domain-defined ordering or aggregate, rejecting ties. Check target cardinality too. Exact-pin source tests show duplicate relevant WHEN MATCHED updates can error, contradicting the baseline topic blanket claim that duplicates never error. Do not rely on that check as a universal source uniqueness constraint; unmatched duplicate rows can still insert.',
 'Possibly duplicate/null source and target keys, tie-breaking or rejection policy.', 'Normalized source and deterministic merge outcome, or validation rejection.', 'Normalization reads source; successful merge changes table atomically.',
 ['baseline:content/topics/dml.md','baseline:content/api/deltalake_core.operations.merge.md','cargo:crates/core/src/operations/merge/mod.rs'],['deltalake_core::operations::merge::MergeBuilder'],
 'Exercise duplicate update/delete/no-op clauses and null-safe behavior at the exact pin. Source inspection is not a run of upstream tests.')
add('E09A',['Append with CommitProperties application transaction','Append with metadata only','Manual CommitBuilder'],
 'Append once with CommitProperties::default().with_application_transaction(Transaction::new(app_id,batch_version)) passed to with_commit_properties. Bind the marker to an immutable batch identity and record enough external provenance to resolve an uncertain result.',
 'Batches, stable application ID and application sequence number.', 'DeltaTable and a durable txn action accompanying the rows.', 'Publishes Add and application transaction actions together.',
 [TX,'cargo:crates/core/src/kernel/models/actions.rs'],['deltalake_core::kernel::transaction::CommitProperties::with_application_transaction','deltalake_core::kernel::models::actions::Transaction'],
 'Marker recording alone does not suppress a repeated append; E09B demonstrates the distinction.')
add('E09B',['Marker-only replay','Reload/check transaction_version before write under serialization','External durable exactly-once coordinator'],
 'For serialized writers per app ID, refresh the table, read snapshot.transaction_version(log_store,app_id), and skip an already committed sequence. The >= comparison requires a strictly monotonic application sequence and immutable marker-to-payload binding. Otherwise append with that marker. After ambiguous failure, refresh and inspect before retry. Concurrent check-then-write is not atomic: serialize by app ID or supply a proven coordination design, handle transaction conflicts and re-read. Never promise exactly-once from a marker alone.',
 'Stable marker/payload, refreshed snapshot, serialization or stronger coordinator.', 'Either existing committed result or one new append; persisted marker supports replay decisions.', 'Guard reads transaction history; chosen append commits rows and marker.',
 [TX,'cargo:crates/core/src/table/state.rs','cargo:crates/core/src/kernel/transaction/conflict_checker.rs','cargo:crates/core/src/kernel/transaction/mod.rs'],['deltalake_core::table::state::DeltaTableState::transaction_version','deltalake_core::kernel::transaction::CommitProperties::with_application_transaction'],
 'Runtime fixture proves serialized replay suppression and duplicate unguarded replay. Concurrent writers, marker expiration and payload mismatch handling are not tested; external serialization is an explicit precondition.')
add('E10A',['Fix validation and resubmit','Blind retry unchanged input','Assume any error means commit failed'],
 'For a positively identified validation error before publication, correct the schema/predicate/constraint violation and retry the corrected batch using the application replay policy. Verify current version and rows before claiming no logical change. Keep validation errors distinct from conflicts and ambiguous post-commit errors.',
 'Validation error, known publication stage, rejected batch and current table.', 'Corrected input or actionable rejection; no new logical version from the failed operation.', 'A rejected write may have staged unreferenced files; reclaim through retention-aware maintenance.',
 ['cargo:crates/core/src/operations/write/mod.rs',W],['deltalake_core::operations::write::WriteBuilder'],
 'E06B proves pre-publication predicate validation behavior for its fixture; not every error string establishes the publication stage.')
add('E10B',['Treat any hook error as rollback','Inspect durable table/log transaction marker','Retry immediately'],
 'Treat the outcome as ambiguous until the failing callback and durable log are checked. post_execute runs after the commit; post-commit hooks can fail after numbered-log publication. Refresh/reopen and inspect marker, version and commit metadata; if present, report committed-with-hook-failure and repair hook side effects without reappending. If absence is established, retry using a replay-safe path. Make external hook effects idempotent independently.',
 'Hook identity/stage, operation ID, marker, error and current log.', 'Reconciled committed/uncommitted/unknown state plus separate hook-repair result.', 'Diagnostic reads; any repair is explicit and separate from data replay.',
 ['baseline:content/traits/CustomExecuteHandler.md','cargo:crates/core/src/operations/write/mod.rs','cargo:crates/core/src/kernel/transaction/mod.rs'],['deltalake_core::operations::CustomExecuteHandler','deltalake_core::kernel::transaction::PostCommit'],
 'No injected-hook runtime fixture here. If marker identity or log visibility is insufficient, retain unknown status rather than claiming rollback.')
add('E11A',['Snapshot scan/provider','CDF interval','Directory listing of Parquet'],
 'Refresh or open the desired head and use scan_table() for batches or a provider for SQL. Return rows of that one snapshot with its version, without CDF metadata or preimages.',
 'Table root and explicit freshness requirement.', 'Current snapshot rows and observed version.', 'Read-only snapshot scan.',
 [T,'baseline:content/api/deltalake_core.operations.load.md'],['deltalake_core::table::DeltaTable::scan_table'],
 'Current is the observed snapshot boundary; later commits require another refresh.')
add('E11B',['scan_cdf and DeltaCdfTableProvider','Snapshot diff','Current snapshot scan'],
 'Use table.scan_cdf().with_starting_version(start).with_ending_version(end), wrap with DeltaCdfTableProvider::try_new for SQL or build its physical plan. Enable CDF before changes occur. Consume insert/delete/update_preimage/update_postimage and commit metadata with explicit application semantics; checkpoint successfully applied commit versions, not timestamps.',
 'Inclusive version interval and retained CDF-enabled history, prepared session.', 'Rows plus _change_type, _commit_version, _commit_timestamp.', 'Reads change files or inferred Add/Remove changes; does not mutate source table.',
 [CDF,'baseline:content/api/deltalake_core.delta_datafusion.cdf.scan.md',CDFS],['deltalake_core::table::DeltaTable::scan_cdf','deltalake_core::delta_datafusion::cdf::scan::DeltaCdfTableProvider'],
 'Physical metadata arrays can be dictionary encoded; consumer must inspect/adapt them. The fixture verifies all four change kinds across versions 1..3, not schema evolution or every CDF producer.')
add('E12A',['Bounded CDF interval','Unbounded tail','Snapshot diff'],
 'Read with explicit starting and ending versions after confirming the whole inclusive range is retained and CDF-enabled. Process all batches before advancing a durable consumer checkpoint to end. Make sink application/replay idempotent so a crash before checkpoint cannot corrupt results.',
 'Valid inclusive interval, retained log/data and consumer checkpoint.', 'Complete bounded change rows and acknowledged end version.', 'Read-only source access; consumer may commit its own outputs/checkpoint.',
 [CDF,'baseline:content/topics/change-data-feed.md',CDFS],['deltalake_core::operations::load_cdf::CdfLoadBuilder'],
 'Retention validation and sink atomicity are application obligations; fixture E11B covers only a small valid range.')
add('E12B',['Strict bounded reads with head checks','with_allow_out_of_range for polling','Pretend absent history is empty'],
 'Resolve head and CDF enablement boundary first. For strict completeness, reject/defer a request beyond head; implementation clamps an oversized ending version to head, so never checkpoint the originally requested future end. A start beyond head can fail or return empty with with_allow_out_of_range. That flag is useful for polling but does not manufacture pre-enable history. Split at enablement only with an explicit snapshot bootstrap/reconciliation contract; retain disabled/missing-history errors.',
 'Requested interval, observed head, enable/disable history and retention.', 'Validated interval/results or explicit incomplete/unsupported range.', 'Read-only; no retroactive CDF enablement.',
 [CDF,CDFS],['deltalake_core::operations::load_cdf::CdfLoadBuilder::with_allow_out_of_range'],
 'These boundary behaviors are source-inspected; no boundary runtime test here. Concurrent head growth and schema changes need additional handling.')
add('E13A',['Optimize compaction','Optimize Z-order','Vacuum'],
 'Use table.optimize() for compaction, optionally scope changed partitions with with_filters and choose target size/concurrency. Z-order is available if measured selective queries justify it. Vacuum removes obsolete files and does not consolidate the active small files that slow scans.',
 'Current table, affected partitions, target file-size and concurrency budget.', 'Optimized table plus operation metrics and fewer/larger active files.', 'Rewrites active files and commits replacement actions; obsolete files remain.',
 ['baseline:content/topics/maintenance.md','baseline:content/catalogs/operations.md'],['deltalake_core::operations::optimize::OptimizeBuilder'],
 'Actual scan improvement and cost require workload measurements; not executed in this evaluation.')
add('E13B',['Vacuum dry-run then execute','Optimize','Manual deletion'],
 'Run table.vacuum().with_dry_run(true) with an explicitly chosen retention and mode; inspect VacuumMetrics.files_deleted as the preview candidates. Only a later non-dry run deletes eligible files. Retain the safety check and ensure retention covers active readers and promised history. Choose Full if storage orphans must be found.',
 'Table, retention policy, mode, protected versions if applicable.', 'Preview candidate paths with dry_run=true; execution would return deletion metrics.', 'Preview lists/reads; actual vacuum irreversibly deletes eligible files.',
 ['baseline:content/api/deltalake_core.operations.vacuum.md','cargo:crates/core/src/operations/vacuum.rs'],['deltalake_core::operations::vacuum::VacuumBuilder::with_dry_run'],
 'Preview is not a reservation of future candidates; revalidate before destructive execution. No vacuum was run here.')
add('E14A',['Clone and load_version(0)','Restore version 0','Overwrite with copied old data'],
 'Use a separate handle (or clone) and load_version(0), then query its snapshot. Keep the existing current handle/provider unchanged. This is time travel, so no restore commit is needed.',
 'Version 0 and retained files/log.', 'Rows/schema of version 0.', 'Read-only history replay and scans; does not alter table head.',
 [T,'baseline:content/topics/table-identity.md'],['deltalake_core::table::DeltaTable::load_version'],
 'Fails if required history/files have been removed; retained files are a prerequisite.')
add('E14B',['RestoreBuilder to version 0','Time travel load_version(0)','Overwrite old values manually'],
 'Use current_table.restore().with_version_to_restore(0).await?. This publishes a new head representing the old data state. Keep missing-file checks enabled and avoid protocol downgrade unless explicitly justified.',
 'Current table, version 0, still-existing referenced files.', '(DeltaTable, RestoreMetrics) at a new version, with RESTORE history.', 'Adds/removes active-file references in a new transaction; preserves intermediate history subject to retention.',
 ['baseline:content/api/deltalake_core.operations.restore.md','baseline:content/corpus/tests/it_datafusion/command_restore.rs'],['deltalake_core::operations::restore::RestoreBuilder'],
 'The fixture verifies data/head/history and intermediate time travel. It does not cover protocol changes or missing-file restoration.')
add('E15A',['Normal Delta provider','Plain Parquet reader','Reject every mapped table'],
 'Use the normal Delta provider after protocol/schema compatibility checks. Exact-pin upstream provider tests cover a column-mapped fixture and logical column names. Confirm the actual mapping mode and metadata, then test projection/filtering and renames against your table; do not treat the feature-name catalog alone as proof.',
 'Mapped table and its reader protocol/schema metadata.', 'Logical column names/values after mapping transforms.', 'Read-only provider scan.',
 [P,'baseline:content/corpus/tests/it_datafusion/datafusion_table_provider.rs'],['deltalake_core::delta_datafusion::table_provider::next::DeltaScan'],
 'Upstream test source was inspected but not run; no blanket guarantee for every mapping mode/feature combination.')
add('E15B',['CDF builder/provider on mapped table','Separate compatible CDF producer','Assume normal scan support implies CDF support'],
 'At this pin CdfLoadBuilder::build_with_metrics explicitly rejects any column mapping mode other than None using UnsupportedColumnMapping(Read). DeltaCdfTableProvider delegates there, so it does not bypass the restriction. Use a verified compatible engine/producer or change the integration requirements; do not silently disable mapping on an existing table.',
 'Same mapped table and desired CDF interval.', 'Unsupported-column-mapping error for this path; no certified CDF rows.', 'No table changes.',
 [CDFS,'baseline:content/api/deltalake_core.delta_datafusion.cdf.scan.md'],['deltalake_core::operations::load_cdf::CdfLoadBuilder::build_with_metrics'],
 'The rejection is source-confirmed, not executed in this consumer. Normal provider support cannot establish CDF support.')
add('E16A',['table.update_datafusion_session','Explicit runtime registration','Create unrelated default session'],
 'Call table.update_datafusion_session(&caller_state) before planning. It ensures the table root store is registered if missing. Use the public method; its underlying DeltaSessionExt helper is crate-private. Preserve the caller runtime and its policy.',
 'Caller Session and table log/root object store.', 'Session able to resolve the table root.', 'Mutates object-store registry only when lookup fails; helper is check-then-register, not atomic.',
 [T,SES,'cargo:crates/core/src/delta_datafusion/table_provider.rs'],['deltalake_core::table::DeltaTable::update_datafusion_session'],
 'An existing but incorrect mapping is not repaired; see E16B. Concurrent registrations need caller coordination.')
add('E16B',['Ensure helper again','Explicit RuntimeEnv::register_object_store replacement','New isolated context'],
 'Identify the canonical root key via log_store.root_url().as_object_store_url() and explicitly register log_store.root_object_store(None) into the existing runtime. Coordinate shared-runtime users before replacing a mapping. The ensure helper intentionally leaves an existing mapping untouched, even when wrong.',
 'Known wrong root mapping and correct unprefixed root object store.', 'Correct store lookup and successful existing-provider scans.', 'Replaces shared runtime registry entry; affects later operations using that root.',
 [SES,'baseline:content/api/deltalake_core.delta_datafusion.engine.storage.md','cargo:crates/core/src/delta_datafusion/table_provider.rs'],['deltalake_core::delta_datafusion::engine::storage::AsObjectStoreUrl','deltalake_core::table::DeltaTable::update_datafusion_session'],
 'The test checks wrong-store identity survives ensure, read fails, explicit replacement succeeds. It does not simulate an already executing read or shared cloud bucket.')
add('H01',['DeltaCdfTableProvider with normal SQL filter','CdfLoadBuilder::build plus explicit FilterExec','Only pass low-level pruning predicate'],
 'Prefer DeltaCdfTableProvider for the filtered relation: its scan compiles the predicate, supplies it to CDF sources for pruning, then wraps the result in FilterExec before projection/limit. For a lower-level build(session,Some(predicate)), add an exact FilterExec yourself; a Parquet pruning predicate alone is not proof that every output row satisfies a non-partition filter.',
 'CDF interval, non-partition predicate referencing full CDF schema.', 'Exactly matching rows with the selected projection.', 'Read-only plans; pruning can reduce I/O, FilterExec enforces row semantics.',
 [CDFS,'cargo:crates/core/src/delta_datafusion/cdf/scan.rs','baseline:content/api/deltalake_core.delta_datafusion.cdf.scan.md'],['deltalake_core::delta_datafusion::cdf::scan::DeltaCdfTableProvider::scan','deltalake_core::operations::load_cdf::CdfLoadBuilder::build'],
 'Exact plan/source inspected, not independently executed with a non-partition predicate in this evaluation.')
add('H02',['Explicit Delta schema and tested Arrow conversions','Blindly infer/assume lossless roundtrip','Normalize Arrow before writing'],
 'Declare the Delta StructType, then test the actual Arrow-to-Delta-to-Arrow conversion and stored values. Cover null parent structs versus null children, nullable list elements/map values, non-null nested violations, field order and metadata. For timestamps test seconds/millis/micros/nanos, timezone-aware versus timezone-free meaning, precision loss and overflow. Verify nanosecond feature availability in the actual consumer, not only the reference envelope. Reject or explicitly normalize unsupported/lossy cases.',
 'Representative nested/null/timestamp batches, explicit schema and allowed normalization policy.', 'Validated conversion contract and roundtrip value expectations or rejected inputs.', 'Conversions can change logical type/precision/nullability; successful writes persist those choices.',
 ['baseline:content/topics/schema-and-types.md','baseline:content/index/coverage.tsv','cargo:crates/core/src/writer/record_batch.rs'],['buoyant_kernel::schema::StructType','deltalake_core::writer::record_batch::RecordBatchWriter'],
 'No nested/timestamp probes run here. Do not claim all timestamps normalize identically or all Arrow types are representable; unit/type/feature behavior requires exact tests.')
add('H03',['VacuumMode::Full with protected versions','VacuumMode::Lite','Retain everything until policy established'],
 'Use Full when orphan files untracked by the log must be discovered; Lite is cheaper but targets eligible log-known removals and cannot discover arbitrary storage orphans. Supply with_keep_versions(required_versions) to protect files referenced by those snapshots, retain adequate retention and preview first. This keep-versions API is experimental. Preserve the corresponding logs/checkpoints through a separate history-retention policy; file protection alone cannot guarantee old-version loading.',
 'Root inventory, log removals, retention, explicit historical versions to preserve.', 'Eligible deletion candidates excluding active/protected/recent files.', 'Full lists storage; actual execution deletes unreferenced eligible files irreversibly.',
 ['baseline:content/api/deltalake_core.operations.vacuum.md','cargo:crates/core/src/operations/vacuum.rs'],['deltalake_core::operations::vacuum::VacuumMode','deltalake_core::operations::vacuum::VacuumBuilder::with_keep_versions'],
 'No destructive vacuum executed. Verify protected versions by loading before and after, and separately protect long-running readers/uncommitted writer files.')
add('H04',['GlueDataCatalog name-to-location resolution','UnityCatalog lookup and DataFusion catalog providers','Hardcoded storage URL/manual catalog replica'],
 'For Glue, build GlueDataCatalog::from_env or with_config and call DataCatalog::get_table_storage_location(catalog_id,database,table); open the resolved Delta URL with the correct backend handlers and credentials, then register its provider. For Unity use UnityCatalog and its built-in UnityCatalogList/UnityCatalogProvider/UnitySchemaProvider to expose catalog.schema.table to DataFusion; use supported credential-vending configuration. Discover via the catalog while the Delta log remains authoritative for state.',
 'Catalog identifier, database/schema/table, authenticated catalog client and backend configuration.', 'Resolved location or DataFusion catalog/schema/provider hierarchy.', 'Catalog and storage reads, credential refresh/lookup; no cloud mutation required for discovery.',
 ['baseline:content/traits/DataCatalog.md','baseline:content/api/deltalake_catalog_glue.md','baseline:content/api/deltalake_catalog_unity.datafusion.md','baseline:content/topics/catalogs.md','baseline:content/index/coverage.tsv'],['deltalake_catalog_glue::GlueDataCatalog','deltalake_catalog_unity::datafusion::UnityCatalogList','deltalake_catalog_unity::datafusion::UnityCatalogProvider','deltalake_catalog_unity::datafusion::UnitySchemaProvider'],
 'Cloud auth, catalog refresh/caching, feature flags and deployment behavior are unverified; consumer enables datafusion only and no cloud operation was run.')
add('H05',['RecordBatchWriter::flush_and_commit','flush plus manual CommitBuilder','Assume write/flush publishes'],
 'Construct RecordBatchWriter::for_table, write batches and choose flush_and_commit(&mut table) for ordinary publication. write streams/buffers Parquet and can stage files as target sizes are reached; flush finalizes/stages the window and returns Vec<Add> but does not publish them. If manually committing those Adds, own metadata/protocol and transaction handling too. flush_and_commit stages then commits Adds (and evolved metadata where applicable), updates table state and returns the new version.',
 'Loaded table, conforming batches, writer window and optional CommitProperties.', 'write: (); flush: Add actions; flush_and_commit: committed version and updated table handle.', 'Only log publication makes rows visible. I/O failure can abort a whole flush window; validation-before-storage errors can preserve it. Failed commit keeps this writer window staged for retry, but outcome still needs reconciliation.',
 ['baseline:content/api/deltalake_core.writer.record_batch.md','baseline:content/traits/DeltaWriter.md','cargo:crates/core/src/writer/mod.rs','cargo:crates/core/src/writer/record_batch.rs'],['deltalake_core::writer::record_batch::RecordBatchWriter','deltalake_core::writer::DeltaWriter::flush','deltalake_core::writer::DeltaWriter::flush_and_commit'],
 'Manual flush + commit must also carry evolved metadata. The source notes table/writer identity is not guaranteed by the passed handle; keep them bound. No low-level writer runtime fixture here.')
add('H06',['Keep recommendation because old API unchanged','Reassess candidates at new pin','Automatically migrate to newest built-in'],
 'Reopen the candidate comparison: discover the new built-in, compare requirements, inputs/outputs/effects, extension limits and simpler alternatives against the old choice, and retain or revise the recommendation with an explicit reason. Refresh exact-pin signatures, source/protocol/feature envelope and runnable evidence. An unchanged API signature does not prove unchanged semantics, defaults or relative suitability. Mark old negative/absence claims and performance assumptions for revalidation; rerun affected consumer probes before presenting refreshed evidence as current.',
 'New immutable commit/kernel/dependency pins, candidate-surface diff and prior requirements.', 'Versioned recommendation, coverage/unknowns and fresh evidence references.', 'Reference/evidence update only until a separately reviewed application change is chosen.',
 ['baseline:SKILL.md','baseline:content/catalogs/operations.md','baseline:content/index/coverage.tsv'],['deltalake_core::table::DeltaTable'],
 'No actual future revision or new built-in was supplied; this is a decision procedure, not a claim that a named future capability exists.')
assert len(responses)==38
source=(OUT/'consumer/tests/decisions.rs').read_text(); test_names=re.findall(r'async fn (e\d+[ab]_[a-z0-9_]+)\(',source)
for item in responses:
    matching=[n for n in test_names if n.split('_')[0].upper()==item['id']]
    item['verification']={'status':'passed' if matching else 'not_run','kind':'consumer_runtime' if matching else 'source_supported_decision','test':matching[0] if matching else None,'log':'cargo-test-03.log' if matching else None}
    if matching: item['evidence'].append('output:consumer/tests/decisions.rs')
roots={'baseline':str(BASE),'cargo':str(CARGO),'datafusion':str(DF),'output':str(OUT)}
doc={'evaluation':'blind_baseline','verified_date':'2026-09-18','profile':task_doc['profile'],'source_roots':roots,'scope':'Frozen baseline reference only plus exact Cargo-cache dependency source; no network, no live cloud, no active/sibling skills or expected answers.','responses':responses}
(OUT/'responses.json').write_text(json.dumps(doc,indent=2)+'\n')
map_items=[]
for item in responses:
 refs=[]
 for ref in item['evidence']:
  kind,rel=ref.split(':',1); path=Path(roots[kind])/rel
  refs.append({'reference':ref,'path':str(path),'sha256':hashlib.sha256(path.read_bytes()).hexdigest()})
 test=item['verification']['test']; span=None
 if test:
  lines=source.splitlines(); start=next(i for i,l in enumerate(lines,1) if 'async fn '+test+'(' in l)
  end=next((i for i in range(start,len(lines)+1) if lines[i-1]=='#[tokio::test]'),len(lines)+1)-1
  span={'path':str(OUT/'consumer/tests/decisions.rs'),'function':test,'start_line':start,'end_line':end}
 map_items.append({'id':item['id'],'canonical_symbols':item['canonical_symbols'],'references':refs,'consumer_test':span})
(OUT/'source-map.json').write_text(json.dumps({'roots':roots,'tasks':map_items},indent=2)+'\n')
print('Wrote',len(responses),'responses and',len(test_names),'consumer-test mappings')
