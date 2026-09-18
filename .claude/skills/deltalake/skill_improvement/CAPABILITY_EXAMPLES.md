# Worked examples of the proposed reference depth

These are planning examples, not the replacement skill. Each identifies evidence strength and the
remaining verification work. Exact symbols are tied to delta-rs `58f07cd62bfbce3649a7e1c87c696288068ae184`
and the profile in [assessment](ASSESSMENT.md).

## 1. Read rows with the required freshness

**Choice:** use `DeltaTable::table_provider()` for DataFusion query composition; use
`DeltaTable::scan_table()` for the existing projected-stream convenience path. A raw Parquet
directory reader is a different capability: it does not apply Delta's active-file state.

**Inputs and outputs:** `table_provider(&self)` yields `TableProviderBuilder`; awaiting it yields
`datafusion::error::Result<Arc<dyn TableProvider>>`. `scan_table(&self)` yields an inferred
`LoadBuilder`; awaiting yields `DeltaResult<(DeltaTable, SendableRecordBatchStream)>`. The
unnameable builder can be used through its public returned value. Do not invent an import through
its private module. A stream output does not establish a bound on all planning/operator memory.

**State contract:** a loaded table contributes its snapshot when the provider is built. A provider
does not refresh when another table handle advances. Refresh the table and build/register the
provider corresponding to the desired snapshot. If repeatability is the requirement, retain the
old provider intentionally.

**Non-obvious precedence:** the provider builder uses `table_version` only when it must load a
snapshot. `table.table_provider()` on a loaded table already supplies a snapshot. Appending
`.with_table_version(0)` does not replace that snapshot in this pin. Explicitly load the desired
version on a table handle or use a builder whose inputs require loading that version. This
distinction belongs alongside the setter, not buried in source.

**Evidence:** [provider construction](evidence/sources/delta-rs/crates/core/src/delta_datafusion/table_provider.rs),
[load output](evidence/sources/delta-rs/crates/core/src/operations/load.rs),
`provider_snapshot_and_unnameable_load_builder` and
`delta_snapshot_excludes_removed_parquet_files` in the [probe](evidence/probes/tests/contracts.rs).
The latest [runtime receipt](evidence/runtime-receipt.json) includes the snapshot/version-precedence
control. Column mapping/deletion-vector scans and invalid import compilation are future controls.

## 2. Retry a write without confusing markers and replay policy

**Choice:** prefer the high-level write plus `CommitProperties` when its built-in behavior meets
the operation's requirements. Escalate to `CommitBuilder` for a concrete need to supply actions or
coordinate the lower-level commit stages; do not implement a transaction loop merely to attach metadata.

**Inputs and outputs:** `DeltaTable::write(self, batches)` consumes the handle and builds a write;
the awaited successful output is the updated table. `with_application_transaction(Transaction::new(app, n))`
adds a persisted application transaction marker. Query it through the snapshot's
`transaction_version(log_store, app)` API. Application version and Delta table version are distinct.

**Replay contract:** the local sequential probe appends the same batch twice with the same marker
and observes two rows. The marker is not automatic duplicate-write suppression. A caller can inspect
recorded progress and decide to skip a replay; a concurrent exactly-once protocol needs additional
analysis of conflicts, publication races and marker retention. A successful single-thread lookup
is not evidence that a check-then-write sequence is race-free.

**Failure contract:** the high-level await may return an error after log publication. The injected
post-commit-hook test observes the new version after such an error. Design recovery around the
observed table/marker state and operation phase, rather than assuming `Err` means “nothing happened.”
This does not imply that every error publishes data or that every conflict is safely retryable.

**Evidence:** [commit stages and properties](evidence/sources/delta-rs/crates/core/src/kernel/transaction/mod.rs),
[conflict checks](evidence/sources/delta-rs/crates/core/src/kernel/transaction/conflict_checker.rs),
and the `repeated_transaction_marker_does_not_suppress_sequential_append` /
`operation_error_can_follow_a_durable_commit` tests. These test log visibility in memory storage;
they do not establish crash durability or remote-store guarantees.

**Implementation consequence for the skill:** replace the current syntax hint's guarantee with a
question about actual retry policy and commit location. A `.with_commit_properties(...)` call may
contain only descriptive metadata. A `.write(...)` call may belong to a staging writer or an
unrelated client. Preserve [syntax controls](evidence/query-fixture.rs) demonstrating both limits.

## 3. Preserve an existing DataFusion execution policy

**Choice:** many DML builders accept `with_session_state(Arc<dyn Session>)`, but the trait object's
nominal type does not guarantee preservation of the caller's execution environment.

At this pin, `SessionFallbackPolicy` distinguishes:

| Caller/choice | Source-described behavior | Design implication |
|---|---|---|
| Concrete SessionState | The resolver recognizes the concrete state | Reuse the intended state, while still inspecting operation-specific planner changes |
| Other Session, InternalDefaults | Warn and use internal defaults | Do not assume custom runtime/config/UDFs are honored solely because the setter accepts the object |
| Other Session, DeriveFromTrait | Derive a state using trait-exposed capabilities | Review which runtime/config/registries survive and which concrete-state details cannot |
| Other Session, RequireSessionState | Return an error | Useful when silent fallback would violate the caller's required policy |

The provider builder's `with_session` supplies scan configuration defaults; that is not the same
contract as operation session resolution. Object-store ensure-registration also leaves an existing
mapping in place. Its successful return is not proof that a pre-existing mapping is the desired one.

**Evidence strength:** exact [session source](evidence/sources/delta-rs/crates/core/src/delta_datafusion/session.rs)
and [write member docs](evidence/sources/delta-rs/crates/core/src/operations/write/mod.rs).
The new custom-Session/UDF/runtime control is proposed, not executed. Preserve the full member
documentation—the policy detail currently exceeds a short summary.

