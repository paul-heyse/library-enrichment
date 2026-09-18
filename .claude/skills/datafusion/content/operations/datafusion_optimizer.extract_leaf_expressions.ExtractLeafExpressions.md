# `datafusion_optimizer::extract_leaf_expressions::ExtractLeafExpressions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.extract_leaf_expressions.ExtractLeafExpressions.json).

<a id="op-0a731b241a2648730b745694"></a>
## ExtractLeafExpressions

`struct` · `datafusion_optimizer::extract_leaf_expressions::ExtractLeafExpressions` · datafusion-optimizer 55.1.0

```rust
struct ExtractLeafExpressions
```

Source: `src/extract_leaf_expressions.rs:103`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Extracts `MoveTowardsLeafNodes` sub-expressions from non-projection nodes
into **extraction projections** (pass 1 of 2).

This handles Filter, Sort, Limit, Aggregate, and Join nodes. For Projection
nodes, extraction and pushdown are handled by [`PushDownLeafProjections`](../operations/datafusion_optimizer.extract_leaf_expressions.PushDownLeafProjections.md#op-4b118316543e73e6c579a135).

# Key Concepts

**Extraction projection**: a projection inserted *below* a node that
pre-computes a cheap expression and exposes it under an alias
(`__datafusion_extracted_N`). The parent node then references the alias
instead of the original expression.

**Recovery projection**: a projection inserted *above* a node to restore
the original output schema when extraction changes it.
Schema-preserving nodes (Filter, Sort, Limit) gain extra columns from
the extraction projection that bubble up; the recovery projection selects
only the original columns to hide the extras.

# Example

Given a filter with a struct field access:

```text
Filter: user['status'] = 'active'
  TableScan: t [id, user]
```

This rule:
1. Inserts an **extraction projection** below the filter:
2. Adds a **recovery projection** above to hide the extra column:

```text
Projection: id, user                                                        <-- recovery projection
  Filter: __datafusion_extracted_1 = 'active'
    Projection: user['status'] AS __datafusion_extracted_1, id, user         <-- extraction projection
      TableScan: t [id, user]
```

**Important:** The `PushDownFilter` rule is aware of projections created by this rule
and will not push filters through them. It uses `ExpressionPlacement` to detect
`MoveTowardsLeafNodes` expressions and skip filter pushdown past them.

<a id="op-67367e99325c4a4ada5e78b6"></a>
## default

`function` · `datafusion_optimizer::extract_leaf_expressions::ExtractLeafExpressions::default` · datafusion-optimizer 55.1.0

```rust
fn default() -> ExtractLeafExpressions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::extract_leaf_expressions::ExtractLeafExpressions", "path": "ExtractLeafExpressions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [102, 10], "end": [102, 17], "filename": "src/extract_leaf_expressions.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/extract_leaf_expressions.rs:102`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b12fd55a7e7ebe1a2e4c940e"></a>
## fmt

`function` · `datafusion_optimizer::extract_leaf_expressions::ExtractLeafExpressions::fmt` · datafusion-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::extract_leaf_expressions::ExtractLeafExpressions", "path": "ExtractLeafExpressions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [102, 19], "end": [102, 24], "filename": "src/extract_leaf_expressions.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/extract_leaf_expressions.rs:102`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7365e50aa6519f82eef2f17b"></a>
## name

`function` · `datafusion_optimizer::extract_leaf_expressions::ExtractLeafExpressions::name` · datafusion-optimizer 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::extract_leaf_expressions::ExtractLeafExpressions", "path": "ExtractLeafExpressions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 1], "end": [135, 2], "filename": "src/extract_leaf_expressions.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/extract_leaf_expressions.rs:113`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-18ff3013d53dc305f8a76fba"></a>
## new

`function` · `datafusion_optimizer::extract_leaf_expressions::ExtractLeafExpressions::new` · datafusion-optimizer 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::extract_leaf_expressions::ExtractLeafExpressions", "path": "ExtractLeafExpressions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [110, 2], "filename": "src/extract_leaf_expressions.rs"}, "trait": null, "trait_path": null}`

Source: `src/extract_leaf_expressions.rs:107`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Create a new [`ExtractLeafExpressions`](../operations/datafusion_optimizer.extract_leaf_expressions.ExtractLeafExpressions.md#op-0a731b241a2648730b745694)

<a id="op-9091545770309c5e8e5a7183"></a>
## rewrite

`function` · `datafusion_optimizer::extract_leaf_expressions::ExtractLeafExpressions::rewrite` · datafusion-optimizer 55.1.0

```rust
fn rewrite(&self, plan: LogicalPlan, config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::extract_leaf_expressions::ExtractLeafExpressions", "path": "ExtractLeafExpressions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 1], "end": [135, 2], "filename": "src/extract_leaf_expressions.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/extract_leaf_expressions.rs:117`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
