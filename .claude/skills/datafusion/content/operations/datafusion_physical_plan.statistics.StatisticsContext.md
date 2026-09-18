# `datafusion_physical_plan::statistics::StatisticsContext`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.statistics.StatisticsContext.json).

<a id="op-37580381f9423ac5020b296f"></a>
## StatisticsContext

`struct` · `datafusion_physical_plan::statistics::StatisticsContext` · datafusion-physical-plan 55.1.0

```rust
struct StatisticsContext
```

Source: `src/statistics.rs:121`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Owns the bottom-up traversal and per-walk memoization cache for statistics
computation. Call [`StatisticsContext::compute`](../operations/datafusion_physical_plan.statistics.StatisticsContext.md#op-eb410103938f7a575227fe03) to walk a plan tree.

<a id="op-eb410103938f7a575227fe03"></a>
## compute

`function` · `datafusion_physical_plan::statistics::StatisticsContext::compute` · datafusion-physical-plan 55.1.0

```rust
fn compute(&self, plan: &dyn ExecutionPlan, args: &StatisticsArgs) -> Result<Arc<Statistics>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::statistics::StatisticsContext", "path": "StatisticsContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [131, 1], "end": [204, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:154`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Computes statistics for `plan`, resolving children first and passing
the results to [`ExecutionPlan::statistics_from_inputs`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-7dad80ba3b610abc3c532d8a).

When `args.partition()` is `Some(idx)`, `idx` is validated against the
plan's partition count.

<a id="op-55721dca1f7fb6eda3fa8d0d"></a>
## default

`function` · `datafusion_physical_plan::statistics::StatisticsContext::default` · datafusion-physical-plan 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::statistics::StatisticsContext", "path": "StatisticsContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [125, 1], "end": [129, 2], "filename": "src/statistics.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/statistics.rs:126`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f276a411a2c292ba97afbd4c"></a>
## new

`function` · `datafusion_physical_plan::statistics::StatisticsContext::new` · datafusion-physical-plan 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::statistics::StatisticsContext", "path": "StatisticsContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [131, 1], "end": [204, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:133`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Creates a context with an empty cache.

<a id="op-509ae904f9264c46a37574c3"></a>
## reset_cache

`function` · `datafusion_physical_plan::statistics::StatisticsContext::reset_cache` · datafusion-physical-plan 55.1.0

```rust
fn reset_cache(&self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::statistics::StatisticsContext", "path": "StatisticsContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [131, 1], "end": [204, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:145`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Clears the memoization cache.

The cache is keyed by raw plan-node pointers, which are only stable
while the current plan tree is alive. Reset between optimizer passes
(which rewrite the plan) when reusing one context across them, so stale
pointer keys cannot collide.
