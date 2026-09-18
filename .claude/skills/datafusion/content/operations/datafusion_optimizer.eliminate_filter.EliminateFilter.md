# `datafusion_optimizer::eliminate_filter::EliminateFilter`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.eliminate_filter.EliminateFilter.json).

<a id="op-bdb24402e21ac82bef3d5ee9"></a>
## EliminateFilter

`struct` · `datafusion_optimizer::eliminate_filter::EliminateFilter` · datafusion-optimizer 55.1.0

```rust
struct EliminateFilter
```

Source: `src/eliminate_filter.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Optimization rule that eliminate the scalar value (true/false/null) filter
with an [LogicalPlan::EmptyRelation](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-51e2a0bf1028cc782c1a5679)

This saves time in planning and executing the query.
Note that this rule should be applied after simplify expressions optimizer rule.

<a id="op-10c78d1997527de807eac9fd"></a>
## apply_order

`function` · `datafusion_optimizer::eliminate_filter::EliminateFilter::apply_order` · datafusion-optimizer 55.1.0

```rust
fn apply_order(&self) -> Option<ApplyOrder>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::eliminate_filter::EliminateFilter", "path": "EliminateFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [78, 2], "filename": "src/eliminate_filter.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/eliminate_filter.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1240542ff4f026ee63b48058"></a>
## default

`function` · `datafusion_optimizer::eliminate_filter::EliminateFilter::default` · datafusion-optimizer 55.1.0

```rust
fn default() -> EliminateFilter
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::eliminate_filter::EliminateFilter", "path": "EliminateFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 10], "end": [33, 17], "filename": "src/eliminate_filter.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/eliminate_filter.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9c565bc59424cea48b258361"></a>
## fmt

`function` · `datafusion_optimizer::eliminate_filter::EliminateFilter::fmt` · datafusion-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::eliminate_filter::EliminateFilter", "path": "EliminateFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 19], "end": [33, 24], "filename": "src/eliminate_filter.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/eliminate_filter.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bfa45054eef57e7f7f02c7bc"></a>
## name

`function` · `datafusion_optimizer::eliminate_filter::EliminateFilter::name` · datafusion-optimizer 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::eliminate_filter::EliminateFilter", "path": "EliminateFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [78, 2], "filename": "src/eliminate_filter.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/eliminate_filter.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-081c4418d2a25e22fc4e1b2a"></a>
## new

`function` · `datafusion_optimizer::eliminate_filter::EliminateFilter::new` · datafusion-optimizer 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::eliminate_filter::EliminateFilter", "path": "EliminateFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 1], "end": [41, 2], "filename": "src/eliminate_filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/eliminate_filter.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-55229f819110ad1cb8d3dfad"></a>
## rewrite

`function` · `datafusion_optimizer::eliminate_filter::EliminateFilter::rewrite` · datafusion-optimizer 55.1.0

```rust
fn rewrite(&self, plan: LogicalPlan, _config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::eliminate_filter::EliminateFilter", "path": "EliminateFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [78, 2], "filename": "src/eliminate_filter.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/eliminate_filter.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-62009ddf0ba6e51fa66a7602"></a>
## supports_rewrite

`function` · `datafusion_optimizer::eliminate_filter::EliminateFilter::supports_rewrite` · datafusion-optimizer 55.1.0

```rust
fn supports_rewrite(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::eliminate_filter::EliminateFilter", "path": "EliminateFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [78, 2], "filename": "src/eliminate_filter.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/eliminate_filter.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
