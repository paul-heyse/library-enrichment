# Evidence 05 — Delta Lake: operations and the DataFusion seam

**Source:** the `deltalake` capability repository, read 2026-09-16.
Covers goal **(3)** persistence in a Delta basis and goal **(4)** relational actions done
relationally. **This dossier contains the most important correction in the round-1 evidence.**

---

## 1. Correction: `DeltaScan` implements 7 of `TableProvider`'s methods, not all of them

Evidence 01 §5 proposed that the proposal's merge passes become SQL `MERGE INTO` statements via
`TableProvider::merge_into`. **That is not available.** Read from
`content/index/methods.tsv`, the methods `DeltaScan` implements via
`datafusion_session::table::TableProvider` are exactly:

```
scan                       schema                  table_type
supports_filters_pushdown  insert_into
get_logical_plan           get_table_definition
```

Absent, and therefore falling back to the trait's defaults: `merge_into`, `update`,
`delete_from`, `truncate`, `constraints`, `statistics`, `get_column_default`, `scan_with_args`.

Consequences:

- **Mutations do not go through SQL DML on the provider.** They go through delta-rs's own
  operation builders (§2). The work stays inside DataFusion — the builders take DataFusion plans
  and session state — but the *seam* is a builder, not a `LogicalPlan::Dml` node.
- `constraints()` returning `None` means DataFusion's optimizer gets no primary-key or uniqueness
  information from a Delta table, so the functional-dependency reasoning of evidence 01 §3 does
  not fire. **Probe PB02 (evidence 11) measured what that costs:** with a key declared, a
  `SELECT DISTINCT key` loses its aggregation entirely and a self-join on the key becomes a
  `LeftSemi Join`. **A thin wrapping `TableProvider` that delegates `scan` and adds
  `constraints`/`get_column_default`/`scan_with_args` is therefore justified, and belongs in the
  consolidated extension module (goal 5).** `statistics` is deliberately *not* in that list —
  see the next bullet.
- `statistics()` returning `None` **does not mean cardinality is absent** — corrected by probe
  PB02b (evidence 11). delta-rs supplies statistics at the `ExecutionPlan` level instead:
  `DeltaScanExec` reports `Rows=Exact(n)` from the transaction log plus per-column
  min/max/null/distinct from Parquet. Implementing `statistics()` on a wrapper adds nothing and
  is ignored.

Also present and useful:

```
deltalake_core::delta_datafusion::DeltaTableFactory            impl TableProviderFactory
deltalake_core::delta_datafusion::cdf::scan::DeltaCdfTableProvider  impl TableProvider
deltalake_core::delta_datafusion::{DeltaLogicalCodec, DeltaPhysicalCodec}
deltalake_core::delta_datafusion::{DataFusionMixins, DeltaColumn, expr, planner, session, engine}
```

`DeltaTableFactory` means Delta tables can be created by `CREATE EXTERNAL TABLE` and registered
through `SessionStateBuilder::with_table_factories`. **`DeltaCdfTableProvider` means the change
feed is itself a queryable table** — incremental re-derivation is a SQL join against a table, not
a bespoke diffing routine. The two codecs make plans containing Delta scans serialisable, which
is what allows a decision packet to carry a plan rather than a rendered string.

---

## 2. The operation builders

Modules under `deltalake_core::operations`:

```
create  write  merge  update  delete  load  load_cdf
constraints  drop_constraints  add_column  drop_column_not_null  add_feature
update_field_metadata  update_table_metadata  set_tbl_properties
optimize  vacuum  restore  filesystem_check  generate  convert_to_delta
get_num_idx_cols_and_stats_columns  CustomExecuteHandler
```

### `MergeBuilder` — the merge passes

