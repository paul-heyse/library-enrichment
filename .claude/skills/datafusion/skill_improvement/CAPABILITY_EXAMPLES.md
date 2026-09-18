# Worked capability references

These examples show the proposed level of decision support. Evidence was inspected 2026-09-18
against DataFusion 55.1.0 and Arrow/Parquet 59.3.0. They are a reviewed sample, not an exhaustive
catalog. Executed observations refer to the seven tests in
[contracts.rs](evidence/probes/tests/contracts.rs), with [results](evidence/logs/rust-probes.log).
Other statements are source/documentation observations or explicitly conditional recommendations.

## 1. Select rows or columns without leaving Arrow

**Task:** transform an existing batch while retaining columnar data. First determine the selection
representation and desired order; “filter rows” is too imprecise to choose the operation.

| Requirement | Candidate | Input → output | Important distinction |
|---|---|---|---|
| Contiguous row range | `RecordBatch::slice` / array slicing | Batch/array plus offset and length → sliced representation | Sharing buffers can retain the source allocation; a small logical result does not imply small retained memory. |
| Select/reorder columns | `RecordBatch::project` | Batch plus column indices → batch | Projection changes columns, not which rows qualify. |
| Keep positions where a predicate is true | `arrow_select::filter::filter_record_batch` | `&RecordBatch`, `&BooleanArray` → `Result<RecordBatch, ArrowError>` | Preserves selected source-row order; false/null predicate positions are not selected. |
| Reuse the same predicate for several arrays/batches | `FilterBuilder` → `FilterPredicate` | Boolean mask → reusable selection representation | Reuses predicate processing; review when `.optimize()` is worthwhile. It is not a mandatory performance improvement for every shape. |
| Gather/reorder/repeat indexed positions | `arrow_select::take::take` / `take_record_batch` | Array/batch plus integer indices → selected array/batch | Index order and duplicates matter; nullable indices can introduce null values. |
| Selection over a relation with optimizer visibility | DataFusion filter/projection/order expressions | Relational plan → new relational plan | Use when selection belongs inside the query; calling a kernel after collection hides that work from query optimization. |

**Inputs and outputs worth knowing before implementation.** `take` has a `TakeOptions` argument;
`take_record_batch` does not. In the retained implementation the batch convenience function calls
`take(..., None)` on each column. If index validity is not already established, the array-level
operation with `check_bounds: true` gives a checked error path. The documented unchecked option
can panic on out-of-range indices; a `Result` return type does not imply all invalid inputs become
`Err`. Selected arrays may share buffers where the kernel can avoid allocation.

`take` with indices `[2, 0, 2, null]` over `[10, 20, 30]` returned `[30, 10, 30, null]` in the probe.
Checked index `3` returned an error. The mask `[true, null, true]` selected rows zero and two, and
the direct batch filter preserved the tested field and schema metadata. That observation is not
a guarantee about all DataFusion operators or metadata transformations.

**Composition boundary:** a zero-column batch still has a row count. The probe constructs three
rows with `RecordBatchOptions`; filtering returns two rows, while this release's
`take_record_batch` returns an error because it reconstructs with `RecordBatch::try_new` and has
no column from which to infer a row count. For that case, preserve the row count explicitly in
your composition. Do not infer that two batch-selection APIs have interchangeable domains.

Also distinguish array-level null output from a batch's field contract: `RecordBatch::try_new`
rejects the tested null-containing array when its top-level field is declared non-nullable. A
nullable gather index therefore needs compatible output schema handling.

**Evidence:** [selection source](evidence/sources/arrow-select-59.3.0/src/take.rs),
[filter source](evidence/sources/arrow-select-59.3.0/src/filter.rs),
[batch construction](evidence/sources/arrow-array-59.3.0/src/record_batch.rs).
Executed tests: `take_preserves_requested_order_duplicates_and_null_indices`,
`filter_null_is_not_selected_and_metadata_is_retained`,
`empty_schema_exposes_different_batch_selection_contracts`, and
`batch_construction_checks_top_level_nullability`.

## 2. Encode composite keys for comparison

**Task:** compare multiple Arrow columns repeatedly, or build local grouping/sorting machinery
around columnar input. Candidate: `arrow_row::RowConverter`, with `SortField` per column.

**Choose when:** a row-comparison representation is useful inside an algorithm that is already
working directly with Arrow arrays. Prefer an existing DataFusion sort/group/distinct operation
when it expresses the whole relational task. Also consider `arrow_ord::sort::lexsort_to_indices`
for a direct sorting-index result rather than maintaining your own encoded rows.

