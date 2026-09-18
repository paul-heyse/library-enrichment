# `datafusion_physical_optimizer::limit_pushdown::LimitPushdown`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.limit_pushdown.LimitPushdown.json).

<a id="op-5e4f8ca3ba0b51aa7a5956ca"></a>
## LimitPushdown

`struct` · `datafusion_physical_optimizer::limit_pushdown::LimitPushdown` · datafusion-physical-optimizer 55.1.0

```rust
struct LimitPushdown
```

Source: `src/limit_pushdown.rs:85`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

This rule inspects [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673)'s and pushes down the fetch limit from
the parent to the child if applicable.

<a id="op-e6571b6cabdcfc2f1f0504be"></a>
## default

`function` · `datafusion_physical_optimizer::limit_pushdown::LimitPushdown::default` · datafusion-physical-optimizer 55.1.0

```rust
fn default() -> LimitPushdown
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::limit_pushdown::LimitPushdown", "path": "LimitPushdown"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 10], "end": [84, 17], "filename": "src/limit_pushdown.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/limit_pushdown.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff3e23a3546bbc0574925852"></a>
## fmt

`function` · `datafusion_physical_optimizer::limit_pushdown::LimitPushdown::fmt` · datafusion-physical-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::limit_pushdown::LimitPushdown", "path": "LimitPushdown"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 19], "end": [84, 24], "filename": "src/limit_pushdown.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/limit_pushdown.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e0c50f43b2b9bab4cc64c69c"></a>
## name

`function` · `datafusion_physical_optimizer::limit_pushdown::LimitPushdown::name` · datafusion-physical-optimizer 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::limit_pushdown::LimitPushdown", "path": "LimitPushdown"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 1], "end": [132, 2], "filename": "src/limit_pushdown.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/limit_pushdown.rs:125`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3686e7d359ed0afb176a1a6d"></a>
## new

`function` · `datafusion_physical_optimizer::limit_pushdown::LimitPushdown::new` · datafusion-physical-optimizer 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::limit_pushdown::LimitPushdown", "path": "LimitPushdown"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [103, 1], "end": [108, 2], "filename": "src/limit_pushdown.rs"}, "trait": null, "trait_path": null}`

Source: `src/limit_pushdown.rs:105`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef3d794eabb7ca22fb812d7c"></a>
## optimize

`function` · `datafusion_physical_optimizer::limit_pushdown::LimitPushdown::optimize` · datafusion-physical-optimizer 55.1.0

```rust
fn optimize(&self, plan: Arc<dyn ExecutionPlan>, _config: &ConfigOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::limit_pushdown::LimitPushdown", "path": "LimitPushdown"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 1], "end": [132, 2], "filename": "src/limit_pushdown.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/limit_pushdown.rs:111`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-19ae61e629734554749f1649"></a>
## schema_check

`function` · `datafusion_physical_optimizer::limit_pushdown::LimitPushdown::schema_check` · datafusion-physical-optimizer 55.1.0

```rust
fn schema_check(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::limit_pushdown::LimitPushdown", "path": "LimitPushdown"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 1], "end": [132, 2], "filename": "src/limit_pushdown.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/limit_pushdown.rs:129`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
