# `datafusion_physical_expr::analysis`

Crate `datafusion-physical-expr` · 3 public items · structured records in [`model/datafusion_physical_expr.analysis.json`](../model/datafusion_physical_expr.analysis.json)

## analyze

`function` · `datafusion_physical_expr::analysis::analyze`

Also reachable as `datafusion::physical_expr::analyze`, `datafusion_physical_expr::analyze`

```rust
fn analyze(expr: &std::sync::Arc<dyn PhysicalExpr>, context: AnalysisContext, schema: &arrow::datatypes::Schema) -> datafusion_common::Result<AnalysisContext>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.analysis.analyze.md).


Attempts to refine column boundaries and compute a selectivity value.

The function accepts boundaries of the input columns in the `context` parameter.
It then tries to tighten these boundaries based on the provided `expr`.
The resulting selectivity value is calculated by comparing the initial and final boundaries.
The computation assumes that the data within the column is uniformly distributed and not sorted.

# Arguments

* `context` - The context holding input column boundaries.
* `expr` - The expression used to shrink the column boundaries.

# Returns

* `AnalysisContext` constructed by pruned boundaries and a selectivity value.

---

## AnalysisContext

`struct` · `datafusion_physical_expr::analysis::AnalysisContext`

Also reachable as `datafusion::physical_expr::AnalysisContext`, `datafusion_physical_expr::AnalysisContext`

```rust
struct AnalysisContext
```

**Fields**: `boundaries`, `selectivity`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn new(boundaries: Vec<ExprBoundaries>) -> Self
fn try_from_statistics(input_schema: &Schema, statistics: &[ColumnStatistics]) -> Result<Self>
fn with_selectivity(self, selectivity: f64) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.analysis.AnalysisContext.md).


The shared context used during the analysis of an expression. Includes
the boundaries for all known columns.

---

## ExprBoundaries

`struct` · `datafusion_physical_expr::analysis::ExprBoundaries`

Also reachable as `datafusion::physical_expr::ExprBoundaries`, `datafusion_physical_expr::ExprBoundaries`

```rust
struct ExprBoundaries
```

**Fields**: `column`, `interval`, `distinct_count`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn try_from_column(schema: &Schema, col_stats: &ColumnStatistics, col_index: usize) -> Result<Self>
fn try_new_unbounded(schema: &Schema) -> Result<Vec<Self>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.analysis.ExprBoundaries.md).


Represents the boundaries (e.g. min and max values) of a particular column

This is used range analysis of expressions, to determine if the expression
limits the value of particular columns (e.g. analyzing an expression such as
`time < 50` would result in a boundary interval for `time` having a max
value of `50`).

---