**Contract.** Construct the converter from ordered data types and per-field `SortOptions`.
`convert_columns(&[ArrayRef]) -> Result<Rows, ArrowError>` creates a comparison-oriented byte
representation. `Rows` exposes borrowed `Row` values; an `OwnedRow` owns one row's bytes when a
row must outlive the borrowed view. `convert_rows` returns `Vec<ArrayRef>`; it does not reconstruct
field names, field metadata, or a complete batch schema for you.

Rows used for comparison must come from the same converter, according to the documented contract.
Keep converter configuration/lifetime associated with the encoded rows. Choose null placement and
descending order deliberately. Comparisons are not a substitute for every application's equality,
collation, or canonical-identity rules. Characterize floats, nested types, and extension semantics
before treating row comparisons as domain equality.

**A particularly important output detail:** dictionary inputs are hydrated. The probe encoded a
`Dictionary<Int8, Utf8>`, decoded it, and observed a `Utf8` array with the same values, not a
dictionary array. Value preservation and physical-type preservation are separate properties.

**Boundary:** the upstream type documentation says row encoding may change between releases.
Do not select it as a durable interoperable key format merely because bytes are accessible. IPC,
Parquet, or an application-defined canonical encoding address different persistence requirements.
If persistent ordering is required, define and test that contract explicitly.

**Evidence:** [module and implementation source](evidence/sources/arrow-row-59.3.0/src/lib.rs),
[existing API page](../content/api/arrow_row.md).
Executed tests: `row_order_changes_with_null_placement` and
`row_decode_hydrates_dictionary_and_preserves_values`. No comparative throughput benchmark was run.

## 3. Convert values with an explicit failure policy

**Task:** convert Arrow arrays to another `DataType`. Candidate:
`arrow_cast::cast::cast_with_options`; DataFusion cast expressions are the neighboring choice when
conversion belongs inside a relational query.

**Contract.** Inputs are `&dyn Array`, target `&DataType`, and `&CastOptions`; output is
`Result<ArrayRef, ArrowError>`. A type-pair support query can establish that a cast path exists,
but cannot guarantee that every value is representable or valid. Review the conversion-specific
contract for overflow, parsing, precision/scale, timezone, nested/dictionary representation, and
metadata. These behaviors cannot be inferred from the generic return type.

The `safe` option's name is easy to misinterpret: in this Arrow API `safe: true` uses nulls for
supported casts' value-conversion failures, while `safe: false` requests an error. The default is
`true`. Unsupported type combinations can still be errors; the flag is not a promise that all
casts succeed or preserve information.

**Executed example:** `Utf8["42", "bad", null] → Int32` with default options produced
`[42, null, null]`. With `safe: false` it returned an error; the control with only `"42"` succeeded.
The null result loses the distinction between an input null and a failed conversion unless the
caller separately preserves that information.

**Implementation consequence:** choose the failure policy before deciding whether the target
field may remain non-nullable. An array cast returns an array; field/schema metadata and business
validation are separate responsibilities. Do not transfer this API's `safe` terminology to other
libraries or assume all DataFusion casting entry points use identical defaults.

**Evidence:** [cast implementation and options](evidence/sources/arrow-cast-59.3.0/src/cast/mod.rs),
test `cast_safe_true_nulls_failed_parses_but_false_errors`. The probe covers string-to-Int32 only;
decimal, timestamp, overflow, and nested conversions remain unprobed in this assessment.

## 4. Choose how to execute and consume a DataFrame

**Task:** run a prepared relational query and decide where its output/state will live.

| Operation | Input → output | Choice implication |
|---|---|---|
| `DataFrame::collect(self)` | DataFrame → async `Result<Vec<RecordBatch>>` | Materializes all output before returning it; suitable when retaining that result is intentional. |
| `DataFrame::execute_stream(self)` | DataFrame → async `Result<SendableRecordBatchStream>` | Gives a single output stream to consume incrementally. |
| `DataFrame::execute_stream_partitioned(self)` | DataFrame → async `Result<Vec<SendableRecordBatchStream>>` | Exposes output partitions separately; the caller manages their consumption. |
| `DataFrame::cache(self)` | DataFrame → async `Result<DataFrame>` | Executes/materializes results into a new in-memory DataFrame for reuse. |

These methods consume the DataFrame value. `execute_stream` creates a physical plan before
returning its stream; stream construction and stream consumption have distinct failure points.
The stream yields fallible batches. Returning a stream does not mean data-source metadata access,
planning, and other preparation never happened during the call.