```rust
fn new<E: Into<Expression>>(log_store: LogStoreRef, snapshot: Option<EagerSnapshot>, predicate: E, source: DataFrame) -> Self

fn when_matched_update<F>(self, builder: F) -> DeltaResult<MergeBuilder> where F: FnOnce(UpdateBuilder) -> UpdateBuilder
fn when_matched_delete<F>(self, builder: F) -> DeltaResult<MergeBuilder> where F: FnOnce(DeleteBuilder) -> DeleteBuilder
fn when_not_matched_insert<F>(self, builder: F) -> DeltaResult<MergeBuilder> where F: FnOnce(InsertBuilder) -> InsertBuilder
fn when_not_matched_by_source_update<F>(self, …) -> DeltaResult<MergeBuilder>
fn when_not_matched_by_source_delete<F>(self, …) -> DeltaResult<MergeBuilder>

fn with_session_state(self, state: Arc<dyn Session>) -> Self
fn with_merge_schema(self, merge_schema: bool) -> Self
fn with_source_alias<S: ToString>(self, alias: S) -> Self
fn with_target_alias<S: ToString>(self, alias: S) -> Self
fn with_safe_cast(self, safe_cast: bool) -> Self
fn with_streaming(self, streaming: bool) -> Self
fn with_commit_properties(self, commit_properties: CommitProperties) -> Self
fn with_writer_properties(self, writer_properties: WriterProperties) -> Self
fn with_session_fallback_policy(self, policy: SessionFallbackPolicy) -> Self
fn with_custom_execute_handler(self, handler: Arc<dyn CustomExecuteHandler>) -> Self
```

**`source: DataFrame` is the load-bearing detail.** The merge source is an arbitrary DataFusion
DataFrame — which means it can be any query over the unified catalog, including recursive CTEs
and UDF-bearing projections. The proposal's Passes A–F therefore remain fully relational:

```
Pass A  package/crate/source identity     MERGE source = SELECT … FROM staging.cargo_metadata
Pass B  declarations before children      MERGE source = SELECT … ORDER BY depth
Pass C  non-identity mappings             MERGE into snapshot.native_binding
Pass D  semantic and operational facts    MERGE into program.* from hir/mir staging
Pass E  conservative type normalisation   MERGE into program.type_term
Pass F  derived behaviour                 MERGE into evidence.* with Derivation rows
```

with `with_merge_schema(true)` giving schema evolution at merge time, and `when_not_matched_by_source_*`
covering the retraction case the proposal needs when an extraction run no longer reports a fact.

### `WriteBuilder` — bulk load

```rust
fn with_input_plan(self, plan: LogicalPlan) -> Self
fn with_input_execution_plan(self, plan: Arc<LogicalPlan>) -> Self
fn with_input_batches(self, batches: impl IntoIterator<Item = RecordBatch>) -> Self
fn with_partition_columns(self, partition_columns: impl IntoIterator<Item = impl Into<String>>) -> Self
fn with_replace_where(self, predicate: impl Into<Expression>) -> Self
fn with_save_mode(self, save_mode: SaveMode) -> Self
fn with_schema_mode(self, schema_mode: SchemaMode) -> Self
```

`with_input_plan(LogicalPlan)` is the direct path from a query to a table — no intermediate
materialisation. `with_replace_where` gives idempotent per-run reload: re-running extraction
family `F` for snapshot `S` replaces exactly `family = 'F' AND snapshot_id = 'S'`.

### `ConstraintBuilder` — validation committed to the table

```rust
fn with_constraint<S: Into<String>, E: Into<Expression>>(self, name: S, expression: E) -> Self
fn with_constraints<S: Into<String>, E: Into<Expression>>(self, constraints: HashMap<S, E>) -> Self
fn with_session_state(self, session: Arc<dyn Session>) -> Self
```

Named CHECK constraints whose expression is a DataFusion `Expression`, enforced by the writer.
Paired with `drop_constraints::DropConstraintBuilder`. This is where the proposal's structural
invariants live — a row that violates one cannot be committed, so no validation pass is needed.

### `CdfLoadBuilder` — incremental re-derivation

```rust
fn with_starting_version(self, starting_version: Version) -> Self
fn with_ending_version(self, ending_version: Version) -> Self
fn with_starting_timestamp(self, timestamp: DateTime<Utc>) -> Self
fn with_ending_timestamp(self, timestamp: DateTime<Utc>) -> Self
fn with_allow_out_of_range(self) -> Self
```

