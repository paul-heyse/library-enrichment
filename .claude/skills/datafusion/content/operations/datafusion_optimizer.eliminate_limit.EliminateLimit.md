# `datafusion_optimizer::eliminate_limit::EliminateLimit`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.eliminate_limit.EliminateLimit.json).

<a id="op-2efab94a51182343aea05c70"></a>
## EliminateLimit

`struct` · `datafusion_optimizer::eliminate_limit::EliminateLimit` · datafusion-optimizer 55.1.0

```rust
struct EliminateLimit
```

Source: `src/eliminate_limit.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Optimizer rule to replace `LIMIT 0` or `LIMIT` whose ancestor LIMIT's skip is
greater than or equal to current's fetch

It can cooperate with `propagate_empty_relation` and `limit_push_down`. on a
plan with an empty relation.

This rule also removes OFFSET 0 from the [LogicalPlan](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-2f2092c4f87ff1cc0b33c3da)

<a id="op-62cd3d278c6bc7c85aeb29b0"></a>
## apply_order

`function` · `datafusion_optimizer::eliminate_limit::EliminateLimit::apply_order` · datafusion-optimizer 55.1.0

```rust
fn apply_order(&self) -> Option<ApplyOrder>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::eliminate_limit::EliminateLimit", "path": "EliminateLimit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [88, 2], "filename": "src/eliminate_limit.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/eliminate_limit.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c17f57329b50d409edb28d3"></a>
## default

`function` · `datafusion_optimizer::eliminate_limit::EliminateLimit::default` · datafusion-optimizer 55.1.0

```rust
fn default() -> EliminateLimit
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::eliminate_limit::EliminateLimit", "path": "EliminateLimit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 10], "end": [33, 17], "filename": "src/eliminate_limit.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/eliminate_limit.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-87af202f0350e5b0c29f2d62"></a>
## fmt

`function` · `datafusion_optimizer::eliminate_limit::EliminateLimit::fmt` · datafusion-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::eliminate_limit::EliminateLimit", "path": "EliminateLimit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 19], "end": [33, 24], "filename": "src/eliminate_limit.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/eliminate_limit.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7830f14ce3d5d379b773ccb1"></a>
## name

`function` · `datafusion_optimizer::eliminate_limit::EliminateLimit::name` · datafusion-optimizer 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::eliminate_limit::EliminateLimit", "path": "EliminateLimit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [88, 2], "filename": "src/eliminate_limit.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/eliminate_limit.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb84622d95f4249273505432"></a>
## new

`function` · `datafusion_optimizer::eliminate_limit::EliminateLimit::new` · datafusion-optimizer 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::eliminate_limit::EliminateLimit", "path": "EliminateLimit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 1], "end": [41, 2], "filename": "src/eliminate_limit.rs"}, "trait": null, "trait_path": null}`

Source: `src/eliminate_limit.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa5c3c540588a4c4ebe766d8"></a>
## rewrite

`function` · `datafusion_optimizer::eliminate_limit::EliminateLimit::rewrite` · datafusion-optimizer 55.1.0

```rust
fn rewrite(&self, plan: LogicalPlan, _config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>, datafusion_common::DataFusionError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::eliminate_limit::EliminateLimit", "path": "EliminateLimit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [88, 2], "filename": "src/eliminate_limit.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/eliminate_limit.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-22f27c0704a98945e57bc6ef"></a>
## supports_rewrite

`function` · `datafusion_optimizer::eliminate_limit::EliminateLimit::supports_rewrite` · datafusion-optimizer 55.1.0

```rust
fn supports_rewrite(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::eliminate_limit::EliminateLimit", "path": "EliminateLimit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [88, 2], "filename": "src/eliminate_limit.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/eliminate_limit.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