**Memory contract.** Incremental consumption avoids retaining the entire result merely because
of `collect`. It does not prove a bounded working set for the query. Operator state, input buffers,
prefetch, caches, and consumer retention all matter. The retained `with_memory_limit` documentation
explicitly notes that the limit is not respected in all cases. Treat pool-accounted memory and
process memory as different quantities.

The default runtime uses an unbounded pool, while its disk-manager builder defaults to an OS
temporary directory. Those facts do not establish that a particular operator will spill under a
particular workload. Configure memory pressure, spill policy/location/capacity, and concurrency
with the participating operators in mind; inspect the actual plan and metrics.

**Ordering and lifecycle.** A single stream is not an implicit global sort. Require explicit
ordering where the result contract needs it. Partitioned consumption must account for task and
buffer ownership. Upstream documents that dropping the stream aborts execution and frees its
resources; application-owned background tasks or retained batches require their own lifecycle
analysis. Do not turn that statement into a claim about unrelated work.

**Evidence:** [DataFrame methods](evidence/sources/datafusion-55.1.0/src/dataframe/mod.rs),
[runtime](evidence/sources/datafusion-execution-55.1.0/src/runtime_env.rs),
[disk manager](evidence/sources/datafusion-execution-55.1.0/src/disk_manager.rs), and
[memory-pool documentation](evidence/sources/datafusion-execution-55.1.0/src/memory_pool/mod.rs).
Source/documentation reviewed; resource/cancellation behavior was not executed in this assessment.

## 5. Expose a source and advertise only sound pushdown

**Task:** make existing data queryable while reusing as much DataFusion behavior as possible.
Compare existing providers before implementing `datafusion_session::table::TableProvider`:
`MemTable` for materialized batches, `ViewTable` for a logical-plan view, and `ListingTable` with
file-source/format components for supported file-oriented sources. A custom source may still
reuse those components instead of implementing the full execution stack.

**Scan contract at this pin:** `scan(&self, &dyn Session, Option<&Vec<usize>>, &[Expr],
Option<usize>)` asynchronously returns `Result<Arc<dyn ExecutionPlan>>`. It constructs a plan;
it does not return the records themselves. `scan_with_args` takes `ScanArgs` and returns
`ScanResult`; inspect the actual arguments in the selected release rather than transferring
signatures from current-branch examples.

| Input or promise | Obligation |
|---|---|
| Projection | Return the requested output columns in the appropriate order. Columns needed only to evaluate pushed filters may be absent from the output projection. |
| Filter list | Preserve the intended conjunction and its null/type semantics. A supplied predicate is not merely a string to pattern-match loosely. |
| `Exact` | Fully enforce the predicate at the source: the planner may remove the residual filter. |
| `Inexact` | Soundly retain all potentially qualifying rows; false positives may be removed later. Incorrectly dropped matches cannot be recovered. |
| `Unsupported` | Leave evaluation to DataFusion. This can be the correct choice where pruning adds no useful work reduction or soundness is unclear. |
| Limit | Interpret in conjunction with filtering. The retained scan docs describe producing at least the requested qualifying count when available, potentially more; inexact filtering prevents this limit pushdown. It is not an unconditional cap on unfiltered input. |
| Statistics/constraints | Report what is actually known. An optimizer-facing declaration does not itself enforce source data integrity. |

The documented logical order is filters → limit → projection. Reducing I/O using statistics and
evaluating every predicate row-by-row are different capabilities. The reference should explain
which a provider offers, and whether pruning is conservative for nulls, unsupported expressions,
casts, and missing statistics.

**Why this changes the recommendation:** supplying only required methods says nothing sufficient
about speed or correctness. `scan` already receives the limit; a minimal implementation can use
it correctly. Likewise, “approximate is safe” must mean no false negatives, not merely “often right.”

**Verification to add during implementation:** compare complete results—including multiplicities
and null behavior—with an unpruned baseline; project away a filter column; use small limits and
false-positive candidates; inspect the residual filter in the physical plan. Test a deliberately
incorrect exactness claim as a negative control. Those provider execution tests were not run here.

**Evidence:** [scan contract](evidence/sources/datafusion-session-55.1.0/src/table.rs),
[provider topic and candidates](../content/topics/custom-table-providers.md),
[upstream guide retained in the skill](../content/corpus/guides/library-user-guide/custom-table-providers.md).
Soundness and test recommendations are interpretations of that contract.

## 6. Build, type-check, simplify, or evaluate an expression

