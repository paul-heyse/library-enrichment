# `datafusion_datasource_parquet::page_filter::PagePruningAccessPlanFilter`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_parquet.page_filter.PagePruningAccessPlanFilter.json).

<a id="op-4d5fab838352cbb61003124f"></a>
## PagePruningAccessPlanFilter

`struct` · `datafusion_datasource_parquet::page_filter::PagePruningAccessPlanFilter` · datafusion-datasource-parquet 55.1.0

```rust
struct PagePruningAccessPlanFilter
```

Source: `src/page_filter.rs:112`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Filters a [`ParquetAccessPlan`](../operations/datafusion_datasource_parquet.access_plan.ParquetAccessPlan.md#op-2786e9792f5bf477380471b9) based on the [Parquet PageIndex], if present

It does so by evaluating statistics from the [`ParquetColumnIndex`](../operations/parquet.file.metadata.ParquetColumnIndex.md#op-80ab0cca370795f8250c6d69) and
[`ParquetOffsetIndex`](../operations/parquet.file.metadata.ParquetOffsetIndex.md#op-ab57bc084f55b3929a33a434) and converting them to [`RowSelection`](../operations/parquet.arrow.arrow_reader.selection.RowSelection.md#op-42015b29e86ada83b6a84ed5).

[Parquet PageIndex]: https://github.com/apache/parquet-format/blob/master/PageIndex.md

For example, given a row group with two column (chunks) for `A`
and `B` with the following with page level statistics:

```text
┏━━ ━━━ ━━━ ━━━ ━━━ ━━━ ━━━ ━━━ ━━━ ━━━ ━━━ ━━━ ━━━
   ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─   ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─   ┃
┃     ┌──────────────┐  │     ┌──────────────┐  │  ┃
┃  │  │              │     │  │              │     ┃
┃     │              │  │     │     Page     │  │
   │  │              │     │  │      3       │     ┃
┃     │              │  │     │   min: "A"   │  │  ┃
┃  │  │              │     │  │   max: "C"   │     ┃
┃     │     Page     │  │     │ first_row: 0 │  │
   │  │      1       │     │  │              │     ┃
┃     │   min: 10    │  │     └──────────────┘  │  ┃
┃  │  │   max: 20    │     │  ┌──────────────┐     ┃
┃     │ first_row: 0 │  │     │              │  │
   │  │              │     │  │     Page     │     ┃
┃     │              │  │     │      4       │  │  ┃
┃  │  │              │     │  │   min: "D"   │     ┃
┃     │              │  │     │   max: "G"   │  │
   │  │              │     │  │first_row: 100│     ┃
┃     └──────────────┘  │     │              │  │  ┃
┃  │  ┌──────────────┐     │  │              │     ┃
┃     │              │  │     └──────────────┘  │
   │  │     Page     │     │  ┌──────────────┐     ┃
┃     │      2       │  │     │              │  │  ┃
┃  │  │   min: 30    │     │  │     Page     │     ┃
┃     │   max: 40    │  │     │      5       │  │
   │  │first_row: 200│     │  │   min: "H"   │     ┃
┃     │              │  │     │   max: "Z"   │  │  ┃
┃  │  │              │     │  │first_row: 250│     ┃
┃     └──────────────┘  │     │              │  │
   │                       │  └──────────────┘     ┃
┃   ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘   ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘  ┃
┃       ColumnChunk            ColumnChunk         ┃
┃            A                      B
 ━━━ ━━━ ━━━ ━━━ ━━━ ━━━ ━━━ ━━━ ━━━ ━━━ ━━━ ━━━ ━━┛

  Total rows: 300
```

Given the predicate `A > 35 AND B = 'F'`:

Using `A > 35`: can rule out all of values in Page 1 (rows 0 -> 199)

Using `B = 'F'`: can rule out all values in Page 3 and Page 5 (rows 0 -> 99, and 250 -> 299)

So we can entirely skip rows 0->199 and 250->299 as we know they
can not contain rows that match the predicate.

# Implementation notes

Single column predicates are evaluated using the PageIndex information
for that column to determine which row ranges can be skipped based.

The resulting [`RowSelection`](../operations/parquet.arrow.arrow_reader.selection.RowSelection.md#op-42015b29e86ada83b6a84ed5)'s are combined into a final
row selection that is added to the [`ParquetAccessPlan`](../operations/datafusion_datasource_parquet.access_plan.ParquetAccessPlan.md#op-2786e9792f5bf477380471b9).

<a id="op-3ca105cedde17b3c07b8a1c7"></a>
## filter_number

`function` · `datafusion_datasource_parquet::page_filter::PagePruningAccessPlanFilter::filter_number` · datafusion-datasource-parquet 55.1.0

```rust
fn filter_number(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::page_filter::PagePruningAccessPlanFilter", "path": "PagePruningAccessPlanFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [138, 1], "end": [389, 2], "filename": "src/page_filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/page_filter.rs:376`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Returns the number of filters in the [`PagePruningAccessPlanFilter`](../operations/datafusion_datasource_parquet.page_filter.PagePruningAccessPlanFilter.md#op-4d5fab838352cbb61003124f)

<a id="op-f75da63e8285177bd8ee5f73"></a>
## fmt

`function` · `datafusion_datasource_parquet::page_filter::PagePruningAccessPlanFilter::fmt` · datafusion-datasource-parquet 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::page_filter::PagePruningAccessPlanFilter", "path": "PagePruningAccessPlanFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 10], "end": [111, 15], "filename": "src/page_filter.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/page_filter.rs:111`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-44eb17586a0b4623a495edcf"></a>
## new

`function` · `datafusion_datasource_parquet::page_filter::PagePruningAccessPlanFilter::new` · datafusion-datasource-parquet 55.1.0

```rust
fn new(expr: &Arc<dyn PhysicalExpr>, schema: SchemaRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::page_filter::PagePruningAccessPlanFilter", "path": "PagePruningAccessPlanFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [138, 1], "end": [389, 2], "filename": "src/page_filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/page_filter.rs:142`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Create a new [`PagePruningAccessPlanFilter`](../operations/datafusion_datasource_parquet.page_filter.PagePruningAccessPlanFilter.md#op-4d5fab838352cbb61003124f) from a physical
expression.

<a id="op-8fa0da0d4bc0cd17c4a26128"></a>
## prune_plan_with_page_index

`function` · `datafusion_datasource_parquet::page_filter::PagePruningAccessPlanFilter::prune_plan_with_page_index` · datafusion-datasource-parquet 55.1.0

```rust
fn prune_plan_with_page_index(&self, access_plan: ParquetAccessPlan, arrow_schema: &Schema, parquet_schema: &SchemaDescriptor, parquet_metadata: &ParquetMetaData, file_metrics: &ParquetFileMetrics) -> ParquetAccessPlan
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::page_filter::PagePruningAccessPlanFilter", "path": "PagePruningAccessPlanFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [138, 1], "end": [389, 2], "filename": "src/page_filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/page_filter.rs:176`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Returns an updated [`ParquetAccessPlan`](../operations/datafusion_datasource_parquet.access_plan.ParquetAccessPlan.md#op-2786e9792f5bf477380471b9) by applying predicates to the
parquet page index, if any
