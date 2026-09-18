# `datafusion_physical_optimizer::projection_pushdown::ProjectionPushdown`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.projection_pushdown.ProjectionPushdown.json).

<a id="op-127d78f8ec509253915e06a7"></a>
## ProjectionPushdown

`struct` · `datafusion_physical_optimizer::projection_pushdown::ProjectionPushdown` · datafusion-physical-optimizer 55.1.0

```rust
struct ProjectionPushdown
```

Source: `src/projection_pushdown.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

This rule inspects `ProjectionExec`'s in the given physical plan and tries to
remove or swap with its child.

Furthermore, tries to push down projections from nested loop join filters that only depend on
one side of the join. By pushing these projections down, functions that only depend on one side
of the join must be evaluated for the cartesian product of the two sides.

<a id="op-c4300a688feb026b68a161e4"></a>
## default

`function` · `datafusion_physical_optimizer::projection_pushdown::ProjectionPushdown::default` · datafusion-physical-optimizer 55.1.0

```rust
fn default() -> ProjectionPushdown
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::projection_pushdown::ProjectionPushdown", "path": "ProjectionPushdown"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 10], "end": [49, 17], "filename": "src/projection_pushdown.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/projection_pushdown.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-05f4cc2d2f9267ceddd52cd4"></a>
## fmt

`function` · `datafusion_physical_optimizer::projection_pushdown::ProjectionPushdown::fmt` · datafusion-physical-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::projection_pushdown::ProjectionPushdown", "path": "ProjectionPushdown"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 19], "end": [49, 24], "filename": "src/projection_pushdown.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/projection_pushdown.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1f98c5115f743585e948a00c"></a>
## name

`function` · `datafusion_physical_optimizer::projection_pushdown::ProjectionPushdown::name` · datafusion-physical-optimizer 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::projection_pushdown::ProjectionPushdown", "path": "ProjectionPushdown"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [87, 2], "filename": "src/projection_pushdown.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/projection_pushdown.rs:80`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-22df586009f70f347d4e254c"></a>
## new

`function` · `datafusion_physical_optimizer::projection_pushdown::ProjectionPushdown::new` · datafusion-physical-optimizer 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::projection_pushdown::ProjectionPushdown", "path": "ProjectionPushdown"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [57, 2], "filename": "src/projection_pushdown.rs"}, "trait": null, "trait_path": null}`

Source: `src/projection_pushdown.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-696dc108e4cedf882a2f8750"></a>
## optimize

`function` · `datafusion_physical_optimizer::projection_pushdown::ProjectionPushdown::optimize` · datafusion-physical-optimizer 55.1.0

```rust
fn optimize(&self, plan: Arc<dyn ExecutionPlan>, _config: &ConfigOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::projection_pushdown::ProjectionPushdown", "path": "ProjectionPushdown"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [87, 2], "filename": "src/projection_pushdown.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/projection_pushdown.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de434806f4e176ac8ab27f1c"></a>
## schema_check

`function` · `datafusion_physical_optimizer::projection_pushdown::ProjectionPushdown::schema_check` · datafusion-physical-optimizer 55.1.0

```rust
fn schema_check(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::projection_pushdown::ProjectionPushdown", "path": "ProjectionPushdown"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [87, 2], "filename": "src/projection_pushdown.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/projection_pushdown.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