**Task:** transform or evaluate an expression without implementing another expression engine.
Distinguish the phase before choosing an API:

| Need | Candidate family | Input/output and precondition |
|---|---|---|
| Construct a built-in function call | Existing `expr_fn` helper / function object | Expressions → logical expression; discover the built-in before writing a custom scalar function. |
| Ask its type/nullability | `ExprSchemable` | Expression plus schema context → type/nullability information; this is not data execution. |
| Traverse/rewrite trees | `TreeNode` and expression rewriter utilities | Tree plus visitor/rewriter → transformed tree/control result; preserve required semantic properties. |
| Coerce and simplify | `ExprSimplifier` and its context | Expression + schema/context → compatible/simplified expression; coercion and simplification are distinct operations. |
| Evaluate against batches | Physical expression construction/evaluation | Logical expression + appropriate schema/planning context → physical expression; batch → `ColumnarValue`. |
| Execute a relation | Logical/physical plan APIs | Expressions embedded in a plan → record-batch output with relational semantics. |

The retained `ScalarUDF::call` wraps arguments in an `Expr::ScalarFunction`; it does not establish
that all argument types have been analyzed/coerced. `ExprSimplifier::simplify` expressly expects
types compatible with the operators and points to `coerce` for preparation. An agent should know
this before constructing trees outside the ordinary SQL/DataFrame planning route.

If a custom function is necessary, its implementation contract is broader than `invoke`:
argument signature/coercion, return field and nullability, volatility, scalar-versus-array
handling, simplification, ordering, and bounds propagation can affect planning and execution.
Expose the hooks relevant to the function's semantics; do not advise implementing every optional
hook regardless of need. A false optimizer claim can be worse than leaving a hook unimplemented.

**Evidence:** [call construction and function contracts](evidence/sources/datafusion-expr-55.1.0/src/udf.rs),
[simplifier](evidence/sources/datafusion-optimizer-55.1.0/src/simplify_expressions/expr_simplifier.rs),
[existing expressions topic](../content/topics/expressions.md).
Source/documentation reviewed; no standalone expression runtime probe was run here.

## 7. Skip Parquet work at the appropriate stage

**Task:** avoid reading or decoding data that cannot contribute to the result. Start with the
DataFusion Parquet provider/source when its scan pipeline expresses the task. Use lower-level
`parquet` reader controls when building or adapting the reader itself.

| Mechanism | What its input means | What it changes |
|---|---|---|
| Row-group selection | Chosen row-group indices | Removes whole row groups before later row selection |
| `ProjectionMask` | Selected schema roots or leaves, according to construction | Restricts columns decoded; nested leaf numbering is not simply output column numbering |
| `RowSelection` | Row ranges to skip/select in the retained row-group sequence | Restricts row decoding; positions must account for previously excluded row groups |
| `RowFilter` / `ArrowPredicateFn` | Predicates with their own column projections | Evaluates row predicates during decoding, allowing later work to avoid rejected rows |
| Reader offset/limit | Counts after the documented row-selection/filter stages | Controls the final qualifying range rather than blindly truncating unfiltered input |

`ArrowPredicate::evaluate` receives a `RecordBatch` containing the predicate's projected columns
and returns `Result<BooleanArray, ArrowError>` with the same length. True retains a row; false
or null rejects it. A `RowFilter` combines its predicates conjunctively. The predicate projection
and final output projection serve different purposes.

The retained builder documentation explicitly places row-group selection before `RowSelection`,
and row filters after both. A selection calculated in original-file row coordinates can therefore
be wrong after excluding a row group. This is the kind of input contract that belongs in a brief,
not behind several thousand lines of API inventory.

**Implementation considerations:** page-index availability can affect how much physical work is
skipped. Distinguish conservative pruning from exact row filtering, and test values as well as
read/decode metrics. Predicate ordering, required columns, storage latency, and selectivity affect
the benefit; a method's availability alone is not a performance result. Reader-side limits and
DataFusion provider limit hints should not be conflated.

**Evidence:** [reader builder](evidence/sources/parquet-59.3.0/src/arrow/arrow_reader/mod.rs),
[predicate/filter contracts](evidence/sources/parquet-59.3.0/src/arrow/arrow_reader/filter.rs),
[row-selection representation](evidence/sources/parquet-59.3.0/src/arrow/arrow_reader/selection/mod.rs),
[existing Parquet topic](../content/topics/parquet.md).
Source/documentation reviewed; no Parquet I/O or performance probe was run here.
