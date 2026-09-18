# `datafusion_optimizer::extract_leaf_expressions::PushDownLeafProjections`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.extract_leaf_expressions.PushDownLeafProjections.json).

<a id="op-4b118316543e73e6c579a135"></a>
## PushDownLeafProjections

`struct` · `datafusion_optimizer::extract_leaf_expressions::PushDownLeafProjections` · datafusion-optimizer 55.1.0

```rust
struct PushDownLeafProjections
```

Source: `src/extract_leaf_expressions.rs:713`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Pushes extraction projections down through schema-preserving nodes towards
leaf nodes (pass 2 of 2, after [`ExtractLeafExpressions`](../operations/datafusion_optimizer.extract_leaf_expressions.ExtractLeafExpressions.md#op-0a731b241a2648730b745694)).

Handles two types of projections:
- **Pure extraction projections** (all `__datafusion_extracted` aliases + columns):
  pushes through Filter/Sort/Limit, merges into existing projections, or routes
  into multi-input node inputs (Join, SubqueryAlias, etc.)
- **Mixed projections** (user projections containing `MoveTowardsLeafNodes`
  sub-expressions): splits into a recovery projection + extraction projection,
  then pushes the extraction projection down.

# Example: Pushing through a Filter

After pass 1, the extraction projection sits directly below the filter:
```text
Projection: id, user                                                              <-- recovery
  Filter: __datafusion_extracted_1 = 'active'
    Projection: user['status'] AS __datafusion_extracted_1, id, user               <-- extraction
      TableScan: t [id, user]
```

Pass 2 pushes the extraction projection through the recovery and filter,
and a subsequent `OptimizeProjections` pass removes the (now-redundant)
recovery projection:
```text
Filter: __datafusion_extracted_1 = 'active'
  Projection: user['status'] AS __datafusion_extracted_1, id, user                 <-- extraction (pushed down)
    TableScan: t [id, user]
```

<a id="op-0be68486a36b6dfa289bc530"></a>
## apply_order

`function` · `datafusion_optimizer::extract_leaf_expressions::PushDownLeafProjections::apply_order` · datafusion-optimizer 55.1.0

```rust
fn apply_order(&self) -> Option<ApplyOrder>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::extract_leaf_expressions::PushDownLeafProjections", "path": "PushDownLeafProjections"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [721, 1], "end": [744, 2], "filename": "src/extract_leaf_expressions.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/extract_leaf_expressions.rs:726`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e3b605e4e70a875d9c77a243"></a>
## default

`function` · `datafusion_optimizer::extract_leaf_expressions::PushDownLeafProjections::default` · datafusion-optimizer 55.1.0

```rust
fn default() -> PushDownLeafProjections
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::extract_leaf_expressions::PushDownLeafProjections", "path": "PushDownLeafProjections"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [712, 10], "end": [712, 17], "filename": "src/extract_leaf_expressions.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/extract_leaf_expressions.rs:712`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-89d295ee19c33bb7d1639f67"></a>
## fmt

`function` · `datafusion_optimizer::extract_leaf_expressions::PushDownLeafProjections::fmt` · datafusion-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::extract_leaf_expressions::PushDownLeafProjections", "path": "PushDownLeafProjections"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [712, 19], "end": [712, 24], "filename": "src/extract_leaf_expressions.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/extract_leaf_expressions.rs:712`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bf791139edd8840b502e942b"></a>
## name

`function` · `datafusion_optimizer::extract_leaf_expressions::PushDownLeafProjections::name` · datafusion-optimizer 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::extract_leaf_expressions::PushDownLeafProjections", "path": "PushDownLeafProjections"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [721, 1], "end": [744, 2], "filename": "src/extract_leaf_expressions.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/extract_leaf_expressions.rs:722`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-94360002bceeb3b8f81e19a8"></a>
## new

`function` · `datafusion_optimizer::extract_leaf_expressions::PushDownLeafProjections::new` · datafusion-optimizer 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::extract_leaf_expressions::PushDownLeafProjections", "path": "PushDownLeafProjections"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [715, 1], "end": [719, 2], "filename": "src/extract_leaf_expressions.rs"}, "trait": null, "trait_path": null}`

Source: `src/extract_leaf_expressions.rs:716`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b46711a8de646755a9d32cc2"></a>
## rewrite

`function` · `datafusion_optimizer::extract_leaf_expressions::PushDownLeafProjections::rewrite` · datafusion-optimizer 55.1.0

```rust
fn rewrite(&self, plan: LogicalPlan, config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::extract_leaf_expressions::PushDownLeafProjections", "path": "PushDownLeafProjections"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [721, 1], "end": [744, 2], "filename": "src/extract_leaf_expressions.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/extract_leaf_expressions.rs:730`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
