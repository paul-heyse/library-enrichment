# `datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.expressions.dynamic_filters.DynamicFilterPhysicalExpr.json).

<a id="op-2f06013c01568d705f497f61"></a>
## DynamicFilterPhysicalExpr

`struct` · `datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr` · datafusion-physical-expr 55.1.0

```rust
struct DynamicFilterPhysicalExpr
```

Source: `src/expressions/dynamic_filters/mod.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

A dynamic [`PhysicalExpr`](../operations/datafusion_physical_expr_common.physical_expr.PhysicalExpr.md#op-fe8284c43330456b0d4e6af7) that can be updated by anyone with a reference to it.

Any `ExecutionPlan` that uses this expression and holds a reference to it internally should probably also
implement `ExecutionPlan::reset_state` to remain compatible with recursive queries and other situations where
the same `ExecutionPlan` is reused with different data.

For more background, please also see the [Dynamic Filters: Passing Information Between Operators During Execution for 25x Faster Queries blog]

[Dynamic Filters: Passing Information Between Operators During Execution for 25x Faster Queries blog]: https://datafusion.apache.org/blog/2025/09/10/dynamic-filters

<a id="op-426bb92dc83a58cdf7b0a969"></a>
## children

`function` · `datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr::children` · datafusion-physical-expr 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr", "path": "DynamicFilterPhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [497, 1], "end": [639, 2], "filename": "src/expressions/dynamic_filters/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/dynamic_filters/mod.rs:498`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4b78b425b2e293eff1262d18"></a>
## current

`function` · `datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr::current` · datafusion-physical-expr 55.1.0

