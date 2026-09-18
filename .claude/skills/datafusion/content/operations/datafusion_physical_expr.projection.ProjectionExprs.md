# `datafusion_physical_expr::projection::ProjectionExprs`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.projection.ProjectionExprs.json).

<a id="op-9af61cebb79ab505edeb3c19"></a>
## ProjectionExprs

`struct` · `datafusion_physical_expr::projection::ProjectionExprs` · datafusion-physical-expr 55.1.0

```rust
struct ProjectionExprs
```

Source: `src/projection.rs:137`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

A collection of  [`ProjectionExpr`](../operations/datafusion_physical_expr.projection.ProjectionExpr.md#op-77aa0dab554890daa533d155) instances, representing a complete
projection operation.

Projection operations are used in query plans to select specific columns or
compute new columns based on existing ones.

See [`ProjectionExprs::from_indices`](../operations/datafusion_physical_expr.projection.ProjectionExprs.md#op-7fdd9f5e2089b2d2e5d28545) to select a subset of columns by
indices.

<a id="op-1373e48aa3c6b2c09d7d938e"></a>
## as_ref

`function` · `datafusion_physical_expr::projection::ProjectionExprs::as_ref` · datafusion-physical-expr 55.1.0

```rust
fn as_ref(&self) -> &[ProjectionExpr]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionExprs", "path": "ProjectionExprs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [173, 1], "end": [177, 2], "filename": "src/projection.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"slice": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionExpr", "path": "ProjectionExpr"}}}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}, "trait_path": "core::convert::AsRef"}`

Source: `src/projection.rs:174`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8031803e959d11a0e85c67a2"></a>
## clone

`function` · `datafusion_physical_expr::projection::ProjectionExprs::clone` · datafusion-physical-expr 55.1.0

```rust
fn clone(&self) -> ProjectionExprs
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionExprs", "path": "ProjectionExprs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 17], "end": [136, 22], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/projection.rs:136`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c0459256f88793317b276bd"></a>
## column_indices

`function` · `datafusion_physical_expr::projection::ProjectionExprs::column_indices` · datafusion-physical-expr 55.1.0

```rust
fn column_indices(&self) -> Vec<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionExprs", "path": "ProjectionExprs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 1], "end": [840, 2], "filename": "src/projection.rs"}, "trait": null, "trait_path": null}`

Source: `src/projection.rs:396`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Extract the column indices used in this projection.
For example, for a projection `SELECT a AS x, b + 1 AS y`, where `a` is at index 0 and `b` is at index 1,
this function would return `[0, 1]`.
Repeated indices are returned only once, and the order is ascending.

<a id="op-8a8f5ccee34d61cbeae537fe"></a>
## create_expression_metrics

`function` · `datafusion_physical_expr::projection::ProjectionExprs::create_expression_metrics` · datafusion-physical-expr 55.1.0

```rust
fn create_expression_metrics(&self, metrics: &ExecutionPlanMetricsSet, partition: usize) -> ExpressionEvaluatorMetrics
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionExprs", "path": "ProjectionExprs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 1], "end": [840, 2], "filename": "src/projection.rs"}, "trait": null, "trait_path": null}`

Source: `src/projection.rs:600`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1568effb9880eb13f61b53cb"></a>
## eq

`function` · `datafusion_physical_expr::projection::ProjectionExprs::eq` · datafusion-physical-expr 55.1.0

```rust
fn eq(&self, other: &ProjectionExprs) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionExprs", "path": "ProjectionExprs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 24], "end": [136, 33], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/projection.rs:136`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a8f5d121aae7158785e90ba1"></a>
## expr_iter

`function` · `datafusion_physical_expr::projection::ProjectionExprs::expr_iter` · datafusion-physical-expr 55.1.0

```rust
fn expr_iter(&self) -> impl Iterator<Item = Arc<dyn PhysicalExpr>> + '_
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionExprs", "path": "ProjectionExprs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 1], "end": [840, 2], "filename": "src/projection.rs"}, "trait": null, "trait_path": null}`

Source: `src/projection.rs:268`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Iterate over a clone of the projection expressions.

<a id="op-93f156f6d862445588d30162"></a>
## fmt

`function` · `datafusion_physical_expr::projection::ProjectionExprs::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionExprs", "path": "ProjectionExprs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [142, 1], "end": [147, 2], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/projection.rs:143`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c00be8184f082222063e1d48"></a>
## fmt

`function` · `datafusion_physical_expr::projection::ProjectionExprs::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionExprs", "path": "ProjectionExprs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 10], "end": [136, 15], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/projection.rs:136`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b067085375bf1dc024c259da"></a>
## from

`function` · `datafusion_physical_expr::projection::ProjectionExprs::from` · datafusion-physical-expr 55.1.0

```rust
fn from(value: &[ProjectionExpr]) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionExprs", "path": "ProjectionExprs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [157, 1], "end": [163, 2], "filename": "src/projection.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"slice": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionExpr", "path": "ProjectionExpr"}}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/projection.rs:158`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-efbe2e6146c5adc566ba5216"></a>
## from

`function` · `datafusion_physical_expr::projection::ProjectionExprs::from` · datafusion-physical-expr 55.1.0

```rust
fn from(value: Vec<ProjectionExpr>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionExprs", "path": "ProjectionExprs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 1], "end": [155, 2], "filename": "src/projection.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionExpr", "path": "ProjectionExpr"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/projection.rs:150`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-235f2b42000c2669c0472154"></a>
## from_expressions

`function` · `datafusion_physical_expr::projection::ProjectionExprs::from_expressions` · datafusion-physical-expr 55.1.0

```rust
fn from_expressions(exprs: impl Into<Arc<[ProjectionExpr]>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionExprs", "path": "ProjectionExprs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 1], "end": [840, 2], "filename": "src/projection.rs"}, "trait": null, "trait_path": null}`

Source: `src/projection.rs:188`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Make a new [`ProjectionExprs`](../operations/datafusion_physical_expr.projection.ProjectionExprs.md#op-9af61cebb79ab505edeb3c19) from expressions.

<a id="op-7fdd9f5e2089b2d2e5d28545"></a>
## from_indices

`function` · `datafusion_physical_expr::projection::ProjectionExprs::from_indices` · datafusion-physical-expr 55.1.0

```rust
fn from_indices(indices: &[usize], schema: &Schema) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionExprs", "path": "ProjectionExprs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 1], "end": [840, 2], "filename": "src/projection.rs"}, "trait": null, "trait_path": null}`

Source: `src/projection.rs:237`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Creates a [`ProjectionExpr`](../operations/datafusion_physical_expr.projection.ProjectionExpr.md#op-77aa0dab554890daa533d155) from a list of column indices.

This is a convenience method for creating simple column-only projections, where each projection expression is a reference to a column
in the input schema.

# Behavior
- Ordering: the output projection preserves the exact order of indices provided in the input slice
  For example, `[2, 0, 1]` will produce projections for columns 2, 0, then 1 in that order
- Duplicates: Duplicate indices are allowed and will create multiple projection expressions referencing the same source column
  For example, `[0, 0]` creates 2 separate projections both referencing column 0

# Panics
Panics if any index in `indices` is out of bounds for the provided schema.

# Example

```rust
use arrow::datatypes::{DataType, Field, Schema};
use datafusion_physical_expr::projection::ProjectionExprs;
use std::sync::Arc;

// Create a schema with three columns
let schema = Arc::new(Schema::new(vec![
    Field::new("a", DataType::Int32, false),
    Field::new("b", DataType::Utf8, false),
    Field::new("c", DataType::Float64, false),
]));

// Project columns at indices 2 and 0 (c and a) - ordering is preserved
let projection = ProjectionExprs::from_indices(&[2, 0], &schema);

// This creates: SELECT c@2 AS c, a@0 AS a
assert_eq!(projection.as_ref().len(), 2);
assert_eq!(projection.as_ref()[0].alias, "c");
assert_eq!(projection.as_ref()[1].alias, "a");

// Duplicate indices are allowed
let projection_with_dups = ProjectionExprs::from_indices(&[0, 0, 1], &schema);
assert_eq!(projection_with_dups.as_ref().len(), 3);
assert_eq!(projection_with_dups.as_ref()[0].alias, "a");
assert_eq!(projection_with_dups.as_ref()[1].alias, "a"); // duplicate
assert_eq!(projection_with_dups.as_ref()[2].alias, "b");
```

<a id="op-db8a09829f1e03f5b7b5faaf"></a>
## from_iter

`function` · `datafusion_physical_expr::projection::ProjectionExprs::from_iter` · datafusion-physical-expr 55.1.0

```rust
fn from_iter<T: IntoIterator<Item = ProjectionExpr>>(exprs: T) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionExprs", "path": "ProjectionExprs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [165, 1], "end": [171, 2], "filename": "src/projection.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionExpr", "path": "ProjectionExpr"}}}], "constraints": []}}, "id": "core::iter::traits::collect::FromIterator", "path": "FromIterator"}, "trait_path": "core::iter::traits::collect::FromIterator"}`

Source: `src/projection.rs:166`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cb6b4fc3e6633d6ab1bfbb0a"></a>
## iter

`function` · `datafusion_physical_expr::projection::ProjectionExprs::iter` · datafusion-physical-expr 55.1.0

```rust
fn iter(&self) -> impl Iterator<Item = &ProjectionExpr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionExprs", "path": "ProjectionExprs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 1], "end": [840, 2], "filename": "src/projection.rs"}, "trait": null, "trait_path": null}`

Source: `src/projection.rs:250`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Returns an iterator over the projection expressions

<a id="op-8561b2936688862371138bae"></a>
## make_projector

`function` · `datafusion_physical_expr::projection::ProjectionExprs::make_projector` · datafusion-physical-expr 55.1.0

```rust
fn make_projector(&self, input_schema: &Schema) -> Result<Projector>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionExprs", "path": "ProjectionExprs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 1], "end": [840, 2], "filename": "src/projection.rs"}, "trait": null, "trait_path": null}`

Source: `src/projection.rs:541`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create a new [`Projector`](../operations/datafusion_physical_expr.projection.Projector.md#op-c9e69d382b9c8e776cb0751e) from this projection and an input schema.

A [`Projector`](../operations/datafusion_physical_expr.projection.Projector.md#op-c9e69d382b9c8e776cb0751e) can be used to apply this projection to record batches.

# Errors
This function returns an error if the output schema cannot be constructed from the input schema
with the given projection expressions.
For example, if an expression only works with integer columns but the input schema has a string column at that index.

<a id="op-246e29552c0cc1de805fd62a"></a>
## make_projector_with_schema_metadata

`function` · `datafusion_physical_expr::projection::ProjectionExprs::make_projector_with_schema_metadata` · datafusion-physical-expr 55.1.0

```rust
fn make_projector_with_schema_metadata(&self, input_schema: &Schema, projected_schema: &Schema) -> Result<Projector>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionExprs", "path": "ProjectionExprs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 1], "end": [840, 2], "filename": "src/projection.rs"}, "trait": null, "trait_path": null}`

Source: `src/projection.rs:561`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create a new [`Projector`](../operations/datafusion_physical_expr.projection.Projector.md#op-c9e69d382b9c8e776cb0751e) using field and schema metadata from
`projected_schema`.

Field names, data types, and nullability are still derived from the physical
projection expressions and `input_schema`; only field and schema metadata are
taken from `projected_schema`.

# Errors

Returns an error if the projection cannot be applied to `input_schema`, or if
`projected_schema` has a different number of fields than the projection.

<a id="op-fbadfd4da2d68329890066f5"></a>
## new

`function` · `datafusion_physical_expr::projection::ProjectionExprs::new` · datafusion-physical-expr 55.1.0

```rust
fn new(exprs: impl IntoIterator<Item = ProjectionExpr>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionExprs", "path": "ProjectionExprs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 1], "end": [840, 2], "filename": "src/projection.rs"}, "trait": null, "trait_path": null}`

Source: `src/projection.rs:181`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Make a new [`ProjectionExprs`](../operations/datafusion_physical_expr.projection.ProjectionExprs.md#op-9af61cebb79ab505edeb3c19) from expressions iterator.

<a id="op-665c6c4f4b004d63bb43d6b7"></a>
## ordered_column_indices

`function` · `datafusion_physical_expr::projection::ProjectionExprs::ordered_column_indices` · datafusion-physical-expr 55.1.0

```rust
fn ordered_column_indices(&self) -> Vec<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionExprs", "path": "ProjectionExprs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 1], "end": [840, 2], "filename": "src/projection.rs"}, "trait": null, "trait_path": null}`

Source: `src/projection.rs:440`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Extract the ordered column indices for a column-only projection.

This function assumes that all expressions in the projection are simple column references.
It returns the column indices in the order they appear in the projection.

# Panics

Panics if any expression in the projection is not a simple column reference. This includes:
- Computed expressions (e.g., `a + 1`, `CAST(a AS INT)`)
- Function calls (e.g., `UPPER(name)`, `SUM(amount)`)
- Literals (e.g., `42`, `'hello'`)
- Complex nested expressions (e.g., `CASE WHEN ... THEN ... END`)

# Returns

A vector of column indices in projection order. Unlike [`column_indices()`](Self::column_indices),
this function:
- Preserves the projection order (does not sort)
- Preserves duplicates (does not deduplicate)

# Example

For a projection `SELECT c, a, c` where `a` is at index 0 and `c` is at index 2,
this function would return `[2, 0, 2]`.

Use [`column_indices()`](Self::column_indices) instead if the projection may contain
non-column expressions or if you need a deduplicated sorted list.

# Panics

Panics if any expression in the projection is not a simple column reference.

<a id="op-f86f5d0b156f02cf391b34f3"></a>
## project_expr

`function` · `datafusion_physical_expr::projection::ProjectionExprs::project_expr` · datafusion-physical-expr 55.1.0

```rust
fn project_expr(&self, expr: &Arc<dyn PhysicalExpr>) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionExprs", "path": "ProjectionExprs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 1], "end": [840, 2], "filename": "src/projection.rs"}, "trait": null, "trait_path": null}`

Source: `src/projection.rs:520`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

"project" an expression using these projection's expressions

For example, consider
* an expression `c1 + c2 > 5`, and a schema `[c1, c2]`
* a projection `c1 + c2 as c1_c2`

* This method would rewrite the expression to `c1_c2 > 5`

<a id="op-e5ef22f9caa46a31134f5370"></a>
## project_schema

`function` · `datafusion_physical_expr::projection::ProjectionExprs::project_schema` · datafusion-physical-expr 55.1.0

```rust
fn project_schema(&self, input_schema: &Schema) -> Result<Schema>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionExprs", "path": "ProjectionExprs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 1], "end": [840, 2], "filename": "src/projection.rs"}, "trait": null, "trait_path": null}`

Source: `src/projection.rs:463`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Project a schema according to this projection.

For example, given a projection:
* `SELECT a AS x, b + 1 AS y`
* where `a` is at index 0
* `b` is at index 1

If the input schema is `[a: Int32, b: Int32, c: Int32]`, the output
schema would be `[x: Int32, y: Int32]`.

Note that [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf) metadata are preserved from the input schema.

<a id="op-b880a09f8f09ffa603979399"></a>
## project_statistics

`function` · `datafusion_physical_expr::projection::ProjectionExprs::project_statistics` · datafusion-physical-expr 55.1.0

```rust
fn project_statistics(&self, stats: Statistics, output_schema: &Schema) -> Result<Statistics>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionExprs", "path": "ProjectionExprs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 1], "end": [840, 2], "filename": "src/projection.rs"}, "trait": null, "trait_path": null}`

Source: `src/projection.rs:712`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Project statistics according to this projection.
For example, for a projection `SELECT a AS x, b + 1 AS y`, where `a` is at index 0 and `b` is at index 1,
if the input statistics has column statistics for columns `a`, `b`, and `c`, the output statistics would have column statistics for columns `x` and `y`.

# Example

```rust
use arrow::datatypes::{DataType, Field, Schema};
use datafusion_common::stats::{ColumnStatistics, Precision, Statistics};
use datafusion_physical_expr::projection::ProjectionExprs;
use datafusion_common::Result;
use datafusion_common::ScalarValue;
use std::sync::Arc;

fn main() -> Result<()> {
    // Input schema: a: Int32, b: Int32, c: Int32
    let input_schema = Arc::new(Schema::new(vec![
        Field::new("a", DataType::Int32, false),
        Field::new("b", DataType::Int32, false),
        Field::new("c", DataType::Int32, false),
    ]));

    // Input statistics with column stats for a, b, c
    let input_stats = Statistics {
        num_rows: Precision::Exact(100),
        total_byte_size: Precision::Exact(1200),
        column_statistics: vec![
            // Column a stats
            ColumnStatistics::new_unknown()
                .with_null_count(Precision::Exact(0))
                .with_min_value(Precision::Exact(ScalarValue::Int32(Some(0))))
                .with_max_value(Precision::Exact(ScalarValue::Int32(Some(100))))
                .with_distinct_count(Precision::Exact(100)),
            // Column b stats
            ColumnStatistics::new_unknown()
                .with_null_count(Precision::Exact(0))
                .with_min_value(Precision::Exact(ScalarValue::Int32(Some(10))))
                .with_max_value(Precision::Exact(ScalarValue::Int32(Some(60))))
                .with_distinct_count(Precision::Exact(50)),
            // Column c stats
            ColumnStatistics::new_unknown()
                .with_null_count(Precision::Exact(5))
                .with_min_value(Precision::Exact(ScalarValue::Int32(Some(-10))))
                .with_max_value(Precision::Exact(ScalarValue::Int32(Some(200))))
                .with_distinct_count(Precision::Exact(25)),
        ],
    };

    // Create a projection that selects columns c and a (indices 2 and 0)
    let projection = ProjectionExprs::from_indices(&[2, 0], &input_schema);

    // Compute output schema
    let output_schema = projection.project_schema(&input_schema)?;

    // Project the statistics
    let output_stats = projection.project_statistics(input_stats, &output_schema)?;

    // The output should have 2 column statistics (for c and a, in that order)
    assert_eq!(output_stats.column_statistics.len(), 2);

    // First column in output is c (was at index 2)
    assert_eq!(
        output_stats.column_statistics[0].min_value,
        Precision::Exact(ScalarValue::Int32(Some(-10)))
    );
    assert_eq!(
        output_stats.column_statistics[0].null_count,
        Precision::Exact(5)
    );

    // Second column in output is a (was at index 0)
    assert_eq!(
        output_stats.column_statistics[1].min_value,
        Precision::Exact(ScalarValue::Int32(Some(0)))
    );
    assert_eq!(
        output_stats.column_statistics[1].distinct_count,
        Precision::Exact(100)
    );

    // Total byte size is recalculated based on projected columns
    assert_eq!(
        output_stats.total_byte_size,
        Precision::Exact(800), // each Int32 column is 4 bytes * 100 rows * 2 columns
    );

    // Number of rows remains the same
    assert_eq!(output_stats.num_rows, Precision::Exact(100));

    Ok(())
}
```

<a id="op-f3ebf047e61f14ec799f591d"></a>
## projected_column_position

`function` · `datafusion_physical_expr::projection::ProjectionExprs::projected_column_position` · datafusion-physical-expr 55.1.0

```rust
fn projected_column_position(&self, column: &Column) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionExprs", "path": "ProjectionExprs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 1], "end": [840, 2], "filename": "src/projection.rs"}, "trait": null, "trait_path": null}`

Source: `src/projection.rs:833`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Returns the output position of `column` if this projection contains it.

This only matches projection expressions that are exactly [`Column`](../operations/datafusion_physical_expr.expressions.column.Column.md#op-8a412ca3bf3f178f4986ec28) expressions.
Computed expressions, even if they reference `column`, do not match. The
comparison uses [`Column`](../operations/datafusion_physical_expr.expressions.column.Column.md#op-8a412ca3bf3f178f4986ec28) equality, so both the name and index must match.
If the same column appears more than once, this returns the first matching
position.

# Example

```rust
use datafusion_common::ScalarValue;
use datafusion_physical_expr::expressions::{Column, Literal};
use datafusion_physical_expr::projection::{ProjectionExpr, ProjectionExprs};
use std::sync::Arc;

let projection = ProjectionExprs::new([
    ProjectionExpr::new(Arc::new(Column::new("b", 1)), "b"),
    ProjectionExpr::new(
        Arc::new(Literal::new(ScalarValue::Int32(Some(42)))),
        "answer",
    ),
    ProjectionExpr::new(Arc::new(Column::new("a", 0)), "a"),
]);

assert_eq!(
    projection.projected_column_position(&Column::new("b", 1)),
    Some(0)
);
assert_eq!(
    projection.projected_column_position(&Column::new("a", 0)),
    Some(2)
);

// The literal projection is not a Column expression.
assert_eq!(
    projection.projected_column_position(&Column::new("answer", 1)),
    None
);

// Columns not present in the projection also return None.
assert_eq!(
    projection.projected_column_position(&Column::new("c", 2)),
    None
);
```

<a id="op-fa91e6dfe8ce4f2b0ddc7a2b"></a>
## projection_mapping

`function` · `datafusion_physical_expr::projection::ProjectionExprs::projection_mapping` · datafusion-physical-expr 55.1.0

```rust
fn projection_mapping(&self, input_schema: &SchemaRef) -> Result<ProjectionMapping>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionExprs", "path": "ProjectionExprs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 1], "end": [840, 2], "filename": "src/projection.rs"}, "trait": null, "trait_path": null}`

Source: `src/projection.rs:255`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Creates a ProjectionMapping from this projection

<a id="op-28eb1badc8651ee2b82ae3ea"></a>
## try_map_exprs

`function` · `datafusion_physical_expr::projection::ProjectionExprs::try_map_exprs` · datafusion-physical-expr 55.1.0

```rust
fn try_map_exprs<F>(self, f: F) -> Result<Self> where F: FnMut(Arc<dyn PhysicalExpr>) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionExprs", "path": "ProjectionExprs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 1], "end": [840, 2], "filename": "src/projection.rs"}, "trait": null, "trait_path": null}`

Source: `src/projection.rs:300`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Apply a fallible transformation to the [`PhysicalExpr`](../operations/datafusion_physical_expr_common.physical_expr.PhysicalExpr.md#op-fe8284c43330456b0d4e6af7) of each projection.

This method transforms the expression in each [`ProjectionExpr`](../operations/datafusion_physical_expr.projection.ProjectionExpr.md#op-77aa0dab554890daa533d155) while preserving
the alias. This is useful for rewriting expressions, such as when adapting
expressions to a different schema.

# Example

```rust
use std::sync::Arc;
use arrow::datatypes::{DataType, Field, Schema};
use datafusion_common::Result;
use datafusion_physical_expr::expressions::Column;
use datafusion_physical_expr::projection::ProjectionExprs;
use datafusion_physical_expr::PhysicalExpr;

// Create a schema and projection
let schema = Arc::new(Schema::new(vec![
    Field::new("a", DataType::Int32, false),
    Field::new("b", DataType::Int32, false),
]));
let projection = ProjectionExprs::from_indices(&[0, 1], &schema);

// Transform each expression (this example just clones them)
let transformed = projection.try_map_exprs(|expr| Ok(expr))?;
assert_eq!(transformed.as_ref().len(), 2);
# Ok::<(), datafusion_common::DataFusionError>(())
```

<a id="op-3f2d36be1e759be509f4291a"></a>
## try_merge

`function` · `datafusion_physical_expr::projection::ProjectionExprs::try_merge` · datafusion-physical-expr 55.1.0

```rust
fn try_merge(&self, other: &ProjectionExprs) -> Result<ProjectionExprs>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionExprs", "path": "ProjectionExprs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 1], "end": [840, 2], "filename": "src/projection.rs"}, "trait": null, "trait_path": null}`

Source: `src/projection.rs:381`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Apply another projection on top of this projection, returning the combined projection.
For example, if this projection is `SELECT c@2 AS x, b@1 AS y, a@0 as z` and the other projection is `SELECT x@0 + 1 AS c1, y@1 + z@2 as c2`,
we return a projection equivalent to `SELECT c@2 + 1 AS c1, b@1 + a@0 as c2`.

# Example

```rust
use datafusion_common::{Result, ScalarValue};
use datafusion_expr::Operator;
use datafusion_physical_expr::expressions::{BinaryExpr, Column, Literal};
use datafusion_physical_expr::projection::{ProjectionExpr, ProjectionExprs};
use std::sync::Arc;

fn main() -> Result<()> {
    // Example from the docstring:
    // Base projection: SELECT c@2 AS x, b@1 AS y, a@0 AS z
    let base = ProjectionExprs::new(vec![
        ProjectionExpr {
            expr: Arc::new(Column::new("c", 2)),
            alias: "x".to_string(),
        },
        ProjectionExpr {
            expr: Arc::new(Column::new("b", 1)),
            alias: "y".to_string(),
        },
        ProjectionExpr {
            expr: Arc::new(Column::new("a", 0)),
            alias: "z".to_string(),
        },
    ]);

    // Top projection: SELECT x@0 + 1 AS c1, y@1 + z@2 AS c2
    let top = ProjectionExprs::new(vec![
        ProjectionExpr {
            expr: Arc::new(BinaryExpr::new(
                Arc::new(Column::new("x", 0)),
                Operator::Plus,
                Arc::new(Literal::new(ScalarValue::Int32(Some(1)))),
            )),
            alias: "c1".to_string(),
        },
        ProjectionExpr {
            expr: Arc::new(BinaryExpr::new(
                Arc::new(Column::new("y", 1)),
                Operator::Plus,
                Arc::new(Column::new("z", 2)),
            )),
            alias: "c2".to_string(),
        },
    ]);

    // Expected result: SELECT c@2 + 1 AS c1, b@1 + a@0 AS c2
    let result = base.try_merge(&top)?;

    assert_eq!(result.as_ref().len(), 2);
    assert_eq!(result.as_ref()[0].alias, "c1");
    assert_eq!(result.as_ref()[1].alias, "c2");

    Ok(())
}
```

# Errors
This function returns an error if any expression in the `other` projection cannot be
applied on top of this projection.

<a id="op-17ddaa2d6115725dd89dea92"></a>
## unproject_expr

`function` · `datafusion_physical_expr::projection::ProjectionExprs::unproject_expr` · datafusion-physical-expr 55.1.0

```rust
fn unproject_expr(&self, expr: &Arc<dyn PhysicalExpr>) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionExprs", "path": "ProjectionExprs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 1], "end": [840, 2], "filename": "src/projection.rs"}, "trait": null, "trait_path": null}`

Source: `src/projection.rs:500`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

"unproject" an expression by applying this projection in reverse,
returning a new set of expressions that reference the original input
columns.

For example, consider
* an expression `c1_c2 > 5`, and a schema `[c1, c2]`
* a projection `c1 + c2 as c1_c2`

This method would rewrite the expression to `c1 + c2 > 5`
