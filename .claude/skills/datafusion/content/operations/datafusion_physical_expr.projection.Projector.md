# `datafusion_physical_expr::projection::Projector`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.projection.Projector.json).

<a id="op-c9e69d382b9c8e776cb0751e"></a>
## Projector

`struct` · `datafusion_physical_expr::projection::Projector` · datafusion-physical-expr 55.1.0

```rust
struct Projector
```

Source: `src/projection.rs:919`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Applies a projection to record batches.

A [`Projector`](../operations/datafusion_physical_expr.projection.Projector.md#op-c9e69d382b9c8e776cb0751e) uses a set of projection expressions to transform
and a pre-computed output schema to project record batches accordingly.

The main reason to use a `Projector` is to avoid repeatedly computing
the output schema for each batch, which can be costly if the projection
expressions are complex.

<a id="op-07ee14759f65ac1144f959ba"></a>
## clone

`function` · `datafusion_physical_expr::projection::Projector::clone` · datafusion-physical-expr 55.1.0

```rust
fn clone(&self) -> Projector
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::Projector", "path": "Projector"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [918, 10], "end": [918, 15], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/projection.rs:918`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-37cbce30a1382ed306af17eb"></a>
## fmt

`function` · `datafusion_physical_expr::projection::Projector::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::Projector", "path": "Projector"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [918, 17], "end": [918, 22], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/projection.rs:918`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fcc9edb933e96dae1db1e1d4"></a>
## output_schema

`function` · `datafusion_physical_expr::projection::Projector::output_schema` · datafusion-physical-expr 55.1.0

```rust
fn output_schema(&self) -> &SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::Projector", "path": "Projector"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [926, 1], "end": [981, 2], "filename": "src/projection.rs"}, "trait": null, "trait_path": null}`

Source: `src/projection.rs:974`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b785c0c0fb07694ca1086aa1"></a>
## project_batch

`function` · `datafusion_physical_expr::projection::Projector::project_batch` · datafusion-physical-expr 55.1.0

```rust
fn project_batch(&self, batch: &RecordBatch) -> Result<RecordBatch>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::Projector", "path": "Projector"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [926, 1], "end": [981, 2], "filename": "src/projection.rs"}, "trait": null, "trait_path": null}`

Source: `src/projection.rs:952`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Project a record batch according to this projector's expressions.

# Errors
This function returns an error if any expression evaluation fails
or if the output schema of the resulting record batch does not match
the pre-computed output schema of the projector.

<a id="op-bf4b67c80b53ca2815906aaf"></a>
## projection

`function` · `datafusion_physical_expr::projection::Projector::projection` · datafusion-physical-expr 55.1.0

```rust
fn projection(&self) -> &ProjectionExprs
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::Projector", "path": "Projector"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [926, 1], "end": [981, 2], "filename": "src/projection.rs"}, "trait": null, "trait_path": null}`

Source: `src/projection.rs:978`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f46bdf7d6208277514f969ed"></a>
## with_metrics

`function` · `datafusion_physical_expr::projection::Projector::with_metrics` · datafusion-physical-expr 55.1.0

```rust
fn with_metrics(&self, metrics: &ExecutionPlanMetricsSet, partition: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::Projector", "path": "Projector"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [926, 1], "end": [981, 2], "filename": "src/projection.rs"}, "trait": null, "trait_path": null}`

Source: `src/projection.rs:931`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Construct the projector with metrics. After execution, related metrics will
be tracked inside `ExecutionPlanMetricsSet`

See [`ExpressionEvaluatorMetrics`](../operations/datafusion_physical_expr_common.metrics.expression.ExpressionEvaluatorMetrics.md#op-06e55f20f63c30b0ef3c8c77) for details.