```rust
fn current(&self) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr", "path": "DynamicFilterPhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [185, 1], "end": [495, 2], "filename": "src/expressions/dynamic_filters/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/dynamic_filters/mod.rs:274`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Get the current expression.
This will return the current expression with any children
remapped to match calls to [`PhysicalExpr::with_new_children`](../operations/datafusion_physical_expr_common.physical_expr.PhysicalExpr.md#op-88d2e4279fdf8b0302cb6dd2).

Called per batch on the RowFilter path (via
[`PhysicalExpr::evaluate`](../operations/datafusion_physical_expr_common.physical_expr.PhysicalExpr.md#op-1fe79d89368238daf9372c3d)). The remap walk is O(tree size) and, for
dynamic filters that carry a large `InListExpr` (join key IN list),
dominated by `InListExpr::with_new_children` cloning the whole list.
The inner generation only changes when [`Self::update`](../operations/datafusion_physical_expr.expressions.dynamic_filters.DynamicFilterPhysicalExpr.md#op-23d7df76b1a24067304f22bf) fires, so we
cache the remapped expression per generation and return it directly
on subsequent per-batch calls.

<a id="op-a59e7741a890dfd540dd91ae"></a>
## data_type

`function` · `datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr::data_type` · datafusion-physical-expr 55.1.0

```rust
fn data_type(&self, input_schema: &Schema) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr", "path": "DynamicFilterPhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [497, 1], "end": [639, 2], "filename": "src/expressions/dynamic_filters/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/dynamic_filters/mod.rs:524`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3c3f52fb4327135f8dae8c17"></a>
## eq

`function` · `datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr::eq` · datafusion-physical-expr 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr", "path": "DynamicFilterPhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [165, 1], "end": [175, 2], "filename": "src/expressions/dynamic_filters/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expressions/dynamic_filters/mod.rs:166`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-53fc44b8412c54251d71392a"></a>
## evaluate

`function` · `datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr::evaluate` · datafusion-physical-expr 55.1.0

```rust
fn evaluate(&self, batch: &arrow::record_batch::RecordBatch) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr", "path": "DynamicFilterPhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [497, 1], "end": [639, 2], "filename": "src/expressions/dynamic_filters/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/dynamic_filters/mod.rs:569`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-23a2d07dfc9161df05abcaaf"></a>
## expression_id

`function` · `datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr::expression_id` · datafusion-physical-expr 55.1.0

```rust
fn expression_id(&self) -> Option<u64>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr", "path": "DynamicFilterPhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [497, 1], "end": [639, 2], "filename": "src/expressions/dynamic_filters/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/dynamic_filters/mod.rs:598`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3a86a9841e8bce9191a95e0b"></a>
## fmt

`function` · `datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr", "path": "DynamicFilterPhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [99, 1], "end": [114, 2], "filename": "src/expressions/dynamic_filters/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expressions/dynamic_filters/mod.rs:100`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aba02e0a8fba1637f9a3b6b5"></a>
## fmt

`function` · `datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr", "path": "DynamicFilterPhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 1], "end": [183, 2], "filename": "src/expressions/dynamic_filters/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/expressions/dynamic_filters/mod.rs:180`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-646816a29f1fda9d6f5d14c9"></a>
## fmt_sql

`function` · `datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr::fmt_sql` · datafusion-physical-expr 55.1.0

```rust
fn fmt_sql(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr", "path": "DynamicFilterPhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [497, 1], "end": [639, 2], "filename": "src/expressions/dynamic_filters/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/dynamic_filters/mod.rs:584`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d778e143c2bd136f8996f5c"></a>
## hash

`function` · `datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr::hash` · datafusion-physical-expr 55.1.0

```rust
fn hash<H: std::hash::Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr", "path": "DynamicFilterPhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [153, 1], "end": [163, 2], "filename": "src/expressions/dynamic_filters/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expressions/dynamic_filters/mod.rs:154`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f22936679c98aae793c5b26d"></a>
## is_used

`function` · `datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr::is_used` · datafusion-physical-expr 55.1.0

```rust
fn is_used(&Arc<self>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr", "path": "DynamicFilterPhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [185, 1], "end": [495, 2], "filename": "src/expressions/dynamic_filters/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/dynamic_filters/mod.rs:441`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Check if this dynamic filter is being actively used by any consumers.

Returns `true` if there are references beyond the producer (e.g., the HashJoinExec
that created the filter). This is useful to avoid computing expensive filter
expressions when no consumer will actually use them.

# Implementation Details

We check both Arc counts to handle two cases:
- Transformed filters (via `with_new_children`) share the inner Arc (inner count > 1)
- Direct clones (via `Arc::clone`) increment the outer count (outer count > 1)

<a id="op-40615d6cfb2210d0e9814d53"></a>
## mark_complete

`function` · `datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr::mark_complete` · datafusion-physical-expr 55.1.0

```rust
fn mark_complete(&self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr", "path": "DynamicFilterPhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [185, 1], "end": [495, 2], "filename": "src/expressions/dynamic_filters/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/dynamic_filters/mod.rs:350`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Mark this dynamic filter as complete and broadcast to all waiters.

This signals that all expected updates have been received.
Waiters using [`Self::wait_complete`](../operations/datafusion_physical_expr.expressions.dynamic_filters.DynamicFilterPhysicalExpr.md#op-a7d829183e38193511b8544d) will be notified.

<a id="op-dbfe082c30b7866688e4e516"></a>
## new

`function` · `datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr::new` · datafusion-physical-expr 55.1.0

```rust
fn new(children: Vec<Arc<dyn PhysicalExpr>>, inner: Arc<dyn PhysicalExpr>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr", "path": "DynamicFilterPhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [185, 1], "end": [495, 2], "filename": "src/expressions/dynamic_filters/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/dynamic_filters/mod.rs:213`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create a new [`DynamicFilterPhysicalExpr`](../operations/datafusion_physical_expr.expressions.dynamic_filters.DynamicFilterPhysicalExpr.md#op-2f06013c01568d705f497f61)
from an initial expression and a list of children.
The list of children is provided separately because
the initial expression may not have the same children.
For example, if the initial expression is just `true`
it will not reference any columns, but we may know that
we are going to replace this expression with a real one
that does reference certain columns.
In this case you **must** pass in the columns that will be
used in the final expression as children to this function
since DataFusion is generally not compatible with dynamic
*children* in expressions.

To determine the children you can:

- Use [`collect_columns`] to collect the columns from the expression.
- Use existing information, such as the sort columns in a `SortExec`.

Generally the important bit is that the *leaf children that reference columns
do not change* since those will be used to determine what columns need to read or projected
when evaluating the expression.

Any `ExecutionPlan` that uses this expression and holds a reference to it internally should probably also
implement `ExecutionPlan::reset_state` to remain compatible with recursive queries and other situations where
the same `ExecutionPlan` is reused with different data.

[`collect_columns`]: crate::utils::collect_columns

<a id="op-f5b9c6539f3f90ca46380f7a"></a>
## nullable

`function` · `datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr::nullable` · datafusion-physical-expr 55.1.0

```rust
fn nullable(&self, input_schema: &Schema) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr", "path": "DynamicFilterPhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [497, 1], "end": [639, 2], "filename": "src/expressions/dynamic_filters/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/dynamic_filters/mod.rs:547`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea8e9ae31e533f3ba2171df7"></a>
## snapshot

`function` · `datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr::snapshot` · datafusion-physical-expr 55.1.0

```rust
fn snapshot(&self) -> Result<Option<Arc<dyn PhysicalExpr>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr", "path": "DynamicFilterPhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [497, 1], "end": [639, 2], "filename": "src/expressions/dynamic_filters/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/dynamic_filters/mod.rs:588`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-866695f0a5cfcdad080beca9"></a>
## snapshot_generation

`function` · `datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr::snapshot_generation` · datafusion-physical-expr 55.1.0

```rust
fn snapshot_generation(&self) -> u64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr", "path": "DynamicFilterPhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [497, 1], "end": [639, 2], "filename": "src/expressions/dynamic_filters/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/dynamic_filters/mod.rs:593`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-27e18071cbe02ed0074a7b8b"></a>
## try_from_proto

`function` · `datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr::try_from_proto` · datafusion-physical-expr 55.1.0

```rust
fn try_from_proto(proto: &datafusion_proto_models::protobuf::PhysicalExprNode, ctx: &datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx<'_>) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr", "path": "DynamicFilterPhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [642, 1], "end": [706, 2], "filename": "src/expressions/dynamic_filters/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/dynamic_filters/mod.rs:648`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Reconstruct a [`DynamicFilterPhysicalExpr`](../operations/datafusion_physical_expr.expressions.dynamic_filters.DynamicFilterPhysicalExpr.md#op-2f06013c01568d705f497f61) from a proto node.

Called by the `ExprType::DynamicFilter` arm in `datafusion-proto`'s
`parse_physical_expr_with_converter`. Follows the same
`PhysicalExprDecodeCtx`-based pattern used by `Column`, `BinaryExpr`, etc.

<a id="op-0aa95c8eb5efb77f7a32d559"></a>
## try_to_proto

`function` · `datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr::try_to_proto` · datafusion-physical-expr 55.1.0

```rust
fn try_to_proto(&self, ctx: &datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalExprNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr", "path": "DynamicFilterPhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [497, 1], "end": [639, 2], "filename": "src/expressions/dynamic_filters/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/dynamic_filters/mod.rs:602`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-23d7df76b1a24067304f22bf"></a>
## update

`function` · `datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr::update` · datafusion-physical-expr 55.1.0

```rust
fn update(&self, new_expr: Arc<dyn PhysicalExpr>) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr", "path": "DynamicFilterPhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [185, 1], "end": [495, 2], "filename": "src/expressions/dynamic_filters/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/dynamic_filters/mod.rs:316`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Update the current expression and notify all waiters.
Any children of this expression must be a subset of the original children
passed to the constructor.
This should be called e.g.:
- When we've computed the probe side's hash table in a HashJoinExec
- After every batch is processed if we update the TopK heap in a SortExec using a TopK approach.

<a id="op-a7d829183e38193511b8544d"></a>
## wait_complete

`function` · `datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr::wait_complete` · datafusion-physical-expr 55.1.0

```rust
async fn wait_complete(&self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr", "path": "DynamicFilterPhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [185, 1], "end": [495, 2], "filename": "src/expressions/dynamic_filters/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/dynamic_filters/mod.rs:390`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Wait asynchronously until this dynamic filter is marked as complete.

This method returns immediately if the filter is already complete.
Otherwise, it waits until [`Self::mark_complete`](../operations/datafusion_physical_expr.expressions.dynamic_filters.DynamicFilterPhysicalExpr.md#op-40615d6cfb2210d0e9814d53) is called.

Unlike [`Self::wait_update`](../operations/datafusion_physical_expr.expressions.dynamic_filters.DynamicFilterPhysicalExpr.md#op-032d74064fa03ebd011acfe7), this method guarantees that when it returns,
the filter is fully complete with no more updates expected.

Producers (e.g.) HashJoinExec may never update the expression or mark it as completed if there are no consumers.
If you call this method on a dynamic filter created by such a producer and there are no consumers registered this method would wait indefinitely.
This should not happen under normal operation and would indicate a programming error either in your producer or in DataFusion if the producer is a built in node.

<a id="op-032d74064fa03ebd011acfe7"></a>
## wait_update

`function` · `datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr::wait_update` · datafusion-physical-expr 55.1.0

```rust
async fn wait_update(&self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr", "path": "DynamicFilterPhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [185, 1], "end": [495, 2], "filename": "src/expressions/dynamic_filters/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/dynamic_filters/mod.rs:370`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Wait asynchronously for any update to this filter.

This method will return when [`Self::update`](../operations/datafusion_physical_expr.expressions.dynamic_filters.DynamicFilterPhysicalExpr.md#op-23d7df76b1a24067304f22bf) is called and the generation increases.
It does not guarantee that the filter is complete.

Producers (e.g.) HashJoinExec may never update the expression or mark it as completed if there are no consumers.
If you call this method on a dynamic filter created by such a producer and there are no consumers registered this method would wait indefinitely.
This should not happen under normal operation and would indicate a programming error either in your producer or in DataFusion if the producer is a built in node.

<a id="op-5b68e432b545ee4b5cee2b94"></a>
## with_new_children

`function` · `datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr::with_new_children` · datafusion-physical-expr 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr", "path": "DynamicFilterPhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [497, 1], "end": [639, 2], "filename": "src/expressions/dynamic_filters/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/dynamic_filters/mod.rs:506`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
