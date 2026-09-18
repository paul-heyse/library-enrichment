# `datafusion_optimizer::replace_distinct_aggregate::ReplaceDistinctWithAggregate`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.replace_distinct_aggregate.ReplaceDistinctWithAggregate.json).

<a id="op-924ce5b595743633a244c2d5"></a>
## ReplaceDistinctWithAggregate

`struct` · `datafusion_optimizer::replace_distinct_aggregate::ReplaceDistinctWithAggregate` · datafusion-optimizer 55.1.0

```rust
struct ReplaceDistinctWithAggregate
```

Source: `src/replace_distinct_aggregate.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Optimizer that replaces logical [[Distinct](../operations/datafusion_expr.logical_plan.plan.Distinct.md#op-28302c630380381883e33eed)] with a logical [[Aggregate](../operations/datafusion_expr.logical_plan.plan.Aggregate.md#op-ec4cb4b20e4e0b500a880004)]

```text
SELECT DISTINCT a, b FROM tab
```

Into
```text
SELECT a, b FROM tab GROUP BY a, b
```

On the other hand, for a `DISTINCT ON` query the replacement is
a bit more involved and effectively converts
```text
SELECT DISTINCT ON (a) b FROM tab ORDER BY a DESC, c
```

into
```text
SELECT b FROM (
    SELECT a, FIRST_VALUE(b ORDER BY a DESC, c) AS b
    FROM tab
    GROUP BY a
)
ORDER BY a DESC
```

In case there are no columns, the [[Distinct](../operations/datafusion_expr.logical_plan.plan.Distinct.md#op-28302c630380381883e33eed)] is replaced by a [[Limit](../operations/datafusion_expr.logical_plan.plan.Limit.md#op-67e94f8f23b1855beec8a417)]

```text
SELECT DISTINCT * FROM empty_table
```

Into
```text
SELECT * FROM empty_table LIMIT 1
```

<a id="op-ed57c14913888a40f69f4406"></a>
## apply_order

`function` · `datafusion_optimizer::replace_distinct_aggregate::ReplaceDistinctWithAggregate::apply_order` · datafusion-optimizer 55.1.0

```rust
fn apply_order(&self) -> Option<ApplyOrder>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::replace_distinct_aggregate::ReplaceDistinctWithAggregate", "path": "ReplaceDistinctWithAggregate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [209, 2], "filename": "src/replace_distinct_aggregate.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/replace_distinct_aggregate.rs:206`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-05bde16b88fc584be113e3cb"></a>
## default

`function` · `datafusion_optimizer::replace_distinct_aggregate::ReplaceDistinctWithAggregate::default` · datafusion-optimizer 55.1.0

```rust
fn default() -> ReplaceDistinctWithAggregate
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::replace_distinct_aggregate::ReplaceDistinctWithAggregate", "path": "ReplaceDistinctWithAggregate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 10], "end": [68, 17], "filename": "src/replace_distinct_aggregate.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/replace_distinct_aggregate.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-89fab61e54e0f19f19f97dd6"></a>
## fmt

`function` · `datafusion_optimizer::replace_distinct_aggregate::ReplaceDistinctWithAggregate::fmt` · datafusion-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::replace_distinct_aggregate::ReplaceDistinctWithAggregate", "path": "ReplaceDistinctWithAggregate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 19], "end": [68, 24], "filename": "src/replace_distinct_aggregate.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/replace_distinct_aggregate.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f9f7cff364b813881d34559c"></a>
## name

`function` · `datafusion_optimizer::replace_distinct_aggregate::ReplaceDistinctWithAggregate::name` · datafusion-optimizer 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::replace_distinct_aggregate::ReplaceDistinctWithAggregate", "path": "ReplaceDistinctWithAggregate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [209, 2], "filename": "src/replace_distinct_aggregate.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/replace_distinct_aggregate.rs:202`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1f0bab1dc467180d7be95518"></a>
## new

`function` · `datafusion_optimizer::replace_distinct_aggregate::ReplaceDistinctWithAggregate::new` · datafusion-optimizer 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::replace_distinct_aggregate::ReplaceDistinctWithAggregate", "path": "ReplaceDistinctWithAggregate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 1], "end": [76, 2], "filename": "src/replace_distinct_aggregate.rs"}, "trait": null, "trait_path": null}`

Source: `src/replace_distinct_aggregate.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0496659d0f2287f8deaa7c49"></a>
## rewrite

`function` · `datafusion_optimizer::replace_distinct_aggregate::ReplaceDistinctWithAggregate::rewrite` · datafusion-optimizer 55.1.0

```rust
fn rewrite(&self, plan: LogicalPlan, config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::replace_distinct_aggregate::ReplaceDistinctWithAggregate", "path": "ReplaceDistinctWithAggregate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [209, 2], "filename": "src/replace_distinct_aggregate.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/replace_distinct_aggregate.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-59de3ad48b7dbabbfd0c1cd3"></a>
## supports_rewrite

`function` · `datafusion_optimizer::replace_distinct_aggregate::ReplaceDistinctWithAggregate::supports_rewrite` · datafusion-optimizer 55.1.0

```rust
fn supports_rewrite(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::replace_distinct_aggregate::ReplaceDistinctWithAggregate", "path": "ReplaceDistinctWithAggregate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [209, 2], "filename": "src/replace_distinct_aggregate.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/replace_distinct_aggregate.rs:79`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