## 4. Select the right write or merge path

**Choice:** distinguish append, full overwrite, predicate replacement and key-based merge before
choosing a builder. Avoid materializing a source that already exists as an appropriate logical plan:
`WriteBuilder::with_input_plan` consumes `LogicalPlan`. The deprecated
`with_input_execution_plan` also takes a logical plan wrapped in Arc, despite its name.

For predicate replacement, the exact method is `with_replace_where`, used with overwrite mode.
Schema mode, cast policy and partition layout are additional independent dimensions. At this pin
write casting uses `with_cast_safety`; merge/update use `with_safe_cast`. Discover by intent while
returning the exact owner-specific method, not one guessed name for the whole family.

For merge, the `when_*` methods construct ordered clauses through closures and can return
`DeltaResult<MergeBuilder>` before the final await. The helper `merge::UpdateBuilder` is distinct
from `operations::update::UpdateBuilder`. Full input/output contracts must retain closure bounds,
intermediate `?`, aliases, source schema and result/metric shapes. Do not turn a textual occurrence
of “UpdateBuilder” into a constructor edge for the standalone update operation.

**Evidence strength:** [write source](evidence/sources/delta-rs/crates/core/src/operations/write/mod.rs),
[merge source](evidence/sources/delta-rs/crates/core/src/operations/merge/mod.rs) and raw type records.
The proposed write/merge matrix still needs executed positive/negative fixtures for replacement,
duplicate matches, null keys, clause order, schema changes and resource behavior. `with_streaming`
alone is not a proof of bounded whole-operation memory.

## 5. Read change events, including their eligibility conditions

**Choice:** a snapshot query answers current rows; CDF answers changes over an eligible interval.
Route `DeltaTable::scan_cdf` to `CdfLoadBuilder`, then either use
`DeltaCdfTableProvider::try_new(builder)` for DataFusion composition or explicitly build an
`Arc<dyn ExecutionPlan>` with the required session. This is not an IntoFuture builder.

The interval, change metadata/schema, table enablement and retained history are part of the input
contract. Ending version is documented as inclusive; timestamp interpretation and out-of-range
policy need their own cases. Do not infer behavior from the version method's name or from Python docs.

**Predicate distinction:** internal partition pruning is not the complete row filter. The CDF
provider wraps the physical predicate in `FilterExec`. A lower-level plan composition must preserve
the row-level correctness requirement rather than assuming skipped files implement it.

**Operation-specific feature support:** even if column mapping is admitted by ordinary table reads,
this pin's CDF build path explicitly rejects a non-None column-mapping mode. This is why support must
be represented per operation, not once per library or enum.

**Evidence strength:** [CDF builder](evidence/sources/delta-rs/crates/core/src/operations/load_cdf.rs)
and [provider](evidence/sources/delta-rs/crates/core/src/delta_datafusion/cdf/scan.rs), source-verified.
Boundary, change-image and residual-filter runtime tests are proposed. Resume offsets belong to the
caller; do not promise an event-processing protocol from a finite plan-returning API.

## 6. Ask whether a table feature is usable for this operation

The current catalogs provide two useful vocabularies: Delta wrapper features and kernel features.
They must not be flattened into a list of certified capabilities. A reviewed row should look like:

| Feature | Recognized | Compiled admission evidence | Operation boundary | Runtime status here |
|---|---|---|---|---|
| IdentityColumns | Present in wrapper/kernel vocabularies | Not inserted into this pin's default writer feature set | Do not infer identity-column writes from enum membership | not_run |
| ColumnMapping | Present | Reader/writer admission inserts are gated on DataFusion | CDF build explicitly rejects mapped tables; ordinary reads need their own fixture | not_run |
| TimestampNanos | Present | Admission gated by `nanosecond-timestamps` | Docs capture enables it; local probe profile does not | not_run |

This sample matrix is deliberately partial. Inspect legacy protocol-version handling and actual
feature requirements as well as explicit feature sets. Table property mutation and protocol
upgrade are not interchangeable actions. Preserve the difference between a supported high-level
operation, kernel support and a feature merely parsed/preserved.

**Evidence:** [protocol checker](evidence/sources/delta-rs/crates/core/src/kernel/transaction/protocol.rs),
[CDF check](evidence/sources/delta-rs/crates/core/src/operations/load_cdf.rs) and profile artifacts.

## 7. Choose maintenance by the effect required

**Choice:** optimize reorganizes files; checkpointing supports log replay; log cleanup and vacuum
remove different retained artifacts; time travel reads an earlier state; restore creates a current
state based on history. Route by required effect before showing each builder's options.

At this pin, `VacuumBuilder` defaults to **Lite**, `dry_run = false`, and retention enforcement
enabled. Full mode considers orphaned Parquet files in addition to its other work. A task asking
only to preview candidates needs `with_dry_run(true)`. `with_keep_versions` is explicitly described
as experimental. These are concrete configuration contracts, not a generic “defaults are safe” rule.

Outputs include `(DeltaTable, VacuumMetrics)`; the result's metrics distinguish candidates from
actual deletions. The decision brief should join retention, active/historic readers, CDF, kept
versions and restore eligibility. Do not claim old data remains readable merely because a log
version still exists, or describe vacuum as table compaction.

**Evidence strength:** [vacuum source](evidence/sources/delta-rs/crates/core/src/operations/vacuum.rs),
[restore](evidence/sources/delta-rs/crates/core/src/operations/restore.rs),
[optimize](evidence/sources/delta-rs/crates/core/src/operations/optimize.rs).
Controlled-clock dry-run/deletion/time-travel fixtures remain proposed. No actual user table was
subjected to maintenance in this assessment.
