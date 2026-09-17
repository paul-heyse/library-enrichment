# The emitted span contract

Derived from 9 upstream trace snapshots across 10 scenarios. Every row is **`recorded`**: upstream's observation under upstream's harness. `probes.py` promotes to `confirmed` only what a probe reproduces with a control that comes out the other way.

## Spans

| Span | Target | Level | Fields | Seen in | Conditional on |
|---|---|---|---:|---:|---|
| `InstrumentedExec` | `integration_utils` | INFO | 49 | 9/10 | unconditional |
| `Phase` | `integration_utils` | INFO | 4 | 9/10 | unconditional |
| `Rule` | `integration_utils` | INFO | 1 | 9/10 | unconditional |
| `create_physical_plan` | `datafusion_tracing::planner` | INFO | 2 | 9/10 | unconditional |
| `get_opts` ⚠ | `instrumented_object_store::instrumented_object_store` | INFO | 5 | 1/10 | `with_object_store_collection` |
| `get_ranges` ⚠ | `instrumented_object_store::instrumented_object_store` | INFO | 4 | 1/10 | `with_object_store_collection` |
| `parse_sql` | `integration_utils` | INFO | 2 | 9/10 | unconditional |
| `read_query` | `integration_utils` | INFO | 1 | 9/10 | unconditional |
| `run_traced_query` | `integration_utils` | INFO | 1 | 9/10 | unconditional |

## The target is the CALLER's crate, not this one

Every `instrument_with_*_spans!` arm that omits `target:` expands to

```rust
$crate::instrument_with_spans!(target: module_path!(), $lvl, options: $options, ...)
```

and `module_path!()` expands at the **call site**. So the spans this library emits carry the target of whichever crate invoked the macro -- which is why every execution-plan row above reads `integration_utils` rather than `datafusion_tracing`. A subscriber filtered with `EnvFilter::new("datafusion_tracing=info")` therefore receives **nothing**, silently. Filter on your own crate, or pass `target:` explicitly.

`instrumented-object-store` behaves the opposite way: it calls `tracing` directly rather than through a macro, so its spans do carry `instrumented_object_store::instrumented_object_store`. The two halves of this library do not agree about targets, and the table above is the evidence.

## The metrics field vocabulary is open

`datafusion-tracing/src/metrics.rs` names every metric field as

```rust
format!("datafusion.metrics.{}", metric.value().name())
```

so the set of `datafusion.metrics.*` fields is whatever the DataFusion nodes in your plan report, not a list this library defines. What `span-fields.tsv` holds is what was observed under the pinned queries. A field absent from it is **unobserved, not unavailable** -- look at your node's `MetricsSet`, which is DataFusion's surface, not this one's.

## Scenarios

The nine trace snapshots differ one option at a time, which is what lets the `Conditional on` column above be a measurement. Options are read from `tests/integration_tests.rs` at build time.

| Scenario | Query | Options |
|---|---|---|
| `01_basic` | `select_one` | none |
| `02_basic_metrics` | `select_one` | `with_metrics_collection()` |
| `03_basic_preview` | `select_one` | `with_row_limit()` |
| `04_basic_compact_preview` | `select_one` | `with_compact_preview()`, `with_row_limit()` |
| `05_basic_all_options` | `select_one` | `with_compact_preview()`, `with_metrics_collection()`, `with_row_limit()` |
| `06_object_store_all_options` | `order_nations` | `with_compact_preview()`, `with_metrics_collection()`, `with_object_store_collection()`, `with_row_limit()` |
| `07_scrabble_all_options` | `tpch_scrabble` | `ignore_full_trace()`, `ignore_preview_spans()`, `with_compact_preview()`, `with_metrics_collection()`, `with_row_limit()` |
| `08_recursive` | `recursive` | `collapse_recursive_exec_duplicates()` |
| `09_recursive_all_options` | `recursive` | `collapse_recursive_exec_duplicates()`, `with_compact_preview()`, `with_metrics_collection()`, `with_row_limit()` |
| `10_topk_lineitem` | `topk_lineitem` | none |

## One snapshot upstream ships is not valid JSON

`06_object_store_all_options_trace.snap` carries `Some\("…"\)` inside a JSON string: two invalid escapes and an unescaped quote. The cause is upstream's own insta filter in `tests/test_utils/insta_settings.rs`, whose replacement text carries the regex escapes `\(` and `\)` verbatim and drops the `\\"` its pattern matched. The replacement should have read `e_tag: Some(\\"ffffffff-fffffffffffff-fff\\")`.

This build substitutes that exact literal and nothing else, and flags every row derived from the file with `repaired` in the last column. Any other snapshot that fails to parse stops the build rather than receiving a second repair.

## Preview renders

35 per-node captures of what a `preview_fn` returned. `previews.tsv` carries node, scenario, line count, widest line and whether the render is `compact` -- the `|===|` header rule that `pretty_format_compact_batch` draws and `pretty_format_batches` does not.
