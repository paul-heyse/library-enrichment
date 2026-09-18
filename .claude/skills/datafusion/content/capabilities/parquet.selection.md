# Compose row-group pruning, row selection and decode filters

with_row_groups applies before RowSelection. Selection coordinates describe only retained groups; predicate decode and output projection are additional stages with distinct I/O effects.

Reviewed 2026-09-18. reviewed decision brief; runtime scope is limited to named tests.

## Choice

| Candidate | Choose when | Consideration |
|---|---|---|
| DataFusion ParquetSource | Pruning should follow a relational predicate | Reuses planner, statistics and source integration. |
| RowSelection | Known row ranges can be skipped | Rebase to retained groups; page indexes can make skipping more efficient. |
| RowFilter | Predicates need decoded values | Decode predicate columns before final projected output; measure predicate ordering/I/O. |
| row-group statistics / bloom / page index | Metadata can exclude candidates | Depends on metadata written and predicates supported; these are not exact row filters. |

## Contract

**coordinates.** Rows belonging to excluded groups must not be counted in RowSelection. For 3-row groups [0,1,2] retaining [0,2], original row 7 has selected-domain position 4.
Claim `parquet.selection.coordinates`; upstream_contract_interpretation; evidence: upstream.

**shape.** ArrowReaderBuilder consumes configuration and builds a batch reader. RowSelection describes alternating selected/skipped row counts; output values and schema depend on projection/filter configuration.
Claim `parquet.selection.shape`; upstream_contract_interpretation; evidence: upstream.

**io.** Selection restricts rows decoded and can avoid fetching pages. Page metadata enables efficient skipping, but output equality alone does not establish bytes read or decode cost.
Claim `parquet.selection.io`; upstream_contract_interpretation; evidence: upstream.

## Implementation

- Choose groups first, then generate/rebase selection coordinates.
- Retain predicate columns until evaluation even when absent from output.
- Compare row identities against an unpruned reader; instrument I/O separately for performance claims.

## Limits and unknowns

- The fixture records metadata and scan bytes requested from its in-memory ChunkReader plus output identities. It does not measure network traffic, decoder CPU, page-index or bloom-filter effectiveness.

## Exact contracts

- [`parquet::arrow::arrow_reader::ArrowReaderBuilder::with_row_groups`](../operations/parquet.arrow.arrow_reader.ArrowReaderBuilder.md#op-a242cc258e7519b5c6886ed9) — `fn with_row_groups(self, row_groups: Vec<usize>) -> Self`
- [`parquet::arrow::arrow_reader::ArrowReaderBuilder::with_row_selection`](../operations/parquet.arrow.arrow_reader.ArrowReaderBuilder.md#op-521e662260c093bdc9d5c113) — `fn with_row_selection(self, selection: RowSelection) -> Self`
- [`parquet::arrow::arrow_reader::ArrowReaderBuilder::with_row_filter`](../operations/parquet.arrow.arrow_reader.ArrowReaderBuilder.md#op-8bea3b5ddc8fa0db0c89f3ec) — `fn with_row_filter(self, filter: RowFilter) -> Self`
- [`parquet::arrow::arrow_reader::selection::RowSelection`](../operations/parquet.arrow.arrow_reader.selection.RowSelection.md#op-42015b29e86ada83b6a84ed5) — `struct RowSelection`

## Evidence

`upstream` refers to the exact contracts above, preserving source spans and raw types.
- [probe](../../skill_improvement/evidence/implementation/probe-results.json): Only the named assertions in the recorded Cargo profile.
  Tests: parquet_selection_is_relative_to_retained_row_groups
