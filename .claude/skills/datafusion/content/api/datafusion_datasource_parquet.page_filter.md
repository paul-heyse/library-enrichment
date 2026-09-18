# `datafusion_datasource_parquet::page_filter`

Crate `datafusion-datasource-parquet` · 1 public items · structured records in [`model/datafusion_datasource_parquet.page_filter.json`](../model/datafusion_datasource_parquet.page_filter.json)

## PagePruningAccessPlanFilter

`struct` · `datafusion_datasource_parquet::page_filter::PagePruningAccessPlanFilter`

Also reachable as `datafusion::datasource::physical_plan::parquet::PagePruningAccessPlanFilter`, `datafusion_datasource_parquet::PagePruningAccessPlanFilter`

```rust
struct PagePruningAccessPlanFilter
```

**Derives**: Debug

**Methods** (3)

```rust
fn filter_number(&self) -> usize
fn new(expr: &Arc<dyn PhysicalExpr>, schema: SchemaRef) -> Self
fn prune_plan_with_page_index(&self, access_plan: ParquetAccessPlan, arrow_schema: &Schema, parquet_schema: &SchemaDescriptor, parquet_metadata: &ParquetMetaData, file_metrics: &ParquetFileMetrics) -> ParquetAccessPlan
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource_parquet.page_filter.PagePruningAccessPlanFilter.md).


Filters a [`ParquetAccessPlan`] based on the [Parquet PageIndex], if present

It does so by evaluating statistics from the [`ParquetColumnIndex`] and
[`ParquetOffsetIndex`] and converting them to [`RowSelection`].

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

The resulting [`RowSelection`]'s are combined into a final
row selection that is added to the [`ParquetAccessPlan`].

---
