# `datafusion_physical_plan::joins::join_filter`

Crate `datafusion-physical-plan` · 1 public items · structured records in [`model/datafusion_physical_plan.joins.join_filter.json`](../model/datafusion_physical_plan.joins.join_filter.json)

## JoinFilter

`struct` · `datafusion_physical_plan::joins::join_filter::JoinFilter`

Also reachable as `datafusion_physical_plan::joins::utils::JoinFilter`

```rust
struct JoinFilter
```

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug

**Methods** (6)

```rust
fn build_column_indices(left_indices: Vec<usize>, right_indices: Vec<usize>) -> Vec<ColumnIndex>
fn column_indices(&self) -> &[ColumnIndex]
fn expression(&self) -> &Arc<dyn PhysicalExpr>
fn new(expression: Arc<dyn PhysicalExpr>, column_indices: Vec<ColumnIndex>, schema: SchemaRef) -> JoinFilter
fn schema(&self) -> &SchemaRef
fn swap(&self) -> JoinFilter
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.joins.join_filter.JoinFilter.md).


Filter applied before join output. Fields are crate-public to allow
downstream implementations to experiment with custom joins.

---
