# `datafusion_physical_expr::aggregate::LoweredAggregateBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.aggregate.LoweredAggregateBuilder.json).

<a id="op-36aba5bdd178a73ca8401d06"></a>
## LoweredAggregateBuilder

`struct` · `datafusion_physical_expr::aggregate::LoweredAggregateBuilder` · datafusion-physical-expr 55.1.0

```rust
struct LoweredAggregateBuilder<'a>
```

Source: `src/aggregate.rs:418`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Builder for converting a logical aggregate [`Expr`](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc) into physical aggregate
planning pieces.

This builder handles the logical-to-physical work needed for aggregate
planning: unwrapping aggregate aliases, choosing the output name, preserving
user-facing display text, lowering aggregate arguments, lowering the optional
filter, and lowering aggregate `ORDER BY` expressions.

<a id="op-15147961b114007774805074"></a>
## build

`function` · `datafusion_physical_expr::aggregate::LoweredAggregateBuilder::build` · datafusion-physical-expr 55.1.0

```rust
fn build(self) -> Result<LoweredAggregate>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_expr::aggregate::LoweredAggregateBuilder", "path": "LoweredAggregateBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [430, 1], "end": [574, 2], "filename": "src/aggregate.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate.rs:485`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Lower the logical aggregate expression into physical aggregate pieces.

<a id="op-788bad141e915cc3ef9530fb"></a>
## new

`function` · `datafusion_physical_expr::aggregate::LoweredAggregateBuilder::new` · datafusion-physical-expr 55.1.0

```rust
fn new(expr: &'a Expr, logical_input_schema: &'a DFSchema, physical_input_schema: &'a Schema, execution_props: &'a ExecutionProps, planning_ctx: &'a PhysicalPlanningContext) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_expr::aggregate::LoweredAggregateBuilder", "path": "LoweredAggregateBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [430, 1], "end": [574, 2], "filename": "src/aggregate.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate.rs:440`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create a builder for lowering `expr`.

`logical_input_schema` is used to resolve logical expressions such as
columns, while `physical_input_schema` is the input schema used by the
physical aggregate expression. `planning_ctx` is used when creating
physical expressions that reference uncorrelated scalar subqueries.
Callers creating physical aggregates outside of physical planning should
pass `&PhysicalPlanningContext::default()`, in which case converting a
scalar-subquery expression returns a planning error.

<a id="op-5a566773ead6cbf3dd65c3ba"></a>
## with_human_display

`function` · `datafusion_physical_expr::aggregate::LoweredAggregateBuilder::with_human_display` · datafusion-physical-expr 55.1.0

```rust
fn with_human_display(self, human_display: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_expr::aggregate::LoweredAggregateBuilder", "path": "LoweredAggregateBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [430, 1], "end": [574, 2], "filename": "src/aggregate.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate.rs:475`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Override the human-readable display text for the aggregate.

This is useful when a caller has already computed the exact display text
it wants to preserve. When this override is used, aliases with metadata
are still unwrapped for planning, but alias metadata is not copied to the
aggregate output field.

<a id="op-4ea9384e4aaaac76817050d2"></a>
## with_name

`function` · `datafusion_physical_expr::aggregate::LoweredAggregateBuilder::with_name` · datafusion-physical-expr 55.1.0

```rust
fn with_name(self, name: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_expr::aggregate::LoweredAggregateBuilder", "path": "LoweredAggregateBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [430, 1], "end": [574, 2], "filename": "src/aggregate.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate.rs:464`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Override the output column name for the aggregate.

If this is not set, the builder uses the alias from `expr` when present,
or derives the physical name from the aggregate expression.
