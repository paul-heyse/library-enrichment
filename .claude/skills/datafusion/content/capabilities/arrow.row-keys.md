# Encode composite keys for local comparisons

RowConverter is a comparison representation with converter-specific ordering, not a durable wire format. Decoding preserves values but hydrates dictionaries.

Reviewed 2026-09-18. reviewed decision brief; runtime scope is limited to named tests.

## Choice

| Candidate | Choose when | Consideration |
|---|---|---|
| RowConverter | Repeated local composite comparisons | Associate rows with the same converter; configure descending/null placement per field. |
| lexsort_to_indices | Only sorted row positions are needed | Avoid managing your own encoded-row lifecycle. |
| DataFrame sort/group/distinct | The operation is relational | Keep execution and optimization in the engine. |
| IPC / Parquet / specified application encoding | Persistence or cross-process compatibility is required | Define a format/schema compatibility policy; bytes alone do not define durable identity. |

## Contract

**shape.** RowConverter::new takes ordered SortFields. convert_columns takes &[ArrayRef] and returns Rows; convert_rows returns Vec<ArrayRef>, not a reconstructed schema.
Claim `arrow.row-keys.shape`; upstream_contract_interpretation; evidence: upstream.

**lifecycle.** Upstream requires compared rows to come from the same converter. Rows exposes borrowed Row views; OwnedRow owns row bytes. Row encoding may change across releases.
Claim `arrow.row-keys.lifecycle`; upstream_contract_interpretation; evidence: upstream, source.

**dictionary.** Dictionary inputs are hydrated for row encoding. The Int8/Utf8 probe decodes to Utf8, not Dictionary<Int8,Utf8>; exact dictionary assignments require preserving separate representation information or choosing another format.
Claim `arrow.row-keys.dictionary`; upstream_contract_interpretation; evidence: upstream, source.

**semantics.** SortOptions controls null placement and direction. Do not infer application collation, canonical equality or stable float encoding from lexicographic byte comparison.
Claim `arrow.row-keys.semantics`; upstream_contract_interpretation; evidence: upstream.

## Implementation

- Keep converter configuration associated with rows.
- Validate decoded physical types and schema separately from value equality.
- Use a specified interchange format when output must outlive the local comparison contract.

## Limits and unknowns

- Float/nested/extension domain equivalence and cross-release serialization are not qualified.
- The IPC round trip preserves dictionary type, keys and values in one pinned process/profile; it does not certify arbitrary reader/writer version pairs.

## Exact contracts

- [`arrow_row::RowConverter`](../operations/arrow_row.RowConverter.md#op-3289371bf6ccaf9946ba91e3) — `struct RowConverter`
- [`arrow_row::RowConverter::convert_columns`](../operations/arrow_row.RowConverter.md#op-610a34b7b5b57c4d480dfde7) — `fn convert_columns(&self, columns: &[ArrayRef]) -> Result<Rows, ArrowError>`
- [`arrow_row::RowConverter::convert_rows`](../operations/arrow_row.RowConverter.md#op-772d3ca81b1fb03bb6f2e3a4) — `fn convert_rows<'a, I>(&self, rows: I) -> Result<Vec<ArrayRef>, ArrowError> where I: IntoIterator<Item = Row<'a>>`
- [`arrow_row::SortField`](../operations/arrow_row.SortField.md#op-e58d2d9b989df785f7f8db59) — `struct SortField`
- [`arrow_ord::sort::lexsort_to_indices`](../operations/arrow_ord.sort.lexsort_to_indices.md#op-c91c43c26e3b78b4327d4c24) — `fn lexsort_to_indices(columns: &[SortColumn], limit: Option<usize>) -> Result<UInt32Array, arrow_schema::ArrowError>`

## Evidence

`upstream` refers to the exact contracts above, preserving source spans and raw types.
- [probe](../../skill_improvement/evidence/implementation/probe-results.json): Only the named assertions in the recorded Cargo profile.
  Tests: row_order_changes_with_null_placement, row_decode_hydrates_dictionary_and_preserves_values, ipc_dictionary_roundtrip_keeps_representation
- [source](../../skill_improvement/evidence/sources/arrow-row-59.3.0/src/lib.rs): Selected exact published source; source-manifest.json retains provenance.
  Tests: 