### `OptimizeBuilder` — layout maintenance

```rust
enum OptimizeType { Compact, ZOrder }

fn with_type(self, optimize_type: OptimizeType) -> Self
fn with_filters(self, filters: &'a [FilterLiteral<'a>]) -> Self
fn with_target_size(self, target: NonZeroU64) -> Self
fn with_max_concurrent_tasks(self, max_concurrent_tasks: usize) -> Self
fn with_min_commit_interval(self, min_commit_interval: Duration) -> Self
fn with_preserve_insertion_order(self, _preserve_insertion_order: bool) -> Self
fn with_session_state(self, session: Arc<dyn Session>) -> Self
```

Z-ordering on `(entity_id, kind)` is what keeps the identity joins cheap after many merge passes.

---

## 3. The recurring `with_session_state` pattern

`MergeBuilder`, `ConstraintBuilder`, `OptimizeBuilder` and others all take
`Arc<dyn Session>`. **One `SessionState` — carrying the catalog, the UDF registry, the extension
types and the custom rules from evidence 01 §4 — is threaded through every Delta operation.**
That is the structural answer to goals (2) and (5): a UDF registered once is available inside a
merge predicate, a CHECK constraint, an optimize filter and a decision-packet query alike.

`SessionFallbackPolicy` appears on several builders and governs what happens when no session is
supplied; the plan always supplies one, but the policy should be set explicitly rather than
defaulted.

## 4. Open questions for round 2 — three of four now answered

1. ~~Whether the wrapping `TableProvider` can usefully supply `constraints()`.~~ **ANSWERED —
   yes** (probe PB02, evidence 11): a declared primary key eliminates a `DISTINCT` aggregation
   and converts an inner join to a semi-join. ~~Open follow-up is narrower: deriving those
   `Constraints` *automatically* from the table's own Delta `CheckConstraints`, rather than
   declaring them twice.~~ **ANSWERED — no, not in general** (probe PB12, evidence 12): Delta
   stores a constraint as SQL **text** (`delta.constraints.off_nonneg = "off >= 0"`), while
   DataFusion's `Constraints` models only `PrimaryKey`/`Unique` over column indices. An arbitrary
   CHECK has no slot in that type. Declare the key on the wrapper, write the matching CHECK
   separately, and reconcile the two with a build-time consistency check.
2. ~~Whether `MergeBuilder` predicates can reference UDFs registered on the supplied session.~~
   **ANSWERED — yes** (probe PB05, evidence 12), in both the join predicate and match-clause
   predicates; a session without the UDF fails with `Invalid function`. **But
   `SessionFallbackPolicy::InternalDefaults` is the default**, and it discards a
   non-`SessionState` session with only a log line — so §3's "one `SessionState` threaded through
   every Delta operation" holds only if every call site sets `RequireSessionState`.
3. ~~Whether `DeltaCdfTableProvider` supports filter pushdown.~~ **ANSWERED — it accepts every
   filter as `Exact` unconditionally, but narrows only on partition columns** (probe PB10,
   evidence 12). So incremental re-derivation is narrowed by **partitioning on `family`** and by
   `with_starting_version`/`with_ending_version`, never by a predicate on `_commit_version` —
   which is correct but reads the whole change range regardless.
4. ~~Whether `WriteBuilder::with_input_plan` participates in the same transaction as a subsequent
   `ConstraintBuilder` call, or whether constraint addition is always a separate commit.~~
   **ANSWERED — always a separate commit** (probe PB13, evidence 13): create at `v0`, write at
   `v1`, constraint at `v2`. More usefully, a constraint add that fails validation rolls back
   **only itself**, leaving the violating row committed and no constraint recorded — whereas
   adding the constraint first rejects the write and nothing lands. So DDL-first is not a way of
   avoiding the question; it is the measured-correct order.

**All four questions from this dossier are now closed.**
