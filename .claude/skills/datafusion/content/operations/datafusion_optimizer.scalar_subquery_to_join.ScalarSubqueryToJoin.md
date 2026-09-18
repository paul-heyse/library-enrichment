# `datafusion_optimizer::scalar_subquery_to_join::ScalarSubqueryToJoin`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.scalar_subquery_to_join.ScalarSubqueryToJoin.json).

<a id="op-10401da64a1a420623da8f8a"></a>
## ScalarSubqueryToJoin

`struct` · `datafusion_optimizer::scalar_subquery_to_join::ScalarSubqueryToJoin` · datafusion-optimizer 55.1.0

```rust
struct ScalarSubqueryToJoin
```

Source: `src/scalar_subquery_to_join.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Optimizer rule that rewrites scalar subquery filters to joins and places an
additional projection on top of the filter to preserve the original schema.

When [`datafusion_common::config::OptimizerOptions::enable_physical_uncorrelated_scalar_subquery`](../operations/datafusion_common.config.OptimizerOptions.md#op-7a0e28f70e7ec25e550f66d6) is
true (the default), only *correlated* scalar subqueries are rewritten here;
uncorrelated ones are left for physical execution via `ScalarSubqueryExec`.
When the option is false, all scalar subqueries — correlated and
uncorrelated — are rewritten to left joins by this rule.

<a id="op-1fa5f6b0c86d3a56da0cbe53"></a>
## apply_order

`function` · `datafusion_optimizer::scalar_subquery_to_join::ScalarSubqueryToJoin::apply_order` · datafusion-optimizer 55.1.0

```rust
fn apply_order(&self) -> Option<ApplyOrder>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::scalar_subquery_to_join::ScalarSubqueryToJoin", "path": "ScalarSubqueryToJoin"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [249, 2], "filename": "src/scalar_subquery_to_join.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/scalar_subquery_to_join.rs:246`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e93479b9e3181f347b0a02f"></a>
## default

`function` · `datafusion_optimizer::scalar_subquery_to_join::ScalarSubqueryToJoin::default` · datafusion-optimizer 55.1.0

```rust
fn default() -> ScalarSubqueryToJoin
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::scalar_subquery_to_join::ScalarSubqueryToJoin", "path": "ScalarSubqueryToJoin"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 10], "end": [47, 17], "filename": "src/scalar_subquery_to_join.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/scalar_subquery_to_join.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-96689a0ddd560eccd0b5e64c"></a>
## fmt

`function` · `datafusion_optimizer::scalar_subquery_to_join::ScalarSubqueryToJoin::fmt` · datafusion-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::scalar_subquery_to_join::ScalarSubqueryToJoin", "path": "ScalarSubqueryToJoin"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 19], "end": [47, 24], "filename": "src/scalar_subquery_to_join.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/scalar_subquery_to_join.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af5463f6b716a9238b8cad6b"></a>
## name

`function` · `datafusion_optimizer::scalar_subquery_to_join::ScalarSubqueryToJoin::name` · datafusion-optimizer 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::scalar_subquery_to_join::ScalarSubqueryToJoin", "path": "ScalarSubqueryToJoin"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [249, 2], "filename": "src/scalar_subquery_to_join.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/scalar_subquery_to_join.rs:242`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7181beb44ca4bd75dbb025c8"></a>
## new

`function` · `datafusion_optimizer::scalar_subquery_to_join::ScalarSubqueryToJoin::new` · datafusion-optimizer 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::scalar_subquery_to_join::ScalarSubqueryToJoin", "path": "ScalarSubqueryToJoin"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [84, 2], "filename": "src/scalar_subquery_to_join.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar_subquery_to_join.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-99cee2085038a927ad238dbd"></a>
## rewrite

`function` · `datafusion_optimizer::scalar_subquery_to_join::ScalarSubqueryToJoin::rewrite` · datafusion-optimizer 55.1.0

```rust
fn rewrite(&self, plan: LogicalPlan, config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::scalar_subquery_to_join::ScalarSubqueryToJoin", "path": "ScalarSubqueryToJoin"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [249, 2], "filename": "src/scalar_subquery_to_join.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/scalar_subquery_to_join.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f9ac906c1a10f5d618f43bc4"></a>
## supports_rewrite

`function` · `datafusion_optimizer::scalar_subquery_to_join::ScalarSubqueryToJoin::supports_rewrite` · datafusion-optimizer 55.1.0

```rust
fn supports_rewrite(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::scalar_subquery_to_join::ScalarSubqueryToJoin", "path": "ScalarSubqueryToJoin"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [249, 2], "filename": "src/scalar_subquery_to_join.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/scalar_subquery_to_join.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
